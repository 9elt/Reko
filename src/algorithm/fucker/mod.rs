pub mod affinity;
pub mod deviation;
mod indexer;
mod init;
// pub mod stats;
pub mod user;

use std::ops::{Index, IndexMut};

pub type ModelVec = Vec<ModelSlice>;
pub type ModelSlice = Vec<[i32; 9]>;

#[derive(Debug, Clone)]
pub struct Model {
    model: ModelVec,
}

// index and index mut trait implementation
impl Index<usize> for Model {
    type Output = ModelSlice;

    fn index(&self, i: usize) -> &Self::Output {
        &self.model[i]
    }
}

impl IndexMut<usize> for Model {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.model[i]
    }
}

impl Model {
    // contructors
    pub fn empty() -> Self {
        Self {
            model: init::empty(),
        }
    }

    pub fn compare() -> Self {
        Self {
            model: init::compare(),
        }
    }

    pub fn average() -> Self {
        Self {
            model: init::average(),
        }
    }

    pub fn from_vec(model: ModelVec) -> Self {
        Self { model }
    }

    // methods
    pub fn len(&self) -> usize {
        self.model.len()
    }

    pub fn copy_to_vec(&self) -> ModelVec {
        self.model.to_owned()
    }

    pub fn to_vec(self) -> ModelVec {
        self.model
    }
}
