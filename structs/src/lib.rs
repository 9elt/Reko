use chrono::{prelude::Datelike, NaiveDateTime};
use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

pub type RekoResult<T> = Result<T, RekoError>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RekoError {
    pub code: u16,
    pub message: String,
}

impl RekoError {
    pub fn new<S: ToString>(code: u16, message: S) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Anime {
    pub id: i32,
    pub title: String,
    pub airing_date: Option<NaiveDateTime>,
    pub length: Option<i32>,
    pub mean: Option<f32>,
    pub rating: Option<String>,
    pub picture: Option<String>,
    pub parent: Option<i32>,
    pub aired: bool,
    pub stats: Vec<i32>,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ListEntry {
    pub id: i32,
    pub score: i32,
    pub watched: bool,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub hash: Hash,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimilarUser {
    pub username: String,
    pub hash: Hash,
    pub similarity: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestingUser {
    pub username: String,
    pub hash: Hash,
}

impl RequestingUser {
    pub fn from_user(user: &User) -> Self {
        Self {
            username: user.username.to_owned(),
            hash: user.hash.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DetailedListEntry {
    pub id: i32,
    pub stats: Vec<i32>,
    pub score: i32,
    pub mean: i32,
}

impl DetailedListEntry {
    pub fn new(anime: Anime, entry: &ListEntry) -> Self {
        Self {
            id: anime.id,
            stats: anime.stats,
            score: entry.score,
            mean: match anime.mean {
                Some(mean) => mean as i32,
                None => 0,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recommendation {
    pub id: i32,
    pub details: RecommendationDetails,
    pub score: i32,
    pub user: SimilarUser,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendationDetails {
    pub title: String,
    pub airing_date: Option<NaiveDateTime>,
    pub length: Option<i32>,
    pub mean: Option<f32>,
    pub rating: Option<String>,
    pub picture: Option<String>,
    pub genres: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimilarResponseWrapper {
    requester: RequestingUser,
    similar: Vec<SimilarUser>,
}

impl SimilarResponseWrapper {
    pub fn new(user: &User, v: Vec<SimilarUser>) -> Self {
        Self {
            requester: RequestingUser::from_user(user),
            similar: v,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompareResponseWrapper {
    requester: RequestingUser,
    compare: SimilarUser,
}

impl CompareResponseWrapper {
    pub fn new(user: &User, v: SimilarUser) -> Self {
        Self {
            requester: RequestingUser::from_user(user),
            compare: v,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendationResponseWrapper {
    requester: RequestingUser,
    recommendations: Vec<Recommendation>,
}

impl RecommendationResponseWrapper {
    pub fn new(user: &User, v: Vec<Recommendation>) -> Self {
        Self {
            requester: RequestingUser::from_user(user),
            recommendations: v,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum Hash {
    BigInt(u64),
    Hex(String),
}

impl Hash {
    pub fn to_u64(&self) -> u64 {
        match self {
            Self::BigInt(n) => n.to_owned(),
            Self::Hex(h) => u64::from_str_radix(h, 16).unwrap_or(0),
        }
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                Self::BigInt(n) => format!("{:02x}", n),
                Self::Hex(s) => s.to_owned(),
            }
            .as_str(),
        )
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

pub struct Stat(i32);

impl Stat {
    pub fn new(uid: &i32) -> Self {
        Self(uid.to_owned())
    }
    pub fn uid(self) -> i32 {
        self.0
    }
    pub fn is_ok(&self) -> bool {
        self.0 != -1
    }
    pub fn from_airing_date(date: NaiveDateTime) -> Self {
        let year = date.year();

        if year < 1990 {
            Self(61)
        } else if year < 2000 {
            Self(40)
        } else if year < 2010 {
            Self(14)
        } else if year < 2018 {
            Self(1)
        } else {
            Self(6)
        }
    }
    pub fn from_series_len(num_episodes: i32) -> Self {
        if num_episodes == 1 {
            Self(16)
        } else if num_episodes < 9 {
            Self(33)
        } else if num_episodes < 19 {
            Self(0)
        } else if num_episodes < 33 {
            Self(12)
        } else {
            Self(23)
        }
    }
    pub fn from_rating(rating: String) -> Self {
        match rating.as_str() {
            "g" => Self(50),
            "pg" => Self(59),
            "pg_13" => Self(11),
            "r" => Self(7),
            "r+" => Self(24),
            "rx" => Self(72),
            _ => Self(-1),
        }
    }
    pub fn from_genre(genre_id: i32) -> Self {
        match genre_id {
            1 => Self(2),   // Action
            2 => Self(13),  // Adventure
            4 => Self(4),   // Comedy
            8 => Self(9),   // Drama
            10 => Self(3),  // Fantasy
            22 => Self(10), // Romance
            24 => Self(17), // SciFi
            37 => Self(15), // Supernatural
            5 => Self(70),  // AvantGarde
            46 => Self(25), // AwardWinning
            28 => Self(76), // BoysLove
            26 => Self(77), // GirlsLove
            47 => Self(68), // Gourmet
            14 => Self(36), // Horror
            7 => Self(19),  // Mystery
            36 => Self(39), // SliceofLife
            30 => Self(41), // Sports
            41 => Self(29), // Suspense
            9 => Self(21),  // Ecchi
            49 => Self(86), // Erotica
            12 => Self(71), // Hentai
            43 => Self(74), // Josei
            15 => Self(73), // Kids
            42 => Self(18), // Seinen
            25 => Self(38), // Shoujo
            27 => Self(5),  // Shounen
            50 => Self(28), // AdultCast
            57 => Self(47), // GagHumor
            58 => Self(26), // Gore
            35 => Self(27), // Harem
            13 => Self(32), // Historical
            62 => Self(30), // Isekai
            63 => Self(53), // Iyashikei
            64 => Self(43), // LovePolygon
            17 => Self(49), // MartialArts
            18 => Self(35), // Mecha
            38 => Self(31), // Military
            19 => Self(45), // Music
            6 => Self(34),  // Mythology
            20 => Self(42), // Parody
            40 => Self(20), // Psychological
            23 => Self(8),  // School
            31 => Self(22), // SuperPower
            76 => Self(37), // Survival
            78 => Self(44), // TimeTravel
            32 => Self(46), // Vampire
            51 => Self(69), // Anthropomorphic
            52 => Self(57), // CGDCT
            53 => Self(67), // Childcare
            54 => Self(82), // CombatSports
            81 => Self(85), // Crossdressing
            55 => Self(78), // Delinquents
            39 => Self(55), // Detective
            56 => Self(89), // Educational
            59 => Self(64), // HighStakesGame
            60 => Self(81), // IdolsFemale
            61 => Self(90), // IdolsMale
            65 => Self(88), // MagicalSexShift
            66 => Self(66), // MahouShoujo
            67 => Self(87), // Medical
            68 => Self(60), // OrganizedCrime
            69 => Self(52), // OtakuCulture
            70 => Self(79), // PerformingArts
            71 => Self(91), // Pets
            3 => Self(84),  // Racing
            72 => Self(48), // Reincarnation
            73 => Self(80), // ReverseHarem
            74 => Self(51), // RomanticSubtext
            21 => Self(63), // Samurai
            75 => Self(83), // Showbiz
            29 => Self(62), // Space
            11 => Self(65), // StrategyGame
            77 => Self(54), // TeamSports
            79 => Self(56), // VideoGame
            80 => Self(75), // VisualArts
            48 => Self(58), // Workplace
            _ => Self(-1),
        }
    }
    pub fn to_genre(&self) -> Option<String> {
        let g = match &self.0 {
            2 => Some("Action"),
            13 => Some("Adventure"),
            4 => Some("Comedy"),
            9 => Some("Drama"),
            3 => Some("Fantasy"),
            10 => Some("Romance"),
            17 => Some("SciFi"),
            15 => Some("Supernatural"),
            70 => Some("AvantGarde"),
            25 => Some("AwardWinning"),
            76 => Some("BoysLove"),
            77 => Some("GirlsLove"),
            68 => Some("Gourmet"),
            36 => Some("Horror"),
            19 => Some("Mystery"),
            39 => Some("SliceofLife"),
            41 => Some("Sports"),
            29 => Some("Suspense"),
            21 => Some("Ecchi"),
            86 => Some("Erotica"),
            71 => Some("Hentai"),
            74 => Some("Josei"),
            73 => Some("Kids"),
            18 => Some("Seinen"),
            38 => Some("Shoujo"),
            5 => Some("Shounen"),
            28 => Some("AdultCast"),
            47 => Some("GagHumor"),
            26 => Some("Gore"),
            27 => Some("Harem"),
            32 => Some("Historical"),
            30 => Some("Isekai"),
            53 => Some("Iyashikei"),
            43 => Some("LovePolygon"),
            49 => Some("MartialArts"),
            35 => Some("Mecha"),
            31 => Some("Military"),
            45 => Some("Music"),
            34 => Some("Mythology"),
            42 => Some("Parody"),
            20 => Some("Psychological"),
            8 => Some("School"),
            22 => Some("SuperPower"),
            37 => Some("Survival"),
            44 => Some("TimeTravel"),
            46 => Some("Vampire"),
            69 => Some("Anthropomorphic"),
            57 => Some("CGDCT"),
            67 => Some("Childcare"),
            82 => Some("CombatSports"),
            85 => Some("Crossdressing"),
            78 => Some("Delinquents"),
            55 => Some("Detective"),
            89 => Some("Educational"),
            64 => Some("HighStakesGame"),
            81 => Some("IdolsFemale"),
            90 => Some("IdolsMale"),
            88 => Some("MagicalSexShift"),
            66 => Some("MahouShoujo"),
            87 => Some("Medical"),
            60 => Some("OrganizedCrime"),
            52 => Some("OtakuCulture"),
            79 => Some("PerformingArts"),
            91 => Some("Pets"),
            84 => Some("Racing"),
            48 => Some("Reincarnation"),
            80 => Some("ReverseHarem"),
            51 => Some("RomanticSubtext"),
            63 => Some("Samurai"),
            83 => Some("Showbiz"),
            62 => Some("Space"),
            65 => Some("StrategyGame"),
            54 => Some("TeamSports"),
            56 => Some("VideoGame"),
            75 => Some("VisualArts"),
            58 => Some("Workplace"),
            _ => None,
        };

        match g {
            Some(s) => Some(s.to_string()),
            None => None,
        }
    }
}
