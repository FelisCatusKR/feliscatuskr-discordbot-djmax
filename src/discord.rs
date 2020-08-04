pub mod general;
pub mod djmax;

use serenity::{
    client::Client,
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            group,
            command,
        },
    },
    prelude::*,
};

use crate::discord::general::PING_COMMAND;
use crate::discord::djmax::{
    SEARCH_BY_TITLE_COMMAND,
    SEARCH_BY_4B_LEVEL_COMMAND,
    SEARCH_BY_5B_LEVEL_COMMAND,
    SEARCH_BY_6B_LEVEL_COMMAND,
    SEARCH_BY_8B_LEVEL_COMMAND,
};

#[group]
#[commands(ping)]
struct General;

#[group]
#[prefixes("djmax", "d", "디제이맥스", "디맥")]
#[default_command(search_by_title)]
#[commands(search_by_title, search_by_4b_level, search_by_5b_level, search_by_6b_level, search_by_8b_level)]
struct Djmax;

struct Handler;

impl EventHandler for Handler {}

pub fn establish_client(token: &String) -> Client {
    let mut client = Client::new(token, Handler).expect("Err creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP)
        .group(&DJMAX_GROUP)
    );

    client
}
