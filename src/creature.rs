use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas,Texture,RenderTarget};
use sdl2::rect::Rect;
use rusqlite::{Connection, OpenFlags};

use std::collections::HashMap;
use std::path::PathBuf;

use super::map::Map;

use super::DB_FILENAME;
const DB_VIEW_DISTANCE:        &'static str = "visible_distance";
const DB_RESOURCE_MAX:         &'static str = "resource_max";
const DB_RESOURCE_COUNT_START: &'static str = "resource_start";

pub struct Player {
    view_distance:       u8,
    view_resource:       u8,
    view_resource_max:   u8,
    view_resource_count: u8,

    pub x: usize,
    pub y: usize,
}
impl Player {
    pub fn init(start_x: usize, start_y: usize) -> Player {
        // Default values
        let mut player = Player {
            view_distance: 5,
            view_resource: 10,
            view_resource_max: 10,
            view_resource_count: 3,
            x: start_x,
            y: start_y
        };

        // Reading game settings
        let db_path: PathBuf = [".", DB_FILENAME].iter().collect();
        let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
        if let Ok(db_connection) = Connection::open_with_flags(&db_path, flags)
        {
            let query = String::from("select value ")
                + "from game_settings "
                + "where setting like ?;"
            ;

            player.view_distance = db_connection.query_row(
                &query,
                &[&DB_VIEW_DISTANCE],
                |row| {
                    let distance: u8 = row.get(0);
                    distance
                }
            ).unwrap();

            player.view_resource_count = db_connection.query_row(
                &query,
                &[&DB_RESOURCE_COUNT_START],
                |row| {
                    let start: u8 = row.get(0);
                    start
                }
            ).unwrap();

            player.view_resource_max = db_connection.query_row(
                &query,
                &[&DB_RESOURCE_MAX],
                |row| {
                    let max: u8 = row.get(0);
                    max
                }
            ).unwrap();

            player.view_resource = player.view_resource_max;
        }

        player
    }

    pub fn get_view_distance(&self) -> u8 {
        match self.view_resource {
            0 => 0,
            _ => self.view_distance,
        }
    }

    pub fn refill_view_resource(&mut self) -> Result<(), &'static str> {
        match self.view_resource_count {
            0 => Err("I don't have enough oil."),
            _ => {
                self.view_resource = self.view_resource_max;
                self.view_resource_count -= 1;
                Ok(())
            }
        }
    }

    pub fn drain_view_resource(&mut self) {
        if self.view_resource > 0 {
            self.view_resource -= 1;
        }
    }

    pub fn add_view_resource_count(&mut self) -> Result<(), &'static str> {
        match self.view_resource_count.checked_add(1) {
            Some(new_count) => {
                self.view_resource_count = new_count;
                Ok(())
            },
            None => Err("I can't take that — it's too much for me."),
        }
    }

    fn move_relative(&mut self, x_mod: isize, y_mod: isize, map: &Map) {
        let mut new_x: isize = self.x as isize + x_mod;
        let mut new_y: isize = self.y as isize + y_mod;

        if new_x < 0 {new_x = 0};
        if new_y < 0 {new_y = 0};
        let new_x: usize = new_x as usize;
        let new_y: usize = new_y as usize;

        if map.tiles[new_x as usize][new_y as usize].passable {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn update(&mut self, event: &Event, map: &Map) {
        match *event {
            Event::KeyDown {keycode: Some(Keycode::Up), .. }
                => self.move_relative(0, -1, map),
            Event::KeyDown {keycode: Some(Keycode::Down), .. }
                => self.move_relative(0, 1, map),
            Event::KeyDown {keycode: Some(Keycode::Left), .. }
                => self.move_relative(-1, 0, map),
            Event::KeyDown {keycode: Some(Keycode::Right), .. }
                => self.move_relative(1, 0, map),
            _   => ()
        }
    }

    pub fn draw<T: RenderTarget>
    (
        &self,
        textures: &HashMap<String, Texture>,
        canvas: &mut Canvas<T>
    )
    {
        let texture: &Texture = &textures["player.png"];
        let texture_side: u32 = texture.query().width;
        let mut place: Rect = Rect::new(
            ((self.x as u32) * texture_side) as i32,
            ((self.y as u32) * texture_side) as i32,
            texture_side,
            texture_side
        );
        canvas.copy(texture, None, place)
            .expect("Texture rendering error!");
    }
}

struct Kobold {
    alive: bool,
    pub x: u8,
    pub y: u8,
}
impl Kobold {
    pub fn init() -> Kobold {
        // Default
        let kobold = Kobold {
            alive: true,
            x: 0,
            y: 0,
        };

        kobold
    }

    pub fn update(&mut self) {
        if self.alive {
        }
    }
}
