extern crate rusqlite;
extern crate rand;
extern crate sdl2;

use std::path::PathBuf;
use rusqlite::{Connection, OpenFlags};
use rusqlite::types::FromSql;

pub const DB_FILENAME: &'static str = "data.sqlite3";

pub fn get_setting<T: FromSql>
    (setting_name: &str) -> Result<T, ()>
{
    // Setting up database connection
    let db_path: PathBuf = [".", DB_FILENAME].iter().collect();
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
    let db_connection = Connection::open_with_flags(&db_path, flags)
        .expect("Cannot read data.");

    match db_connection.query_row
    (
        "select value from game_settings where setting like ?;",
        &[&setting_name],
        |row| {
            let value: T = row.get(0);
            value
        }
    )
    {
        Ok(v) => Ok(v),
        Err(_) => Err(())
    }
}

pub mod creature;
pub mod map;
pub mod graphics;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
