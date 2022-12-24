use super::{Model, ModelVec};

pub struct UserModel {
    pub stats: Model,
    pub deviation: Model,
}

impl UserModel {
    pub fn from(stats: Model, deviation: Model) -> Self {
        Self { stats, deviation }
    }

    pub fn to_array(self) -> [ModelVec; 2] {
        [self.stats.to_vec(), self.deviation.to_vec()]
    }
}
