//! Ethereum heed

use crate::{DecentEntity, DecentNet, DecentNetState, Heeder};
use async_trait::async_trait;
use std::str::FromStr;
use web3::api::Web3;
use web3::transports::http::Http;
use web3::types::{Address, BlockNumber, U256};

/// Represents the state of the Ethereum blockchain
pub struct EthereumState;

/// Ethereum blockchain heeder
pub struct EthereumHeed {
    rpc_url: String,
}

impl Default for EthereumHeed {
    fn default() -> Self {
        EthereumHeed {
            rpc_url: "https://ropsten.infura.io/v3/0dc278ed5f3e4812b08f4b348aa6ab95".to_string(),
        }
    }
}

#[async_trait]
impl Heeder<U256> for EthereumHeed {
    /// heed ethereum
    async fn heed(&self, to_heed: Vec<String>) -> Vec<DecentEntity<U256>> {
        let transport = Http::new(&self.rpc_url).unwrap();
        let rpc_client = Web3::new(transport);
        let block_number = rpc_client.eth().block_number().await.unwrap();
        let mut entities: Vec<DecentEntity<U256>> = vec![];

        for i in 0..to_heed.len() {
            let address = Address::from_str(&to_heed[i]).unwrap();

            let amount = rpc_client
                .eth()
                .balance(address, Some(BlockNumber::Number(block_number)))
                .await
                .unwrap();
            let entity: DecentEntity<U256> = DecentEntity {
                location: to_heed[i].clone(),
                amount: Some(amount),
                network: DecentNet::Ethereum,
                net_state: DecentNetState {
                    id: Some(format!("{}", block_number)),
                },
            };

            entities.push(entity)
        }

        entities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DecentNet;

    #[test]
    fn test_ethereum_heed() {}
}
