extern crate diesel;
extern crate diesel_migrations;

use diesel::prelude::*;

pub fn establish_connection() -> SqliteConnection{
	let conn = SqliteConnection::establish("file.db")
          .unwrap_or_else(|_| panic!("Error creating test database"));
    println!("Database connection established");
    conn
}