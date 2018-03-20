extern crate sevend;
extern crate sdl2;

use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sevend::map::Map;
use sevend::objects::{Player, Resources};
use sevend::objects::EventResourceFound;
use sevend::objects::EventResourceGone;
use sevend::objects::EventResourceRefill;
use sevend::objects::EventObstacleFound;
use sevend::objects::EventCurioFound;
use sevend::graphics::*;

fn main() {
    // Initializing SDL2 variables
    let (mut canvas, mut eventpump, event_system) = init_sdl2();
    sdl2::mixer::open_audio(44_100, sdl2::mixer::AUDIO_S16LSB, sdl2::mixer::DEFAULT_CHANNELS, 1_024);
    let _sdl_mixer = sdl2::mixer::init(sdl2::mixer::INIT_MP3)
        .expect("SDL Mixer initialization error.");

    // Init textures
    let texture_creator = canvas.texture_creator();
    let textures = init_textures(&texture_creator);

    // Updating window configuration
    configure_window(canvas.window_mut(), &textures);

    // Init game variables
    let mut map = Map::init();
    let mut player = Player::init(map.start.0, map.start.1);
    let mut resources = Resources::init(&map, &player);
    map.update(&player);
    sevend::objects::init_custom_events(&event_system);
    let mut happy_end = false;
    let mut end = false;

    // Init GUI parts
    let mut textline = TextLine::init();
    let background_image = Background::init();
    let mut resource_counter = ResourceCounter::init(&player);
    let mut textscene = TextScene::init();

    // Init GUI elements
    let background = GUIElement::init("bg");
    let gamearea = GUIElement::init("map");
    let text = GUIElement::init("text");
    let resource_place = GUIElement::init("flask");
    let scene = GUIElement::init("scene");

    'running: loop {
        // Events handling
        for event in eventpump.poll_iter() {
            match event {
                Event::Quit{..}
                    => {
                        break 'running;
                    },
                Event::KeyDown{keycode: Some(Keycode::Return), ..}
                    => {
                        if textscene.active {
                            textscene.active = false;

                            if textscene.scene.starts_with("end_") {
                                break 'running;
                            }

                            if end {
                                textscene.active = true;
                                textscene.scene = if happy_end {
                                    String::from("end_good")
                                } else {
                                    String::from("end_bad")
                                }
                            }
                        }
                    },
                Event::KeyDown{keycode: Some(kcode), ..}
                    => {
                        if !textscene.active {
                            // Update game
                            player.update(
                                &kcode,
                                &map,
                                &resources,
                                &event_system
                            );
                            map.update(&player);
                            resource_counter.update(&player);
                            textline.update();
                        }
                    },
                ref custom_event if custom_event.is_user_event()
                    => {
                        //{{{ EventResourceRefill
                        if let Some(resource_refill) = custom_event
                            .as_user_event_type::<EventResourceRefill>()
                        {
                            if resource_refill.success {
                                textline.set_situation("resource_refill");
                            } else {
                                textline.set_situation("resource_absent");
                            }
                        }
                        //}}}

                        //{{{ EventResourceFound
                        if let Some(resource_found) = custom_event
                            .as_user_event_type::<EventResourceFound>()
                        {
                            resources.process_event(&resource_found);
                            player.add_view_resource_count();

                            textline.set_any_situation("resource_found");
                        }
                        //}}}

                        //{{{ EventResourceGone
                        if let Some(resource_gone) = custom_event
                            .as_user_event_type::<EventResourceGone>()
                        {
                            textline.set_any_situation("resource_gone");
                        }
                        //}}}

                        //{{{ EventObstacleFound
                        if let Some(obstacle_found) = custom_event
                            .as_user_event_type::<EventObstacleFound>()
                        {
                            textline.set_situation(&obstacle_found.text);
                        }
                        //}}}

                        //{{{ EventCurioFound
                        if let Some(curio_found) = custom_event
                            .as_user_event_type::<EventCurioFound>()
                        {
                            textscene.active = true;
                            textscene.scene = curio_found.scene;
                            match textscene.scene.as_str() {
                                "item" => happy_end = true,
                                "lair" => if !happy_end {
                                    end = true;
                                },
                                "children" => {
                                    happy_end = true;
                                    end = true;
                                },
                                _ => ()
                            }
                        }
                        //}}}
                    },
                _ => ()
            }
        }


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
        resource_place.draw(
            &textures, &mut canvas,
            vec![&resource_counter]
        );
        if textscene.active {
            scene.draw(
                &textures, &mut canvas,
                vec![&textscene]
            );
        } else {
            text.draw(
                &textures, &mut canvas,
                vec![&textline]
            );
        }

        // Stop drawing
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(16));
    }
}
