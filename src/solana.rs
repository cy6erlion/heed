use crate::{DecentEntity, DecentNetState, Heeder};
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
impl Heeder for SolanaHeed {
    /// heed solana
    fn heed(&self, to_heed: Vec<String>) -> Vec<DecentEntity> {
        let rpc_client = RpcClient::new(String::from(&self.rpc_url));
        let latest_block = rpc_client.get_latest_blockhash().unwrap();
        let pubkeys: Vec<Pubkey> = to_heed
            .iter()
            .map(|l| Pubkey::from_str(&l).unwrap())
            .collect();
        let accounts = rpc_client.get_multiple_accounts(&pubkeys).unwrap();
        let mut entities: Vec<DecentEntity> = vec![];

        for i in 0..accounts.len() {
            match &accounts[i] {
                Some(a) => {
                    // convert solana Account to a DecentEntity
                    let mut entity: DecentEntity = a.clone().into();
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
