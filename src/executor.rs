use alloy::{
    contract::Error, primitives::{address, Address}, providers::{Provider, ProviderBuilder, RootProvider}, rpc::types::TransactionRequest, transports::http::{Client, Http}
};
use log::{error, info};
use tokio::time::sleep;

use crate::scheduler::hook::ClvrHook::{self, lastBatchBlockReturn, BATCH_PERIODReturn};

pub type HttpTransport = Http<Client>;

// Executor is responsible for waiting for the scheduled batch to be ready, and then executing the batch
pub struct Executor {
    provider: RootProvider<HttpTransport>,
    hook_contract: ClvrHook::ClvrHookInstance<HttpTransport, RootProvider<HttpTransport>>,
    pool_key: String,
}

impl Executor {
    pub async fn new() -> Self {
        let provider = Self::create_provider();

        let hook_address = std::env::var("HOOK_ADDRESS")
            .expect("HOOK_ADDRESS must be set")
            .parse::<Address>()
            .expect("HOOK_ADDRESS must be a valid address");

        let hook_contract = ClvrHook::new(hook_address, provider.clone());

        let pool_key_hex = std::env::var("CLVR_POOL_KEY")
            .expect("CLVR_POOL_KEY must be set");

        Self {
            provider,
            hook_contract,
            pool_key: pool_key_hex,
        }
    }

    pub async fn get_block_number(&self) -> u64 {
        self.provider.get_block_number().await.unwrap()
    }

    pub async fn get_batch_period(&self) -> u64 {
        let result = self.hook_contract.BATCH_PERIOD().call().await;
        Self::safe_unwrap::<BATCH_PERIODReturn>(result)
            ._0
            .try_into()
            .expect("Failed to convert u256 to u64")
    }

    pub async fn get_last_batch_block(&self) -> u64 {
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
    }
}
