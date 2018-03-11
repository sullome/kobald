extern crate rusqlite;
extern crate rand;
extern crate sdl2;

pub const DB_FILENAME: &'static str = "data.sqlite3";
pub const MAP_OFFSET: i32 = 24;

pub mod creature;
pub mod map;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
