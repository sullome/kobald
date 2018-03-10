--{{{ Tables
create table cards (tiles BLOB);
create table images(name TEXT, image BLOB);
create table messages(situation TEXT, message TEXT);
create table game_settings(setting TEXT, value NUMERIC);
--}}}

--{{{ Inserts
insert into game_settings(setting, value) values
    ('visible_distance', 6 ),
    ('resource_max',     10),
    ('resource_start',   3);
insert into images(name, image) values
    ('wall.png',   readfile('data/tiles/wall.png')  ),
    ('floor.png',  readfile('data/tiles/floor.png') ),
    ('player.png', readfile('data/icons/player.png'));
insert into messages(situation, message) values
    ('start',    'Start of the journey.'       ),
    ('children', 'Happy end.'                  ),
    ('body',     'Terrible revelation.'        ),
    ('lair',     'Bad end.'                    ),
    ('item',     'What is this?'               ),
    ('rest',     'This looks like a safe place'),
    ('wall',     ''                            ),
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
