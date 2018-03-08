extern crate sevend;
extern crate rusqlite;
extern crate sdl2;

use std::path::PathBuf;
use std::collections::HashMap;
use std::time::Duration;

use sdl2::rwops::RWops;
use sdl2::image::ImageRWops;
use sdl2::render::{TextureCreator,Texture};
use sdl2::pixels::Color;
use sdl2::event::Event;
use rusqlite::{Connection, DatabaseName, OpenFlags};
use std::io::Read; // For Blob

use sevend::map::Map;
use sevend::creature::Player;

use sevend::DB_FILENAME;
const DB_IMAGES_TABLE:  &'static str = "images";
const DB_IMAGES_COLUMN: &'static str = "image";

const GAME_NAME: &'static str = "Kobalt";
const TEXTURE_SIDE: usize = 8;

fn init_textures<T>
    (texture_creator: &TextureCreator<T>) -> HashMap<String, Texture>
{
    let mut textures: HashMap<String,Texture> = HashMap::new();

    let db_path: PathBuf = [".", DB_FILENAME].iter().collect();
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
    let db_connection = Connection::open_with_flags(&db_path, flags)
        .expect("Cannot read data.");

    let query = String::from("select name, rowid from ")
        + DB_IMAGES_TABLE
        + ";"
    ;
    let mut statement = db_connection.prepare(&query)
        .expect("Cannot prepary query.");

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
            let mut image_blob = db_connection.blob_open(
                DatabaseName::Main,
                DB_IMAGES_TABLE,
                DB_IMAGES_COLUMN,
                id,
                true              // Read-Only
            ).expect("Cannot read image blob.");

            let mut image_bytes: Vec<u8> = Vec::new();
            image_blob
                .read_to_end(&mut image_bytes)
                .expect("Cannot read image bytes.");

            let abstract_stream: RWops = RWops::from_bytes(&image_bytes)
                .expect("Cannot open image bytes as a stream.");

            let image_surface = abstract_stream
                .load()
                .expect("Cannot create surface from image stream.");

            let texture = texture_creator
                .create_texture_from_surface(&image_surface)
                .expect("Cannot create texture from image surface.");

            textures.insert(name, texture);
        }
    }

    textures
}

fn main() {
    // Init game variables
    let mut map = Map::init();
    let map_side = map.tiles[0].len();
    let mut player = Player::init(map.start.0, map.start.1);

    // Init SDL2 and it's subsystems
    let sdl_context = sdl2::init()
        .expect("SDL initialization error.");
    let sdl_video = sdl_context.video()
        .expect("SDL video subsystem initialization error.");
    let _sdl_image = sdl2::image::init(sdl2::image::INIT_PNG)
        .expect("SDL Image initialization error.");
    let mut sdl_eventpump = sdl_context.event_pump()
        .expect("SDL Event Pump initialization error.");

    // Init main window
    let window_side: u32 = (map_side * TEXTURE_SIDE) as u32;
    let window = sdl_video.window(GAME_NAME, window_side, window_side)
        .position_centered()
        .build()
        .expect("Window build error.");
    let mut canvas = window.into_canvas()
        .build()
        .expect("Canvas creation error.");

    // Init textures
    let texture_creator = canvas.texture_creator();
    let textures = init_textures(&texture_creator);

    // Set background color and render it
    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.clear();
    canvas.present();

    'running: loop {
        // Events handling
        for event in sdl_eventpump.poll_iter() {
            match event {
                Event::Quit{..} => {
                    println!("QUIT");
                    break 'running;
                },
                e => player.update(&e, &map)
            }
        }

        // Update game
        map.update(&player);

        // Start drawing
        canvas.clear();

        map.draw(&textures, &mut canvas);
        player.draw(&textures, &mut canvas);

        // Stop drawing
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(16));
    }
}
