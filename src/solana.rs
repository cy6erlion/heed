//! Solana heed

use crate::{DecentEntity, DecentNetState, Heeder};
use async_trait::async_trait;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

/// Represents the state of the Solana blockchain
pub struct SolanaState;

/// Solana blockchain heeder
pub struct SolanaHeed {
    rpc_url: String,
}

impl Default for SolanaHeed {
    fn default() -> Self {
        SolanaHeed {
            rpc_url: "https://api.devnet.solana.com".to_string(),
        }
    }
}

#[async_trait]
impl Heeder<u64> for SolanaHeed {
    /// heed solana
    async fn heed(&self, to_heed: Vec<String>) -> Vec<DecentEntity<u64>> {
        let rpc_client = RpcClient::new(String::from(&self.rpc_url));
        let latest_block = rpc_client.get_latest_blockhash().unwrap();
        let pubkeys: Vec<Pubkey> = to_heed
            .iter()
            .map(|l| Pubkey::from_str(&l).unwrap())
            .collect();
        let accounts = rpc_client.get_multiple_accounts(&pubkeys).unwrap();
        let mut entities: Vec<DecentEntity<u64>> = vec![];

        for i in 0..accounts.len() {
            match &accounts[i] {
                Some(a) => {
                    // convert solana Account to a DecentEntity
                    let mut entity: DecentEntity<u64> = a.clone().into();
                    entity.location = to_heed[i].clone();
                    // set network state
                    entity.net_state = DecentNetState {
                        id: Some(latest_block.to_string()),
                    };
                    entities.push(entity);
                }
                None => (),
            }
        }

        entities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DecentNet;

    #[test]
    fn test_solana_heed() {
        let solana_heeder: SolanaHeed = Default::default();
        let to_heed = vec![String::from("F1xq9yBeB8otmiWhza9rmrnjTVpsZNuqS2GvAS4QUiwB")];
        let entities = solana_heeder.heed(to_heed);

        assert_eq!(entities[0].network, DecentNet::Solana);
        assert_eq!(
            entities[0].location,
            "F1xq9yBeB8otmiWhza9rmrnjTVpsZNuqS2GvAS4QUiwB".to_string()
        );

        match entities[0].amount {
            Some(amount) => assert!(true),
            None => panic!("Solana amount cannot be None"),
        }

        match &entities[0].net_state.id {
            Some(id) => {
                println!("{}", id);
                // Base64 ecoded SHA256 (6 bits per char)
                assert_eq!(id.len(), 44)
            }
            None => panic!("Solana entity network state (lates block) cannot be None"),
        }
    }
}
