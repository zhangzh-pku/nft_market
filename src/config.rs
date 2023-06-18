use serde::{Deserialize, Serialize};
use web3::types::{Address, H160};
use std::default::Default;
use std::fs::File;
use std::io::Read;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::str::FromStr;

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::default());
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub infura_apikey: String,
    pub contract_address: String,
    pub account_address: String,
    pub private_key: String
}

impl Default for Config {
    fn default() -> Self {
        Config {
            infura_apikey: String::from("infura_apikey"),
            contract_address: String::from("contract_address"),
            account_address: String::from("account_address"),
            private_key: String::from("private_key"),
        }
    }
}

impl Config {
    pub fn from_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_json::from_str(&contents)?;

        let mut config_lock = CONFIG.lock().unwrap();
        *config_lock = config;

        Ok(())
    }

    pub fn get_infura_apikey() -> String {
        let config_lock = CONFIG.lock().unwrap();
        config_lock.infura_apikey.clone()
    }

    pub fn get_contract_address() -> Result<Address, Box<dyn std::error::Error>> {
        let config_lock = CONFIG.lock().unwrap();
        let address = H160::from_str(&config_lock.contract_address)?;
        Ok(address.into())
    }
    

    pub fn get_instance() -> &'static Mutex<Config> {
        &CONFIG
    }

    pub fn get_my_account() -> Result<Address, Box<dyn std::error::Error>>{
        let config_lock = CONFIG.lock().unwrap();
        let address = H160::from_str(&config_lock.account_address.to_string())?;
        Ok(address.into())
    }

    pub fn get_my_private_key() -> String{
        let config_lock = CONFIG.lock().unwrap();
        // private_key = SecretKey::from_str(&config_lock.private_key)?;
        config_lock.private_key.clone()
    }
}
