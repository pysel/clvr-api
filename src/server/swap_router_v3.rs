use std::str::FromStr;

use alloy::{primitives::{aliases::U24, Address, U160, U256}, sol};
use serde::{Deserialize, Serialize};
use ISwapRouter::ExactInputSingleParams;

sol!(
    #[sol(rpc)]
    SwapRouterV3,
    "abis/SwapRouterV3.json",
);

/*
ExactInputSingleParamsIntermediate is used to convert between the API types and the internal type:

pub struct ExactInputSingleParams {
    pub tokenIn: Address,
    pub tokenOut: Address,
    pub fee: Uint<24, 1>,
    pub recipient: Address,
    pub deadline: Uint<256, 4>,
    /* … */
}
 */
#[derive(Serialize, Deserialize)]
pub struct ExactInputSingleParamsIntermediate {
    pub token_in: String,
    pub token_out: String,
    pub fee: U24,
    pub recipient: String,
    pub deadline: U256,
    pub amount_in: U256,
    pub amount_out_minimum: U256,
    pub sqrt_price_limit_x96: U160,
}

impl From<ExactInputSingleParamsIntermediate> for ExactInputSingleParams {
    fn from(params: ExactInputSingleParamsIntermediate) -> Self {
        ExactInputSingleParams {
            tokenIn: Address::from_str(&params.token_in).unwrap(), 
            tokenOut: Address::from_str(&params.token_out).unwrap(), 
            fee: params.fee, 
            recipient: Address::from_str(&params.recipient).unwrap(), 
            deadline: params.deadline,
            amountIn: params.amount_in,
            amountOutMinimum: params.amount_out_minimum,
            sqrtPriceLimitX96: params.sqrt_price_limit_x96,
        }
    }
}

use std::fmt::{Debug, Error, Formatter};
impl Debug for ExactInputSingleParams {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "ExactInputSingleParams {{ tokenIn: {:?}, tokenOut: {:?}, fee: {:?}, recipient: {:?}, deadline: {:?}, amountIn: {:?}, amountOutMinimum: {:?}, sqrtPriceLimitX96: {:?} }}", 
        self.tokenIn, self.tokenOut, self.fee, self.recipient, self.deadline, self.amountIn, self.amountOutMinimum, self.sqrtPriceLimitX96)
    }
}
