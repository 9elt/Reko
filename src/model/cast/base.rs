use chrono::Datelike;

pub fn rating_to_model_index(rating: i16) -> [usize; 2] {
    [1, (rating as usize - 1)]
}

pub fn date_to_model_index(date: chrono::NaiveDate) -> [usize; 2] {
    let year = date.year();
    if year < 1991 {
        [0, 0]
    } else if year < 2000 {
        [0, 1]
    } else if year < 2010 {
        [0, 2]
    } else if year < 2016 {
        [0, 3]
    } else {
        [0, 4]
    }
}

pub fn n_episodes_to_model_index(n_episodes: i16) -> [usize; 2] {
    if n_episodes == 1 {
        [2, 0]
    } else if n_episodes < 9 {
        [2, 1]
    } else if n_episodes < 19 {
        [2, 2]
    } else if n_episodes < 33 {
        [2, 3]
    } else {
        [2, 4]
    }
}

pub fn genre_id_to_model_index(genre_id: i16) -> [usize; 2] {
    match genre_id {
        //genres
        1 => [3, 0],
        2 => [3, 1],
        5 => [3, 2],
        46 => [3, 3],
        28 => [3, 4],
        4 => [3, 5],
        8 => [3, 6],
        10 => [3, 7],
        26 => [3, 8],
        47 => [3, 9],
        14 => [3, 10],
        7 => [3, 11],
        22 => [3, 12],
        24 => [3, 13],
        36 => [3, 14],
        30 => [3, 15],
        37 => [3, 16],
        41 => [3, 17],
        9 => [3, 18],
        49 => [3, 19],
        12 => [3, 20],
        //themes
        50 => [4, 0],
        51 => [4, 1],
        52 => [4, 2],
        53 => [4, 3],
        54 => [4, 4],
        81 => [4, 5],
        55 => [4, 6],
        39 => [4, 7],
        56 => [4, 8],
        57 => [4, 9],
        58 => [4, 10],
        35 => [4, 11],
        59 => [4, 12],
        13 => [4, 13],
        60 => [4, 14],
        61 => [4, 15],
        62 => [4, 16],
        63 => [4, 17],
        64 => [4, 18],
        65 => [4, 19],
        66 => [4, 20],
        17 => [4, 21],
        18 => [4, 22],
        67 => [4, 23],
        38 => [4, 24],
        19 => [4, 25],
        6 => [4, 26],
        68 => [4, 27],
        69 => [4, 28],
        20 => [4, 29],
        70 => [4, 30],
        71 => [4, 31],
        40 => [4, 32],
        3 => [4, 33],
        72 => [4, 34],
        73 => [4, 35],
        74 => [4, 36],
        21 => [4, 37],
        23 => [4, 38],
        75 => [4, 39],
        29 => [4, 40],
        11 => [4, 41],
        31 => [4, 42],
        76 => [4, 43],
        77 => [4, 44],
        78 => [4, 45],
        32 => [4, 46],
        79 => [4, 47],
        80 => [4, 48],
        48 => [4, 49],
        //demographics
        43 => [5, 0],
        15 => [5, 1],
        42 => [5, 2],
        25 => [5, 3],
        27 => [5, 4],
        _ => [0, 0],
    }
}

/*
Genres index : id/name

0 : 1/Action
1 : 2/Adventure
2 : 5/Avant_Garde
3 : 46/Award_Winning
4 : 28/Boys_Love
5 : 4/Comedy
6 : 8/Drama
7 : 10/Fantasy
8 : 26/Girls_Love
9 : 47/Gourmet
10 : 14/Horror
11 : 7/Mystery
12 : 22/Romance
13 : 24/Sci_Fi
14 : 36/Slice_of_Life
15 : 30/Sports
16 : 37/Supernatural
17 : 41/Suspense
18 : 9/Ecchi
19 : 49/Erotica
20 : 12/Hentai

Themes index : id/name

0 : 50/Adult_Cast
1 : 51/Anthropomorphic
2 : 52/CGDCT
3 : 53/Childcare
4 : 54/Combat_Sports
5 : 81/Crossdressing
6 : 55/Delinquents
7 : 39/Detective
8 : 56/Educational
9 : 57/Gag_Humor
10 : 58/Gore
11 : 35/Harem
12 : 59/High_Stakes_Game
13 : 13/Historical
14 : 60/Idols_Female
15 : 61/Idols_Male
16 : 62/Isekai
17 : 63/Iyashikei
18 : 64/Love_Polygon
19 : 65/Magical_Sex_Shift
20 : 66/Mahou_Shoujo
21 : 17/Martial_Arts
22 : 18/Mecha
23 : 67/Medical
24 : 38/Military
25 : 19/Music
26 : 6/Mythology
27 : 68/Organized_Crime
28 : 69/Otaku_Culture
29 : 20/Parody
30 : 70/Performing_Arts
31 : 71/Pets
32 : 40/Psychological
33 : 3/Racing
34 : 72/Reincarnation
35 : 73/Reverse_Harem
36 : 74/Romantic_Subtext
37 : 21/Samurai
38 : 23/School
39 : 75/Showbiz
40 : 29/Space
41 : 11/Strategy_Game
42 : 31/Super_Power
43 : 76/Survival
44 : 77/Team_Sports
45 : 78/Time_Travel
46 : 32/Vampire
47 : 79/Video_Game
48 : 80/Visual_Arts
49 : 48/Workplace

Demographics index : id/name

0 : 43/Josei
1 : 15/Kids
2 : 42/Seinen
3 : 25/Shoujo
4 : 27/Shounen
*/
