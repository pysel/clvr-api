use alloy::{
    contract::Error, dyn_abi::SolType, hex, primitives::{address, Address, U256}, providers::{Provider, ProviderBuilder, RootProvider}, rpc::types::TransactionRequest, transports::http::{Client, Http}
};
use log::{error, info};
use std::time::Duration;
use tokio::time::sleep;

use crate::{clvr::model::{clvr_model::CLVRModel, Omega}, scheduler::hook::ClvrHook::{self, getCurrentPriceReturn, getCurrentReservesReturn, lastBatchBlockReturn, BATCH_PERIODReturn, PoolKey}, trades::implementation::Trade};

pub type HttpTransport = Http<Client>;

// Executor is responsible for waiting for the scheduled batch to be ready, and then executing the batch
pub struct Executor {
    provider: RootProvider<HttpTransport>,
    hook_contract: ClvrHook::ClvrHookInstance<HttpTransport, RootProvider<HttpTransport>>,
    pool_key: PoolKey,
}

impl Executor {
    pub async fn new() -> Self {
        let provider = Self::create_provider();

        let hook_address = std::env::var("HOOK_ADDRESS")
            .expect("HOOK_ADDRESS must be set")
            .parse::<Address>()
            .expect("HOOK_ADDRESS must be a valid address");

        let hook_contract = ClvrHook::new(hook_address, provider.clone());

        let pool_key_hex = std::env::var("CLVR_POOL_KEY").expect("CLVR_POOL_KEY must be set");
        let pool_key_decoded = hex::decode(pool_key_hex).unwrap();
        let pool_key = PoolKey::abi_decode(&pool_key_decoded, true).unwrap();

        Self {
            provider,
            hook_contract,
            pool_key: pool_key,
        }
    }

    pub async fn run(&self) {
        loop {
            if !self.is_batch_block().await {
                sleep(Duration::from_secs(1)).await;
                continue;
            }

            let scheduled_swaps = self.hook_contract.getScheduledSwaps(self.pool_key.clone()).call().await;
            
            let mut omega = Omega::new();

            if let Ok(swaps) = scheduled_swaps {
                if swaps._0.len() == 0 {
                    info!("No swaps to execute");
                    sleep(Duration::from_secs(1)).await;
                    continue;
                }

                for swap in swaps._0 {
                    let trade = Trade::from(swap);
                    omega.push(Box::new(trade));
                }
            }

            let (reserve0, reserve1) = self.get_current_reserves().await;
            let current_price = self.get_current_price().await;
            let clvr_model = CLVRModel::new(reserve0, reserve1);

            clvr_model.clvr_order(current_price, &mut omega);

            info!("Omega: {:?}", omega);
        }
    }

    async fn get_current_price(&self) -> U256 {
        let result = self.hook_contract.getCurrentPrice(self.pool_key.clone()).call().await;
        Self::safe_unwrap::<getCurrentPriceReturn>(result)._0
    }

    async fn get_current_reserves(&self) -> (U256, U256) {
        let result = self.hook_contract.getCurrentReserves(self.pool_key.clone()).call().await;
        let (reserve0, reserve1) = {
            let reserves = Self::safe_unwrap::<getCurrentReservesReturn>(result);
            (reserves._0, reserves._1)
        };

        (reserve0, reserve1)
    }

    async fn is_batch_block(&self) -> bool {
        let block_number = self.get_block_number().await;
        let last_batch_block = self.get_last_batch_block().await;
        let batch_period = self.get_batch_period().await;

        block_number >= last_batch_block + batch_period
    }

    async fn get_block_number(&self) -> u64 {
        self.provider.get_block_number().await.unwrap()
    }

    async fn get_batch_period(&self) -> u64 {
        let result = self.hook_contract.BATCH_PERIOD().call().await;
        Self::safe_unwrap::<BATCH_PERIODReturn>(result)
            ._0
            .try_into()
            .expect("Failed to convert u256 to u64")
    }

    async fn get_last_batch_block(&self) -> u64 {
        let result = self.hook_contract.lastBatchBlock().call().await;
        Self::safe_unwrap::<lastBatchBlockReturn>(result)
            ._0
            .try_into()
            .expect("Failed to convert u256 to u64")
    }

    fn safe_unwrap<T>(value: Result<T, Error>) -> T {
        match value {
            Ok(value) => value,
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    fn create_provider() -> RootProvider<HttpTransport> {
        let rpc_url = std::env::var("ETHEREUM_RPC_URL").expect("ETHEREUM_RPC_URL must be set");
        let rpc_url = rpc_url
            .parse()
            .expect("ETHEREUM_RPC_URL must be a valid URL");
        let provider = ProviderBuilder::new().on_http(rpc_url);

        provider
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_executor() {
        dotenv::dotenv().ok();

        let executor = Executor::new().await;

        let block_number = executor.get_block_number().await;
        println!("Block number: {}", block_number);

        let batch_period = executor.get_batch_period().await;
        println!("Batch period: {}", batch_period);

        let last_batch_block = executor.get_last_batch_block().await;
        println!("Last batch block: {}", last_batch_block);

        println!("Pool key: {:?}", executor.pool_key.tickSpacing);

        executor.run().await;
    }
}
