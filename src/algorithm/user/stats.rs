use crate::algorithm::model::{Model, Indexer};
use crate::helper::DetailedListEntry;

/// ### User statistics model
/// Generates a statistics model from an anime list
pub fn stats_model(list: Vec<DetailedListEntry>) -> Model<i16> {
    let mut model = Model::<i32>::empty();

    for entry in list.iter() {
        let info = match EntryData::new(entry) {
            Ok(val) => val,
            Err(_) => continue,
        };

        // increment model with entry data
        model
            // general
            .incr_stat(Indexer::general(), &info)
            // airing decade
            .incr_optional(&entry.airing_date(), Indexer::date, &info)
            // rating
            .incr_optional(&entry.rating(), Indexer::rating, &info)
            // series length
            .incr_optional(&entry.num_episodes(), Indexer::num_episodes, &info)
            // genres | themes | demographics
            .incr_optional_seq(entry.genres(), Indexer::genre, &info);
    }

    // average model statistics
    for x in 0..model.len() {
        let total: i32 = model[x].iter().map(|i| i[0]).sum();
        for y in 0..model[x].len() {
            model.average_stat(x, y, total);
        }
    }

    model.to_i16()
}

////////////////////////////////////////////////////////////////////////////////
// Private Model implementation
////////////////////////////////////////////////////////////////////////////////

impl Model<i32> {
    /// ### calls `incr_optional` sequentially
    fn incr_optional_seq<T>(
        &mut self,
        stats: &Option<Vec<Option<T>>>,
        idxr: fn(value: &T) -> Indexer,
        info: &EntryData,
    ) -> &mut Self {
        match stats {
            Some(vector) => {
                for stat in vector.iter() {
                    self.incr_optional(&stat, idxr, &info);
                }
            }
            None => (),
        };
        self
    }

    /// ### Optionally increment `Model` statistic at `index` with `EntryData`
    fn incr_optional<T>(
        &mut self,
        stat: &Option<T>,
        idxr: fn(value: &T) -> Indexer,
        info: &EntryData,
    ) -> &mut Self {
        match stat {
            Some(value) => self.incr_stat(idxr(value), info),
            None => self,
        }
    }

    /// ### Increment `Model` statistic at `index` with `EntryData`
    fn incr_stat(&mut self, index: Indexer, info: &EntryData) -> &mut Self {
        if !index.has_errors() {
            self[index.x][index.y][0] += 1;
            self[index.x][index.y][1] += info.score;
            self[index.x][index.y][2] += info.deviation;
            self[index.x][index.y][3] += info.score_counter;
            self[index.x][index.y][info.status + 3] += 1;
        }
        self
    }

    /// ### average `Model` statistic at index `[x][y]`
    fn average_stat(&mut self, x: usize, y: usize, total: i32) -> &mut Self {
        self[x][y][1] = div(self[x][y][1], self[x][y][0]);
        self[x][y][2] = div(self[x][y][2], self[x][y][3]);
        for z in 3..9 {
            self[x][y][z] = perc(self[x][y][z], self[x][y][0]);
        }
        if x > 0 {
            self[x][y][0] = perc(self[x][y][0], total);
        }
        self
    }
}

////////////////////////////////////////////////////////////////////////////////
// Entry data
////////////////////////////////////////////////////////////////////////////////

struct EntryData {
    score: i32,
    deviation: i32,
    score_counter: i32,
    status: usize,
}

impl EntryData {
    fn new(entry: &DetailedListEntry) -> Result<Self, ()> {
        let score: i32 = match entry.mean() {
            Some(mean) => mean as i32,
            None => return Err(()),
        };

        let user_score: i32 = entry.score();

        Ok(Self {
            score,
            deviation: match user_score {
                0 => 0,
                _ => user_score - score as i32,
            },
            score_counter: match user_score {
                0 => 0,
                _ => 1,
            },
            status: entry.status() as usize,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////
// Utility
////////////////////////////////////////////////////////////////////////////////

/// ### divides `num` by `den`
fn div(num: i32, den: i32) -> i32 {
    match den {
        0 => 0,
        _ => num / den,
    }
}

/// ### percentage of `num` on `den`
fn perc(num: i32, den: i32) -> i32 {
    div(num * 1000, den)
}
