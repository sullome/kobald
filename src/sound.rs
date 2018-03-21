use std::collections::HashMap;
use std::path::PathBuf;

use sdl2;
use sdl2::rwops::RWops;
use sdl2::mixer::LoaderRWops;
use sdl2::mixer::Chunk;
use rusqlite::{Connection, OpenFlags, DatabaseName};
use std::io::Read; // For Blob

use super::DB_FILENAME;

pub fn load_sounds() -> HashMap<String, Chunk> {
    // Setting up database connection
    let db_path: PathBuf = [".", DB_FILENAME].iter().collect();
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
    let db_connection = Connection::open_with_flags(&db_path, flags)
        .expect("Cannot read data");

    // Getting effects
    let query = "select name, rowid from sound_effects;";
    let mut statement = db_connection.prepare(&query)
        .expect("Cannot prepare query");

    let mut effects: HashMap<String, Chunk> = HashMap::new();
    for maybe_row_content in statement.query_map
    (
        &[],
        |row| {
            let name: String = row.get(0);
            let id:      i64 = row.get(1);
            (id, name)
        }
    ).unwrap()
    {
        if let Ok((id, name)) = maybe_row_content {
            let mut blob = db_connection.blob_open(
                DatabaseName::Main,
                "sound_effects",
                "effect",
                id,
                true                // Read-Only
            ).expect("Cannot read image blob.");

            let mut bytes: Vec<u8> = Vec::new();
            blob.read_to_end(&mut bytes)
                .expect("Cannot read image bytes.");
            let stream = RWops::from_bytes(&bytes)
                .expect("Cannot open image bytes as a stream.");
            let effect: Chunk = stream.load_wav()
                .expect("Cannot load sound effect");

            effects.insert(name, effect);
        }
    }

    effects
}

pub fn play_effect(effect: &Chunk) {
    let channels = sdl2::mixer::Channel::all();
    channels.play(effect, 0);
}
