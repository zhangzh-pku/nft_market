#![feature(decl_macro)]
mod config;
mod eth;
mod http;
mod types;

fn main() {
    config::Config::from_file("config.json").unwrap();
    http::run_server();
}
