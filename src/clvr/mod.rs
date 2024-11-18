mod algorithm;
pub mod model;

#[cfg(test)]
mod algorithm_tests;

use alloy::sol;

// Uniswap V4 SwapParams
sol! {
    struct SwapParams {
        bool zeroForOne;
        int256 amountSpecified;
        uint160 sqrtPriceLimitX96;
    }
}
