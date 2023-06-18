use serde::Serialize;
use web3::types::U256;

#[derive(Serialize)]
pub struct NftBalance {
    pub balance: U256,
}
