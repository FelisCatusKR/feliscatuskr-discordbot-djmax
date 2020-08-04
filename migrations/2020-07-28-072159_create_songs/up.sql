-- Your SQL goes here

CREATE TABLE IF NOT EXISTS songs (
  id INTEGER NOT NULL PRIMARY KEY,
  title TEXT NOT NULL,
  artist TEXT NOT NULL,
  min_bpm FLOAT,
  max_bpm FLOAT NOT NULL,
  category TEXT NOT NULL,
  dlc TEXT,
  four_button_0 INTEGER NOT NULL,
  four_button_1 INTEGER,
  four_button_2 INTEGER,
  four_button_3 INTEGER,
  five_button_0 INTEGER NOT NULL,
  five_button_1 INTEGER,
  five_button_2 INTEGER,
  five_button_3 INTEGER,
  six_button_0 INTEGER NOT NULL,
  six_button_1 INTEGER,
  six_button_2 INTEGER,
  six_button_3 INTEGER,
  eight_button_0 INTEGER NOT NULL,
  eight_button_1 INTEGER,
  eight_button_2 INTEGER,
  eight_button_3 INTEGER
);
