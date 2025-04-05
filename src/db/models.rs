use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

pub trait DbModel {
    fn create_query() -> &'static str;
}

#[derive(Debug)]
pub struct Signup {
    id: Uuid,
    date: NaiveDate,
}

impl DbModel for Signup {
    fn create_query() -> &'static str {
        "CREATE TABLE signup (
            id BLOB PRIMARY KEY,
            date TEXT NOT NULL   
    )"
    }
}

#[derive(Debug)]
pub struct Game {
    signup_id: Uuid,
    time: NaiveTime,
    opponent_id: Uuid,
}

impl DbModel for Game {
    fn create_query() -> &'static str {
        "CREATE TABLE game (
            id BLOB PRIMARY KEY,
            time TEXT NOT NULL,
            opponent_id BLOB NOT NULL,
    )"
    }
}

#[derive(Debug)]
pub struct Opponent {
    id: Uuid,
    name: String,
}

impl DbModel for Opponent {
    fn create_query() -> &'static str {
        "CREATE TABLE opponent (
            id BLOB PRIMARY KEY,
            name TEXT NOT NULL   
    )"
    }
}
