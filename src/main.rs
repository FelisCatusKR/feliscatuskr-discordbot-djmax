#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod db;

use std::error::Error;
use std::fs::File;
use std::process;
use std::env;

use dotenv::dotenv;

use crate::db::{establish_connection, models::Song};

fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let file_path = env::var("CSV_FILE_NAME").expect("CSV_FILE_NAME must be set");
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let conn = establish_connection();

    for result in rdr.deserialize() {
        let song: Song = result?;
        Song::create_or_update(&song, &conn);
    }

    for result in Song::by_4b_level(14, &conn) {
        println!("{:?}", result);
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}