--{{{ Tables
create table cards (ctype TEXT, tiles BLOB);
create table tile_images(name TEXT, image BLOB);
--}}}

--{{{ Inserts
insert into cards(ctype, tiles) values
    ('field',  readfile('data/cards/field1')         ),
    ('field',  readfile('data/cards/field2')         ),
    ('field',  readfile('data/cards/field3')         ),
    ('field',  readfile('data/cards/field4')         ),
    ('field',  readfile('data/cards/field5')         ),
    ('field',  readfile('data/cards/field6')         ),
    ('field',  readfile('data/cards/field7')         ),
    ('field',  readfile('data/cards/field8')         ),
    ('field',  readfile('data/cards/field9')         ),
    ('field',  readfile('data/cards/field10')        ),
    ('field',  readfile('data/cards/field11')        ),
    ('field',  readfile('data/cards/field12')        ),
    ('field',  readfile('data/cards/field13')        ),
    ('field',  readfile('data/cards/field14')        ),
    ('field',  readfile('data/cards/field15')        ),
    ('field',  readfile('data/cards/field16')        ),
    ('field',  readfile('data/cards/field17')        ),
    ('field',  readfile('data/cards/field18')        ),

    ('end',    readfile('data/cards/end_entry')      ),
    ('end',    readfile('data/cards/end_children')   ),
    ('end',    readfile('data/cards/end_body')       ),
    ('end',    readfile('data/cards/end_lair')       ),
    ('end',    readfile('data/cards/end_item')       ),
    ('end',    readfile('data/cards/end_rest')       ),

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
