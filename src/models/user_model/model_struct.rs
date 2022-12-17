use chrono::Datelike;

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

    pub fn from_single_value(value: StatType) -> Self {
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
    pub fn empty() -> Self {
        Self::from_single_value(0)
    }

    /// contructor for an empty affinity model (all values equal 4095)
    pub fn empty_affinity() -> Self {
        Self::from_single_value(4095)
    }

    /// contructor for the average of all users models
    pub fn average() -> Self {
        Self {
            model: avg::model(),
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    // General Stats Getters
    ////////////////////////////////////////////////////////////////////////////////

    pub fn list_length(&self) -> StatType {
        self.model[0][0][0]
    }

    pub fn general(&self) -> ModelHelper {
        ModelHelper::from_stat(self, 0, 0)
    }

    pub fn completed(&self) -> ModelHelper {
        ModelHelper::from_stat(self, 0, 1)
    }

    pub fn plan_to_watch(&self) -> ModelHelper {
        ModelHelper::from_stat(self, 0, 2)
    }

    pub fn watching(&self) -> ModelHelper {
        ModelHelper::from_stat(self, 0, 3)
    }

    pub fn on_hold(&self) -> ModelHelper {
        ModelHelper::from_stat(self, 0, 4)
    }

    pub fn dropped(&self) -> ModelHelper {
        ModelHelper::from_stat(self, 0, 5)
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Detailed Stats Getters
    ////////////////////////////////////////////////////////////////////////////////

    pub fn airing_decades(&self) -> ModelHelper {
        ModelHelper::from_stat_type(self, 1)
    }

    pub fn ratings(&self) -> ModelHelper {
        ModelHelper::from_stat_type(self, 2)
    }

    pub fn series_length(&self) -> ModelHelper {
        ModelHelper::from_stat_type(self, 3)
    }

    pub fn genres(&self) -> ModelHelper {
        ModelHelper::from_stat_type(self, 4)
    }

    pub fn themes(&self) -> ModelHelper {
        ModelHelper::from_stat_type(self, 5)
    }

    pub fn demographics(&self) -> ModelHelper {
        ModelHelper::from_stat_type(self, 6)
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Anime data to model stats conversions
    ////////////////////////////////////////////////////////////////////////////////

    pub fn general_status(&mut self, status: usize) -> ModelHelper {
        ModelHelper::from_stat(self, 0, status)
    }

    pub fn date_to_model_airing_decade(&mut self, date: chrono::NaiveDate) -> ModelHelper {
        let year: i32 = date.year();
        if year < 1991 {
            ModelHelper::from_stat(self, 0, 0)
        } else if year < 2000 {
            ModelHelper::from_stat(self, 0, 1)
        } else if year < 2010 {
            ModelHelper::from_stat(self, 0, 2)
        } else if year < 2016 {
            ModelHelper::from_stat(self, 0, 3)
        } else {
            ModelHelper::from_stat(self, 0, 4)
        }
    }

    pub fn rating_id_to_rating(&mut self, rating: i16) -> ModelHelper {
        if rating == 0 {
            ModelHelper::error(self)
        } else {
            ModelHelper::from_stat(self, 2, rating as usize - 1)
        }
    }

    pub fn n_episodes_to_series_length(&mut self, n_episodes: i16) -> ModelHelper {
        if n_episodes == 0 {
            ModelHelper::error(self)
        } else if n_episodes == 1 {
            ModelHelper::from_stat(self, 3, 0)
        } else if n_episodes < 9 {
            ModelHelper::from_stat(self, 3, 1)
        } else if n_episodes < 19 {
            ModelHelper::from_stat(self, 3, 2)
        } else if n_episodes < 33 {
            ModelHelper::from_stat(self, 3, 3)
        } else {
            ModelHelper::from_stat(self, 3, 4)
        }
    }

    pub fn genre_id_to_genres(&mut self, genre_id: i16) -> ModelHelper {
        match genre_id {
            //genres
            1 => ModelHelper::from_stat(self, 4, 0),
            2 => ModelHelper::from_stat(self, 4, 1),
            5 => ModelHelper::from_stat(self, 4, 2),
            46 => ModelHelper::from_stat(self, 4, 3),
            28 => ModelHelper::from_stat(self, 4, 4),
            4 => ModelHelper::from_stat(self, 4, 5),
            8 => ModelHelper::from_stat(self, 4, 6),
            10 => ModelHelper::from_stat(self, 4, 7),
            26 => ModelHelper::from_stat(self, 4, 8),
            47 => ModelHelper::from_stat(self, 4, 9),
            14 => ModelHelper::from_stat(self, 4, 10),
            7 => ModelHelper::from_stat(self, 4, 11),
            22 => ModelHelper::from_stat(self, 4, 12),
            24 => ModelHelper::from_stat(self, 4, 13),
            36 => ModelHelper::from_stat(self, 4, 14),
            30 => ModelHelper::from_stat(self, 4, 15),
            37 => ModelHelper::from_stat(self, 4, 16),
            41 => ModelHelper::from_stat(self, 4, 17),
            9 => ModelHelper::from_stat(self, 4, 18),
            49 => ModelHelper::from_stat(self, 4, 19),
            12 => ModelHelper::from_stat(self, 4, 20),
            //themes
            50 => ModelHelper::from_stat(self, 5, 0),
            51 => ModelHelper::from_stat(self, 5, 1),
            52 => ModelHelper::from_stat(self, 5, 2),
            53 => ModelHelper::from_stat(self, 5, 3),
            54 => ModelHelper::from_stat(self, 5, 4),
            81 => ModelHelper::from_stat(self, 5, 5),
            55 => ModelHelper::from_stat(self, 5, 6),
            39 => ModelHelper::from_stat(self, 5, 7),
            56 => ModelHelper::from_stat(self, 5, 8),
            57 => ModelHelper::from_stat(self, 5, 9),
            58 => ModelHelper::from_stat(self, 5, 10),
            35 => ModelHelper::from_stat(self, 5, 11),
            59 => ModelHelper::from_stat(self, 5, 12),
            13 => ModelHelper::from_stat(self, 5, 13),
            60 => ModelHelper::from_stat(self, 5, 14),
            61 => ModelHelper::from_stat(self, 5, 15),
            62 => ModelHelper::from_stat(self, 5, 16),
            63 => ModelHelper::from_stat(self, 5, 17),
            64 => ModelHelper::from_stat(self, 5, 18),
            65 => ModelHelper::from_stat(self, 5, 19),
            66 => ModelHelper::from_stat(self, 5, 20),
            17 => ModelHelper::from_stat(self, 5, 21),
            18 => ModelHelper::from_stat(self, 5, 22),
            67 => ModelHelper::from_stat(self, 5, 23),
            38 => ModelHelper::from_stat(self, 5, 24),
            19 => ModelHelper::from_stat(self, 5, 25),
            6 => ModelHelper::from_stat(self, 5, 26),
            68 => ModelHelper::from_stat(self, 5, 27),
            69 => ModelHelper::from_stat(self, 5, 28),
            20 => ModelHelper::from_stat(self, 5, 29),
            70 => ModelHelper::from_stat(self, 5, 30),
            71 => ModelHelper::from_stat(self, 5, 31),
            40 => ModelHelper::from_stat(self, 5, 32),
            3 => ModelHelper::from_stat(self, 5, 33),
            72 => ModelHelper::from_stat(self, 5, 34),
            73 => ModelHelper::from_stat(self, 5, 35),
            74 => ModelHelper::from_stat(self, 5, 36),
            21 => ModelHelper::from_stat(self, 5, 37),
            23 => ModelHelper::from_stat(self, 5, 38),
            75 => ModelHelper::from_stat(self, 5, 39),
            29 => ModelHelper::from_stat(self, 5, 40),
            11 => ModelHelper::from_stat(self, 5, 41),
            31 => ModelHelper::from_stat(self, 5, 42),
            76 => ModelHelper::from_stat(self, 5, 43),
            77 => ModelHelper::from_stat(self, 5, 44),
            78 => ModelHelper::from_stat(self, 5, 45),
            32 => ModelHelper::from_stat(self, 5, 46),
            79 => ModelHelper::from_stat(self, 5, 47),
            80 => ModelHelper::from_stat(self, 5, 48),
            48 => ModelHelper::from_stat(self, 5, 49),
            //demographics
            43 => ModelHelper::from_stat(self, 6, 0),
            15 => ModelHelper::from_stat(self, 6, 1),
            42 => ModelHelper::from_stat(self, 6, 2),
            25 => ModelHelper::from_stat(self, 6, 3),
            27 => ModelHelper::from_stat(self, 6, 4),
            _ => ModelHelper::error(self)
        }
    }

}

pub struct ModelHelper<'a> {
    model: &'a UserModel,
    stat_type: usize,
    stat: usize,
}

impl<'a> ModelHelper<'a> {
    // CONSTRUCTORS

    pub fn from_stat_type(model: &'a UserModel, stat_type: usize) -> Self {
        Self {
            model,
            stat_type,
            stat: 0,
        }
    }

    pub fn from_stat(model: &'a UserModel, stat_type: usize, stat: usize) -> Self {
        Self {
            model,
            stat_type,
            stat,
        }
    }

    pub fn error(model: &'a UserModel) -> Self {
        Self {
            model,
            stat_type: 0,
            stat: 6,
        } 
    }

    /// # select a specific stat
    /// updates the stat field on the ModelHelper instance 
    /// 
    /// the [average model](src/models/user_model/avg.rs) has comments for all stats indexes
    /// 
    /// **required on detailed stats**  
    /// *! do not use it on general stats*
    /// 
    /// # example
    /// ```
    /// let model = UserModel::average();
    /// let _2010s_anime_stats: ModelHelper = model.airing_decades().stat(3);
    /// let _2020s_anime_stats: ModelHelper = model.airing_decades().stat(4);
    /// 
    /// _2010s_anime_stats.score(); // &755
    /// 
    /// ```
    pub fn stat(&mut self, i: usize) -> &mut Self {
        self.stat = i;
        self
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Stats GETTERS
    ////////////////////////////////////////////////////////////////////////////////

    /// # percentage
    /// returns the **percentage** of a (general or detailed) **stat**  
    ///
    /// *! **model.general().percentage()** EQUALS to **model.list_length()***
    /// 
    /// # example
    /// ```
    /// let model = UserModel::average();
    /// model.airing_decades().stat(3).percentage(); // &331
    /// model.completed().percentage(); // &642
    /// 
    /// ```
    pub fn percentage(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][0]
    }

    /// # average (MAL) mean score
    /// returns the **average score** of a (general or detailed) **stat**  
    /// 
    /// ## important
    /// the MAL mean score is way more important than the user score, when it
    /// comes to models comparison, therefore the average score stored by the
    /// user model is an **average** of all the **mal mean scores** of a given statistic.
    /// 
    /// to get the average of the **actual user score** you can add together **score()**
    /// and **score_deviation()**
    /// 
    /// # example
    /// ```
    /// let model = UserModel::average();
    /// model.airing_decades().stat(3).score(); // &755
    /// model.completed().score(); // &758
    /// 
    /// ```
    pub fn score(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][1]
    }

    /// # score deviation
    /// returns the **score deviation** of a (general or detailed) **stat**  
    ///
    /// # example
    /// ```
    /// let model = UserModel::average();
    /// model.airing_decades().stat(3).score_deviation(); // &-39
    /// model.completed().score_deviation(); // &-25
    /// 
    /// ```
    pub fn score_deviation(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][2]
    }

    /// # scored percentage
    /// returns the **scored** percentage of a (general or detailed) **stat**  
    ///
    /// # example
    /// ```
    /// let model = UserModel::average();
    /// model.airing_decades().stat(3).scored_percentage(); // &660
    /// model.completed().scored_percentage(); // &915
    /// 
    /// ```
    pub fn scored_percentage(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][3]
    }

    // statuses percentage

    /// # completed percentage
    /// returns the **completed** percentage of a **detailed stat**  
    /// 
    /// *! Do NOT use this method on general stats*
    /// # example
    /// ```
    /// let model = UserModel::average();
    /// model.airing_decades().stat(3).completed(); // &675
    /// 
    /// ```
    pub fn completed(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][4]
    }

    /// # plan to watch percentage
    /// returns the **plan to watch** percentage of a **detailed stat**  
    /// 
    /// *! Do NOT use this method on general stats*
    /// # example
    /// ```
    /// let model = UserModel::average();
    /// model.airing_decades().stat(3).plan_to_watch(); // &229
    /// 
    /// ```
    pub fn plan_to_watch(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][5]
    }

    /// # watching percentage
    /// returns the **watching** percentage of a **detailed stat**  
    /// 
    /// *! Do NOT use this method on general stats*
    /// # example
    /// ```
    /// let model = UserModel::average();
    /// model.airing_decades().stat(3).watching(); // &26
    /// 
    /// ```
    pub fn watching(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][6]
    }

    /// # on hold percentage
    /// returns the **on hold** percentage of a **detailed stat**  
    /// 
    /// *! Do NOT use this method on general stats*
    /// # example
    /// ```
    /// let model = UserModel::average();
    /// model.airing_decades().stat(3).on_hold(); // &27
    /// 
    /// ```
    pub fn on_hold(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][7]
    }

    /// # dropped percentage
    /// returns the **dropped** percentage of a **detailed stat**  
    /// 
    /// *! Do NOT use this method on general stats*
    /// # example
    /// ```
    /// let model = UserModel::average();
    /// model.airing_decades().stat(3).dropped(); // &33
    /// 
    /// ```
    pub fn dropped(&self) -> &StatType {
        &self.model.model[self.stat_type][self.stat][8]
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Stats SETTERS
    ////////////////////////////////////////////////////////////////////////////////
    pub fn set_percentage(&self, v: StatType) {
        self.model.model[self.stat_type][self.stat][0] = v;
    }

    pub fn incr_percentage(&self) {
        self.model.model[self.stat_type][self.stat][0] += 1;
    }

    pub fn set_score(&self, v: StatType) {
        self.model.model[self.stat_type][self.stat][1] = v;
    }

    pub fn incr_score(&self, v: StatType) {
        self.model.model[self.stat_type][self.stat][1] += v;
    }

    pub fn set_score_deviation(&self, v: StatType) {
        self.model.model[self.stat_type][self.stat][2] = v;
    }

    pub fn incr_score_deviation(&self, v: StatType) {
        self.model.model[self.stat_type][self.stat][2] += v;
    }

    pub fn set_scored_percentage(&self, v: StatType) {
        self.model.model[self.stat_type][self.stat][3] = v;
    }

    pub fn incr_scored_percentage(&self, v: StatType) {
        self.model.model[self.stat_type][self.stat][3] += v;
    }

    pub fn set_status(&self, v: StatType, status_index: usize) {
        self.model.model[self.stat_type][self.stat][3 + status_index] = v;
    }

    pub fn incr_status(&self, status_index: usize) {
        self.model.model[self.stat_type][self.stat][3 + status_index] += 1;
    }
}
