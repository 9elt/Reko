pub fn new_model() -> Vec<Vec<[i32; 9]>> {
    vec![

        //  0 => General Stats
        vec![

                /*
                    Score Stats [i32; 9]

                    0 => List Length (anime watched)

                    1 => Overall Average Score,
                    2 => Overall Score deviation,
                    3 => Overall Scored %,


                    4, 5, 6, 7, 8 => ! Empty
                */

            //  0 => Score
            [0,   0, 0, 0,     0, 0, 0, 0, 0],

                /*
                    Statuses Stats [i32; 9]

                    0 => Status %,

                    1 => Status Average Score,
                    2 => Status Score deviation,
                    3 => Status Scored %,


                    4, 5, 6, 7, 8 => ! Empty
                */

            //  1 => Completed (status id: 1)
            [0,   0, 0, 0,     0, 0, 0, 0, 0],
            //  2 => Plan to Watch (status id: 2)
            [0,   0, 0, 0,     0, 0, 0, 0, 0],
            //  3 => Watching (status id: 3)
            [0,   0, 0, 0,     0, 0, 0, 0, 0],
            //  4 => On Hold (status id: 4)
            [0,   0, 0, 0,     0, 0, 0, 0, 0],
            //  5 => Dropped (status id: 5)
            [0,   0, 0, 0,     0, 0, 0, 0, 0],

            //  6 => Errors (when not empty some error occurred)
            [0,   0, 0, 0,     0, 0, 0, 0, 0],
        ],

            /*
                Detailed Stats [i32; 9]

                0 => Watched %,

                1 => Average Score,
                2 => Score deviation,
                3 => Scored %,

                4 => Completed %,
                5 => Plan to Watch %,
                6 => Watching %,
                7 => On Hold %,
                8 => Dropped %
            */

        //  1 => Airing Decades
        vec![
            //  0 => 1980s (0 - 1989)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  1 => 1990s (1990 - 1999)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  2 => 2000s (2000 - 2009)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  3 => 2010s (2010 - 2015)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  4 => 2020s (2016 - now)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
        ],

        //  2 => Ratings
        vec![
            //  0 => g (rating id: 1)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  1 => pg (rating id: 2)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  2 => pg 13 (rating id: 3)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  3 => r (rating id: 4)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  4 => r+ (rating id: 5)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  5 => rx (rating id: 6)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
        ],

        //  3 => Series Length
        vec![
            //  0 => ~ 1 episode (1)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  1 => ~ 6 episodes (2 - 8)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  2 => ~ 12 episodes (9 - 18)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  3 => ~ 24 episodes (19 - 32)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  4 => ~ 48 episodes (over 33)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
        ],

        // 4 => Genres
        vec![
            //  0 => Action (genre id: 1)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  1 => Adventure (genre id: 2)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  2 => Avant_Garde (genre id: 5)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  3 => Award_Winning (genre id: 46)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  4 => Boys_Love (genre id: 28)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  5 => Comedy (genre id: 4)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  6 => Drama (genre id: 8)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  7 => Fantasy (genre id: 10)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  8 => Girls_Love (genre id: 26)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  9 => Gourmet (genre id: 47)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  10 => Horror (genre id: 14)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  11 => Mystery (genre id: 7)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  12 => Romance (genre id: 22)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  13 => Sci_Fi (genre id: 24)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  14 => Slice_of_Life (genre id: 36)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  15 => Sports (genre id: 30)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  16 => Supernatural (genre id: 37)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  17 => Suspense (genre id: 41)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  18 => Ecchi (genre id: 9)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  19 => Erotica (genre id: 49)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  20 => Hentai (genre id: 12)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
        ],

        //  5 => Themes
        vec![
            //  0 => Adult_Cast (genre id: 50)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  1 => Anthropomorphic (genre id: 51)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  2 => CGDCT (genre id: 52)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  3 => Childcare (genre id: 53)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  4 => Combat_Sports (genre id: 54)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  5 => Crossdressing (genre id: 81)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  6 => Delinquents (genre id: 55)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  7 => Detective (genre id: 39)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  8 => Educational (genre id: 56)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  9 => Gag_Humor (genre id: 57)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  10 => Gore (genre id: 58)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  11 => Harem (genre id: 35)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  12 => High_Stakes_Game (genre id: 59)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  13 => Historical (genre id: 13)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  14 => Idols_Female (genre id: 60)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  15 => Idols_Male (genre id: 61)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  16 => Isekai (genre id: 62)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  17 => Iyashikei (genre id: 63)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  18 => Love_Polygon (genre id: 64)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  19 => Magical_Sex_Shift (genre id: 65)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  20 => Mahou_Shoujo (genre id: 66)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  21 => Martial_Arts (genre id: 17)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  22 => Mecha (genre id: 18)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  23 => Medical (genre id: 67)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  24 => Military (genre id: 38)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  25 => Music (genre id: 19)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  26 => Mythology (genre id: 6)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  27 => Organized_Crime (genre id: 68)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  28 => Otaku_Culture (genre id: 69)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  29 => Parody (genre id: 20)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  30 => Performing_Arts (genre id: 70)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  31 => Pets (genre id: 71)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  32 => Psychological (genre id: 40)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  33 => Racing (genre id: 3)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  34 => Reincarnation (genre id: 72)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  35 => Reverse_Harem (genre id: 73)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  36 => Romantic_Subtext (genre id: 74)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  37 => Samurai (genre id: 21)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  38 => School (genre id: 23)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  39 => Showbiz (genre id: 75)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  40 => Space (genre id: 29)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  41 => Strategy_Game (genre id: 11)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  42 => Super_Power (genre id: 31)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  43 => Survival (genre id: 76)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  44 => Team_Sports (genre id: 77)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  45 => Time_Travel (genre id: 78)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  46 => Vampire (genre id: 32)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  47 => Video_Game (genre id: 79)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  48 => Visual_Arts (genre id: 80)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  49 => Workplace (genre id: 48)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
        ],

        //  6 => Demographics
        vec![
            //  0 => Josei (genre id: 43)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  1 => Kids (genre id: 15)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  2 => Seinen (genre id: 42)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  3 => Shoujo (genre id: 25)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
            //  4 => Shounen (genre id: 27)
            [0,   0, 0, 0,   0, 0, 0, 0, 0],
        ],
    ]
}