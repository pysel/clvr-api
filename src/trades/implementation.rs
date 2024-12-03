use crate::{scheduler::hook::ClvrHook::SwapParamsExtended, trades::{ITrade, TradeDirection}};
use alloy::primitives::U256;
use serde::{Deserialize, Serialize};
use crate::scheduler::hook::IPoolManager::SwapParams;

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

impl From<SwapParamsExtended> for Trade {
    fn from(swap: SwapParamsExtended) -> Self {
        let swap_params = swap.params;

        Trade::new(
            U256::try_from(-swap_params.amountSpecified).unwrap(), 
            match swap_params.zeroForOne {
                true => TradeDirection::Sell,
                false => TradeDirection::Buy,
            }
        )
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

#[cfg(test)]
mod tests {
    use alloy::primitives::{address, Address, I256, U160};

    use super::*;

    #[test]
    fn test_from_swap_params_extended() {
        let amount = U256::from(100e18);
        let amount_signed = -I256::try_from(amount).unwrap();

        let address = Address::parse_checksummed("0x8C4BcBE6b9eF47855f97E675296FA3F6fafa5F1A", None).unwrap();

        let swap = SwapParamsExtended{
            params: SwapParams{
                zeroForOne: true, 
                amountSpecified: amount_signed, 
                sqrtPriceLimitX96: U160::ZERO,
            }, 
            recepient: address,
            sender: address
        };
        let trade = Trade::from(swap);

        assert_eq!(trade.get_amount_in(), amount);
        assert_eq!(trade.get_direction(), TradeDirection::Sell);
    }
}
