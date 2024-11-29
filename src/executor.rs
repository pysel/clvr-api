use std::time::Duration;

use alloy::{providers::{Provider, ProviderBuilder, RootProvider}, rpc::types::TransactionRequest, transports::http::{Client, Http}};
use log::info;
use tokio::time::sleep;
pub type QueryTransport = Http<Client>;

// Executor is responsible for waiting for the scheduled batch to be ready, and then executing the batch
pub struct Executor {
    provider: RootProvider<QueryTransport>,

    block_period: u64,
    last_batch_block: u64,
}

impl Executor {
    pub fn new() -> Self {
        let provider = Self::create_provider();

        Self { provider, block_period: 0, last_batch_block: 0}
    }

    fn create_provider() -> RootProvider<QueryTransport> {
        let rpc_url = std::env::var("ETHEREUM_RPC_URL").expect("ETHEREUM_RPC_URL must be set");
        let rpc_url = rpc_url.parse().expect("ETHEREUM_RPC_URL must be a valid URL");
        let provider = ProviderBuilder::new().on_http(rpc_url);

        provider
    }

}

