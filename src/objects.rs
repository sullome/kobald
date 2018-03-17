use sdl2::EventSubsystem;
use sdl2::keyboard::Keycode;
use rusqlite::{Connection, OpenFlags};
use rand::{Rng, StdRng};

use std::path::PathBuf;

use super::map::Map;
use super::get_setting;

use super::DB_FILENAME;

//{{{ Player
pub struct Player {
    view_distance:       u8,
    view_resource:       u8,
    view_resource_max:   u8,
    view_resource_count: u8,

    pub x: usize,
    pub y: usize,
}
impl Player {
    pub fn init(start_x: usize, start_y: usize) -> Player //{{{
    {
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
    //}}}

    pub fn get_view_distance(&self) -> u8 //{{{
    {
        match self.view_resource {
            0 => 0,
            _ => self.view_distance,
        }
    }
    //}}}

    pub fn get_resource_state(&self) -> f32 //{{{
    {
        self.view_resource as f32 / self.view_resource_max as f32
    }
    //}}}

    pub fn refill_view_resource(&mut self) -> Result<(), ()> //{{{
    {
        match self.view_resource_count {
            0 => Err(()),
            _ => {
                self.view_resource = self.view_resource_max;
                self.view_resource_count -= 1;
                Ok(())
            }
        }
    }
    //}}}

    pub fn drain_view_resource(&mut self) //{{{
    {
        if self.view_resource > 0 {
            self.view_resource -= 1;
        }
    }
    //}}}

    pub fn add_view_resource_count(&mut self) //{{{
    {
        if let Some(new_count) = self.view_resource_count.checked_add(1) {
            self.view_resource_count = new_count;
        };
    }
    //}}}

    fn move_relative(&mut self, x_mod: isize, y_mod: isize, map: &Map) //{{{
    {
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
    //}}}

    pub fn update //{{{
    (
        &mut self,
        key: &Keycode,
        map: &Map,
        resources: &Resources,
        event_system: &EventSubsystem
    )
    {
        // Save some fields before update.
        // Needed for after-update checks
        let previous_view_resource = self.view_resource;

        //{{{ Reaction to keypresses
        match *key {
            // Movement
            Keycode::Up
            | Keycode::Kp8
            | Keycode::Num8
                => self.move_relative(0, -1, map),
            Keycode::Down
            | Keycode::Kp2
            | Keycode::Num2
                => self.move_relative(0, 1, map),
            Keycode::Left
            | Keycode::Kp4
            | Keycode::Num4
                => self.move_relative(-1, 0, map),
            Keycode::Right
            | Keycode::Kp6
            | Keycode::Num6
                => self.move_relative(1, 0, map),
            Keycode::Kp1
            | Keycode::Num1
                => self.move_relative(-1, 1, map),
            Keycode::Kp3
            | Keycode::Num3
                => self.move_relative(1, 1, map),
            Keycode::Kp7
            | Keycode::Num7
                => self.move_relative(-1, -1, map),
            Keycode::Kp9
            | Keycode::Num9
                => self.move_relative(1, -1, map),

            // Actions
            Keycode::R
                => {
                    let refill_result_event = EventResourceRefill {
                        success: match self.refill_view_resource() {
                            Ok(_)  => true,
                            Err(_) => false
                        }
                    };
                    event_system
                        .push_custom_event(refill_result_event)
                        .unwrap();
                },

            _   => return
        }
        //}}}

        //{{{ Checks of new state

        // Was resource found?
        for (i, &resource_location) in resources.locations.iter().enumerate() {
            if resource_location == (self.x, self.y) {
                let resource_found = EventResourceFound{index: i};
                event_system.push_custom_event(resource_found).unwrap();
            }
        }

        // Was resource gone?
        if previous_view_resource > 0 && self.view_resource == 0 {
            let resource_gone = EventResourceGone{};
            event_system.push_custom_event(resource_gone).unwrap();
        }
        //}}}
    }
    //}}}
}
//}}}

//{{{ Resources
pub struct Resources {
    locations: Vec<(usize, usize)>
}
impl Resources {
    pub fn init(map: &Map, player: &Player) -> Resources {
        let mut rng: StdRng = StdRng::new()
            .expect("Cannot read randomness from OS");
        let sections_side: u32 = match get_setting("resource_distance") {
            Some(value) => value,
            None        => player.view_resource_max as u32
        };
        let sections_side: usize = sections_side as usize;
        let mut locations: Vec<(usize, usize)> = Vec::with_capacity(
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
                    locations.push(*location);
                }
                /*
                 * End of work with sections
                 */

                y += sections_side;
            }

            x += sections_side;
        }

    Resources { locations }
    }

    pub fn process_event(&mut self, custom_event: &EventResourceFound) {
        self.locations.remove(custom_event.index);
    }
}
//}}}

//{{{ Monster
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
//}}}

/*
 * Custom events
 */
pub struct EventResourceFound  {index: usize}
pub struct EventResourceGone   {}
pub struct EventResourceRefill {pub success: bool}

pub fn init_custom_events(sdl_event: &EventSubsystem) {
    sdl_event.register_custom_event::<EventResourceFound>()
        .expect("Failed to register event.");
    sdl_event.register_custom_event::<EventResourceGone>()
        .expect("Failed to register event.");
    sdl_event.register_custom_event::<EventResourceRefill>()
        .expect("Failed to register event.");
}
