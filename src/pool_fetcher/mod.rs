use alloy::{primitives::{aliases::U24, Address}, providers::RootProvider};
use crate::executor::QueryTransport;
pub mod v3;

pub trait PoolFetcher: Send {
    fn get_pool_address(&self, provider: RootProvider<QueryTransport>, token_x: Address, token_y: Address, fee: U24) -> Address;
}
