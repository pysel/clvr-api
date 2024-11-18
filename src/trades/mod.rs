use alloy::primitives::U256;
use serde::{Serialize, Deserialize};

pub mod implementation;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum TradeDirection {
    Buy,
    Sell,
}

pub trait ITrade {
    fn get_direction(&self) -> TradeDirection;
    fn get_amount_in(&self) -> U256; // INVARIANT: when direction == Buy, amount_in is in tokens y, when direction == Sell, amount_in is in tokens x
}
