
use alloy::{primitives::{aliases::U24, Address, U160, U256}, sol};
use serde::{Deserialize, Serialize};

sol!(
    #[sol(rpc)]
    USDC,
    "abis/tokens/USDC.json",
);

sol!(
    #[sol(rpc)]
    USDT,
    "abis/tokens/USDT.json",
);
