use sdl2;
use sdl2::Sdl;
use sdl2::mixer::{DEFAULT_FREQUENCY, DEFAULT_FORMAT, DEFAULT_CHANNELS};
use sdl2::mixer::INIT_MP3;

pub fn init(sdl_context: &Sdl) {
    let _sdl_audio = sdl_context.audio()
        .expect("SDL AudioSubsystem initialization failed");

    let chunk_size = 1_024;
    sdl2::mixer::open_audio(
        DEFAULT_FREQUENCY,
        DEFAULT_FORMAT,
        DEFAULT_CHANNELS,
        chunk_size
    ).expect("Failed to open audio device");

    let _sdl_mixer_context = sdl2::mixer::init(INIT_MP3)
        .expect("SDL Mixer initialization failed");

    sdl2::mixer::allocate_channels(1);
}
