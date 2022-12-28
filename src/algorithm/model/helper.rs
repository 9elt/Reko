////////////////////////////////////////////////////////////////////////////////
// Model index helper
////////////////////////////////////////////////////////////////////////////////

use chrono::Datelike;
use super::Indexer;

fn ok(x: usize, y: usize) -> Indexer {
    Indexer { x, y, errors: false, }
}

fn err() -> Indexer {
    Indexer { x: 0, y: 0, errors: true, }
}

impl Indexer {
    pub fn has_errors(&self) -> bool {
        self.errors
    }

    pub fn general() -> Self {
        ok(0, 0)
    }

    /// ### Model `Airing Decades` index from `airing date`
    /// `Airing Decades` has index `1` in the model
    pub fn date(date: &chrono::NaiveDate) -> Self {
        let year: i32 = date.year();
        // 1980s
        if year < 1991 {
            ok(1, 0)
        // 1990s
        } else if year < 2000 {
            ok(1, 1)
        // 2000s
        } else if year < 2010 {
            ok(1, 2)
        // 2010s
        } else if year < 2016 {
            ok(1, 3)
        // 2020s
        } else {
            ok(1, 4)
        }
    }

    /// ### Model `Rating` index from `rating id`
    /// `Rating` has index `2` in the model
    /// 
    /// `rating id` is generated internally from the rating name 
    pub fn rating(rating: &i16) -> Self {
        if rating == &0 {
            err()
        } else {
            ok(2, (rating - 1) as usize)
        }
    }

    /// ### Model `Series Length` index from `number of episodes`
    /// `Series Length` has index `3` in the model
    /// 
    /// `number of episodes` is provided by **MyAnimeList api**
    pub fn num_episodes(num_episodes: &i16) -> Self {
        // Errors
        if num_episodes == &0 {
            err()
        // 1 Episode
        } else if num_episodes == &1 {
            ok(3, 0)
        // 2-8 Episodes
        } else if num_episodes < &9 {
            ok(3, 1)
        // 9-18 Episodes
        } else if num_episodes < &19 {
            ok(3, 2)
        // 19-32 Episodes
        } else if num_episodes < &33 {
            ok(3, 3)
        // 33+ Episodes
        } else {
            ok(3, 4)
        }
    }

    /// ### Model `Genres` / `Themes` / `Demographics` index from `genre id`
    /// `MAJOR Genres` has index `4` in the model
    /// 
    /// `minor Genres` has index `5` in the model
    /// 
    /// `MAJOR Themes` has index `6` in the model
    /// 
    /// `minor Themes` has index `7` in the model
    /// 
    /// `Demographics` has index `8` in the model
    /// 
    /// `genre id` is provided by **MyAnimeList api** and represents all
    /// genres, themes and demographics
    pub fn genre(genre: &i16) -> Self {
        match genre {
            //MAJOR Genres
            1 => ok(4, 0),
            2 => ok(4, 1),
            4 => ok(4, 2),
            8 => ok(4, 3),
            10 => ok(4, 4),
            22 => ok(4, 5),
            24 => ok(4, 6),
            37 => ok(4, 7),

            // minor Genres
            5 => ok(5, 0),
            46 => ok(5, 1),
            28 => ok(5, 2),
            26 => ok(5, 3),
            47 => ok(5, 4),
            14 => ok(5, 5),
            7 => ok(5, 6),
            36 => ok(5, 7),
            30 => ok(5, 8),
            41 => ok(5, 9),
            9 => ok(5, 10),
            49 => ok(5, 11),
            12 => ok(5, 12),

            // MAJOR Themes
            50 => ok(6, 0),
            57 => ok(6, 1),
            58 => ok(6, 2),
            35 => ok(6, 3),
            13 => ok(6, 4),
            62 => ok(6, 5),
            63 => ok(6, 6),
            64 => ok(6, 7),
            17 => ok(6, 8),
            18 => ok(6, 9),
            38 => ok(6, 10),
            19 => ok(6, 11),
            6 => ok(6, 12),
            20 => ok(6, 13),
            40 => ok(6, 14),
            23 => ok(6, 15),
            31 => ok(6, 16),
            76 => ok(6, 17),
            78 => ok(6, 18),
            32 => ok(6, 19),

            // minor Themes
            51 => ok(7, 0),
            52 => ok(7, 1),
            53 => ok(7, 2),
            54 => ok(7, 3),
            81 => ok(7, 4),
            55 => ok(7, 5),
            39 => ok(7, 6),
            56 => ok(7, 7),
            59 => ok(7, 8),
            60 => ok(7, 9),
            61 => ok(7, 10),
            65 => ok(7, 11),
            66 => ok(7, 12),
            67 => ok(7, 13),
            68 => ok(7, 14),
            69 => ok(7, 15),
            70 => ok(7, 16),
            71 => ok(7, 17),
            3 => ok(7, 18),
            72 => ok(7, 19),
            73 => ok(7, 20),
            74 => ok(7, 21),
            21 => ok(7, 22),
            75 => ok(7, 23),
            29 => ok(7, 24),
            11 => ok(7, 25),
            77 => ok(7, 26),
            79 => ok(7, 27),
            80 => ok(7, 28),
            48 => ok(7, 29),

            // Demographics
            43 => ok(8, 0),
            15 => ok(8, 1),
            42 => ok(8, 2),
            25 => ok(8, 3),
            27 => ok(8, 4),

            _ => err(),
        }
    }
}
