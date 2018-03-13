extern crate sevend;
extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sevend::map::Map;
use sevend::creature::Player;
use sevend::graphics::*;

fn main() {
    // Initializing SDL2 variables
    let (mut canvas, mut eventpump) = init_sdl2();

    // Init textures
    let texture_creator = canvas.texture_creator();
    let textures = init_textures(&texture_creator);

    // Updating window configuration
    configure_window(canvas.window_mut(), &textures);

    // Init game variables
    let mut map = Map::init();
    let mut player = Player::init(map.start.0, map.start.1);

    // Init GUI parts
    let mut textline = TextLine::init();
    let background_image = Background::init();

    // Init GUI elements
    let background = GUIElement::init("bg");
    let gamearea = GUIElement::init("map");
    let text = GUIElement::init("text");

    'running: loop {
        // Events handling
        for event in eventpump.poll_iter() {
            match event {
                Event::Quit{..}
                    => {
                        println!("QUIT");
                        break 'running;
                    },
                Event::KeyDown {keycode: Some(Keycode::S), .. }
                    => { },
                e => player.update(&e, &map)
            }
        }

        // Update game
        map.update(&player);

        // Start drawing
        canvas.clear();

        background.draw(
            &textures, &mut canvas,
            vec![&background_image]
        );
        gamearea.draw(
            &textures, &mut canvas,
            vec![&map, &player]
        );
        text.draw(
            &textures, &mut canvas,
            vec![&textline]
        );

        // Stop drawing
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(16));
    }
}
