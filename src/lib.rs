extern crate rusqlite;
extern crate rand;

mod map {
    use rand::{Rng, StdRng};

    const CARDS_FIELDS_COUNT: usize = 18;
    const CARDS_ENDS_COUNT:   usize = 5;

    const CARDS_MAP_SIDE:     usize = 3;

    #[derive(Copy,Clone)]
    enum Tile_Type {
        Floor,
        Wall,
        Obstacle,
        Curiousity
    }

    #[derive(Clone)]
    struct Tile {
        ttype: Tile_Type,
        passable: bool,

        // Name of the icon
        icon: String
    }

    #[derive(Clone)]
    struct Card (
        // Row of columns!!!
        // tiles[x][y]
        Vec<Vec<Tile>>
    );

    /*
     * Basic initialization handled by this function
     */
    fn init() {
        let field_cards:   Vec<Card> = Vec::with_capacity(CARDS_FIELDS_COUNT);
        let end_cards:     Vec<Card> =
            // We need to fully cover ends "border"
            // But exclude corners
            if CARDS_MAP_SIDE * 4 > CARDS_ENDS_COUNT {
                Vec::with_capacity(CARDS_MAP_SIDE * 4)
            } else {
                Vec::with_capacity(CARDS_ENDS_COUNT)
            }
        ;
        let dead_end_card: Card;
        let corner_card:   Card;

        /*
         * We need to push dead ends to the end_cards,
         * so that amount of end cards is equal (or greater)
         * to the end locations.
         *
         * if CARDS_MAP_SIDE * 4 > CARDS_ENDS_COUNT {
         *     for i in 0..{CARDS_MAP_SIDE * 4 - CARDS_ENDS_COUNT} {
         *         end_cards.push(dead_end_card);
         *     }
         * }
         */
    }

    /*
     * This function generates random field of cards
     */
    fn generate_field(
        fields: &mut Vec<Card>, ends: &mut Vec<Card>,
        dead_end: &Card, corner: &Card
    ) -> Vec<Vec<Card>>
    {
        let mut random_number_generator = StdRng::new()
            .expect("Failed to read randomness from operating system.");
        random_number_generator.shuffle(fields);
        random_number_generator.shuffle(ends);

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
                    (0, 0) => cardfield[x].push(corner.clone()), // Patterns TODO
                      (coord, 0)
                    | (0, coord)
                    | (coord, coord)
                    if coord == end_index
                    => cardfield[x].push(corner.clone()),

                    // Then catch ends
                      (0, ..)
                    | (.., 0)
                    => {
                        if let Some(end) = ends.pop() {
                            cardfield[x].push(end)
                        }
                    },
                      (coord, ..)
                    | (.., coord)
                    if coord == end_index
                    => {
                        if let Some(end) = ends.pop() {
                            cardfield[x].push(end)
                        }
                    },

                    // Now we can catch everything else - main field
                    _ => {
                        if let Some(field) = fields.pop() {
                            cardfield[x].push(field)
                        }
                    }
                }
            }
        }
        cardfield
    }

    /*
     * This funciton takes field of cards
     * and translates it into the game map
     */
    fn generate_map(cards_field: &Vec<Vec<Card>>) -> Vec<Vec<Tile>> {
        // Cards are square and equal. This is a must.
        let corner_card = cards_field[0][0];
        let card_side = len(corner_card);
        let field_side = len(cards_field);

        // Declare default tile
        let tile_wall = Tile{
            ttype: Tile_Type::Wall,
            passable: false,
            icon: String::from("wall.png"),
        };

        // Now we can create our map
        let map_side: usize = field_side * card_side;

        // Row of columns
        let mut map: Vec<Vec<Tile>> = Vec::with_capacity(map_side);

        // Basic initialization
        for x in 0..map_side {
            let column = vec![tile_wall; map_side];
            map.push(column);
        }

        // Feeding actual tiles
        for (field_x, &field_column) in cards_field.iter().enumerate() {
            for (field_y, field) in field_column.iter().enumerate() {
                let offset_x: usize = field_x * field_side;
                let offset_y: usize = field_y * field_side;

                for (x, card_column) in field.iter().enumerate() {
                    for (y, tile) in card_column.iter().enumerate() {
                        map[offset_x + x][offset_y + y] = tile;
                    }
                }
            }
        }
        map
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
