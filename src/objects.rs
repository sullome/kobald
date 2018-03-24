use sdl2::EventSubsystem;
use sdl2::keyboard::Keycode;
use rusqlite::{Connection, OpenFlags};
use rand::{Rng, StdRng, thread_rng};

use std::path::PathBuf;

use super::map::Map;
use super::map::TileType;
use super::get_setting;

use super::DB_FILENAME;

//{{{ Player
pub struct Player {
    view_distance:       u8,
    view_resource:       u8,
    view_resource_max:   u8,
    view_resource_count: u8,

    in_danger: bool,
    view_distance_danger:u8,

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
            view_distance_danger:match get_setting("visible_distance_danger") {
                Some(value) => value,
                None        => 2
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
            in_danger: false,
            x: start_x,
            y: start_y,
        };
        player.view_resource = player.view_resource_max;

        player
    }
    //}}}

    pub fn get_view_distance(&self) -> u8 //{{{
    {
        if self.view_resource > 0 {
            if self.in_danger {
                self.view_distance_danger
            } else {
                self.view_distance
            }
        } else {
            0
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
        -> Result<(), (usize, usize)>
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

        if map.tiles[new_x][new_y].passable {
            self.x = new_x;
            self.y = new_y;
            Ok(())
        } else {
            Err((new_x, new_y))
        }
    }
    //}}}

    pub fn update //{{{
    (
        &mut self,
        key: &Keycode,
        map: &Map,
        monster: &Kobold,
        resources: &Resources,
        event_system: &EventSubsystem
    ) -> bool
    {
        let mut updated: bool = false;

        // Save some fields before update.
        // Needed for after-update checks
        let previous_view_resource = self.view_resource;
        let mut move_result: Option<Result<(), (usize, usize)>> = None;

        //{{{ Reaction to keypresses
        match *key {
            // Movement
            Keycode::Up
            | Keycode::Kp8
            | Keycode::Num8
                => move_result = Some(self.move_relative(0, -1, map)),
            Keycode::Down
            | Keycode::Kp2
            | Keycode::Num2
                => move_result = Some(self.move_relative(0, 1, map)),
            Keycode::Left
            | Keycode::Kp4
            | Keycode::Num4
                => move_result = Some(self.move_relative(-1, 0, map)),
            Keycode::Right
            | Keycode::Kp6
            | Keycode::Num6
                => move_result = Some(self.move_relative(1, 0, map)),
            Keycode::Kp1
            | Keycode::Num1
                => move_result = Some(self.move_relative(-1, 1, map)),
            Keycode::Kp3
            | Keycode::Num3
                => move_result = Some(self.move_relative(1, 1, map)),
            Keycode::Kp7
            | Keycode::Num7
                => move_result = Some(self.move_relative(-1, -1, map)),
            Keycode::Kp9
            | Keycode::Num9
                => move_result = Some(self.move_relative(1, -1, map)),

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
                    updated = true;
                },

            _   => return false
        }
        //}}}

        //{{{ Checks of new state

        // Moved?
        if let Some(result) = move_result {
            match result {
                Ok(_)       => {
                    self.drain_view_resource();

                    if let TileType::Curiosity
                        = map.tiles[self.x][self.y].ttype {
                        let curio_found = EventCurioFound {
                            scene: map.tiles[self.x][self.y].search_text.clone()
                        };
                        event_system.push_custom_event(curio_found).unwrap();
                    }

                    updated = true;
                },
                Err((x, y)) => {
                    if let TileType::Obstacle = map.tiles[x][y].ttype {
                        let obstacle_found = EventObstacleFound{
                            text: map.tiles[x][y].search_text.clone()
                        };
                        event_system
                            .push_custom_event(obstacle_found)
                            .unwrap();
                        updated = true;
                    }
                },
            }
        }

        self.in_danger = false;
        if monster.alive {
            // Monster is close?
            let self_loc = (self.x, self.y);
            let monster_loc = (monster.x, monster.y);

            if map.get_distance(&self_loc, &monster_loc)
                < monster.danger_distance
            {
                if let Some(dist) = map
                    .get_path_distance(&self_loc, &monster_loc)
                {
                    if dist < monster.danger_distance {
                        self.in_danger = true;
                        let in_danger = EventPlayerInDanger{};
                        event_system.push_custom_event(in_danger).unwrap();
                    }
                }
            }

            // Monster met?
            if self_loc == monster_loc {
                let meet_monster = EventPlayerMeetMonster{};
                event_system.push_custom_event(meet_monster).unwrap();
            }
        }

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

        updated
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
pub struct Kobold {
    alive: bool,
    danger_distance: u8,
    pub x: usize,
    pub y: usize,
}
impl Kobold {
    pub fn init(map: &Map) -> Kobold {
        // Default
        let mut kobold = Kobold {
            alive: true,
            danger_distance: match get_setting("kobold_danger_dist") {
                Some(value) => value,
                None        => 5,
            } + 1,
            x: 0,
            y: 0,
        };

        match map.get_location("lair") {
            Some((x, y)) => {
                kobold.x = x;
                kobold.y = y;
            },
            None => kobold.alive = false
        }

        kobold
    }

    pub fn update(&mut self, map: &Map) {
        if self.alive {
            let mut possible_steps = map.get_neighbours(&(self.x, self.y));
            possible_steps.retain(
                |location| {
                    let &((x, y), _) = location;
                    map.tiles[x][y].passable
                }
            );
            let maybe_step = thread_rng().choose(&possible_steps);
            if let Some(&((sx, sy), _)) = maybe_step {
                self.x = sx;
                self.y = sy;
            }
        }
    }

    pub fn die(&mut self) {
        self.alive = false;
    }
}
//}}}

/*
 * Custom events
 */
pub struct EventResourceFound     {index: usize}
pub struct EventResourceGone      {}
pub struct EventResourceRefill    {pub success: bool}
pub struct EventObstacleFound     {pub text: String}
pub struct EventCurioFound        {pub scene: String}
pub struct EventPlayerInDanger    {}
pub struct EventPlayerMeetMonster {}

pub fn init_custom_events(sdl_event: &EventSubsystem) {
    sdl_event.register_custom_event::<EventResourceFound>()
        .expect("Failed to register event.");
    sdl_event.register_custom_event::<EventResourceGone>()
        .expect("Failed to register event.");
    sdl_event.register_custom_event::<EventResourceRefill>()
        .expect("Failed to register event.");
    sdl_event.register_custom_event::<EventObstacleFound>()
        .expect("Failed to register event.");
    sdl_event.register_custom_event::<EventCurioFound>()
        .expect("Failed to register event.");
    sdl_event.register_custom_event::<EventPlayerInDanger>()
        .expect("Failed to register event.");
    sdl_event.register_custom_event::<EventPlayerMeetMonster>()
        .expect("Failed to register event.");
}
