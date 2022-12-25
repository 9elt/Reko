use chrono::Datelike;

pub struct Idx {
    pub x: usize,
    pub y: usize,
}

impl Idx {
    pub fn general() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn from_status(status: usize) -> Self {
        Self { x: 0, y: status }
    }

    pub fn from_date(date: chrono::NaiveDate) -> Self {
        let year: i32 = date.year();
        if year < 1991 {
            Self { x: 1, y: 0 }
        } else if year < 2000 {
            Self { x: 1, y: 1 }
        } else if year < 2010 {
            Self { x: 1, y: 2 }
        } else if year < 2016 {
            Self { x: 1, y: 3 }
        } else {
            Self { x: 1, y: 4 }
        }
    }

    pub fn from_rating(rating: i16) -> Self {
        if rating == 0 {
            Self { x: 0, y: 6 }
        } else {
            Self {
                x: 2,
                y: (rating as usize - 1),
            }
        }
    }

    pub fn from_num_episodes(num_episodes: i16) -> Self {
        if num_episodes == 0 {
            Self { x: 0, y: 6 }
        } else if num_episodes == 1 {
            Self { x: 3, y: 0 }
        } else if num_episodes < 9 {
            Self { x: 3, y: 1 }
        } else if num_episodes < 19 {
            Self { x: 3, y: 2 }
        } else if num_episodes < 33 {
            Self { x: 3, y: 3 }
        } else {
            Self { x: 3, y: 4 }
        }
    }

    pub fn from_genre(genre: i16) -> Self {
        match genre {
            //genres
            1 => Self { x: 4, y: 0 },
            2 => Self { x: 4, y: 1 },
            5 => Self { x: 4, y: 2 },
            46 => Self { x: 4, y: 3 },
            28 => Self { x: 4, y: 4 },
            4 => Self { x: 4, y: 5 },
            8 => Self { x: 4, y: 6 },
            10 => Self { x: 4, y: 7 },
            26 => Self { x: 4, y: 8 },
            47 => Self { x: 4, y: 9 },
            14 => Self { x: 4, y: 10 },
            7 => Self { x: 4, y: 11 },
            22 => Self { x: 4, y: 12 },
            24 => Self { x: 4, y: 13 },
            36 => Self { x: 4, y: 14 },
            30 => Self { x: 4, y: 15 },
            37 => Self { x: 4, y: 16 },
            41 => Self { x: 4, y: 17 },
            9 => Self { x: 4, y: 18 },
            49 => Self { x: 4, y: 19 },
            12 => Self { x: 4, y: 20 },

            // themes
            50 => Self { x: 5, y: 0 },
            51 => Self { x: 5, y: 1 },
            52 => Self { x: 5, y: 2 },
            53 => Self { x: 5, y: 3 },
            54 => Self { x: 5, y: 4 },
            81 => Self { x: 5, y: 5 },
            55 => Self { x: 5, y: 6 },
            39 => Self { x: 5, y: 7 },
            56 => Self { x: 5, y: 8 },
            57 => Self { x: 5, y: 9 },
            58 => Self { x: 5, y: 10 },
            35 => Self { x: 5, y: 11 },
            59 => Self { x: 5, y: 12 },
            13 => Self { x: 5, y: 13 },
            60 => Self { x: 5, y: 14 },
            61 => Self { x: 5, y: 15 },
            62 => Self { x: 5, y: 16 },
            63 => Self { x: 5, y: 17 },
            64 => Self { x: 5, y: 18 },
            65 => Self { x: 5, y: 19 },
            66 => Self { x: 5, y: 20 },
            17 => Self { x: 5, y: 21 },
            18 => Self { x: 5, y: 22 },
            67 => Self { x: 5, y: 23 },
            38 => Self { x: 5, y: 24 },
            19 => Self { x: 5, y: 25 },
            6 => Self { x: 5, y: 26 },
            68 => Self { x: 5, y: 27 },
            69 => Self { x: 5, y: 28 },
            20 => Self { x: 5, y: 29 },
            70 => Self { x: 5, y: 30 },
            71 => Self { x: 5, y: 31 },
            40 => Self { x: 5, y: 32 },
            3 => Self { x: 5, y: 33 },
            72 => Self { x: 5, y: 34 },
            73 => Self { x: 5, y: 35 },
            74 => Self { x: 5, y: 36 },
            21 => Self { x: 5, y: 37 },
            23 => Self { x: 5, y: 38 },
            75 => Self { x: 5, y: 39 },
            29 => Self { x: 5, y: 40 },
            11 => Self { x: 5, y: 41 },
            31 => Self { x: 5, y: 42 },
            76 => Self { x: 5, y: 43 },
            77 => Self { x: 5, y: 44 },
            78 => Self { x: 5, y: 45 },
            32 => Self { x: 5, y: 46 },
            79 => Self { x: 5, y: 47 },
            80 => Self { x: 5, y: 48 },
            48 => Self { x: 5, y: 49 },

            // demographics
            43 => Self { x: 6, y: 0 },
            15 => Self { x: 6, y: 1 },
            42 => Self { x: 6, y: 2 },
            25 => Self { x: 6, y: 3 },
            27 => Self { x: 6, y: 4 },
            _ => Self { x: 0, y: 6 },
        }
    }
}
