use std::time::Duration;

use alloy::{providers::{Provider, ProviderBuilder, RootProvider}, rpc::types::TransactionRequest, transports::http::{Client, Http}};
use log::info;
use tokio::time::sleep;
use crate::server::{handlers::ScheduledDatabase, tokens::{USDC, USDT}, Processor};
use crate::pool_fetcher::PoolFetcher;
pub type QueryTransport = Http<Client>;
use crate::pool_fetcher::v3::V3PoolFetcher;

// Executor is responsible for waiting for the scheduled batch to be ready, and then executing the batch
pub struct Executor {
    provider: RootProvider<QueryTransport>,
    pool_fetcher: Box<dyn PoolFetcher>,

    scheduled_db: ScheduledDatabase,

    block_period: u64,
    last_batch_block: u64,
}

impl Executor {
    pub fn new(scheduled_db: ScheduledDatabase) -> Self {
        let block_period = std::env::var("BATCH_SUBMISSION_PERIOD_BLOCKS")
            .expect("BATCH_SUBMISSION_PERIOD_BLOCKS must be set")
            .parse::<u64>()
            .expect("BATCH_SUBMISSION_PERIOD_BLOCKS must be a valid number");

        let provider = Self::create_provider();

        let pool_fetcher = Box::new(V3PoolFetcher::new());

        Self { provider, pool_fetcher, scheduled_db, block_period, last_batch_block: 0}
    }

    fn create_provider() -> RootProvider<QueryTransport> {
        let rpc_url = std::env::var("ETHEREUM_RPC_URL").expect("ETHEREUM_RPC_URL must be set");
        let rpc_url = rpc_url.parse().expect("ETHEREUM_RPC_URL must be a valid URL");
        let provider = ProviderBuilder::new().on_http(rpc_url);

        provider
    }

    pub async fn run(mut self) {
        loop {
            let current_block = self.provider.get_block_number().await.unwrap();
            if current_block > self.last_batch_block + self.block_period {
                info!("Executing batch at block {}", current_block);

                for trade in self.scheduled_db.lock().unwrap().iter() {
                    let token_in = trade.swap_params.tokenIn;
                    let token_out = trade.swap_params.tokenOut;
                    let fee = trade.swap_params.fee;

                    let pool_address = self.pool_fetcher.get_pool_address(self.provider.clone(), token_in, token_out, fee);
                    info!("Pool address: {}", pool_address);
                }
                self.last_batch_block = current_block;
            }

            // sleep if the current block is not the predecessor of the next batch block
            if current_block - self.last_batch_block <= self.block_period - 2 {
                info!("Current block: {}", current_block);
                sleep(Duration::from_millis(5000)).await;
            }
        }
    }

    // fn send_permit_transaction(&self) {
    //     let usdc = USDT
    // }
}

