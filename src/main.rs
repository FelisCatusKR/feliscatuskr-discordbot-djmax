#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;
mod discord;

use std::error::Error;
use std::fs::File;
use std::env;

use dotenv::dotenv;

use crate::db::{establish_connection, models::Song};

fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let file_path = env::var("CSV_FILE_NAME")
        .expect("CSV_FILE_NAME must be set");
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let conn = establish_connection();

    for result in rdr.deserialize() {
        let song: Song = result?;
        Song::create_or_update(&song, &conn);
    }

    // start listening for events by starting a single shard
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let mut client = discord::establish_client(&token);
    client.start()?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        std::process::exit(1);
    }
}