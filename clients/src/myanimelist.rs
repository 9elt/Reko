use structs::Anime as PublicAnime;
use structs::ListEntry as PublicListEntry;

use chrono::{prelude::Datelike, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use reqwest::{header::USER_AGENT, Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use structs::{RekoError, RekoResult};

pub const ENABLE_FAKE_API: bool = true;
pub const FAKE_API_PATH: &str = "/home/nelt/projects/anime/database";

const MAL_API: &str = "https://api.myanimelist.net/v2";
const ANIME_QUERY: &str =
    "?fields=id,title,main_picture,start_date,mean,status,genres,num_episodes,rating,related_anime";
const LIST_QUERY: &str = "?fields=list_status&sort=list_updated_at&nsfw=1";
const PARENT: &[&str] = &["prequel", "parent_story", "full_story"];
const WATCHED: &[&str] = &["completed", "watching"];

#[derive(Clone)]
pub struct MALClient {
    client_id: String,
    client: Client,
}

impl MALClient {
    pub fn new() -> Self {
        let client_id = env::var("MAL_CLIENT_ID")
            .expect("Missing MAL client id")
            .to_string();

        Self {
            client_id,
            client: Client::new(),
        }
    }
    pub async fn list(
        &self,
        user: String,
        updated_at: Option<NaiveDateTime>,
    ) -> RekoResult<Vec<PublicListEntry>> {
        let is_update = updated_at.is_some();
        let updated_at = if is_update {
            updated_at.unwrap()
        } else {
            Utc::now().naive_utc()
        };

        let limit = if is_update {
            let now = Utc::now().naive_utc();
            let days_since = now.signed_duration_since(updated_at).num_days();
            days_since * 3
        } else {
            1000
        };

        let mut offset = 0;
        let mut res: Vec<PublicListEntry> = Vec::new();

        while offset < 16 {
            let raw = match self
                .get::<List>(format!(
                    "{MAL_API}/users/{user}/animelist{LIST_QUERY}&limit={limit}&offset={}",
                    offset * limit
                ))
                .await
            {
                Ok(res) => res,
                Err(code) => {
                    if offset == 0 {
                        return Err(list_err(code, &user));
                    } else {
                        break;
                    }
                }
            };

            let mut entries: Vec<_> = raw
                .data
                .iter()
                .map(|e| e.to_public())
                .filter(|e| !is_update || e.updated_at > updated_at)
                .collect();

            let found = entries.len();
            res.append(&mut entries);

            if raw.paging.next.is_some() && found > 0 {
                offset += 1;
            } else {
                break;
            }

            if offset > 2 {
                thread::sleep(Duration::from_millis(250));
            }
        }

        Ok(res)
    }
    pub async fn anime(&self, id: i32) -> RekoResult<PublicAnime> {
        let raw = if ENABLE_FAKE_API {
            match fake_anime_api(id) {
                Ok(res) => res,
                Err(code) => return Err(anime_err(code)),
            }
        } else {
            match self
                .get::<Anime>(format!("{MAL_API}/anime/{id}{ANIME_QUERY}"))
                .await
            {
                Ok(res) => res,
                Err(code) => return Err(anime_err(code)),
            }
        };

        Ok(raw.to_public())
    }
    async fn get<R: for<'a> Deserialize<'a>>(&self, url: String) -> Result<R, u16> {
        let res = self
            .client
            .get(&url)
            .header(USER_AGENT, "reqwest")
            .header("x-mal-client-id", self.client_id.as_str())
            .send()
            .await;

        match res {
            Ok(res) => match res.status() {
                StatusCode::OK => {
                    println!("GET {} OK", &url);
                    match res.json::<R>().await {
                        Ok(res) => Ok(res),
                        _ => Err(422),
                    }
                }
                code => {
                    let code = code.as_u16();
                    println!("GET {} ERR {}", &url, code);
                    Err(code)
                }
            },
            _ => Err(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct List {
    data: Vec<ListEntry>,
    paging: Pagination,
}

#[derive(Serialize, Deserialize, Debug)]
struct Pagination {
    // previous: Option<String>,
    next: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListEntry {
    node: AnimeNode,
    list_status: EntryStatus,
}

#[derive(Serialize, Deserialize, Debug)]
struct EntryStatus {
    status: String,
    score: i32,
    num_episodes_watched: i32,
    is_rewatching: bool,
    updated_at: String,
}

impl ListEntry {
    fn to_public(&self) -> PublicListEntry {
        let updated_at = match NaiveDateTime::parse_from_str(
            &self.list_status.updated_at,
            "%Y-%m-%dT%H:%M:%S%z",
        ) {
            Ok(date) => Some(date),
            _ => None,
        }
        .unwrap_or(Utc::now().naive_utc());

        PublicListEntry {
            id: self.node.id,
            score: self.list_status.score,
            watched: WATCHED.contains(&self.list_status.status.as_str())
                && self.list_status.num_episodes_watched > 0,
            updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Anime {
    id: i32,
    title: String,
    main_picture: Option<AnimePicture>,
    start_date: Option<String>,
    mean: Option<f32>,
    status: String,
    genres: Option<Vec<AnimeGenre>>,
    num_episodes: Option<i32>,
    rating: Option<String>,
    related_anime: Vec<AnimeRelated>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnimeRelated {
    node: AnimeNode,
    relation_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnimeNode {
    id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnimeGenre {
    id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimePicture {
    pub medium: String,
}

impl Anime {
    fn to_public(self) -> PublicAnime {
        let mut stats: Vec<i32> = Vec::new();

        let airing_date = match self.start_date {
            Some(d) => match NaiveDate::parse_from_str(&d, "%Y-%m-%d") {
                Ok(date) => Some(date.and_time(NaiveTime::default())),
                _ => None,
            },
            None => None,
        };

        if let Some(date) = airing_date {
            stats.push(Stat::from_airing_date(date).uid());
        }

        if let Some(num_episodes) = self.num_episodes {
            stats.push(Stat::from_series_len(num_episodes).uid());
        }

        if let Some(rating) = self.rating.to_owned() {
            let stat = Stat::from_rating(rating);
            if stat.is_valid() {
                stats.push(stat.uid());
            }
        }

        if let Some(genres) = self.genres {
            genres.iter().for_each(|genre| {
                let stat = Stat::from_genre_id(genre.id);
                if stat.is_valid() {
                    stats.push(stat.uid());
                }
            });
        }

        let parent = match self
            .related_anime
            .iter()
            .find(|r| PARENT.contains(&r.relation_type.as_str()))
        {
            Some(p) => Some(p.node.id),
            None => None,
        };

        PublicAnime {
            id: self.id,
            title: self.title,
            airing_date,
            length: self.num_episodes,
            mean: self.mean,
            rating: self.rating,
            picture: match self.main_picture {
                Some(pic) => Some(pic.medium),
                None => None,
            },
            stats,
            parent,
            aired: self.status == "finished_airing",
            updated_at: Utc::now().naive_utc(),
        }
    }
}

struct Stat(i32);

impl Stat {
    fn is_valid(&self) -> bool {
        self.0 != -1
    }
    fn uid(self) -> i32 {
        self.0
    }
    fn from_airing_date(date: NaiveDateTime) -> Self {
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
    fn from_series_len(num_episodes: i32) -> Self {
        if num_episodes == 1 {
            Self(16)
        } else if num_episodes < 9 {
            Self(33)
        } else if num_episodes < 19 {
            Self(0)
        } else if num_episodes < 33 {
            Self(8)
        } else {
            Self(9)
        }
    }
    fn from_rating(rating: String) -> Self {
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
    fn from_genre_id(genre_id: i32) -> Self {
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
}

fn list_err(code: u16, user: &String) -> RekoError {
    let message = match code {
        404 => format!("User {user} not found"),
        403 => format!("User {user} is private"),
        422 => format!("Could not parse User {user} list"),
        _ => format!("Could not fetch User {user} list"),
    };
    RekoError { code, message }
}

fn anime_err(code: u16) -> RekoError {
    let message = String::from(match code {
        404 => "Anime not found",
        403 => "Rate limited",
        422 => "Could not parse anime details",
        _ => "Could not fetch anime details",
    });
    RekoError { code, message }
}

fn fake_anime_api(id: i32) -> Result<Anime, u16> {
    let anime = match fs::read_to_string(format!("{FAKE_API_PATH}/ani-{id}.json")) {
        Ok(f) => match serde_json::from_str::<Anime>(&f) {
            Ok(anime) => anime,
            Err(_) => return Err(422),
        },
        Err(_) => return Err(404),
    };

    Ok(anime)
}
