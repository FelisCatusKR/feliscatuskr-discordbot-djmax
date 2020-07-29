use serde::Deserialize;
use diesel::prelude::*;

use super::schema::songs;
use super::schema::songs::dsl::songs as song_dsl;

#[derive(Debug, Deserialize, Queryable, Insertable, AsChangeset)]
#[serde(rename_all(deserialize = "camelCase"))]
#[table_name = "songs"]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub min_bpm: Option<f64>,
    pub max_bpm: f64,
    pub category: String,
    pub dlc: Option<String>,
    #[serde(rename(deserialize = "4b0"))]
    pub four_button_0: i32,
    #[serde(rename(deserialize = "4b1"))]
    pub four_button_1: Option<i32>,
    #[serde(rename(deserialize = "4b2"))]
    pub four_button_2: Option<i32>,
    #[serde(rename(deserialize = "4b3"))]
    pub four_button_3: Option<i32>,
    #[serde(rename(deserialize = "5b0"))]
    pub five_button_0: i32,
    #[serde(rename(deserialize = "5b1"))]
    pub five_button_1: Option<i32>,
    #[serde(rename(deserialize = "5b2"))]
    pub five_button_2: Option<i32>,
    #[serde(rename(deserialize = "5b3"))]
    pub five_button_3: Option<i32>,
    #[serde(rename(deserialize = "6b0"))]
    pub six_button_0: i32,
    #[serde(rename(deserialize = "6b1"))]
    pub six_button_1: Option<i32>,
    #[serde(rename(deserialize = "6b2"))]
    pub six_button_2: Option<i32>,
    #[serde(rename(deserialize = "6b3"))]
    pub six_button_3: Option<i32>,
    #[serde(rename(deserialize = "8b0"))]
    pub eight_button_0: i32,
    #[serde(rename(deserialize = "8b1"))]
    pub eight_button_1: Option<i32>,
    #[serde(rename(deserialize = "8b2"))]
    pub eight_button_2: Option<i32>,
    #[serde(rename(deserialize = "8b3"))]
    pub eight_button_3: Option<i32>,
}

impl Song {
    pub fn by_id(id: i32, conn: &SqliteConnection) -> Option<Self> {
        if let Ok(record) = song_dsl.find(id).get_result::<Song>(conn) {
            Some(record)
        } else { None }
    }

    pub fn by_title(title_str: &str, conn: &SqliteConnection) -> Vec<Self> {
        use super::schema::songs::dsl::title;

        song_dsl
            .filter(title.like(title_str))
            .load::<Song>(conn)
            .expect("Error loading songs")
    }

    pub fn by_4b_level(level: i32, conn: &SqliteConnection) -> Vec<Self> {
        use super::schema::songs::dsl::{four_button_0, four_button_1, four_button_2, four_button_3};

        song_dsl
            .filter(four_button_0.eq(level))
            .or_filter(four_button_1.eq(level))
            .or_filter(four_button_2.eq(level))
            .or_filter(four_button_3.eq(level))
            .load::<Song>(conn)
            .expect("Error loading songs")
    }

    pub fn by_5b_level(level: i32, conn: &SqliteConnection) -> Vec<Self> {
        use super::schema::songs::dsl::{five_button_0, five_button_1, five_button_2, five_button_3};

        song_dsl
            .filter(five_button_0.eq(level))
            .or_filter(five_button_1.eq(level))
            .or_filter(five_button_2.eq(level))
            .or_filter(five_button_3.eq(level))
            .load::<Song>(conn)
            .expect("Error loading songs")
    }

    pub fn by_6b_level(level: i32, conn: &SqliteConnection) -> Vec<Self> {
        use super::schema::songs::dsl::{six_button_0, six_button_1, six_button_2, six_button_3};

        song_dsl
            .filter(six_button_0.eq(level))
            .or_filter(six_button_1.eq(level))
            .or_filter(six_button_2.eq(level))
            .or_filter(six_button_3.eq(level))
            .load::<Song>(conn)
            .expect("Error loading songs")
    }

    pub fn by_8b_level(level: i32, conn: &SqliteConnection) -> Vec<Self> {
        use super::schema::songs::dsl::{eight_button_0, eight_button_1, eight_button_2, eight_button_3};

        song_dsl
            .filter(eight_button_0.eq(level))
            .or_filter(eight_button_1.eq(level))
            .or_filter(eight_button_2.eq(level))
            .or_filter(eight_button_3.eq(level))
            .load::<Song>(conn)
            .expect("Error loading songs")
    }

    pub fn create_or_update(song: &Song, conn: &SqliteConnection) -> Option<Self> {
        if let Some(x) = Self::by_id(song.id, conn) {
            let target = song_dsl.find(song.id);
            diesel::update(target)
                .set(song)
                .execute(conn)
                .expect("Error saving new song");
        } else {
            diesel::insert_into(song_dsl)
                .values(song)
                .execute(conn)
                .expect("Error saving new song");
        }

        Self::by_id(song.id, conn)
    }
}