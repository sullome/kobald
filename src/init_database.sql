--{{{ Tables
create table cards (ctype TEXT, tiles BLOB);
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
    ('end_entry',    'Start of the journey.'       ),
    ('end_children', 'Happy end.'                  ),
    ('end_body',     'Terrible revelation.'        ),
    ('end_lair',     'Bad end.'                    ),
    ('end_item',     'What is this?'               ),
    ('end_rest',     'This looks like a safe place'),
    ('wall',         ''                            ),
    ('floor',        'Old mines…'                  ),
    ('start',        'Start of the journey.'       ),
    ('obstacle',     'Damn… No way further.'       );
insert into cards(ctype, tiles) values
    ('field', readfile('data/cards/field1') ),
    ('field', readfile('data/cards/field2') ),
    ('field', readfile('data/cards/field3') ),
    ('field', readfile('data/cards/field4') ),
    ('field', readfile('data/cards/field5') ),
    ('field', readfile('data/cards/field6') ),
    ('field', readfile('data/cards/field7') ),
    ('field', readfile('data/cards/field8') ),
    ('field', readfile('data/cards/field9') ),
    ('field', readfile('data/cards/field10')),
    ('field', readfile('data/cards/field11')),
    ('field', readfile('data/cards/field12')),
    ('field', readfile('data/cards/field13')),
    ('field', readfile('data/cards/field14')),
    ('field', readfile('data/cards/field15')),
    ('field', readfile('data/cards/field16')),
    ('field', readfile('data/cards/field17')),
    ('field', readfile('data/cards/field18')),

    ('end_entry',    readfile('data/cards/end_entry')   ),
    ('end_children', readfile('data/cards/end_children')),
    ('end_body',     readfile('data/cards/end_body')    ),
    ('end_lair',     readfile('data/cards/end_lair')    ),
    ('end_item',     readfile('data/cards/end_item')    ),
    ('end_rest',     readfile('data/cards/end_rest')    ),

    ('left',   readfile('data/cards/dead_end_left')  ),
    ('top',    readfile('data/cards/dead_end_top')   ),
    ('right',  readfile('data/cards/dead_end_right') ),
    ('bottom', readfile('data/cards/dead_end_bottom')),

    ('corner', readfile('data/cards/corner')         );
--insert into tile_images(name, image) values
    --('filename.filetype', readfile('data/img/filepath')),
    --('filename.filetype', readfile('data/img/filepath'))
    --;
--}}}
