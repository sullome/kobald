use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use rusqlite::{Connection, OpenFlags};
use rand::{Rng, StdRng};

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

    pub fn update(&mut self, key: &Keycode, map: &Map, objects: &Vec<Resource>)
    {
        match *key {
            // Movement
            Keycode::Up    | Keycode::Kp8
                => self.move_relative(0, -1, map),
            Keycode::Down  | Keycode::Kp2
                => self.move_relative(0, 1, map),
            Keycode::Left  | Keycode::Kp4
                => self.move_relative(-1, 0, map),
            Keycode::Right | Keycode::Kp6
                => self.move_relative(1, 0, map),
            Keycode::Kp1
                => self.move_relative(-1, 1, map),
            Keycode::Kp3
                => self.move_relative(1, 1, map),
            Keycode::Kp7
                => self.move_relative(-1, -1, map),
            Keycode::Kp9
                => self.move_relative(1, -1, map),

            // Actions
            Keycode::R
                => self.refill_view_resource().unwrap(),

            _   => ()
        }
        for resource in objects {
            if resource.x == self.x && resource.y == self.y {
                println!("Found lamp!");
            }
        }
    }
}

pub struct Resource {
    x: usize,
    y: usize
}
impl Resource {
    pub fn init_all(map: &Map, player: &Player) -> Vec<Resource> {
        let mut rng: StdRng = StdRng::new()
            .expect("Cannot read randomness from OS");
        let sections_side: u32 = match get_setting("resource_distance") {
            Some(value) => value,
            None        => (map.tiles.len()/player.view_resource_max as usize) as u32
        };
        let sections_side: usize = sections_side as usize;
        let mut resources: Vec<Resource> = Vec::with_capacity(
            map.tiles.len() / sections_side ^ 2
        );

        /*
         * Two 'while' cycles are needed to segregate map into sections
         * with sides equal to view_resource_max.
         */
        let mut x = 0;
        while x < map.tiles.len() {
            let max_x = (x + sections_side).min(map.tiles.len());

            let mut y = 0;
            while y < map.tiles.len() {
                let max_y = (y + sections_side).min(map.tiles.len());

                /*
                 * Now we can do what's needs to be done in those sections
                 */
                let mut possible_locations: Vec<(usize, usize)> = Vec::new();
                for inner_x in x..max_x {
                    for inner_y in y..max_y {
                        if map.tiles[inner_x][inner_y].passable {
                            possible_locations.push((inner_x, inner_y));
                        }
                    }
                }
                let maybe_location: Option<&(usize, usize)> = rng
                    .choose(&possible_locations);
                if let Some(location) = maybe_location {
                    resources.push(
                        Resource {
                            x: location.0,
                            y: location.1
                        }
                    )
                }
                /*
                 * End of work with sections
                 */

                y += sections_side;
            }

            x += sections_side;
        }

    resources
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
