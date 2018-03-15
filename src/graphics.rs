use std::path::PathBuf;
use std::collections::HashMap;

use sdl2;
use sdl2::rwops::RWops;
use sdl2::image::ImageRWops;
use sdl2::render::{TextureCreator, Texture, Canvas, RenderTarget};
use sdl2::rect::Rect;
use sdl2::video::{Window, WindowPos};
use sdl2::pixels::Color;
use sdl2::EventPump;
use rusqlite::{Connection, DatabaseName, OpenFlags};
use std::io::Read; // For Blob

use super::map::Map;
use super::creature::Player;
use super::get_setting;

use super::DB_FILENAME;
const DB_IMAGES_TABLE:   &'static str = "images";
const DB_IMAGES_COLUMN:  &'static str = "image";
const DB_FONTS_TABLE:    &'static str = "fonts";
const DB_FONTS_COLUMN:   &'static str = "font";
const DB_MESSAGES_TABLE: &'static str = "messages";

pub struct GUIElement {
    drawarea: Rect,
}
impl GUIElement {
    pub fn init(name: &str) -> GUIElement {
        let mut name_string: String = name.to_string() + "_";

        name_string.push('x');
        let x: i32 = match get_setting(&name_string) {
            Ok(value) => value,
            Err(_)    => 0,
        };

        name_string.pop();
        name_string.push('y');
        let y: i32 = match get_setting(&name_string) {
            Ok(value) => value,
            Err(_)    => 0,
        };

        name_string.pop();
        name_string.push('w');
        let width: u32 = match get_setting(&name_string) {
            Ok(value) => value,
            Err(_)    => 1,
        };

        name_string.pop();
        name_string.push('h');
        let height: u32 = match get_setting(&name_string) {
            Ok(value) => value,
            Err(_)    => 1,
        };

        GUIElement {
            drawarea: Rect::new(x, y, width, height),
        }
    }

    pub fn draw
    (
        &self,
        textures: &HashMap<String, Texture>,
        mut canvas: &mut Canvas<Window>,
        parts: Vec<&Drawable>
    )
    {
        // Start drawing this element in its own place
        let previous_drawarea: Rect = canvas.viewport();
        canvas.set_viewport(self.drawarea);

        // Drawing all parts of this GUI element
        for part in parts.iter() {
            part.draw(&textures, &mut canvas);
        }

        // Restoring canvas viewport after all drawings
        canvas.set_viewport(previous_drawarea);
    }
}

pub struct TextLine {
    situation: String,
}
impl TextLine {
    pub fn init() -> TextLine {
        TextLine {
            situation: String::from("start"),
        }
    }

    fn set_text(&mut self, situation: &String) {
        self.situation = situation.clone();
    }
}

pub struct Background {
    texture_name: String,
}
impl Background {
    pub fn init() -> Background {
        Background {
            texture_name: String::from("map.png"),
        }
    }
}

pub trait Drawable {
    fn draw
    (
        &self,
        textures: &HashMap<String, Texture>,
        canvas: &mut Canvas<Window>
    );
}
impl Drawable for Map //{{{
{
    fn draw
    (
        &self,
        textures: &HashMap<String, Texture>,
        canvas: &mut Canvas<Window>
    )
    {
        // Draw visible tiles
        let texture_side: u32 = textures["wall.png"].query().width;
        let mut place: Rect = Rect::new(0, 0, texture_side, texture_side);

        for (x, column) in self.tiles.iter().enumerate() {
            for (y, tile) in column.iter().enumerate() {
                if tile.visible {
                    let texture: &Texture = &textures[&tile.icon];
                    let tx = ((x as u32) * texture_side) as i32;
                    let ty = ((y as u32) * texture_side) as i32;
                    place.set_x(tx);
                    place.set_y(ty);

                    canvas.copy(texture, None, place)
                        .expect("Texture rendering error!");
                }
            }
        }
    }
}
//}}}
impl Drawable for Player //{{{
{
    fn draw
    (
        &self,
        textures: &HashMap<String, Texture>,
        canvas: &mut Canvas<Window>
    )
    {
        let texture: &Texture = &textures["player.png"];
        let texture_side: u32 = texture.query().width;
        let place: Rect = Rect::new(
            ((self.x as u32) * texture_side) as i32,
            ((self.y as u32) * texture_side) as i32,
            texture_side,
            texture_side
        );
        canvas.copy(texture, None, place)
            .expect("Texture rendering error!");
    }
}
//}}}
impl Drawable for TextLine //{{{
{
    fn draw
    (
        &self,
        textures: &HashMap<String, Texture>,
        canvas: &mut Canvas<Window>
    )
    {
        let text_texture: &Texture = &textures[&self.situation];
        let place: Rect = Rect::new(
            0, 0,
            text_texture.query().width,
            text_texture.query().height
        );
        canvas.copy(text_texture, None, place)
            .expect("Text texture rendering error!");
    }
} //}}}
impl Drawable for Background //{{{
{
    fn draw
    (
        &self,
        textures: &HashMap<String, Texture>,
        canvas: &mut Canvas<Window>
    )
    {
        let bg_texture: &Texture = &textures[&self.texture_name];
        let place: Rect = Rect::new(
            0, 0,
            bg_texture.query().width, bg_texture.query().height
        );
        canvas.copy(bg_texture, None, place)
            .expect("Text texture rendering error!");

    }
}
//}}}

/*
 * This function initializes textures for further usage by *draw* functions.
 */
pub fn init_textures<T> //{{{
    (texture_creator: &TextureCreator<T>) -> HashMap<String, Texture>
{
    let mut textures: HashMap<String,Texture> = HashMap::new();

    // Setting up database connection
    let db_path: PathBuf = [".", DB_FILENAME].iter().collect();
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
    let db_connection = Connection::open_with_flags(&db_path, flags)
        .expect("Cannot read data.");

    //{{{ Pictures
    // Query for retrieving images location in DB
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
            // Getting image
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
    //}}}

    //{{{ Messages
    // Initializing SDL TTF
    let sdl_ttf = sdl2::ttf::init()
        .expect("SDL TTF initialization error.");

    // Getting font from database
    let mut font_blob = db_connection.blob_open(
        DatabaseName::Main,
        DB_FONTS_TABLE,
        DB_FONTS_COLUMN,
        1,
        true              // Read-Only
    ).expect("Cannot read font blob.");

    let mut font_bytes: Vec<u8> = Vec::new();
    font_blob
        .read_to_end(&mut font_bytes)
        .expect("Cannot read font bytes.");

    let abstract_stream: RWops = RWops::from_bytes(&font_bytes)
        .expect("Cannot open font bytes as a stream.");

    let font_height: u16 = match get_setting("textline_font_size") {
        Ok(height) => height,
        Err(_)     => 12,
    };
    let font = sdl_ttf
        .load_font_from_rwops(abstract_stream, font_height)
        .expect("Cannot load font from a stream.");

    // Rendering messages with the selected font
    let query = String::from("select * from ") + DB_MESSAGES_TABLE + ";";
    let mut statement = db_connection.prepare(&query)
        .expect("Cannot prepary query.");

    for maybe_row_content in statement.query_map
    (
        &[],
        |row| {
            let situation: String = row.get(0);
            let message:   String = row.get(1);
            (situation, message)
        }
    ).unwrap()
    {
        if let Ok((situation, message)) = maybe_row_content {
            // Rendering message
            let text_surface = font
                .render(&message)
                .blended(Color::RGB(0, 0, 0))
                .expect("Cannot create text surface.");
            let text_texture = texture_creator
                .create_texture_from_surface(text_surface)
                .expect("Cannot render text.");

            textures.insert(situation, text_texture);
        }
    }
    //}}}

    textures
}
//}}}

/*
 * This function initializes SDL2 window
 */
pub fn init_sdl2
    () -> (Canvas<Window>, EventPump)
{
    let game_name: String = match get_setting("game_name") {
        Ok(name) => name,
        Err(_)   => String::from("Debug")
    };

    // Init SDL2 and it's subsystems
    let sdl_context = sdl2::init()
        .expect("SDL initialization error.");
    let sdl_video = sdl_context.video()
        .expect("SDL video subsystem initialization error.");
    let _sdl_image = sdl2::image::init(sdl2::image::INIT_PNG)
        .expect("SDL Image initialization error.");
    let sdl_eventpump = sdl_context.event_pump()
        .expect("SDL Event Pump initialization error.");

    // Init main window
    let window = sdl_video.window(&game_name, 10, 10)
        .build()
        .expect("Window build error.");
    let mut canvas = window.into_canvas()
        .build()
        .expect("Canvas creation error.");


    // Set background canvas
    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.clear();
    canvas.present();

    (canvas, sdl_eventpump)
}

pub fn configure_window
    (window: &mut Window, textures: &HashMap<String, Texture>)
{
    let bg_texture: &Texture = &textures["map.png"];
    let window_width:  u32 = bg_texture.query().width;
    let window_height: u32 = bg_texture.query().height;

    window.set_size(window_width, window_height)
        .expect("Window resizing error.");
    window.set_position(WindowPos::Centered, WindowPos::Centered);
}