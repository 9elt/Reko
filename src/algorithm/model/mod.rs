pub mod helper;

use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};
use std::ops::{Index, IndexMut};
use std::slice::Iter;

use crate::utils::conversion;

pub type ModelVec<T> = Vec<ModelStatType<T>>;
pub type ModelStatType<T> = Vec<ModelStat<T>>;
pub type ModelStat<T> = [T; 9];
pub type PartialModel<'a, T> = Vec<&'a ModelStatType<T>>;

#[derive(Debug, Clone, Deserialize)]
pub struct Model<T> {
    model: ModelVec<T>,
}

/// ## Model indexer
/// Provides useful methods to index `Model`
pub struct Indexer {
    pub x: usize,
    pub y: usize,
    errors: bool,
}


////////////////////////////////////////////////////////////////////////////////
// Serialize implementation
////////////////////////////////////////////////////////////////////////////////

impl<T> Serialize for Model<T>
where
    T: Serialize
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.model.len()))?;
        for element in self.model.iter() {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Index implementation
////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////
// General methods
////////////////////////////////////////////////////////////////////////////////

impl<T> Model<T>
where
    i16: TryFrom<T>,
    T: Copy,
{
    pub fn len(&self) -> usize {
        self.model.len()
    }

    pub fn details(&self) -> PartialModel<T> {
        vec![
            &self.model[1],
            &self.model[2],
            &self.model[3],
            &self.model[4],
            &self.model[5],
            &self.model[6],
            &self.model[7],
            &self.model[8],
        ]
    }

    pub fn _iter(&self) -> Iter<'_, ModelStatType<T>>{
        self.model.iter()
    }

    pub fn to_i16(self) -> Model<i16> {
        self.copy_to_i16()
    }

    pub fn copy_to_i16(&self) -> Model<i16> {
        let mut model_i16 = Model::<i16>::empty();

        for x in 0..self.model.len() {
            for y in 0..self.model[x].len() {
                for z in 0..self.model[x][y].len() {
                    match i16::try_from(self.model[x][y][z]) {
                        Ok(value) => model_i16[x][y][z] = value,
                        Err(_) => model_i16[x][y][z] = i16::MAX,
                    }
                }
            }
        }

        model_i16
    }
}

impl Model<i64> {
    pub fn empty() -> Self {
        Self {
            model: model_vec_from_value(0 as i64),
        }
    }
}

impl Model<i32> {
    pub fn empty() -> Self {
        Self {
            model: model_vec_from_value(0 as i32),
        }
    }
}

impl Model<i16> {
    pub fn empty() -> Self {
        Self {
            model: model_vec_from_value(0 as i16),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// i16 specific methods
////////////////////////////////////////////////////////////////////////////////

impl Model<i16> {
    pub fn compare() -> Self {
        Self {
            model: model_vec_from_value(4095 as i16),
        }
    }

    pub fn from_vec(model: ModelVec<i16>) -> Self {
        Self { model }
    }

    pub fn from_json(value: serde_json::Value) -> Self {
        Self {
            model: conversion::from_json(value)
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// Model Vector Generator
////////////////////////////////////////////////////////////////////////////////

fn model_vec_from_value<T: Copy>(v: T) -> ModelVec<T> {
    vec![
        // general
        vec![[v; 9]],
        // airing decades
        vec![[v; 9]; 5],
        // ratings
        vec![[v; 9]; 6],
        // series lengths
        vec![[v; 9]; 5],
        // major genres
        vec![[v; 9]; 8],
        // minor genres
        vec![[v; 9]; 13],
        // major themes
        vec![[v; 9]; 20],
        // minor themes
        vec![[v; 9]; 30],
        // demographics
        vec![[v; 9]; 5],
    ]
}