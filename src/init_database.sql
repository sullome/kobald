--{{{ Tables
create table cards (tiles BLOB);
create table images(name TEXT, image BLOB);
create table fonts(name TEXT, font BLOB);
create table messages(situation TEXT, message TEXT);
create table scenes(scene TEXT, message TEXT);
create table musics(name TEXT, music BLOB);
create table sound_effects(name TEXT, effect BLOB);
create table game_settings(setting TEXT, value NUMERIC);
--}}}

--{{{ Inserts
insert into game_settings(setting, value) values
    ('game_name',      'Kobold'),
    ('bg_x',                  0),
    ('bg_y',                  0),
    ('bg_w',                696),
    ('bg_h',                696),
    ('map_x',                24),
    ('map_y',                24),
    ('map_w',               576),
    ('map_h',               576),
    ('text_x',               24),
    ('text_y',              624),
    ('text_w',              576),
    ('text_h',               48),
    ('flask_x',             629),
    ('flask_y',             534),
    ('flask_w',              32),
    ('flask_h',             125),
    ('scene_x',              92),
    ('scene_y',              92),
    ('scene_w',             441),
    ('scene_h',             345),
    ('textscene_max_width', 431),
    ('textline_max_width',  576),
    ('textline_font_size',   22),
    ('textline_time_max',     5),
    ('visible_distance',      3),
    ('resource_max',         10),
    ('resource_start',      100),
    ('obstacle_max',          8);
insert into fonts(name, font) values
    ('DejaVu Serif', readfile('data/DejaVuSerif.ttf'));
insert into images(name, image) values
    ('wall.png',     readfile('data/tiles/wall.png')  ),
    ('mark.png',     readfile('data/tiles/mark.png')  ),
    ('floor.png',    readfile('data/tiles/floor.png') ),
    ('player.png',   readfile('data/icons/player.png')),
    ('flask.png',    readfile('data/icons/flask.png') ),
    ('map.png',      readfile('data/map.png')         ),
    ('scene_bg.png', readfile('data/scene_bg.png')    );
insert into musics(name, music) values
    ('la_femme.mp3', readfile('data/sounds/la_femme.mp3'));
insert into sound_effects(name, effect) values
    ('match.wav',   readfile('data/sounds/match_out.wav')    ),
    ('fizzing.wav', readfile('data/sounds/fizzing.wav')      ),
    ('shout.wav',   readfile('data/sounds/monster_shout.wav'));
insert into scenes(scene, message) values
    ('body', 'Oh no… I heard about this from my grandfather. Decades ago this mine was closed because of evil spirit killing miners. Wait, what is that? A note in his hand. «Seek the cursed item. Put a rusty needle out from the rotten heart to free him from his curse.» Looks like he sought something before he died in this cave in. Rest in peace, dead miner.'),
    ('rest', 'A dead end. But I can see the sun through the cracks above. How much time have passed since I entered this forgotten mines?.. My legs hurt and I am starving a little. But it is even worse for children. I must find them before dark. Grandfather told me some stories about this mines… Yes, I think I know where to seek my grandchildren.'),
    ('item', 'What is this place? And a chest? What such curious chest is doing in this mine? Ah! A human heart is inside! It is pierced with a rusty needle and is rotten… and smells horribly! I should close the lid. Oh… what was that?! I accidentally touched the needle and it crumbled to dust, and a horrible roar echoed through the mines… I need to hurry and find children before it is too late!'),
    ('lair', 'What is this horrible place? Looks like a lair of some beast… those bones. There are piles of them! Wait… all of them are human!..'),
    ('children', 'Children! My grandchildren, I found you! Please forgive me, forgive me for being rude. Come back home now, granny will make us all a dinner and you will tell me all about your adventures.'),
    ('monster', 'Wha… Aargh!'),
    ('end_bad', 'Game Over'),
    ('end_good', 'You Succeed!');
insert into messages(situation, message) values
    ('start', 'I must find my grandchildren as fast as I can! Onward!'),
    ('resource_found1', 'It seems miners forgot full bottle of oil here.'),
    ('resource_found2', 'Some oil for my lamp? Perfect.'),
    ('resource_found3', 'Found an oil jug.'),
    ('resource_found4', 'Oh, an oil tank!'),
    ('resource_gone1', 'The lamp burned out. I have to go by touch.'),
    ('resource_gone2', 'No! My lamp died out, I cannot see a thing.'),
    ('resource_gone3', 'That lamp… I need to refill it.'),
    ('resource_gone4', 'Oh, the lamp went out.'),
    ('resource_refill', 'Well, well. Now the way is lit.'),
    ('resource_absent', 'I have no oil. Maybe I can find some in those mines…'),
    ('obstacle1', 'It looks like a ceiling collapsed here.'),
    ('obstacle2', 'Too narrow, I cannot squeeze through.'),
    ('obstacle3', 'A big pit here.'),
    ('obstacle4', 'Cannot get through this rubble.'),
    ('obstacle5', 'Cave in. I need to find another path.'),
    ('obstacle6', 'What a pit, I cannot see the bottom of it.'),
    ('obstacle7', 'A pit that I cannot jump over.'),
    ('danger1', 'Something huge just flashed by!'),
    ('danger2', 'My lamp. Something is wrong with it.'),
    ('danger3', 'I hear strange rustles nearby.'),
    ('enter_close', 'Press [Enter] to close'),
    ('resource_keys', '[R][0]'),
    ('empty', ' ');
insert into cards(tiles) values
    (readfile('data/cards/field1') ),
    (readfile('data/cards/field2') ),
    (readfile('data/cards/field3') ),
    (readfile('data/cards/field4') ),
    (readfile('data/cards/field5') ),
    (readfile('data/cards/field6') ),
    (readfile('data/cards/field7') ),
    (readfile('data/cards/field8') ),
    (readfile('data/cards/field9') ),
    (readfile('data/cards/field10')),
    (readfile('data/cards/field11')),
    (readfile('data/cards/field12')),
    (readfile('data/cards/field13')),
    (readfile('data/cards/field14')),
    (readfile('data/cards/field15')),
    (readfile('data/cards/field16')),
    (readfile('data/cards/field17')),
    (readfile('data/cards/field18'));
--insert into tile_images(name, image) values
    --('filename.filetype', readfile('data/img/filepath')),
    --('filename.filetype', readfile('data/img/filepath'))
    --;
--}}}
