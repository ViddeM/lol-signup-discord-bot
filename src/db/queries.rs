use std::collections::HashMap;

use chrono::NaiveDate;
use eyre::Context;
use rusqlite::Connection;

use super::models::{self, DbModel, Signup};

fn get_connection() -> eyre::Result<Connection> {
    Ok(Connection::open("league_signups.sqlite3").wrap_err("failed to setup")?)
}

pub fn setup() -> eyre::Result<()> {
    println!("Setting up database");
    let conn = get_connection()?;

    conn.execute_batch(&format!(
        "BEGIN;
        {};
        {};
        {};
        COMMIT;",
        models::Signup::create_query(),
        models::Game::create_query(),
        models::Opponent::create_query()
    ))
    .wrap_err("Failed to initialize database")?;

    Ok(())
}

pub fn create_signup(date: NaiveDate, games: HashMap<NaiveDate, String>) -> eyre::Result<Signup> {
    let conn = get_connection()?;

    conn.execute("INSERT INTO signup (id, date) VALUES (?1, ?2)", params![
        uuid::Uuid::generate_v4_uuid(),
        date
    ]);
}
