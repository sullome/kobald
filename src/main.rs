extern crate sevend;
extern crate sdl2;
extern crate rusqlite;

use std::time::Duration;
use std::path::PathBuf;

use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode, NOMOD};
use sdl2::mouse::MouseButton;
use sdl2::mixer::{DEFAULT_FORMAT, DEFAULT_CHANNELS};
use sdl2::mixer::INIT_MP3;
use sdl2::rwops::RWops;
use sdl2::mixer::LoaderRWops;
use sdl2::mixer::Music;
use rusqlite::{Connection, OpenFlags, DatabaseName};
use std::io::Read; // For Blob

use sevend::map::Map;
use sevend::objects::{Player, Resources, Kobold};
use sevend::objects::EventResourceFound;
use sevend::objects::EventResourceGone;
use sevend::objects::EventResourceRefill;
use sevend::objects::EventObstacleFound;
use sevend::objects::EventCurioFound;
use sevend::objects::EventPlayerInDanger;
use sevend::objects::EventPlayerMeetMonster;
use sevend::graphics;
use sevend::graphics::{GUIElement, Drawable};
use sevend::graphics::{TextLine, Background, ResourceCounter, TextScene};
use sevend::graphics::{init_textures, configure_window};
use sevend::sound;

use sevend::DB_FILENAME;

fn main() {
    // Initializing SDL2 variables
    let sdl_context = sdl2::init()
        .expect("SDL initialization error.");

    // Init textures
    let mut canvas = graphics::init(&sdl_context);
    let texture_creator = canvas.texture_creator();
    let textures = init_textures(&texture_creator);

    // Init sounds
    let _sdl_audio = sdl_context.audio()
        .expect("SDL AudioSubsystem initialization failed");

    let chunk_size = 1_024;
    let frequency = 44_100;
    sdl2::mixer::open_audio(
        frequency,
        DEFAULT_FORMAT,
        DEFAULT_CHANNELS,
        chunk_size
    ).expect("Failed to open audio device");

    let _sdl_mixer_context = sdl2::mixer::init(INIT_MP3)
        .expect("SDL Mixer initialization failed");

    sdl2::mixer::allocate_channels(2);

    let effects = sound::load_sounds();

    // Init events
    let mut sdl_eventpump = sdl_context.event_pump()
        .expect("SDL Event Pump initialization error.");
    let sdl_event = sdl_context.event()
        .expect("SDL Event subsystem initialization error.");
    sevend::objects::init_custom_events(&sdl_event);

    // Updating window configuration
    configure_window(canvas.window_mut(), &textures);

    // Init game variables
    let mut map = Map::init()
        .expect("Cannot run the game because of map generation error");
    let start = map.get_location("start").unwrap();
    let mut player = Player::init(start.0, start.1);
    let mut monster = Kobold::init(&map);
    let mut resources = Resources::init(&map, &player);
    map.update(&player);
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

    // Play background music
    let db_path: PathBuf = [".", DB_FILENAME].iter().collect();
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY;
    let db_connection = Connection::open_with_flags(&db_path, flags)
        .expect("Cannot read data");
    let mut music_blob = db_connection.blob_open(
        DatabaseName::Main,
        "musics",
        "music",
        1,
        true                // Read-Only
    ).expect("Cannot read image blob.");
    let mut music_bytes: Vec<u8> = Vec::new();
    music_blob.read_to_end(&mut music_bytes)
        .expect("Cannot read image bytes.");
    let music_stream = RWops::from_bytes(&music_bytes)
        .expect("Cannot open image bytes as a stream.");
    let music = music_stream.load_music()
        .expect("Cannot load music");
    music.play(-1);

    'running: loop {
        // Events handling
        for event in sdl_eventpump.poll_iter() {
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
                Event::MouseButtonDown{
                    mouse_btn: MouseButton::Left,
                    clicks: 1,
                    timestamp: e_timestamp,
                    window_id: e_window_id,
                    x: e_x,
                    y: e_y,
                    ..
                }
                    => {
                        if resource_place.contains(e_x, e_y) {
                            let false_event = Event::KeyDown {
                                timestamp: e_timestamp + 1,
                                window_id: e_window_id,
                                keycode: Some(Keycode::R),
                                scancode: Some(Scancode::R),
                                keymod: NOMOD,
                                repeat: false,
                            };
                            sdl_event.push_event(false_event);
                        }

                        if gamearea.contains(e_x, e_y) {
                            let (gamearea_x, gamearea_y): (i32, i32) = gamearea
                                .into_relative(e_x, e_y);
                            let texture_side: f32 = textures["floor.png"]
                                .query()
                                .width as f32;
                            let map_x: usize =
                                (gamearea_x as f32 / texture_side)
                                .floor() as usize;
                            let map_y: usize =
                                (gamearea_y as f32 / texture_side)
                                .floor() as usize;

                            map.add_mark(map_x, map_y);
                        }
                    },
                Event::KeyDown{keycode: Some(kcode), ..}
                    => {
                        if !textscene.active {
                            // Update game
                            if player.update(
                                &kcode,
                                &map,
                                &monster,
                                &resources,
                                &sdl_event
                            ) {
                                map.update(&player);
                                resource_counter.update(&player);
                                textline.update();
                                monster.update(&map);
                            }
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
                                sound::play_effect(&effects["match.wav"]);
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
                            sound::play_effect(&effects["fizzing.wav"]);
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
                                "item" => {
                                    monster.die();
                                    happy_end = true;
                                    sound::play_effect(&effects["shout.wav"]);
                                },
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

                        //{{{ EventPlayerInDanger
                        if let Some(_in_danger) = custom_event
                            .as_user_event_type::<EventPlayerInDanger>()
                        {
                            textline.set_any_situation("danger");
                        }
                        //}}}

                        //{{{ EventPlayerMeetMonster
                        if let Some(_meet_monster) = custom_event
                            .as_user_event_type::<EventPlayerMeetMonster>()
                        {
                            textscene.active = true;
                            textscene.scene = String::from("monster");
                            end = true;
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
