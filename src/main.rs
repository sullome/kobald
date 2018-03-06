extern crate sevend;

fn main() {
    let map = sevend::map::init();

    let side = map[0].len();
    for y in 0..side {
        for x in 0..side {
            match map[x][y] {
                sevend::map::Tile { ttype: sevend::map::TileType::Wall, .. } =>
                    print!("#"),
                _ => print!(".")
            }
        }
        print!("\n");
    }
}
