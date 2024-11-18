#![allow(non_snake_case)]

use crate::trades::ITrade;
use alloy::primitives::U256;
use std::fmt::Debug;
use std::{
    fmt::{self, Formatter},
    ops::Index,
};

pub mod clvr_model;

// Notation for a particular trades ordering.
// NOTE: Omega is 1-indexed
pub struct Omega(Vec<Box<dyn ITrade>>);

impl Omega {
    pub fn new() -> Self {
        Omega(Vec::new())
    }

    #[cfg(test)]
    pub fn new_from(vec: Vec<Box<dyn ITrade>>) -> Self {
        Omega(vec)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn swap(&mut self, index1: usize, index2: usize) {
        self.0.swap(index1 - 1, index2 - 1); // 1-indexed
    }

    pub fn push(&mut self, trade: Box<dyn ITrade>) {
        self.0.push(trade);
    }
}

impl Index<usize> for Omega {
    type Output = Box<dyn ITrade>;

    // 1-indexed
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i - 1]
    }
}

impl Debug for Omega {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 1..self.len() + 1 {
            write!(
                f,
                "{:?} {:?}, \n",
                self[i].get_direction(),
                self[i].get_amount_in()
            )?;
        }

        Ok(())
    }
}

impl PartialEq for Omega {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for i in 1..self.len() + 1 {
            if self[i].get_amount_in() != other[i].get_amount_in()
                || self[i].get_direction() != other[i].get_direction()
            {
                return false;
            }
        }

        return true;
    }
}

pub trait Model {
    fn y_out(&self, o: &Omega, i: usize) -> U256;
    fn x_out(&self, o: &Omega, i: usize) -> U256;

    fn Y(&self, o: &Omega, i: usize) -> U256;
    fn X(&self, o: &Omega, i: usize) -> U256;

    fn P(&self, o: &Omega, i: usize) -> U256;
}
