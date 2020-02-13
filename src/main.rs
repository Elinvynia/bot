mod commands;
use commands::help::*;
use commands::*;

mod data;
use data::cache::{BotId, BotOwners, DefaultPrefix, GuildPrefixes, ShardManagerContainer};

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

fn main() {
    //Start logging as soon as possible.
    env_logger::init();

    //Load the dotenv for ease of development.
    dotenv::dotenv().ok();

    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config"))
        .expect("Failed to open the config file.");

    create_db();

    //If a token exists in the environment, prefer to use that.
    let token;
    if let Ok(x) = env::var("DISCORD_TOKEN") {
        token = x;
    } else {
        token = settings
            .get_str("discord_token")
            .expect("discord_token not found in settings.");
    }

    //Create the Discord client.
    let mut client = Client::new(&token, Handler).expect("Error creating the client.");

    //Get the application info to use for later.
    let (owners, botid, ownerid) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id, info.owner.id)
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    //Set the cache for each channel to 100 messages.
    client
        .cache_and_http
        .cache
        .write()
        .settings_mut()
        .max_messages(100);

    //Configure the default framework and command groups.
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

    //Fill the data with some previously gathered and default values.
    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<BotId>(botid);
        let x = vec![ownerid];
        data.insert::<BotOwners>(x);
        data.insert::<DefaultPrefix>(
            settings
                .get_str("default_prefix")
                .expect("default_prefix not found in settings."),
        );
        let map = HashMap::new();
        data.insert::<GuildPrefixes>(map);
    }

    //Start the client, autosharded.
    client
        .start_autosharded()
        .expect("Failed to start the client.")
}
