# kobald
This project started as a submission for 7DRL 2018 challenge. However, I quickly realized that it will take more time to finish the plan, so here it is. 1 Month of work.

The project made in Rust and SDL2. For data management SQLite 3 was used

## Installation
First of all, obtain files for your platform: https://github.com/sullome/kobald/releases/tag/v1.0.0
### Linux
You will need next libraries installed:
* SDL2 ≥ v2.0.8
* SDL2 Image ≥ v2.0.3
    * depends on libpng and zlib
* SDL2 TTF ≥ v2.0.14
* SDL2 Mixer ≥ v2.0.2
    * depends on mpg123
* SQLite ≥ v3

In ArchLinux those can be installed with:
```
sudo pacman -S sdl2 sdl2_image sdl2_mixer sdl2_ttf sqlite
```

Afterwards, unpack `.zip` archive and run `kobold` from the directory inside (`data.sqlite3` should be located in it).

### Windows
Unpack `.zip` archive and run `kobold.exe`.

### MacOS
Please check the linux section for the list of required libraries.
There can be problems with libpng, I would be happy to hear from someone who uses MacOS if anything works for him.

## Game guide
This is a game about an old farmer whose grandchildren run away and got lost in the abandoned mines. Old farmer enters the mines with his lantern, a pencil and a small paper list. But he is not alone in the mines…

Your goal is to find the place where children are hiding. Small hint: this place is located somewhere on the map border. And pay attention to the thoughts of yours, in the bottom of the map.

You can place a mark on your map with the left mouse button. Click on the mark again to remove it.

The map is generated randomly, so you always can try again.

## How to build
```
sqlite3 -init src/init_database.sql data.sqlite3 '.quit'
cargo run
```
