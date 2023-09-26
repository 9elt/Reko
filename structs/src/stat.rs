use chrono::{prelude::Datelike, NaiveDateTime};
use super::Stat;

impl Stat {
    pub fn from_airing_date(date: NaiveDateTime) -> Self {
        let year = date.year();
        if year < 1990 {
            Self(0)
        } else if year < 2000 {
            Self(1)
        } else if year < 2010 {
            Self(2)
        } else if year < 2018 {
            Self(3)
        } else {
            Self(4)
        }
    }
    pub fn from_series_len(num_episodes: i32) -> Self {
        if num_episodes == 1 {
            Self(5)
        } else if num_episodes < 9 {
            Self(6)
        } else if num_episodes < 19 {
            Self(7)
        } else if num_episodes < 33 {
            Self(8)
        } else {
            Self(9)
        }
    }
    pub fn from_rating(rating: String) -> Self {
        match rating.as_str() {
            "g" => Self(10),
            "pg" => Self(11),
            "pg_13" => Self(12),
            "r" => Self(13),
            "r+" => Self(14),
            "rx" => Self(15),
            _ => Self(-1),
        }
    }
    pub fn from_genre(genre_id: i32) -> Self {
        match genre_id {
            1 => Self(16),  // Action
            2 => Self(17),  // Adventure
            4 => Self(18),  // Comedy
            8 => Self(19),  // Drama
            10 => Self(20), // Fantasy
            22 => Self(21), // Romance
            24 => Self(22), // SciFi
            37 => Self(23), // Supernatural
            5 => Self(24),  // AvantGarde
            46 => Self(25), // AwardWinning
            28 => Self(26), // BoysLove
            26 => Self(27), // GirlsLove
            47 => Self(28), // Gourmet
            14 => Self(29), // Horror
            7 => Self(30),  // Mystery
            36 => Self(31), // SliceofLife
            30 => Self(32), // Sports
            41 => Self(33), // Suspense
            9 => Self(34),  // Ecchi
            49 => Self(35), // Erotica
            12 => Self(36), // Hentai
            43 => Self(37), // Josei
            15 => Self(38), // Kids
            42 => Self(39), // Seinen
            25 => Self(40), // Shoujo
            27 => Self(41), // Shounen
            50 => Self(42), // AdultCast
            57 => Self(43), // GagHumor
            58 => Self(44), // Gore
            35 => Self(45), // Harem
            13 => Self(46), // Historical
            62 => Self(47), // Isekai
            63 => Self(48), // Iyashikei
            64 => Self(49), // LovePolygon
            17 => Self(50), // MartialArts
            18 => Self(51), // Mecha
            38 => Self(52), // Military
            19 => Self(53), // Music
            6 => Self(54),  // Mythology
            20 => Self(55), // Parody
            40 => Self(56), // Psychological
            23 => Self(57), // School
            31 => Self(58), // SuperPower
            76 => Self(59), // Survival
            78 => Self(60), // TimeTravel
            32 => Self(61), // Vampire
            51 => Self(62), // Anthropomorphic
            52 => Self(63), // CGDCT
            53 => Self(64), // Childcare
            54 => Self(65), // CombatSports
            81 => Self(66), // Crossdressing
            55 => Self(67), // Delinquents
            39 => Self(68), // Detective
            56 => Self(69), // Educational
            59 => Self(70), // HighStakesGame
            60 => Self(71), // IdolsFemale
            61 => Self(72), // IdolsMale
            65 => Self(73), // MagicalSexShift
            66 => Self(74), // MahouShoujo
            67 => Self(75), // Medical
            68 => Self(76), // OrganizedCrime
            69 => Self(77), // OtakuCulture
            70 => Self(78), // PerformingArts
            71 => Self(79), // Pets
            3 => Self(80),  // Racing
            72 => Self(81), // Reincarnation
            73 => Self(82), // ReverseHarem
            74 => Self(83), // RomanticSubtext
            21 => Self(84), // Samurai
            75 => Self(85), // Showbiz
            29 => Self(86), // Space
            11 => Self(87), // StrategyGame
            77 => Self(88), // TeamSports
            79 => Self(89), // VideoGame
            80 => Self(90), // VisualArts
            48 => Self(91), // Workplace
            _ => Self(-1),
        }
    }
    pub fn to_genre(&self) -> Option<String> {
        match self.0 {
            16 => Some("Action".into()),
            17 => Some("Adventure".into()),
            18 => Some("Comedy".into()),
            19 => Some("Drama".into()),
            20 => Some("Fantasy".into()),
            21 => Some("Romance".into()),
            22 => Some("Sci-Fi".into()),
            23 => Some("Supernatural".into()),
            24 => Some("Avant Garde".into()),
            25 => Some("Award Winning".into()),
            26 => Some("Boys Love".into()),
            27 => Some("Girls Love".into()),
            28 => Some("Gourmet".into()),
            29 => Some("Horror".into()),
            30 => Some("Mystery".into()),
            31 => Some("Slice of Life".into()),
            32 => Some("Sports".into()),
            33 => Some("Suspense".into()),
            34 => Some("Ecchi".into()),
            35 => Some("Erotica".into()),
            36 => Some("Hentai".into()),
            37 => Some("Josei".into()),
            38 => Some("Kids".into()),
            39 => Some("Seinen".into()),
            40 => Some("Shoujo".into()),
            41 => Some("Shounen".into()),
            42 => Some("Adult Cast".into()),
            43 => Some("Gag Humor".into()),
            44 => Some("Gore".into()),
            45 => Some("Harem".into()),
            46 => Some("Historical".into()),
            47 => Some("Isekai".into()),
            48 => Some("Iyashikei".into()),
            49 => Some("Love Polygon".into()),
            50 => Some("Martial Arts".into()),
            51 => Some("Mecha".into()),
            52 => Some("Military".into()),
            53 => Some("Music".into()),
            54 => Some("Mythology".into()),
            55 => Some("Parody".into()),
            56 => Some("Psychological".into()),
            57 => Some("School".into()),
            58 => Some("Super Power".into()),
            59 => Some("Survival".into()),
            60 => Some("Time Travel".into()),
            61 => Some("Vampire".into()),
            62 => Some("Anthropomorphic".into()),
            63 => Some("CGDCT".into()),
            64 => Some("Childcare".into()),
            65 => Some("Combat Sports".into()),
            66 => Some("Crossdressing".into()),
            67 => Some("Delinquents".into()),
            68 => Some("Detective".into()),
            69 => Some("Educational".into()),
            70 => Some("High Stakes Game".into()),
            71 => Some("Idols (Female)".into()),
            72 => Some("Idols (Male)".into()),
            73 => Some("Magical Sex Shift".into()),
            74 => Some("Mahou Shoujo".into()),
            75 => Some("Medical".into()),
            76 => Some("Organized Crime".into()),
            77 => Some("Otaku Culture".into()),
            78 => Some("Performing Arts".into()),
            79 => Some("Pets".into()),
            80 => Some("Racing".into()),
            81 => Some("Reincarnation".into()),
            82 => Some("Reverse Harem".into()),
            83 => Some("Romantic Subtext".into()),
            84 => Some("Samurai".into()),
            85 => Some("Showbiz".into()),
            86 => Some("Space".into()),
            87 => Some("Strategy Game".into()),
            88 => Some("Team Sports".into()),
            89 => Some("Video Game".into()),
            90 => Some("Visual Arts".into()),
            91 => Some("Workplace".into()),
            _ => None,
        }
    }
    pub fn hash_pos(&self) -> Option<usize> {
        match self.0 {
            12 => Some(0),
            7 => Some(1),
            3 => Some(2),
            16 => Some(3),
            20 => Some(4),
            41 => Some(5),
            18 => Some(6),
            13 => Some(7),
            57 => Some(8),
            4 => Some(9),
            19 => Some(10),
            21 => Some(11),
            17 => Some(12),
            8 => Some(13),
            2 => Some(14),
            23 => Some(15),
            5 => Some(16),
            22 => Some(17),
            39 => Some(18),
            56 => Some(19),
            30 => Some(20),
            58 => Some(21),
            34 => Some(22),
            25 => Some(23),
            44 => Some(24),
            9 => Some(25),
            14 => Some(26),
            33 => Some(27),
            52 => Some(28),
            45 => Some(29),
            47 => Some(30),
            42 => Some(31),
            54 => Some(32),
            6 => Some(33),
            46 => Some(34),
            59 => Some(35),
            51 => Some(36),
            29 => Some(37),
            1 => Some(38),
            40 => Some(39),
            32 => Some(40),
            55 => Some(41),
            31 => Some(42),
            49 => Some(43),
            60 => Some(44),
            81 => Some(45),
            50 => Some(46),
            61 => Some(47),
            83 => Some(48),
            53 => Some(49),
            43 => Some(50),
            10 => Some(51),
            88 => Some(52),
            77 => Some(53),
            89 => Some(54),
            68 => Some(55),
            11 => Some(56),
            48 => Some(57),
            63 => Some(58),
            0 => Some(59),
            76 => Some(60),
            91 => Some(61),
            70 => Some(62),
            87 => None,
            86 => None,
            84 => None,
            74 => None,
            28 => None,
            64 => None,
            24 => None,
            62 => None,
            38 => None,
            36 => None,
            15 => Some(63),
            90 => None,
            37 => Some(64),
            26 => None,
            67 => None,
            27 => None,
            82 => None,
            78 => None,
            65 => None,
            71 => None,
            80 => None,
            85 => None,
            66 => None,
            35 => None,
            75 => None,
            73 => None,
            69 => None,
            72 => None,
            79 => None,
            _ => None,
        }
    }
}
