use super::{Model, ModelVec, user::UserModel};

pub struct AffinityModel {
    gte: Model,
    lte: Model,
    user: UserModel,
    is_score_relevant: bool,
    accuracy: i32,
}

const EXPECTED_TOTAL_DEVIATION: i32 = 45;

#[derive(Debug, Clone)]
struct Deviation {
    max: i32,
    total: i32,
}

impl Deviation {
    fn calc(m: &Model) -> Self {
        let mut total = 0;
        let mut max = 0;

        for x in 0..m.len(){
            for y in 0..m[x].len() {
                for z in 0..m[x][y].len() {
                    total += m[x][y][z].abs();
                    if m[x][y][z].abs() > max {
                        max = m[x][y][z].abs();
                    }
                }
            }
        }

        total /= 892;

        Self { max, total }
    }
}

impl AffinityModel {
    pub fn new(user_model: UserModel) -> Self {
        Self {
            gte: Model::compare(),
            lte: Model::compare(),
            user: user_model,
            is_score_relevant: false,
            accuracy: 10,
        }
    }

    pub fn to_array(self) -> [ModelVec; 2] {
        [self.gte.to_vec(), self.lte.to_vec()]
    }

    pub fn calc(& mut self, accuracy: i32) -> &mut Self {
        self.calc_relevance()
            .calc_accuracy(accuracy)
            .calc_general_stats()
            .calc_detailed_stats()
    }

    fn calc_accuracy(& mut self, accuracy: i32) -> &mut Self {

        let stats = &self.user.stats;
        let dev = &self.user.deviation;

        let deviation = Deviation::calc(&self.user.deviation);

        println!("list length: {}", stats[0][0][0]);
        println!("list length deviation: {}", dev[0][0][0]);
        println!("TOT average deviation: {}", deviation.total);
        println!("MAX average deviation: {}", deviation.max);

        let len = match dev[0][0][0] > 0 {
            true => dev[0][0][0].abs() / -10,
            false => dev[0][0][0].abs() / 10
        };

        let ov = match deviation.total - EXPECTED_TOTAL_DEVIATION < 0 {
            true => 0,
            false => deviation.total - EXPECTED_TOTAL_DEVIATION
        };

        let adj = (ov + len) / (2 + ov / (EXPECTED_TOTAL_DEVIATION / 3));
        // needs iterpolation for adj and lev
        // adj sjo

        println!("lev: {}", len);
        println!("adj: {}", adj);

        self.accuracy = accuracy + adj + len;
        println!("accuracy: {}", self.accuracy);
        self
    }

    fn calc_relevance(& mut self) -> &mut Self {
        let stats = &self.user.stats;
        let dev = &self.user.deviation;

        // scored percentage > 25%
        self.is_score_relevant = stats[0][0][3] > 250;
        self
    }

    fn calc_general_stats(& mut self) -> &mut Self {
        let stats = &self.user.stats;
        let dev = &self.user.deviation;

        // list size limits
        self.gte[0][0][0] = stats[0][0][0] / 2;
        self.lte[0][0][0] = 300 + stats[0][0][0] * 8;
        // average mal mean score +- 0.5
        self.gte[0][0][1] = stats[0][0][1] - (5 * self.accuracy);
        self.lte[0][0][1] = stats[0][0][1] + (5 * self.accuracy);
        if self.is_score_relevant {
            //  average score deviation +- 0.8
            self.gte[0][0][2] = stats[0][0][2] - (8 * self.accuracy);
            self.lte[0][0][2] = stats[0][0][2] + (8 * self.accuracy);
        }
        // completed += 35%
        self.gte[0][1][0] = stats[0][1][0] - (35 * self.accuracy);
        self.lte[0][1][0] = stats[0][1][0] + (35 * self.accuracy);
        // ptw += 35%
        self.gte[0][2][0] = stats[0][2][0] - (35 * self.accuracy);
        self.lte[0][2][0] = stats[0][2][0] + (35 * self.accuracy);
        // watching += 35%
        self.gte[0][3][0] = stats[0][3][0] - (35 * self.accuracy);
        self.lte[0][3][0] = stats[0][3][0] + (35 * self.accuracy);
        // onhold += 35%
        self.gte[0][4][0] = stats[0][4][0] - (35 * self.accuracy);
        self.lte[0][4][0] = stats[0][4][0] + (35 * self.accuracy);
        // dropped += 35%
        self.gte[0][5][0] = stats[0][5][0] - (35 * self.accuracy);
        self.lte[0][5][0] = stats[0][5][0] + (35 * self.accuracy);
        self
    }

    fn calc_detailed_stats(& mut self) -> &mut Self {
        let stats = &self.user.stats;
        let dev = &self.user.deviation;

        let mut count: i32 = 1;
        let mut tot_accuracy: i32 = 0;
        for x in 1..self.gte.len() {
            let mut max_dev: i32 = 0;
            let mut max_val: i32 = 0;
            for y in 0..self.gte[x].len() {
                if dev[x][y][0].abs() > max_dev.abs() {
                    max_dev = dev[x][y][0].abs();
                }
                if stats[x][y][0] > max_val {
                    max_val = stats[x][y][0];
                }
            }
            for y in 0..self.gte[x].len() {
                let stat_accuracy = match Self::is_stat_relevant(
                    dev[x][y][0],
                    max_dev,
                    stats[x][y][0],
                    max_val,
                ) {
                    true => self.accuracy,
                    false => self.accuracy * 5,
                };

                let v = &stats[x][y][0];

                self.gte[x][y][0] = v - (stat_accuracy + v);
                self.lte[x][y][0] = v + (stat_accuracy + v);

                count += 1;
                tot_accuracy += v + (stat_accuracy + v);
            }
        }
        tot_accuracy = tot_accuracy / count;

        println!("average accuracy: {tot_accuracy}");
        self
    }

    fn is_stat_relevant(avg_dev: i32, max_dev: i32, value: i32, max_val: i32) -> bool {
        (avg_dev > max_dev / 2) || (avg_dev < max_dev / -2) || value == 0 || value > (max_val * 2) / 3
    }

    fn _is_stat_score_relevant(
        score_dev: i32,
        tot_score_dev: i32,
        scored_pct: i32,
        tot_scored_pct: i32,
    ) -> bool {
        scored_pct > tot_scored_pct
            && (score_dev < (tot_score_dev - tot_score_dev.abs() / 2)
                || score_dev > (tot_score_dev + tot_score_dev.abs() / 2))
    }
}
