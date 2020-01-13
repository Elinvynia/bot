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
mod events;
use events::Handler;

use dotenv::dotenv;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use serenity::{
    framework::StandardFramework,
    prelude::*,
};

fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Token not found in environment");
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    let pool = Pool::builder()
        .build(ConnectionManager::<SqliteConnection>::new(
            "db/db.sqlite3",
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
