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
    ('start',    'Start of the journey.'       ),
    ('children', 'Happy end.'                  ),
    ('body',     'Terrible revelation.'        ),
    ('lair',     'Bad end.'                    ),
    ('item',     'What is this?'               ),
    ('rest',     'This looks like a safe place'),
    ('wall',     ' '                           ),
    ('floor',    'Old mines…'                  ),
    ('obstacle', 'Damn… No way further.'       );
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
