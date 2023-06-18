use crate::config;

use web3::types::{Address, H160, U256, TransactionReceipt,H256};
use std::{str::FromStr, time::Duration};
use web3::contract::Options;
use crate::types::NftBalance;


pub async fn get_balance(address: &str) -> Result<NftBalance, String> {
    let infura_apikey = config::Config::get_infura_apikey();
    let contract_address = config::Config::get_contract_address().map_err(|e| e.to_string())?;
    let transport_url = format!("https://goerli.infura.io/v3/{}", infura_apikey);
    let transport = web3::transports::Http::new(&transport_url).map_err(|e| e.to_string())?;
    let web3 = web3::Web3::new(transport);
    let user_address: Address = H160::from_str(address).map_err(|e| e.to_string())?;
    let contract = web3::contract::Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../abi/ERC721.json"),
    ).map_err(|e| e.to_string())?;
    let options = Options::default();
    let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();    
    let balance: U256 = rt.block_on(contract.query("balanceOf", (user_address,), None, options, None)).map_err(|e| e.to_string())?;
    Ok(NftBalance { balance })
}


pub async fn mint(contract_address: H160, user_address: H160, my_account: Address, my_private_key: &str, token_uri: &str, amount: u8) -> Result<TransactionReceipt, String> {
    let infura_apikey = config::Config::get_infura_apikey();
    let transport_url = format!("https://goerli.infura.io/v3/{}", infura_apikey);
    let transport = web3::transports::Http::new(&transport_url).map_err(|e| e.to_string())?;
    let web3 = web3::Web3::new(transport);
    let contract = web3::contract::Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("../abi/ERC721.json"),
    ).map_err(|e| e.to_string())?;

    let options = Options::default();

    let unlock = web3.personal().unlock_account(my_account, my_private_key, None).await.map_err(|e| e.to_string())?;

    if !unlock {
        return Err("Failed to unlock account".into());
    }

    // Minting operation
    let params = (user_address, token_uri.to_owned(), amount);

    // Send the transaction
    let tx_result = contract.call("mint", params, my_account, options).await;
    let tx_hash: H256 = match tx_result {
        Ok(result) => result,
        Err(e) => {
            // 处理错误，可以打印错误信息或者进行其他处理
            eprintln!("Error: {}", e);
            // 在这里返回一个默认值或者使用 panic!() 中止程序
            // 或者选择其他恰当的错误处理方法
            panic!()
        }
    };

    // Now we need to wait for the transaction to be mined
    let receipt: TransactionReceipt = loop {
        match web3.eth().transaction_receipt(tx_hash).await {
            Ok(Some(receipt)) => break Ok(receipt),
            Ok(None) => {
                // Wait for a while before trying again
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Err(e) => break Err(e.to_string()),
        }
    }?;

    Ok(receipt)
}
