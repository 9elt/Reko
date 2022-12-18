use super::init;
use std::ops::{Index, IndexMut};

pub type Model = Vec<ModelSlice>;
pub type ModelSlice = Vec<[i32; 9]>;

#[derive(Debug, Clone)]
pub struct UserModel {
    model: Model,
}

// implementing index trait
impl Index<usize> for UserModel {
    type Output = ModelSlice;

    fn index(&self, i: usize) -> &Self::Output {
        &self.model[i]
    }
}

impl IndexMut<usize> for UserModel {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.model[i]
    }
}

impl UserModel {
    pub fn empty() -> Self {
        Self {
            model: init::empty(),
        }
    }

    pub fn average() -> Self {
        Self {
            model: init::average(),
        }
    }

    pub fn from(model: Model) -> Self {
        Self { model }
    }

    pub fn len(&self) -> usize {
        self.model.len()
    }

    pub fn copy_to_vec(&self) -> Model {
        self.model.to_owned()
    }

    pub fn to_vec(self) -> Model {
        self.model
    }
}
