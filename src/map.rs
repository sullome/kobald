use std::path::PathBuf;

use rand::{Rng, StdRng, thread_rng};
use rusqlite::{Connection, DatabaseName, OpenFlags};
use std::io::Read;

use super::get_setting;
use super::objects::Player;

const CARDS_FIELDS_COUNT: usize = 18;
const ENDS_COUNT:         usize = 6;

const CARDS_MAP_SIDE:     usize = 3;

use super::DB_FILENAME;
const DB_TABLE_W_CARDS:        &'static str = "cards";
const DB_TABLE_W_CARDS_COLUMN: &'static str = "tiles";

//{{{ Tile
#[derive(Copy,Clone)]
pub enum TileType {
    Floor,
    Wall,
    Obstacle,
    Curiosity
}

#[derive(Copy,Clone)]
pub enum EndType {
    Children,
    Body,
    Lair,
    Item,
    Rest
}

#[derive(Clone)]
pub struct Tile {
    pub ttype: TileType,
    pub passable: bool,
    pub visible: bool,
    curiosity_checked: bool,
    pub search_text: String,

    // Name of the icon
    pub icon: String
}
impl Tile {
    pub fn init_regular<T: Rng>
    (
        tile_type: TileType,
        db_conn: &Connection,
        rng: &mut T
    )
    -> Option<Tile>
    {
        if let TileType::Curiosity = tile_type {
            return None
        }

        let tile_image = String::from(match tile_type {
            TileType::Wall => "wall.png",
            _ => "floor.png",
        });
        let tile_pass = match tile_type {
            TileType::Wall | TileType::Obstacle => false,
            _ => true,
        };

        let message: String = match tile_type {
            TileType::Obstacle => {
                let query = String::from("select situation ")
                    + "from messages "
                    + "where situation like ?;"
                    ;
                let mut statement = db_conn.prepare(&query).unwrap();

                let obstacles: Vec<String> = statement.query_map(
                    &[&"obstacle%"],
                    |row| {
                        let s:String = row.get(0);
                        s
                    }
                ).unwrap().map(
                    |row| row.unwrap()
                ).collect();

                match rng.choose(&obstacles) {
                    Some(obstacle) => obstacle.clone(),
                    None           => String::from("empty")
                }
            },
            _ => String::from("empty")
        };

        Some(Tile {
            ttype: tile_type,
            passable: tile_pass,
            visible: false,
            curiosity_checked: false,
            search_text: message,
            icon: tile_image
        })
    }

    pub fn init_curio<T: Rng>
        (end_type: EndType, rng: &mut T) -> Option<Tile>
    {
        let scene: String = String::from(
            match end_type {
                EndType::Children => "children",
                EndType::Body     => "body",
                EndType::Lair     => "lair",
                EndType::Item     => "item",
                EndType::Rest     => "rest"
            }
        );

        Some(Tile {
            ttype: TileType::Curiosity,
            passable: true,
            visible: false,
            curiosity_checked: false,
            search_text: scene,
            icon: String::from("floor.png")
        })
    }
}
//}}}

//{{{ Card
#[derive(Clone)]
struct Card {
    // Row of columns!!!
    // tiles[x][y]
    tiles: Vec<Vec<Tile>>
}
impl Card {
    pub fn new(id: i64, db_conn: &Connection) -> Result<Card, ()> {
        if let Ok(mut tiles_blob) = db_conn.blob_open(
            DatabaseName::Main,
            DB_TABLE_W_CARDS,
            DB_TABLE_W_CARDS_COLUMN,
            id,
            true                    // Read-Only
        ) {
            let mut tiles_string = String::with_capacity(
                tiles_blob.size() as usize
            );
            if let Ok(_) = tiles_blob.read_to_string(&mut tiles_string) {
                let card_side: usize = tiles_string.lines().count();

                let mut tiles_chars: Vec<Vec<char>> =
                    // Row of columns
                    Vec::with_capacity(card_side)
                ;
                for _x in 0..card_side {
                    // Columns
                    let column: Vec<char> = vec!['z'; card_side];
                    tiles_chars.push(column);
                }
                for (y, line) in tiles_string.lines().enumerate() {
                    for (x, c) in line.chars().enumerate() {
                        tiles_chars[x][y] = c;
                    }
                }

                let mut random_number_generator = StdRng::new()
                    .expect(
                        "Failed to read randomness from operating system."
                    )
                ;

                let mut card = Card {
                    // Row of columns
                    tiles: Vec::with_capacity(card_side),
                };
                for x in 0..card_side {
                    // Columns themselves
                    card.tiles.push(Vec::with_capacity(card_side));

                    for y in 0..card_side {
                        let tile_char: char = tiles_chars[x][y];
                        let tile: Tile = match tile_char {
                            '#' => Tile::init_regular(
                                TileType::Wall,
                                db_conn,
                                &mut random_number_generator
                            ).unwrap(),
                            '_' => Tile::init_regular(
                                TileType::Floor,
                                db_conn,
                                &mut random_number_generator
                            ).unwrap(),
                            _ => Tile::init_regular(
                                TileType::Wall,
                                db_conn,
                                &mut random_number_generator
                            ).unwrap(),
                        };
                        card.tiles[x].push(tile);
                    }
                }
                Ok(card)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
//}}}

//{{{ Map
pub struct Map {
    // Row of columns!!!
    // tiles[x][y]
    pub tiles: Vec<Vec<Tile>>,
    pub start: (usize, usize)
}
impl Map {
    pub fn init () -> Map {
        let mut fields = init_cards().expect(
            "Failed to init cards."
        );
        let cards_field = generate_field(&mut fields);
        let (tiles, start) = generate_map(&cards_field);
        let mut map = Map {
            tiles,
            start
        };
        map.add_obstacles();
        map
    }

    fn add_obstacles(&mut self) {
        let mut rng = StdRng::new()
            .expect("Failed to initialize randomness");

        let mut possible_locations: Vec<(usize, usize)> = Vec::new();
        let map_side = self.tiles.len();
        for x in 0..map_side {
            for y in 0..map_side {
                if let TileType::Floor = self.tiles[x][y].ttype {
                    possible_locations.push((x, y));
                }
            }
        }
        rng.shuffle(&mut possible_locations);

        let mut max_obstacles: u8 = match get_setting("obstacle_max") {
            Some(value) => value,
            None        => 6
        };
        max_obstacles = rng.gen_range(1, max_obstacles);

        let mut db_path = PathBuf::from(".");
        db_path.push(DB_FILENAME);

        let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
        let db_conn = Connection::open_with_flags(&db_path, flags).unwrap();

        for obstacle in 0..max_obstacles {
            if let Some((x, y)) = possible_locations.pop() {
                let obstacle_tile = Tile::init_regular(
                    TileType::Obstacle,
                    &db_conn,
                    &mut rng
                ).unwrap();
                self.tiles[x][y] = obstacle_tile;
            }
        }
    }

    pub fn update(&mut self, player: &Player) {
        let start_x = match player.x.checked_sub(
            player.get_view_distance() as usize
        ) {
            Some(x) => x,
            None    => 0
        };
        let start_y = match player.y.checked_sub(
            player.get_view_distance() as usize
        ) {
            Some(y) => y,
            None    => 0
        };

        let mut end_x = player.x + player.get_view_distance() as usize;
        let mut end_y = player.y + player.get_view_distance() as usize;
        let map_side = self.tiles.len();
        if (end_x > map_side) {end_x = map_side};
        if (end_y > map_side) {end_y = map_side};

        for x in start_x..end_x {
            for y in start_y..end_y {
                self.tiles[x][y].visible = true;
            }
        }
    }
}
//}}}

/*
 * Basic initialization of cards is handled by this function
 */
fn init_cards //{{{
    () -> Result<Vec<Card>, ()>
{
    // Generic query for retrieving rowids of cards
    let query = String::from("select rowid from ")
        + DB_TABLE_W_CARDS
        + ";"
    ;

    let mut db_path = PathBuf::from(".");
    db_path.push(DB_FILENAME);

    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
    match Connection::open_with_flags(&db_path, flags) {
        Ok(db_connection) => {
            match db_connection.prepare(&query) {
                Ok(mut query_statement) => {
                    // Retrieving cards
                    let field_cards: Vec<Card> = query_statement.query_map(
                        &[],
                        |row| {
                            let rowid: i64 = row.get(0);
                            Card::new(rowid, &db_connection).unwrap()
                        }
                    ).unwrap().map(
                        |row| row.unwrap()
                    ).collect();

                    Ok(field_cards)
                },
                Err(_) => return Err(())
            }
        },
        Err(_) => return Err(())
    }
}
//}}}

/*
 * This function generates random field of cards
 */
fn generate_field //{{{
    (fields: &mut Vec<Card>) -> Vec<Vec<Card>>
{
    let mut random_number_generator = StdRng::new()
        .expect("Failed to read randomness from operating system.");
    random_number_generator.shuffle(fields);

    // Ends makes a "border" of width equal to 1 card around fields
    // UPD: we do not need such border
    let cardfield_side: usize = CARDS_MAP_SIDE;
    let end_index: usize = cardfield_side - 1;

    // Row of columns
    let mut cardfield: Vec<Vec<Card>> = Vec::with_capacity(cardfield_side);

    // Direction is important because of Vec::push()
    for x in 0..cardfield_side {
        // Columns themselves
        cardfield.push(Vec::with_capacity(cardfield_side));

        // Inserting cards
        for y in 0..cardfield_side {
            /*
             * UPD: No border — no need to match anything
             * match (x, y) {
             *     // Catch corners
             *     (0, 0)
             *         => cardfield[x].push(corner.clone()),
             *     (coord, 0) | (0, coord)
             *         if coord == end_index
             *         => cardfield[x].push(corner.clone()),
             *     (coordx, coordy)
             *         if (coordx == end_index && coordy == end_index)
             *         => cardfield[x].push(corner.clone()),

             *     // Then catch ends
             *     (0, ..)
             *         => cardfield[x].push(dead_ends["left"].clone()),
             *     (.., 0)
             *         => cardfield[x].push(dead_ends["top"].clone()),
             *     (coord, ..)
             *         if coord == end_index
             *         => cardfield[x].push(dead_ends["right"].clone()),
             *     (.., coord)
             *         if coord == end_index
             *         => cardfield[x].push(dead_ends["bottom"].clone()),

             *     // Now we can catch everything else - main field
             *     _ => {
             *         if let Some(field) = fields.pop() {
             *             cardfield[x].push(field)
             *         }
             *     }
             * }
             */
            if let Some(field) = fields.pop() {
                cardfield[x].push(field)
            }
        }
    }

    // Now placing special ends
    /*
     * UPD: no border → no special ends
     * let mut possible_locations: Vec<(usize, usize)> = Vec::with_capacity(
     *     CARDS_MAP_SIDE * 4
     * );
     * for x in 1..end_index {
     *     // Top row, without corners
     *     possible_locations.push((x, 0));

     *     // Bottom row, without corners
     *     possible_locations.push((x, end_index));
     * }
     * for y in 1..end_index {
     *     // Left column, without corners
     *     possible_locations.push((0, y));

     *     // Right column, without corners
     *     possible_locations.push((end_index, y));
     * }

     * random_number_generator.shuffle(&mut possible_locations);
     * random_number_generator.shuffle(ends);
     * for (&(x, y), end) in possible_locations.iter().zip(ends.iter()) {
     *     cardfield[x][y] = end.clone();
     * }
     */

    cardfield
}
//}}}

/*
 * This funciton takes field of cards
 * and translates it into the game map
 */
fn generate_map //{{{
(
    cards_field: &Vec<Vec<Card>>
)
-> (Vec<Vec<Tile>>, (usize, usize))
{
    // Cards are square and equal. This is a must.
    let corner_card = &cards_field[0][0];
    let card_side = corner_card.tiles.len();
    let field_side = cards_field.len();

    // Declare default tile
    let tile_wall = Tile {
        ttype: TileType::Wall,
        passable: false,
        visible: false,
        curiosity_checked: false,
        search_text: String::from(""),
        icon: String::from("wall.png"),
    };

    // Now we can create our map
    let map_side: usize = field_side * card_side;

    // Row of columns
    let mut map: Vec<Vec<Tile>> = Vec::with_capacity(map_side);

    // Basic initialization
    for _x in 0..map_side {
        let mut column: Vec<Tile> = Vec::with_capacity(map_side);
        for _y in 0..map_side {
            column.push(tile_wall.clone());
        }
        map.push(column);
    }

    // Feeding actual tiles
    for (field_x, field_column) in cards_field.iter().enumerate() {
        for (field_y, field) in field_column.iter().enumerate() {
            let offset_x: usize = field_x * card_side;
            let offset_y: usize = field_y * card_side;

            for (x, card_column) in field.tiles.iter().enumerate() {
                for (y, tile) in card_column.iter().enumerate() {
                    let tile_x = offset_x + x;
                    let tile_y = offset_y + y;
                    map[tile_x][tile_y] = tile.clone();

                    /*
                     * UPD: Not here and not like that
                     * if let TileType::Start = tile.ttype {
                     *     start = (tile_x, tile_y);
                     * }
                     */
                }
            }
        }
    }

    // Finding dead ends
    let mut possible_locations: Vec<(usize, usize)> = Vec::with_capacity(
        CARDS_MAP_SIDE * 2 * 4
    );
    let max_coord = map.len();
    for y in 1..max_coord {
        // Top row
        if let TileType::Floor = map[0][y].ttype {
            possible_locations.push((0, y));
        }
        // Bottom row
        if let TileType::Floor = map[max_coord - 1][y].ttype {
            possible_locations.push((max_coord - 1, y));
        }
    }
    for x in 1..max_coord {
        // Top row
        if let TileType::Floor = map[x][0].ttype {
            possible_locations.push((x, 0));
        }
        // Bottom row
        if let TileType::Floor = map[x][max_coord - 1].ttype {
            possible_locations.push((x, max_coord - 1));
        }
    }

    let mut random_number_generator = StdRng::new()
        .expect("Failed to read randomness from operating system.");

    let mut ends: Vec<Tile> = vec![
        Tile::init_curio(EndType::Children, &mut random_number_generator).unwrap(),
        Tile::init_curio(EndType::Lair, &mut random_number_generator).unwrap(),
        Tile::init_curio(EndType::Body, &mut random_number_generator).unwrap(),
        Tile::init_curio(EndType::Item, &mut random_number_generator).unwrap(),
        Tile::init_curio(EndType::Rest, &mut random_number_generator).unwrap()
    ];

    random_number_generator.shuffle(&mut possible_locations);
    random_number_generator.shuffle(&mut ends);

    let (start_x, start_y) = possible_locations.pop().unwrap();

    for (&(x, y), end) in possible_locations.iter().zip(ends.iter()) {
        map[x][y] = end.clone();
    }

    (map, (start_x, start_y))
}
//}}}
