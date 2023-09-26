use super::MALClient;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use structs::ListEntry as PublicListEntry;
use structs::{RekoError, RekoResult};
use util::{clamp, days_from, now, sleep};

const LIST_QUERY: &str = "?fields=list_status&sort=list_updated_at&nsfw=1";
const WATCHED: &[&str] = &["completed", "watching"];

impl MALClient {
    pub async fn list(
        &self,
        user: String,
        updated_at: Option<NaiveDateTime>,
    ) -> RekoResult<Vec<PublicListEntry>> {
        let (is_update, updated_at) = match updated_at {
            Some(date) => (true, date),
            None => (false, now()),
        };

        let limit = if is_update {
            clamp(days_from(updated_at) * 3, 5, 1000)
        } else {
            1000
        };

        let mut offset = 0;
        let mut res: Vec<PublicListEntry> = Vec::new();

        while offset < 16 {
            let raw = match self
                .get::<List>(format!(
                    "/users/{user}/animelist{LIST_QUERY}&limit={limit}&offset={}",
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

            res.append(&mut entries);

            if raw.paging.next.is_some() && entries.len() == limit {
                offset += 1;
            } else {
                break;
            }

            if offset > 2 {
                sleep(250);
            }
        }

        Ok(res)
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
struct AnimeNode {
    id: i32,
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
        .unwrap_or(now());

        PublicListEntry {
            id: self.node.id,
            score: self.list_status.score,
            watched: WATCHED.contains(&self.list_status.status.as_str())
                && self.list_status.num_episodes_watched > 0,
            updated_at,
        }
    }
}

fn list_err(code: u16, user: &String) -> RekoError {
    RekoError::new(
        code,
        match code {
            404 => "UserNotFound",
            403 => "PrivateUserList",
            422 => "InvalidUserList",
            _ => "FetchFailed",
        },
        match code {
            404 => format!("User {user} not found"),
            403 => format!("User {user} is private"),
            422 => format!("Could not parse User {user} list"),
            _ => format!("Could not fetch User {user} list"),
        },
    )
}
