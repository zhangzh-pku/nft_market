use rocket::{get, post, routes};
use rocket::http::Status;
use rocket_contrib::json::Json;
use crate::types::NftBalance;
use crate::eth::{get_balance, mint};
use web3::types::{Address, H160};
use crate::config::Config;
use std::str::FromStr;
use futures::executor::block_on;
use serde::Deserialize;

#[derive(Deserialize)]
struct MintResponse{
    private_key: String,
    account_address: String,
    amount: u8,
    token_uri: String,
}


#[get("/nft_balance?<address>")]
fn nft_balance(address: String) -> Result<Json<NftBalance>, Status> {
    match block_on(get_balance(&address)) {
        Ok(balance) => Ok(Json(balance)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/mint", data="<data>")]
fn nft_mint(data: Json<MintResponse>) -> Result<Json<String>, Status>  {
    let contract_address = Config::get_contract_address().map_err(|e| rocket::http::Status::InternalServerError)?;
    let user_address: Address = H160::from_str(&data.account_address).map_err(|e| rocket::http::Status::InternalServerError)?;
    let my_address: Address = Config::get_my_account().map_err(|e|rocket::http::Status::InternalServerError)?;
    
    match block_on(mint(contract_address, user_address, my_address, &data.private_key, &data.token_uri, data.amount)){
        Ok(receipt) => Ok(Json(receipt.block_hash.unwrap_or_default().to_string())),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn run_server() {
    rocket::ignite().mount("/", routes![nft_balance, nft_mint]).launch();
}
