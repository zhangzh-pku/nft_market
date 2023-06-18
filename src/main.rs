#![feature(decl_macro)]
mod http;
mod eth;
mod types;
mod config;


fn main() {
    config::Config::from_file("config.json").unwrap();
    http::run_server();
}
