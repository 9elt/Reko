use super::avg;

type StatType = i32;
const STAT_SIZE: usize = 9;

const GENERAL_LEN: usize = 6;
const AIRING_DECADES_LEN: usize = 5;
const RATINGS_LEN: usize = 6;
const SERIES_LENGTH_LEN: usize = 5;
const GENRES_LEN: usize = 21;
const THEMES_LEN: usize = 50;
const DEMOGRAPHICS_LEN: usize = 5;

#[derive(Clone, Debug)]
pub struct UserModel {
    model: Vec<Vec<[StatType; STAT_SIZE]>>,
}

impl UserModel {
    // CONSTRUCTORS

    fn from_single_value(value: StatType) -> Self {
        Self {
            model: vec![
                vec![[value; STAT_SIZE]; GENERAL_LEN],
                vec![[value; STAT_SIZE]; AIRING_DECADES_LEN],
                vec![[value; STAT_SIZE]; RATINGS_LEN],
                vec![[value; STAT_SIZE]; SERIES_LENGTH_LEN],
                vec![[value; STAT_SIZE]; GENRES_LEN],
                vec![[value; STAT_SIZE]; THEMES_LEN],
                vec![[value; STAT_SIZE]; DEMOGRAPHICS_LEN],
            ],
        }
    }

    /// contructor for an empty model (all values equal 0)
    fn empty() -> Self {
        Self::from_single_value(0)
    }

    /// contructor for an empty affinity model (all values equal 4095)
    fn empty_affinity() -> Self {
        Self::from_single_value(4095)
    }

    /// contructor for the average of all users models
    fn average() -> Self {
        Self {
            model: avg::model(),
        }
    }

    // general stats

    fn list_length(&self) -> StatType {
        self.model[0][0][0]
    }

    fn general(&self) -> ModelHelper {
        ModelHelper::from_stat(self, 0, 0)
    }

    fn completed(&self) -> ModelHelper {
        ModelHelper::from_stat(self, 0, 1)
    }

    fn plan_to_watch(&self) -> ModelHelper {
        ModelHelper::from_stat(self, 0, 2)
    }

    fn watching(&self) -> ModelHelper {
        ModelHelper::from_stat(self, 0, 3)
    }

    fn on_hold(&self) -> ModelHelper {
        ModelHelper::from_stat(self, 0, 4)
    }

    fn dropped(&self) -> ModelHelper {
        ModelHelper::from_stat(self, 0, 5)
    }

    // detailed stats

    fn airing_decades(&self) -> ModelHelper {
        ModelHelper::from_stat_type(self, 1)
    }

    fn ratings(&self) -> ModelHelper {
        ModelHelper::from_stat_type(self, 2)
    }

    fn series_length(&self) -> ModelHelper {
        ModelHelper::from_stat_type(self, 3)
    }

    fn genres(&self) -> ModelHelper {
        ModelHelper::from_stat_type(self, 4)
    }

    fn themes(&self) -> ModelHelper {
        ModelHelper::from_stat_type(self, 5)
    }

    fn demographics(&self) -> ModelHelper {
        ModelHelper::from_stat_type(self, 6)
    }
}

struct ModelHelper<'a> {
    model: &'a UserModel,
    stat_type: usize,
    stat: usize,
}

impl<'a> ModelHelper<'a> {
    // CONSTRUCTORS

    fn from_stat_type(model: &'a UserModel, stat_type: usize) -> Self {
        Self {
            model,
            stat_type,
            stat: 0,
        }
    }

    fn from_stat(model: &'a UserModel, stat_type: usize, stat: usize) -> Self {
        Self {
            model,
            stat_type,
            stat,
        }
    }

    fn stat(&mut self, i: usize) -> &mut Self {
        self.stat = i;
        self
    }

    // get stat info

    fn percentage(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][0]
    }

    fn score(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][1]
    }
    
    fn score_deviation(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][2]
    }

    fn scored_percentage(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][3]
    }

    // get specific statuses

    fn completed(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][4]
    }

    fn plan_to_watch(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][5]
    }

    fn watching(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][6]
    }

    fn on_hold(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][7]
    }

    fn dropped(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][8]
    }
}

fn test() {
    let model = UserModel::empty();
    let x = model.general().score();
    let a = model.airing_decades().stat(4).score_deviation();
    let g = model.themes().stat(35).completed();
}