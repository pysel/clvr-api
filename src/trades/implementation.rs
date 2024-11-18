use crate::trades::{ITrade, TradeDirection};
use alloy::primitives::U256;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Deserialize, Serialize)]
pub struct Trade {
    amount_in: U256,
    direction: TradeDirection,
}

impl Trade {
    pub fn new(amount_in: U256, direction: TradeDirection) -> Self {
        Trade {
            amount_in,
            direction,
        }
    }
}

impl ITrade for Trade {
    fn get_direction(&self) -> TradeDirection {
        self.direction.clone()
    }

    fn get_amount_in(&self) -> U256 {
        self.amount_in
    }
}
