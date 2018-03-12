extern crate sevend;
extern crate rusqlite;
extern crate sdl2;

use std::path::PathBuf;
use std::collections::HashMap;
use std::time::Duration;

use sdl2::rwops::RWops;
use sdl2::image::ImageRWops;
use sdl2::render::{TextureCreator,Texture,Canvas,RenderTarget};
use sdl2::ttf::{Sdl2TtfContext, Font};
use sdl2::rect::Rect;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rusqlite::{Connection, DatabaseName, OpenFlags};
use std::io::Read; // For Blob

use sevend::map::Map;
use sevend::creature::Player;

use sevend::MAP_OFFSET;
use sevend::DB_FILENAME;
const DB_IMAGES_TABLE:  &'static str = "images";
const DB_IMAGES_COLUMN: &'static str = "image";

const GAME_NAME: &'static str = "Kobalt";
const MAP_SIDE: u32 = 696;

struct TextLine {
    text: String,
    max_len: usize,
    max_width: u32,
}
impl TextLine {
    pub fn init(map_background: &Texture) -> TextLine {
        let db_path: PathBuf = [".", DB_FILENAME].iter().collect();
        let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
        let db_connection = Connection::open_with_flags(&db_path, flags)
            .expect("Cannot read data.");
        let query = "select value from game_settings where setting like ?;";

        let max_len: usize = db_connection.query_row(
            query,
            &[&"textline_max_len"],
            |row| {
                let max_len: u32 = row.get(0);
                max_len
            }
        ).unwrap() as usize;

        TextLine {
            text: String::from(" "),
            max_len,
            max_width: map_background.query().width - MAP_OFFSET as u32 * 2,
        }
    }

    pub fn set_text(&mut self, text: &String) {
        self.text = text.clone();
    }

    pub fn draw(&self, ttf_context: &Sdl2TtfContext, canvas: &mut Canvas<Window>) {
        let db_path: PathBuf = [".", DB_FILENAME].iter().collect();
        let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
        let db_connection = Connection::open_with_flags(&db_path, flags)
            .expect("Cannot read data.");

        let mut font_blob = db_connection.blob_open(
            DatabaseName::Main,
            "fonts",
            "font",
            1,
            true              // Read-Only
        ).expect("Cannot read font blob.");

        let mut font_bytes: Vec<u8> = Vec::new();
        font_blob
            .read_to_end(&mut font_bytes)
            .expect("Cannot read font bytes.");

        let abstract_stream: RWops = RWops::from_bytes(&font_bytes)
            .expect("Cannot open font bytes as a stream.");

        let font_height: u16 = 24;
        let font = ttf_context
            .load_font_from_rwops(abstract_stream, font_height)
            .expect("Cannot load font from a stream.");

        let text_surface = font
            .render(&self.text)
            .blended_wrapped(Color::RGB(0, 0, 0), self.max_width)
            .expect("Cannot create text surface.");
        println!("Text: {}x{}", text_surface.width(), text_surface.height());
        let texture_creator = canvas.texture_creator();
        let text_texture = texture_creator
            .create_texture_from_surface(text_surface)
            .expect("Cannot render text.");
        let place: Rect = Rect::new(
            MAP_OFFSET as i32,
            MAP_SIDE as i32 - MAP_OFFSET - 44, //TODO
            text_texture.query().width,
            text_texture.query().height,
        );
        canvas.copy(&text_texture, None, place)
            .expect("Text texture rendering error!");
    }
}

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
    // Init SDL2 and it's subsystems
    let sdl_context = sdl2::init()
        .expect("SDL initialization error.");
    let sdl_video = sdl_context.video()
        .expect("SDL video subsystem initialization error.");
    let _sdl_image = sdl2::image::init(sdl2::image::INIT_PNG)
        .expect("SDL Image initialization error.");
    let mut sdl_eventpump = sdl_context.event_pump()
        .expect("SDL Event Pump initialization error.");
    let sdl_ttf = sdl2::ttf::init()
        .expect("SDL TTF initialization error.");

    // Init main window
    let window = sdl_video.window(GAME_NAME, MAP_SIDE, MAP_SIDE)
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

    // Init game variables
    let mut map = Map::init();
    let mut player = Player::init(map.start.0, map.start.1);
    let mut textline = TextLine::init(&textures["map.png"]);

    'running: loop {
        // Events handling
        for event in sdl_eventpump.poll_iter() {
            match event {
                Event::Quit{..}
                    => {
                        println!("QUIT");
                        break 'running;
                    },
                Event::KeyDown {keycode: Some(Keycode::S), .. }
                    => {
                        textline.set_text(
                            &map.tiles[player.x][player.y].search_text
                        );
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
        textline.draw(&sdl_ttf, &mut canvas);

        // Stop drawing
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(16));
    }
}
