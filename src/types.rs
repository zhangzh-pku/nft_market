use serde::Deserialize;
use serde::Serialize;
use web3::types::H160;
use web3::types::U256;

#[derive(Serialize)]
pub struct NftBalance {
    pub balance: U256,
}

#[derive(Deserialize)]
pub struct MintResponse {
    pub private_key: String,
    pub account_address: String,
    pub amount: u8,
    pub token_uri: String,
}

#[derive(Deserialize)]
pub struct ApproveResponse {
    pub private_key: String,
    pub address_to: String,
    pub token_id: U256,
}

#[derive(Deserialize)]
pub struct TransferFormResponse {
    pub private_key: String,
    pub from: H160,
    pub to: H160,
    pub token_id: U256,
}

#[derive(Deserialize)]
pub struct TransferFormDataResponse {
    pub private_key: String,
    pub from: H160,
    pub to: H160,
    pub token_id: U256,
    pub data: String,
}

#[derive(Deserialize)]
pub struct SetApprovalForAllResponse {
    pub private_key: String,
    pub operator: H160,
    pub approved: bool,
}

#[derive(Deserialize)]
pub struct TransferFromResponse{
    pub private_key: String,
    pub from: H160,
    pub to: H160,
    pub token_id: U256,
}