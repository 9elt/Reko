pub mod helper;

use helper::model_vec_from_value;
use serde::{Serialize, Deserialize};
use std::ops::{Index, IndexMut};

pub type ModelVec<T> = Vec<ModelStatType<T>>;
pub type ModelStatType<T> = Vec<ModelStat<T>>;
pub type ModelStat<T> = [T; 9];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model<T> {
    model: ModelVec<T>,
}

// index and index mut trait implementation
impl<T> Index<usize> for Model<T> {
    type Output = ModelStatType<T>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.model[i]
    }
}

impl<T> IndexMut<usize> for Model<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.model[i]
    }
}

impl Model<i64> {
    // contructors
    pub fn empty() -> Self {
        Self {
            model: model_vec_from_value(0 as i64),
        }
    }

    // methods
    pub fn to_i16(self) -> Model<i16> {
        let mut model_conversion = Model::<i16>::empty();

        for x in 0..self.model.len() {
            for y in 0..self.model[x].len() {
                for z in 0..self.model[x][y].len() {
                    let conversion = i16::try_from(self.model[x][y][z]);
                    match conversion {
                        Ok(converted) => model_conversion[x][y][z] = converted,
                        Err(_) => model_conversion[x][y][z] = 32_767,
                    }
                }
            }
        }

        model_conversion
    }

    pub fn len(&self) -> usize {
        self.model.len()
    }

    pub fn copy_to_i16_vec(&self) -> ModelVec<i16> {
        self.to_owned().to_i16().to_vec()
    }

    pub fn to_i16_vec(self) -> ModelVec<i16> {
        self.to_i16().to_vec()
    }
}

impl Model<i32> {
    // contructors
    pub fn empty() -> Self {
        Self {
            model: model_vec_from_value(0 as i32),
        }
    }

    // methods
    pub fn to_i16(self) -> Model<i16> {
        let mut model_conversion = Model::<i16>::empty();

        for x in 0..self.model.len() {
            for y in 0..self.model[x].len() {
                for z in 0..self.model[x][y].len() {
                    let conversion = i16::try_from(self.model[x][y][z]);
                    match conversion {
                        Ok(converted) => model_conversion[x][y][z] = converted,
                        Err(_) => model_conversion[x][y][z] = 32_767,
                    }
                }
            }
        }

        model_conversion
    }

    pub fn len(&self) -> usize {
        self.model.len()
    }

    pub fn copy_to_i16_vec(&self) -> ModelVec<i16> {
        self.to_owned().to_i16().to_vec()
    }

    pub fn to_i16_vec(self) -> ModelVec<i16> {
        self.to_i16().to_vec()
    }
}

impl Model<i16> {
    // contructors
    pub fn empty() -> Self {
        Self {
            model: model_vec_from_value(0 as i16),
        }
    }

    pub fn compare() -> Self {
        Self {
            model: model_vec_from_value(4095 as i16),
        }
    }

    pub fn from_vec(model: ModelVec<i16>) -> Self {
        Self { model }
    }

    // methods
    pub fn len(&self) -> usize {
        self.model.len()
    }

    pub fn copy_to_vec(&self) -> ModelVec<i16> {
        self.model.to_owned()
    }

    pub fn to_vec(self) -> ModelVec<i16> {
        self.model
    }
}
