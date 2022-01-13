//! Pay attention to decentralized entities.
/// Solana heed
use async_trait::async_trait;
pub mod solana;
/// Decentralized networks
#[derive(Debug, PartialEq)]
pub enum DecentNet {
    Solana,
}
/// Identifies the state of a decentralized network
#[derive(Debug)]
pub struct DecentNetState {
    id: Option<String>,
}
#[async_trait]
/// For implementing heeding on a network
pub trait Heeder {
    /// network specific, heed method.
    fn heed(&self, to_heed: Vec<String>) -> Vec<DecentEntity>;
    async fn heed(&self, to_heed: Vec<String>) -> Vec<DecentEntity<A>>;
}
/// A decentralized entity
pub struct DecentEntity {
    /// The location of the entity inside the network
    pub location: String,
    /// Amount stored, if it is a cryptocurrency account
    pub amount: Option<u64>,
    /// The decentralized network the entity is located in
    pub network: DecentNet,
    /// Identifier of the network state when Entity was fetched
    pub net_state: DecentNetState,
}
/// Convert Solana Account to Entity
impl From<solana_sdk::account::Account> for DecentEntity {
    fn from(account: solana_sdk::account::Account) -> Self {
        DecentEntity {
            // Because solana_sdk::account::Account does not have the
            // location (address) of the account, we initialize it
            // an empty String, it is up to the caller of the .into()
            // the update this address after calling into()
            location: String::from(""),
            amount: Some(account.lamports),
            network: DecentNet::Solana,
            net_state: DecentNetState { id: None },
        }
    }
}
