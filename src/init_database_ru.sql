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
    ('game_name',      'Кобальд'),
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
    ('body', 'О, нет… Я слышал об этом от деда. Несколько десятков лет назад шахту закрыли, потому что здесь поселился злой дух, убивающих шахтеров. Постойка… У него в руке записка. Ух... Что это? «Ищи проклятый предмет. Вынь ржавую игру из гнилого сердца и освободи его от проклятья»? Кажется, он искал что-то, прежде чем погибнуть под обвалом. Покойся с миром, мертвый шахтер'),
    ('rest', 'Тупик. Но я вижу лучи солнца, пробивающиеся сверху. Ох… Сколько же времени уже прошло, как я вошел в эти заброшенные шахты?… Ноги ломят и есть хочется. Впрочем, детям, наверное, еще хуже. Надо поторапливаться и найти их до темноты. Если вспомнить, кажется дед рассказывал мне о плане этой шахты. Да, думаю, теперь, я знаю в каком направлении нужно искать моих внуков.'),
    ('item', 'Ох, что это за место? Сундук? Откуда здесь, в заброшенных шахтах, такой необычный сундук? Ааа! Там человеческое сердце внутри! Оно проткнуто ржавой иглой и сгнило, и ужасно воняет! Надо скорее закрыть крышку. Ох!.. Что это было?! Я случайно коснулся игры и та рассыпалась, а по пещере прокатился ужасный рев… Надо скорее найти моих внуков пока не случилось беды!'),
    ('lair', 'Что это за жуткое место? Тут словно обитает какой-то зверь… Эти кости. Куча костей! Постойка, да они все ЧЕЛОВЕЧЕСКИЕ…'),
    ('children', 'Дети! Мои внуки, наконец-то я нашел вас! Ах, простите, простите меня за то, что я накричал на вас. Идем скорее домой, мои хорошие, бабушка приготовит вам ужин и все мне расскажите. Хвала небу, с вами все в порядке! Идем скорее домой.'),
    ('monster', 'Чт… Аааааа!'),
    ('end_bad', 'Вы проиграли…'),
    ('end_good', 'Вам удалось!');
insert into messages(situation, message) values
    ('start', 'Я должен найти своих внуков как можно скорее. Вперед!'),
    ('resource_found1', 'Кажется шахтеры забыли здесь полную масленку.'),
    ('resource_found2', 'О, еще масленка.'),
    ('resource_found3', 'Я нашел баночку масла для фонаря.'),
    ('resource_found4', 'Масло для фонаря? Отлично!'),
    ('resource_gone1', 'Фонарь потух. Теперь придется идти на ощупь.'),
    ('resource_gone2', 'О, нет! Фонарь потух, теперь я не разберу дороги.'),
    ('resource_gone3', 'Фонарь... Надо скорее заправить его маслом.'),
    ('resource_gone4', 'Ох... Мой фонарь потух.'),
    ('resource_refill', 'Ну, вот. Теперь фонарь светит ярко.'),
    ('resource_absent', 'Но у меня нет масла. Нужно найти масленку, тут их полно разбросано.'),
    ('obstacle1', 'Кажется, здесь обвал.'),
    ('obstacle2', 'Не протиснуться.'),
    ('obstacle3', 'Здесь большая яма.'),
    ('obstacle4', 'Я не смогу пройти тут.'),
    ('obstacle5', 'Обвал. Нужно поискать другой путь.'),
    ('obstacle6', 'Я не вижу дна этой ямы.'),
    ('obstacle7', 'Не смогу перепрыгнуть.'),
    ('obstacle8', 'Нет, нужно поискать другой путь — здесь обвал.'),
    ('danger1', 'Что-то крупное мелькнуло рядом.'),
    ('danger2', 'Лампа. С ней что-то не так.'),
    ('danger3', 'Я слышал странный шорох рядом.'),
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
