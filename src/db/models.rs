use diesel::prelude::*;
use serde::Deserialize;

use crate::db::schema::songs;
use crate::db::schema::songs::dsl::songs as song_dsl;

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
        } else {
            None
        }
    }

    pub fn by_title(title_str: &str, conn: &SqliteConnection) -> Option<Self> {
        use super::schema::songs::dsl::title;

        if title_str == "" {
            None
        } else {
            song_dsl
                .filter(title.like(format!("%{}%", title_str)))
                .first(conn)
                .optional()
                .expect("Error loading songs")
        }
    }

    pub fn by_level(
        button: i32,
        level: i32,
        offset: i64,
        conn: &SqliteConnection,
    ) -> (i64, Vec<Self>) {
        use super::schema::songs::dsl::{
            eight_button_0, eight_button_1, eight_button_2, eight_button_3, five_button_0,
            five_button_1, five_button_2, five_button_3, four_button_0, four_button_1,
            four_button_2, four_button_3, six_button_0, six_button_1, six_button_2, six_button_3,
        };

        match button {
            4 => (
                song_dsl
                    .filter(four_button_0.eq(level))
                    .or_filter(four_button_1.eq(level))
                    .or_filter(four_button_2.eq(level))
                    .or_filter(four_button_3.eq(level))
                    .count()
                    .get_result(conn)
                    .expect("Error loading songs"),
                song_dsl
                    .filter(four_button_0.eq(level))
                    .or_filter(four_button_1.eq(level))
                    .or_filter(four_button_2.eq(level))
                    .or_filter(four_button_3.eq(level))
                    .limit(25)
                    .offset(offset)
                    .load::<Song>(conn)
                    .expect("Error loading songs"),
            ),
            5 => (
                song_dsl
                    .filter(five_button_0.eq(level))
                    .or_filter(five_button_1.eq(level))
                    .or_filter(five_button_2.eq(level))
                    .or_filter(five_button_3.eq(level))
                    .count()
                    .get_result(conn)
                    .expect("Error loading songs"),
                song_dsl
                    .filter(five_button_0.eq(level))
                    .or_filter(five_button_1.eq(level))
                    .or_filter(five_button_2.eq(level))
                    .or_filter(five_button_3.eq(level))
                    .limit(25)
                    .offset(offset)
                    .load::<Song>(conn)
                    .expect("Error loading songs"),
            ),
            6 => (
                song_dsl
                    .filter(six_button_0.eq(level))
                    .or_filter(six_button_1.eq(level))
                    .or_filter(six_button_2.eq(level))
                    .or_filter(six_button_3.eq(level))
                    .count()
                    .get_result(conn)
                    .expect("Error loading songs"),
                song_dsl
                    .filter(six_button_0.eq(level))
                    .or_filter(six_button_1.eq(level))
                    .or_filter(six_button_2.eq(level))
                    .or_filter(six_button_3.eq(level))
                    .limit(25)
                    .offset(offset)
                    .load::<Song>(conn)
                    .expect("Error loading songs"),
            ),
            8 => (
                song_dsl
                    .filter(eight_button_0.eq(level))
                    .or_filter(eight_button_1.eq(level))
                    .or_filter(eight_button_2.eq(level))
                    .or_filter(eight_button_3.eq(level))
                    .count()
                    .get_result(conn)
                    .expect("Error loading songs"),
                song_dsl
                    .filter(eight_button_0.eq(level))
                    .or_filter(eight_button_1.eq(level))
                    .or_filter(eight_button_2.eq(level))
                    .or_filter(eight_button_3.eq(level))
                    .limit(25)
                    .offset(offset)
                    .load::<Song>(conn)
                    .expect("Error loading songs"),
            ),
            _ => (0, vec![]),
        }
    }

    pub fn create_or_update(song: &Song, conn: &SqliteConnection) -> Option<Self> {
        if let None = Self::by_id(song.id, conn) {
            diesel::insert_into(song_dsl)
                .values(song)
                .execute(conn)
                .expect("Error saving new song");
        } else {
            let target = song_dsl.find(song.id);
            diesel::update(target)
                .set(song)
                .execute(conn)
                .expect("Error saving new song");
        }

        Self::by_id(song.id, conn)
    }
}
