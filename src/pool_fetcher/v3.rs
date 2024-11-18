use alloy::{primitives::{aliases::U24, Address}, providers::RootProvider};
use once_cell::sync::Lazy;
use crate::executor::QueryTransport;
use super::PoolFetcher;
use uniswap_v3_sdk::{entities::Pool, prelude::FeeAmount};
use uniswap_sdk_core::entities::token::Token;
use std::collections::HashMap;

const USDC: Lazy<Address> = Lazy::new(|| "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".parse().unwrap());
const USDT: Lazy<Address> = Lazy::new(|| "0xdAC17F958D2ee523a2206206994597C13D831ec7".parse().unwrap());
const WETH: Lazy<Address> = Lazy::new(|| "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C75677D".parse().unwrap());

pub struct V3PoolFetcher {
    decimals_map: HashMap<Address, u8>,
}

impl V3PoolFetcher {
    pub fn new() -> Self {
        let decimals_map: HashMap<Address, u8> = HashMap::new();
        let mut s = V3PoolFetcher {decimals_map};

        s.default_decimals();

        s
    }

    fn default_decimals(&mut self) {
        self.decimals_map.insert(*USDC, 6);
        self.decimals_map.insert(*USDT, 6);
        self.decimals_map.insert(*WETH, 18);
    }
}


impl PoolFetcher for V3PoolFetcher {
    fn get_pool_address(&self, _: RootProvider<QueryTransport>, token_x: Address, token_y: Address, fee: U24) -> Address {
        let token_x = Token::new(crate::get_chain_id(), token_x, self.decimals_map[&token_x], None, None, None, None);
        let token_y = Token::new(crate::get_chain_id(), token_y, self.decimals_map[&token_y], None, None, None, None);

        let fee_amount = FeeAmount::from(fee.to_string().parse::<u32>().unwrap());

        Pool::get_address(&token_x, &token_y, fee_amount, None, None)
    }
}