mod commands;
use commands::help::*;
use commands::*;

mod data;
use data::*;

mod util;
use util::*;

mod db;
use db::*;

mod listeners;
use listeners::Handler;

use serenity::{framework::StandardFramework, prelude::*};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};

use dotenv::dotenv;

fn main() {
    dotenv().ok();
    env_logger::init();
    create_db();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found in environment");
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    let (owners, botid, ownerid) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id, info.owner.id)
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
            .configure(|c| {
                c.owners(owners)
                    .on_mention(Some(botid))
                    .dynamic_prefix(|ctx, msg| {
                        if msg.is_private() {
                            return Some("!".to_string());
                        }

                        if msg.guild_id.is_none() {
                            return Some("!".to_string());
                        }

                        let guildid = msg.guild_id.unwrap();

                        {
                            let data = ctx.data.read();
                            let prefixes = data.get::<Prefix>().unwrap();
                            if let Some(x) = prefixes.get(&guildid) {
                                return Some(x.to_string());
                            }
                        }

                        return Some(
                            get_prefix(&guildid, &ctx)
                                .map_or_else(|_| "!".to_string(), |pref| pref),
                        );
                    })
            })
            .on_dispatch_error(dispatch_error)
            .after(after)
            .normal_message(log_dm)
            .help(&HELP)
            .group(&CONFIG_GROUP)
            .group(&ADMIN_GROUP)
            .group(&FUN_GROUP)
            .group(&GENERAL_GROUP),
    );

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<BotId>(botid);
        let x = vec![ownerid];
        data.insert::<BotOwners>(x);
        let map = HashMap::new();
        data.insert::<Prefix>(map);
    }

    client.start_autosharded().unwrap()
}
