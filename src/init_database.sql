--{{{ Tables
create table cards (tiles BLOB);
create table images(name TEXT, image BLOB);
create table fonts(name TEXT, font BLOB);
create table messages(situation TEXT, message TEXT);
create table game_settings(setting TEXT, value NUMERIC);
--}}}

--{{{ Inserts
insert into game_settings(setting, value) values
    ('game_name',    'Kobold'),
    ('bg_x',                0),
    ('bg_y',                0),
    ('bg_w',              696),
    ('bg_h',              696),
    ('map_x',              24),
    ('map_y',              24),
    ('map_w',             576),
    ('map_h',             576),
    ('text_x',             24),
    ('text_y',            624),
    ('text_w',            576),
    ('text_h',             48),
    ('flask_x',           629),
    ('flask_y',           534),
    ('flask_w',            32),
    ('flask_h',           125),
    ('textline_max_len',   66),
    ('textline_font_size', 24),
    ('visible_distance',    3),
    ('resource_max',       10),
    ('resource_start',      3);
insert into fonts(name, font) values
    ('DejaVu Sans', readfile('data/DejaVuSans.ttf'));
insert into images(name, image) values
    ('wall.png',   readfile('data/tiles/wall.png')  ),
    ('floor.png',  readfile('data/tiles/floor.png') ),
    ('player.png', readfile('data/icons/player.png')),
    ('flask.png',  readfile('data/icons/flask.png') ),
    ('map.png',    readfile('data/map.png')         );
insert into messages(situation, message) values
    ('start', 'I must find my grandchildren as fast as I can! Onward!.'),
    ('resource_found', 'It seems miners forgot full bottle of oil here.'),
    ('resource_found', 'Some oil for my lamp? Perfect.'),
    ('resource_found', 'Found an oil jug.'),
    ('resource_found', 'Oh, an oil tank!'),
    ('resource_gone', 'The lamp burned out. I have to go by touch.'),
    ('resource_gone', 'No! My lamp died out, I cannot see a thing.'),
    ('resource_gone', 'That lamp… I need to refill it.'),
    ('resource_gone', 'Oh, the lamp went out.'),
    ('resource_refill', 'Well, well. Now the way is lit.'),
    ('resource_absent', 'I have no oil. Maybe I can find some in those mines…'),
    ('obstacle', 'It looks like a ceiling collapsed here.'),
    ('obstacle', 'Too narrow, I cannot squeeze through.'),
    ('obstacle', 'A big pit here.'),
    ('obstacle', 'Cannot get through this rubble.'),
    ('obstacle', 'Cave in. I need to find another path.'),
    ('obstacle', 'What a pit, I cannot see the bottom of it.'),
    ('obstacle', 'A pit that I cannot jump over.'),
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
