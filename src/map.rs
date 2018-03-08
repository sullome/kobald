use std::collections::HashMap;
use std::path::PathBuf;

use rand::{Rng, StdRng};
use rusqlite::{Connection, DatabaseName, OpenFlags};
use std::io::Read;
use sdl2::render::{Canvas,Texture,RenderTarget};
use sdl2::rect::Rect;

use super::creature::Player;

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
    pub passable: bool,
    visible: bool,
    curiosity_checked: bool,
    search_text: String,

    // Name of the icon
    icon: String
}
impl Tile {
    pub fn init<T: Rng>
    (
        tile_char: &char,
        card_id: &i64,
        db_conn: &Connection,
        rng: &mut T
    )
    -> Tile
    {
        let tile_type = match *tile_char {
            '#' => TileType::Wall,
            '_' => TileType::Floor,
            's' => TileType::Start,
            'x' => TileType::Obstacle,
            '?' => TileType::Curiousity,
            _   => TileType::Floor,
        };
        let tile_image = String::from(match tile_type {
            TileType::Wall => "wall.png",
            _ => "floor.png",
        });
        let tile_pass = match tile_type {
            TileType::Wall | TileType::Obstacle => false,
            _ => true,
        };

        let query: String = match tile_type {
            TileType::Curiousity => String::from(
                "select message from messages where situation = ?;"
            ),
            _ => String::from("select message ")
                + "from messages "
                + "where situation in ("
                +     "select ctype "
                +     "from " + DB_TABLE_W_CARDS + " "
                +     "where rowid = ?"
                + ");"
                ,
        };
        let mut statement = db_conn.prepare(&query).unwrap();

        let messages: Vec<String> = match tile_type {
            TileType::Curiousity => statement.query_map(
                &[card_id],
                |row| {
                    let s:String = row.get(0);
                    s
                }
            ).unwrap().map(
                |row| row.unwrap()
            ).collect(),
            TileType::Wall => statement.query_map(
                &[&"wall"],
                |row| {
                    let s:String = row.get(0);
                    s
                }
            ).unwrap().map(
                |row| row.unwrap()
            ).collect(),
            TileType::Floor => statement.query_map(
                &[&"floor"],
                |row| {
                    let s:String = row.get(0);
                    s
                }
            ).unwrap().map(
                |row| row.unwrap()
            ).collect(),
            TileType::Start => statement.query_map(
                &[&"start"],
                |row| {
                    let s:String = row.get(0);
                    s
                }
            ).unwrap().map(
                |row| row.unwrap()
            ).collect(),
            TileType::Obstacle => statement.query_map(
                &[&"obstacle"],
                |row| {
                    let s:String = row.get(0);
                    s
                }
            ).unwrap().map(
                |row| row.unwrap()
            ).collect()
        };

        let message: String = match rng.choose(&messages){
            Some(m) => m.clone(),
            None    => String::from("")
        };

        Tile {
            ttype: tile_type,
            passable: tile_pass,
            visible: false,
            curiosity_checked: false,
            search_text: message,
            icon: tile_image
        }
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
                        let tile: Tile = Tile::init(
                            &tile_char,
                            &id,
                            db_conn,
                            &mut random_number_generator
                        );
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
        let (mut fields, mut ends, dead_ends, corner) = init_cards().expect(
            "Failed to init cards."
        );
        let cards_field = generate_field(
            &mut fields, &mut ends, &dead_ends, &corner
        );
        let (tiles, start) = generate_map(&cards_field);
        Map {
            tiles,
            start
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

    pub fn draw<T: RenderTarget>
        (&self, textures: &HashMap<String, Texture>, canvas: &mut Canvas<T>)
    {
        let texture_side: u32 = textures["wall.png"].query().width;
        let mut place: Rect = Rect::new(0, 0, texture_side, texture_side);

        for (x, column) in self.tiles.iter().enumerate() {
            for (y, tile) in column.iter().enumerate() {
                if tile.visible {
                    let texture: &Texture = &textures[&tile.icon];
                    place.set_x(((x as u32) * texture_side) as i32);
                    place.set_y(((y as u32) * texture_side) as i32);

                    canvas.copy(texture, None, place)
                        .expect("Texture rendering error!");
                }
            }
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
                        &[&"end%"],
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
    let mut start: (usize, usize) = (0, 0);
    for (field_x, field_column) in cards_field.iter().enumerate() {
        for (field_y, field) in field_column.iter().enumerate() {
            let offset_x: usize = field_x * card_side;
            let offset_y: usize = field_y * card_side;

            for (x, card_column) in field.tiles.iter().enumerate() {
                for (y, tile) in card_column.iter().enumerate() {
                    let tile_x = offset_x + x;
                    let tile_y = offset_y + y;
                    map[tile_x][tile_y] = tile.clone();

                    if let TileType::Start = tile.ttype {
                        start = (tile_x, tile_y);
                    }
                }
            }
        }
    }
    (map, start)
}
//}}}
