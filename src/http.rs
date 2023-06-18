use crate::config::Config;
use crate::eth::{
    approve, get_balance, mint, safe_transfer_from, safe_transfer_from_with_data,
    set_approval_for_all, transfer_from,
};
use crate::types::{
    ApproveResponse, MintResponse, NftBalance, SetApprovalForAllResponse, TransferFormDataResponse,
    TransferFormResponse, TransferFromResponse,
};
use base64::decode;
use futures::executor::block_on;
use rocket::http::Status;
use rocket::{get, post, routes};
use rocket_contrib::json::Json;
use std::str::FromStr;
use web3::types::{Address, H160};

#[get("/nft_balance?<address>")]
fn nft_balance(address: String) -> Result<Json<NftBalance>, Status> {
    match block_on(get_balance(&address)) {
        Ok(balance) => Ok(Json(balance)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/mint", data = "<data>")]
fn nft_mint(data: Json<MintResponse>) -> Result<Json<String>, Status> {
    let contract_address =
        Config::get_contract_address().map_err(|e| rocket::http::Status::InternalServerError)?;
    let user_address: Address = H160::from_str(&data.account_address)
        .map_err(|e| rocket::http::Status::InternalServerError)?;
    let my_address: Address =
        Config::get_my_account().map_err(|e| rocket::http::Status::InternalServerError)?;

    match block_on(mint(
        contract_address,
        user_address,
        my_address,
        &data.private_key,
        &data.token_uri,
        data.amount,
    )) {
        Ok(receipt) => Ok(Json(receipt.block_hash.unwrap_or_default().to_string())),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/approve", data = "<data>")]
fn nft_approve(data: Json<ApproveResponse>) -> Result<Json<String>, Status> {
    let contract_address =
        Config::get_contract_address().map_err(|e| rocket::http::Status::InternalServerError)?;
    let my_address: Address =
        Config::get_my_account().map_err(|e| rocket::http::Status::InternalServerError)?;
    let address_to: H160 =
        H160::from_str(&data.address_to).map_err(|e| rocket::http::Status::InternalServerError)?;
    match block_on(approve(
        contract_address,
        my_address,
        &data.private_key,
        address_to,
        data.token_id,
    )) {
        Ok(receipt) => Ok(Json(receipt.block_hash.unwrap_or_default().to_string())),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/safe_transfer_from", data = "<data>")]
fn nft_safe_transfer_from(data: Json<TransferFormResponse>) -> Result<Json<String>, Status> {
    let contract_address =
        Config::get_contract_address().map_err(|e| rocket::http::Status::InternalServerError)?;
    let my_address: Address =
        Config::get_my_account().map_err(|e| rocket::http::Status::InternalServerError)?;
    match block_on(safe_transfer_from(
        contract_address,
        my_address,
        &data.private_key,
        data.from,
        data.to,
        data.token_id,
    )) {
        Ok(receipt) => Ok(Json(receipt.block_hash.unwrap_or_default().to_string())),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/safe_transfer_from_data", data = "<data>")]
fn nft_safe_transfer_from_data(
    data: Json<TransferFormDataResponse>,
) -> Result<Json<String>, Status> {
    let contract_address =
        Config::get_contract_address().map_err(|_| Status::InternalServerError)?;
    let my_address: Address = Config::get_my_account().map_err(|_| Status::InternalServerError)?;
    let mut original_data: Option<Vec<u8>> = None;

    if let Ok(decoded_bytes) = decode(data.data.clone()) {
        if let Ok(decoded_data) = String::from_utf8(decoded_bytes) {
            original_data = Some(decoded_data.into_bytes());
            println!("{:?}", original_data);
        } else {
            println!("解码为字符串失败");
        }
    } else {
        println!("解码失败");
    }

    if let Some(_data) = original_data {
        match block_on(safe_transfer_from_with_data(
            contract_address,
            my_address,
            &data.private_key,
            data.from,
            data.to,
            data.token_id,
            _data,
        )) {
            Ok(receipt) => Ok(Json(receipt.block_hash.unwrap_or_default().to_string())),
            Err(_) => Err(Status::InternalServerError),
        }
    } else {
        Err(Status::InternalServerError)
    }
}

#[post("/set_approval_for_all", data = "<data>")]
fn nft_set_approval_for_all(data: Json<SetApprovalForAllResponse>) -> Result<Json<String>, Status> {
    let contract_address =
        Config::get_contract_address().map_err(|_| Status::InternalServerError)?;
    let my_address: Address = Config::get_my_account().map_err(|_| Status::InternalServerError)?;
    match block_on(set_approval_for_all(
        contract_address,
        my_address,
        &data.private_key,
        data.operator,
        data.approved,
    )) {
        Ok(receipt) => Ok(Json(receipt.block_hash.unwrap_or_default().to_string())),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/transfer_from", data = "<data>")]
fn nft_transfer_from(data: Json<TransferFromResponse>) -> Result<Json<String>, Status> {
    let contract_address =
        Config::get_contract_address().map_err(|_| Status::InternalServerError)?;
    let my_address: Address = Config::get_my_account().map_err(|_| Status::InternalServerError)?;
    match block_on(transfer_from(
        contract_address,
        my_address,
        &data.private_key,
        data.from,
        data.to,
        data.token_id,
    )) {
        Ok(receipt) => Ok(Json(receipt.block_hash.unwrap_or_default().to_string())),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub fn run_server() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                nft_balance,
                nft_mint,
                nft_approve,
                nft_safe_transfer_from,
                nft_safe_transfer_from_data,
                nft_set_approval_for_all,
                nft_transfer_from,
            ],
        )
        .launch();
}
