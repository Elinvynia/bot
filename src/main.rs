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
    sync::Arc,
    env
};

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config")).unwrap()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .merge(config::Environment::with_prefix("APP")).unwrap();
    
    dotenv::dotenv().ok();
    env_logger::init();
    create_db();

    let token;
    if let Ok(x) = env::var("DISCORD_TOKEN") {
        token = x;
    }
    else {
        token = settings.get_str("discord_token").expect("discord_token not found in settings.");
    }
    let mut client = Client::new(&token, Handler).expect("Error creating client.");

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
                    .dynamic_prefix(dynamic_prefix)
            })
            .on_dispatch_error(dispatch_error)
            .after(after)
            .normal_message(log_dm)
            .help(&HELP)
            .group(&SETTINGS_GROUP)
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
        data.insert::<DefaultPrefix>(settings.get_str("default_prefix").expect("default_prefix not found in settings."));
        let map = HashMap::new();
        data.insert::<GuildPrefixes>(map);
    }

    client.start_autosharded().unwrap()
}
