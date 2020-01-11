mod commands;
use commands::*;

mod data;
use data::*;

mod functions;
use functions::*;

#[macro_use]
extern crate diesel;

use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;

mod db;

use dotenv::dotenv;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use serenity::{
    framework::StandardFramework,
    model::{gateway::Ready, prelude::*},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    fn message_delete(&self, ctx: Context, channel: ChannelId, deleted_message_id: MessageId) {
        let guildid = channel
            .to_channel(&ctx)
            .unwrap()
            .guild()
            .unwrap()
            .read()
            .guild_id;
        if !check_log_channel(&ctx, &guildid) {
            println!("no log channel");
            return;
        }

        let log_channel = get_log_channel(&ctx, &guildid);

        println!("got log channel");

        if let Some(x) = ctx.cache.read().message(&channel, &deleted_message_id) {
            let _ = log_channel.say(
                &ctx.http,
                format!(
                    "Message by {} deleted in channel {}:\n{}",
                    x.author,
                    x.channel(&ctx.cache).unwrap(),
                    x.content
                ),
            );
        }
    }
}

fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Token not found in environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    let pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(
            "../db/db.sqlite3",
        ))
        .unwrap();

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            let mut data = client.data.write();
            data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
            data.insert::<BotId>(info.id);
            data.insert::<DatabaseConnection>(pool);
            let x = vec![info.owner.id];
            data.insert::<BotOwners>(x);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client
        .cache_and_http
        .cache
        .write()
        .settings_mut()
        .max_messages(100);

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.owners(owners).prefix("!"))
            .normal_message(log_dm)
            .group(&CONFIG_GROUP)
            .group(&ADMIN_GROUP)
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
