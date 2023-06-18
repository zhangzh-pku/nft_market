use crate::config;

use web3::types::{Address, H160, U256};
use std::str::FromStr;
use web3::contract::Options;
use crate::types::NftBalance;



pub fn get_balance(address: &str) -> Result<NftBalance, String> {
    let infura_apikey = config::Config::get_infura_apikey();
    let contract_address = config::Config::get_contract_address().map_err(|e| e.to_string())?;
    let transport_url = format!("https://goerli.infura.io/v3/{}", infura_apikey);
    let transport = web3::transports::Http::new(&transport_url).map_err(|e| e.to_string())?;
    let web3 = web3::Web3::new(transport);
    let contract_address: Address = H160::from_str(&contract_address.to_string()).map_err(|e| e.to_string())?;
    let user_address: Address = H160::from_str(address).map_err(|e| e.to_string())?;

    let contract = web3::contract::Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../abi/ERC721.json"),
    ).map_err(|e| e.to_string())?;

    let options = Options::default();
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    let balance: U256 = rt.block_on(contract.query("balanceOf", (user_address,), None, options, None)).map_err(|e| e.to_string())?;

    Ok(NftBalance { balance })
}
