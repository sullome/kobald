use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rusqlite::{Connection, OpenFlags};

use std::path::PathBuf;

use super::map::Map;
use super::get_setting;

use super::DB_FILENAME;
const DB_RESOURCE_MAX:         &'static str = "resource_max";

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
            view_distance:       match get_setting("visible_distance") {
                Some(value) => value,
                None        => 5
            },
            view_resource_max:   match get_setting("resource_max") {
                Some(value) => value,
                None        => 10
            },
            view_resource_count: match get_setting("resource_start") {
                Some(value) => value,
                None        => 3
            },
            view_resource: 0,
            x: start_x,
            y: start_y
        };
        player.view_resource = player.view_resource_max;

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
        let map_size = map.tiles.len() as isize - 1;
        let mut new_x: isize = self.x as isize + x_mod;
        let mut new_y: isize = self.y as isize + y_mod;

        if new_x < 0 {new_x = 0};
        if new_y < 0 {new_y = 0};
        if new_x >= map_size {new_x = map_size};
        if new_y >= map_size {new_y = map_size};
        let new_x: usize = new_x as usize;
        let new_y: usize = new_y as usize;

        if map.tiles[new_x as usize][new_y as usize].passable {
            self.x = new_x;
            self.y = new_y;

            self.drain_view_resource();
        }
    }

    pub fn update(&mut self, event: &Event, map: &Map, objects: &Vec<Resource>)
    {
        match *event {
            // Movement
            Event::KeyDown {keycode: Some(Keycode::Up), .. }
            | Event::KeyDown {keycode: Some(Keycode::Kp8), .. }
                => self.move_relative(0, -1, map),
            Event::KeyDown {keycode: Some(Keycode::Down), .. }
            | Event::KeyDown {keycode: Some(Keycode::Kp2), .. }
                => self.move_relative(0, 1, map),
            Event::KeyDown {keycode: Some(Keycode::Left), .. }
            | Event::KeyDown {keycode: Some(Keycode::Kp4), .. }
                => self.move_relative(-1, 0, map),
            Event::KeyDown {keycode: Some(Keycode::Right), .. }
            | Event::KeyDown {keycode: Some(Keycode::Kp6), .. }
                => self.move_relative(1, 0, map),
            Event::KeyDown {keycode: Some(Keycode::Kp1), .. }
                => self.move_relative(-1, 1, map),
            Event::KeyDown {keycode: Some(Keycode::Kp3), .. }
                => self.move_relative(1, 1, map),
            Event::KeyDown {keycode: Some(Keycode::Kp7), .. }
                => self.move_relative(-1, -1, map),
            Event::KeyDown {keycode: Some(Keycode::Kp9), .. }
                => self.move_relative(1, -1, map),

            // Actions
            Event::KeyDown {keycode: Some(Keycode::R), .. }
                => self.refill_view_resource(),

            _   => ()
        }
    }
}

struct Resource {
    x: u8,
    y: u8
}
impl Resource {
    pub fn init_all(map: &Map, player: &Player) -> Vec<Resource> {
        let sections = map.tiles.len() / player.view_resource_max;
        let resources: Vec<Resource> = Vec::with_capacity(sections ^ 2);

        /*
         * Two 'while' cycles are needed to segregate map into sections
         * with sides equal to view_resource_max.
         */
        let mut x = 0;
        while x < map.tiles.len() {
            let max_x = x + player.view_resource_max;
            if max_x > map.tiles.len() {
                max_x = map.tiles.len();
            }

            let mut y = 0;
            while y < map.tiles.len() {
                let max_y = y + player.view_resource_max;
                if max_y > map.tiles.len() {
                    max_y = map.tiles.len();
                }

                /*
                 * Now we can do what's needs to be done in those sections
                 */
                let possible_locations: Vec<(usize, usize)> = Vec::new();
                for inner_x in (x..max_x).iter() {
                    for inner_y in (y..max_y).iter() {
                    }
                }
                /*
                 * End of work with sections
                 */

                y += player.view_resource_max;
            }

            x += player.view_resource_max;
        }
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
