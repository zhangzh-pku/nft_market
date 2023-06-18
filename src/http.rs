use rocket::{get, routes};
use rocket_contrib::json::Json;
use crate::types::NftBalance;
use crate::eth::get_balance;

#[get("/nft_balance?<address>")]
fn nft_balance(address: String) -> Option<Json<NftBalance>> {
    let balance = get_balance(&address).ok()?;
    Some(Json(balance))
}

pub fn run_server() {
    rocket::ignite().mount("/", routes![nft_balance]).launch();
}
