use std::error::Error;
use std::fs::File;
use std::process;

use dotenv;
use serde::Deserialize;

// type Record = (u64, String, String, Option<f64>, f64, String, Option<String>, Option<u64>, Option<u64>, Option<u64>, Option<u64>);
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Song {
    id: u64,
    title: String,
    artist: String,
    min_bpm: Option<f64>,
    max_bpm: f64,
    category: String,
    dlc: Option<String>,
    #[serde(rename = "4b0")]
    four_button_0: Option<u64>,
    #[serde(rename = "4b1")]
    four_button_1: Option<u64>,
    #[serde(rename = "4b2")]
    four_button_2: Option<u64>,
    #[serde(rename = "4b3")]
    four_button_3: Option<u64>,
}

fn run() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let key = "CSV_FILE_NAME";
    let file_path = dotenv::var(key)?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let song: Song = result?;
        println!("{:?}", song);
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}