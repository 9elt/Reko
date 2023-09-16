use config::*;

use structs::Anime as PublicAnime;
use structs::ListEntry as PublicListEntry;

use chrono::{prelude::Datelike, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use reqwest::{header::USER_AGENT, Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;

const MAL_API: &str = "https://api.myanimelist.net/v2";
const ANIME_QUERY: &str =
    "?fields=id,title,main_picture,start_date,mean,status,genres,num_episodes,rating,related_anime";
const LIST_QUERY: &str = "?fields=list_status&sort=list_updated_at&nsfw=1";
const RELATED: &[&str] = &["full_story", "parent_story", "prequel"];
const WATCHED: &[&str] = &["completed", "watching"];

type MALResult<T> = Result<T, MALError>;

#[derive(Serialize, Deserialize, Debug)]
pub struct MALError {
    code: u16,
    message: String,
}

impl MALError {
    fn list(code: u16) -> Self {
        let message = String::from(match code {
            404 => "User not found",
            403 => "User is private",
            422 => "Could not parse user list",
            _ => "Could not fetch user list",
        });
        Self { code, message }
    }
    fn anime(code: u16) -> Self {
        let message = String::from(match code {
            404 => "Anime not found",
            403 => "Rate limited",
            422 => "Could not parse anime details",
            _ => "Could not fetch anime details",
        });
        Self { code, message }
    }
}

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
    ) -> MALResult<Vec<PublicListEntry>> {
        let is_update = updated_at.is_some();
        let updated_at = if is_update {
            updated_at.unwrap()
        } else {
            Utc::now().naive_utc()
        };

        let limit = if is_update {
            let now = Utc::now().naive_utc();
            let days_since = now.signed_duration_since(updated_at).num_days();
            days_since * 2
        } else {
            1000
        };

        let mut offset = 0;
        let mut res: Vec<PublicListEntry> = Vec::new();

        while offset < 9 {
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
                        return Err(MALError::list(code));
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

            if raw.paging.next.is_some() && entries.len() > 0 {
                res.append(&mut entries);
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
    pub async fn anime(&self, id: i32) -> MALResult<PublicAnime> {
        let raw = if ENABLE_FAKE_API {
            match fake_anime_api(id) {
                Ok(res) => res,
                Err(code) => return Err(MALError::anime(code)),
            }
        } else {
            match self
                .get::<Anime>(format!("{MAL_API}/anime/{id}{ANIME_QUERY}"))
                .await
            {
                Ok(res) => res,
                Err(code) => return Err(MALError::anime(code)),
            }
        };

        Ok(raw.to_public())
    }
    async fn get<R: for<'a> Deserialize<'a>>(&self, url: String) -> Result<R, u16> {
        let res = self
            .client
            .get(url)
            .header(USER_AGENT, "reqwest")
            .header("x-mal-client-id", self.client_id.as_str())
            .send()
            .await;

        match res {
            Ok(res) => match res.status() {
                StatusCode::OK => match res.json::<R>().await {
                    Ok(res) => Ok(res),
                    _ => Err(422),
                },
                code => Err(code.as_u16()),
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
        let mut prequels: Vec<i32> = Vec::new();

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

        self.related_anime.iter().for_each(|related| {
            if RELATED.contains(&related.relation_type.as_str()) {
                prequels.push(related.node.id);
            }
        });

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
            prequels,
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
    fn from_series_len(num_episodes: i32) -> Self {
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
    fn from_rating(rating: String) -> Self {
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
    fn from_genre_id(genre_id: i32) -> Self {
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
