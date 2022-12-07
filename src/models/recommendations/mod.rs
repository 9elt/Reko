mod affinity;

use crate::helper;

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

    match helper::get_affinity_users(affinity_model.calc().get(), user) {
        Ok(v) => Ok(v),
        Err(_) => Err(500),
    }
}

impl<'a> AffinityModel<'a> {
    fn new(values: &'a Model, avgs: &'a Model) -> Self {
        Self {
            gte: affinity::model(),
            lte: affinity::model(),
            values,
            avgs,
            is_score_relevant: false,
        }
    }

    fn get(&'a mut self) -> [Model; 2] {
        [self.gte.to_owned(), self.lte.to_owned()]
    }

    fn calc(&'a mut self) -> &mut Self {
        self.calc_relevance()
            .calc_general_stats()
            .calc_detailed_stats()
    }

    fn calc_relevance(&'a mut self) -> &mut Self {
        self.is_score_relevant = self.values[0][0][3] > 250;
        self
    }

    fn calc_general_stats(&'a mut self) -> &mut Self {
        self.gte[0][0][0] = self.values[0][0][0] - 300;
        self.lte[0][0][0] = 100 + self.values[0][0][0] * 4;
        if self.is_score_relevant {
            self.gte[0][0][1] = self.values[0][0][1] - 25;
            self.lte[0][0][1] = self.values[0][0][1] + 25;

            self.gte[0][0][2] = self.values[0][0][2] - 15;
            self.lte[0][0][2] = self.values[0][0][2] + 15;
        }
        self
    }

    fn calc_detailed_stats(&'a mut self) -> &mut Self {
        for x in 1..self.gte.len() {
            for y in 0..self.gte[x].len() {
                if (self.avgs[x][y][0] > 60 && self.values[x][y][0] > 3)
                    || (self.avgs[x][y][0] > 25 && self.values[x][y][0] > 10)
                    || (self.avgs[x][y][0] < -100 && self.values[x][y][0] > 3)
                    || (self.avgs[x][y][0] < -25 && self.values[x][y][0] > 10)
                {
                    self.gte[x][y][0] = self.values[x][y][0] - 150;
                    self.lte[x][y][0] = self.values[x][y][0] + 150;

                    if self.is_score_relevant {
                        self.gte[x][y][1] = self.values[x][y][1] - 150;
                        self.lte[x][y][2] = self.values[x][y][2] + 150;
                    }
                }
            }
        }
        self
    }
}
