use crate::helper;
use crate::models::user_model::init;

type Model = Vec<Vec<[i32; 9]>>;

struct AffinityModel<'a> {
    gte: Model,
    lte: Model,
    values: &'a Model,
    avgs: &'a Model,
    is_score_relevant: bool,
}

pub fn get_user_recommendations(
    model: [Vec<Vec<[i32; 9]>>; 2],
    user: &String,
) -> Result<Vec<String>, u16> {
    let mut affinity_model: AffinityModel = AffinityModel::new(&model[0], &model[1]);

    match helper::get_affinity_users(affinity_model.calc(1).to_array(), user) {
        Ok(v) => Ok(v),
        Err(_) => Err(500),
    }
}

impl<'a> AffinityModel<'a> {
    fn new(values: &'a Model, avgs: &'a Model) -> Self {
        Self {
            gte: init::empty_affinity(),
            lte: init::empty_affinity(),
            values,
            avgs,
            is_score_relevant: false,
        }
    }

    fn to_array(&'a mut self) -> [Model; 2] {
        [self.gte.to_owned(), self.lte.to_owned()]
    }

    fn calc(&'a mut self, accuracy: i32) -> &mut Self {
        self.calc_relevance()
            .calc_general_stats(accuracy)
            .calc_detailed_stats(accuracy)
    }

    fn calc_relevance(&'a mut self) -> &mut Self {
        // scored percentage > 25%
        self.is_score_relevant = self.values[0][0][3] > 250;
        self
    }

    fn calc_general_stats(&'a mut self, accuracy: i32) -> &mut Self {
        // list size limits
        self.gte[0][0][0] = self.values[0][0][0] / 2;
        self.lte[0][0][0] = 300 + self.values[0][0][0] * 8;
        // average mal mean score +- 0.5
        self.gte[0][0][1] = self.values[0][0][1] - (50 * accuracy);
        self.lte[0][0][1] = self.values[0][0][1] + (50 * accuracy);
        if self.is_score_relevant {
            //  average score deviation +- 0.8
            self.gte[0][0][2] = self.values[0][0][2] - (80 * accuracy);
            self.lte[0][0][2] = self.values[0][0][2] + (80 * accuracy);
        }
        // completed += 35%
        self.gte[0][1][0] = self.values[0][1][0] - (350 * accuracy);
        self.lte[0][1][0] = self.values[0][1][0] + (350 * accuracy);
        // ptw += 35%
        self.gte[0][2][0] = self.values[0][2][0] - (350 * accuracy);
        self.lte[0][2][0] = self.values[0][2][0] + (350 * accuracy);
        // watching += 35%
        self.gte[0][3][0] = self.values[0][3][0] - (350 * accuracy);
        self.lte[0][3][0] = self.values[0][3][0] + (350 * accuracy);
        // onhold += 35%
        self.gte[0][4][0] = self.values[0][4][0] - (350 * accuracy);
        self.lte[0][4][0] = self.values[0][4][0] + (350 * accuracy);
        // dropped += 35%
        self.gte[0][5][0] = self.values[0][5][0] - (350 * accuracy);
        self.lte[0][5][0] = self.values[0][5][0] + (350 * accuracy);
        self
    }

    fn calc_detailed_stats(&'a mut self, accuracy: i32) -> &mut Self {
        let mut count: i32 = 1;
        let mut tot_accuracy: i32 = 0;
        for x in 1..self.gte.len() {
            let mut max_dev: i32 = 0;
            let mut max_val: i32 = 0;
            for y in 0..self.gte[x].len() {
                if self.avgs[x][y][0].abs() > max_dev.abs() {
                    max_dev = self.avgs[x][y][0].abs();
                }
                if self.values[x][y][0] > max_val {
                    max_val = self.values[x][y][0];
                }
            }
            for y in 0..self.gte[x].len() {
                let stat_accuracy = match Self::is_stat_relevant(
                    self.avgs[x][y][0],
                    max_dev,
                    self.values[x][y][0],
                    max_val,
                ) {
                    true => accuracy,
                    false => accuracy * 5,
                };

                let v = &self.values[x][y][0];

                self.gte[x][y][0] = v - (10 * stat_accuracy + v);
                self.lte[x][y][0] = v + (10 * stat_accuracy + v);

                count += 1;
                tot_accuracy += v + (10 * stat_accuracy + v);
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
