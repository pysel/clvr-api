use std::str::FromStr;

use alloy::{hex, primitives::{Address, PrimitiveSignature}};
use serde::{Deserialize, Serialize};
use crate::server::swap_router_v3::ISwapRouter::ExactInputSingleParams;

use super::swap_router_v3::ExactInputSingleParamsIntermediate;

// API Types
#[derive(Serialize, Deserialize)]
pub struct ScheduleRequest {
    pub from: String,
    /*
    swap_params
    struct ExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        uint24 fee;
        address recipient;
        uint256 deadline;
        uint256 amountIn;
        uint256 amountOutMinimum;
        uint160 sqrtPriceLimitX96;
    }

    encoded as a json string
     */
    pub swap_params: ExactInputSingleParamsIntermediate, 
    pub permit_msg: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize)]
pub struct ScheduleResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct NumTradesResponse {
    pub num_trades: u64,
}

// Internal Types
#[derive(Clone)]
pub struct ScheduledTrade {
    pub from: Address,
    pub swap_params: ExactInputSingleParams,
    pub permit_msg: Vec<u8>,
    pub signature: PrimitiveSignature,
}

impl From<ScheduleRequest> for ScheduledTrade {
    fn from(request: ScheduleRequest) -> Self {
        let from_address = Address::from_str(&request.from).unwrap();
        let swap_params: ExactInputSingleParams = request.swap_params.into();
        let permit_msg: Vec<u8> = hex::decode(request.permit_msg).unwrap();
        let signature: PrimitiveSignature = PrimitiveSignature::from_str(&request.signature).unwrap();
        ScheduledTrade { from: from_address, swap_params, permit_msg, signature }
    }
}

impl ToString for ScheduledTrade {
    fn to_string(&self) -> String {
        format!("Scheduled trade from: {}, swap_params: {:?}, permit_msg: {:?}, signature: {:?}", self.from, self.swap_params, self.permit_msg, self.signature)
    }
}