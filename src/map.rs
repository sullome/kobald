use std::collections::HashMap;
use std::path::PathBuf;
use rand::{Rng, StdRng};
use rusqlite::{Connection, DatabaseName, OpenFlags};
use std::io::Read;

const CARDS_FIELDS_COUNT: usize = 18;
const CARDS_ENDS_COUNT:   usize = 6;

const CARDS_MAP_SIDE:     usize = 3;

use super::DB_FILENAME;
const DB_TABLE_W_CARDS: &'static str        = "cards";
const DB_TABLE_W_CARDS_COLUMN: &'static str = "tiles";

//{{{ Tile
#[derive(Copy,Clone)]
pub enum TileType {
    Start,
    Floor,
    Wall,
    Obstacle,
    Curiousity
}

#[derive(Clone)]
pub struct Tile {
    pub ttype: TileType,
    passable: bool,

    // Name of the icon
    icon: String
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
                            '#' => Tile {
                                ttype: TileType::Wall,
                                passable: false,
                                icon: String::from("wall.png"),
                            },
                            '_' => Tile {
                                ttype: TileType::Floor,
                                passable: true,
                                icon: String::from("floor.png"),
                            },
                            's' => Tile {
                                ttype: TileType::Start,
                                passable: true,
                                icon: String::from("floor.png"),
                            },
                            'x' => Tile {
                                ttype: TileType::Obstacle,
                                passable: false,
                                icon: String::from("floor.png"),
                            },
                            '?' => Tile {
                                ttype: TileType::Curiousity,
                                passable: true,
                                icon: String::from("floor.png"),
                            },
                            _   => Tile {
                                ttype: TileType::Floor,
                                passable: true,
                                icon: String::from("floor.png"),
                            },
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

/*
 * Basic initialization of cards is handled by this function
 */
fn init_cards //{{{
    () -> Result<
        (Vec<Card>, Vec<Card>, HashMap<&'static str, Card>, Card),
        ()
    >
{
    // Generic query for retrieving rowids of cards
    let query = String::from("select rowid from ")
        + DB_TABLE_W_CARDS
        + " where ctype like ?;"
    ;

    let mut db_path = PathBuf::from(".");
    db_path.push(DB_FILENAME);

    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
    match Connection::open_with_flags(&db_path, flags) {
        Ok(db_connection) => {
            match db_connection.prepare(&query) {
                Ok(mut query_statement) => {
                    let field_cards: Vec<Card>;
                    let end_cards:   Vec<Card>;
                    let mut dead_ends:   HashMap<&'static str, Card> =
                        HashMap::with_capacity(4);
                    let corner_card: Card;

                    // Retrieving field cards
                    field_cards = query_statement.query_map(
                        &[&"field"],
                        |row| {
                            let rowid: i64 = row.get(0);
                            Card::new(rowid, &db_connection).unwrap()
                        }
                    ).unwrap().map(
                        |row| row.unwrap()
                    ).collect();

                    // Retrieving end cards
                    end_cards = query_statement.query_map(
                        &[&"end"],
                        |row| {
                            let rowid: i64 = row.get(0);
                            Card::new(rowid, &db_connection).unwrap()
                        }
                    ).unwrap().map(
                        |row| row.unwrap()
                    ).collect();

                    // Retrieving dead end cards
                    for side in ["left", "top", "right", "bottom"].iter() {
                        let rowid: i64 = query_statement
                            .query_row(
                                &[side],
                                |row| row.get(0)
                            )
                            .unwrap()
                        ;
                        dead_ends.insert(
                            side,
                            Card::new(rowid, &db_connection).unwrap()
                        );
                    }

                    // Retrieving corner card
                    corner_card = query_statement.query_row(
                        &[&"corner"],
                        |row| {
                            let rowid: i64 = row.get(0);
                            Card::new(rowid, &db_connection).unwrap()
                        }
                    ).unwrap();

                    Ok((field_cards, end_cards, dead_ends, corner_card))
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
(
    fields: &mut Vec<Card>, ends: &mut Vec<Card>,
    dead_ends: &HashMap<&str, Card>, corner: &Card
) -> Vec<Vec<Card>>
{
    let mut random_number_generator = StdRng::new()
        .expect("Failed to read randomness from operating system.");
    random_number_generator.shuffle(fields);

    // Ends makes a "border" of width equal to 1 card around fields
    let cardfield_side: usize = CARDS_MAP_SIDE + 2;
    let end_index: usize = cardfield_side - 1;

    // Row of columns
    let mut cardfield: Vec<Vec<Card>> = Vec::with_capacity(cardfield_side);

    // Direction is important because of Vec::push()
    for x in 0..cardfield_side {
        // Columns themselves
        cardfield.push(Vec::with_capacity(cardfield_side));

        // Inserting cards
        for y in 0..cardfield_side {
            match (x, y) {
                // Catch corners
                (0, 0)
                    => cardfield[x].push(corner.clone()),
                (coord, 0) | (0, coord)
                    if coord == end_index
                    => cardfield[x].push(corner.clone()),
                (coordx, coordy)
                    if (coordx == end_index && coordy == end_index)
                    => cardfield[x].push(corner.clone()),

                // Then catch ends
                (0, ..)
                    => cardfield[x].push(dead_ends["left"].clone()),
                (.., 0)
                    => cardfield[x].push(dead_ends["top"].clone()),
                (coord, ..)
                    if coord == end_index
                    => cardfield[x].push(dead_ends["right"].clone()),
                (.., coord)
                    if coord == end_index
                    => cardfield[x].push(dead_ends["bottom"].clone()),

                // Now we can catch everything else - main field
                _ => {
                    if let Some(field) = fields.pop() {
                        cardfield[x].push(field)
                    }
                }
            }
        }
    }

    // Now placing special ends
    let mut possible_locations: Vec<(usize, usize)> = Vec::with_capacity(
        CARDS_MAP_SIDE * 4
    );
    for x in 1..end_index {
        // Top row, without corners
        possible_locations.push((x, 0));

        // Bottom row, without corners
        possible_locations.push((x, end_index));
    }
    for y in 1..end_index {
        // Left column, without corners
        possible_locations.push((0, y));

        // Right column, without corners
        possible_locations.push((end_index, y));
    }

    random_number_generator.shuffle(&mut possible_locations);
    random_number_generator.shuffle(ends);
    for (&(x, y), end) in possible_locations.iter().zip(ends.iter()) {
        cardfield[x][y] = end.clone();
    }

    cardfield
}
//}}}

/*
 * This funciton takes field of cards
 * and translates it into the game map
 */
fn generate_map //{{{
    (cards_field: &Vec<Vec<Card>>) -> Vec<Vec<Tile>>
{
    // Cards are square and equal. This is a must.
    let corner_card = &cards_field[0][0];
    let card_side = corner_card.tiles.len();
    let field_side = cards_field.len();

    // Declare default tile
    let tile_wall = Tile {
        ttype: TileType::Wall,
        passable: false,
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
                    map[offset_x + x][offset_y + y] = tile.clone();
                }
            }
        }
    }
    map
}
//}}}

pub fn init
    () -> Vec<Vec<Tile>>
{
    let (mut fields, mut ends, dead_ends, corner) = init_cards().expect(
        "Failed to init cards."
    );
    let cards_field = generate_field(
        &mut fields, &mut ends, &dead_ends, &corner
    );
    generate_map(&cards_field)
}
