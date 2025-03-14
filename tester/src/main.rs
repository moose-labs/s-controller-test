use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, signature::read_keypair_file, signer::Signer,
};
use utils::paths::get_deps_configs;

mod helper;
mod utils;

fn main() {
    println!("Hello, world!");

    let user_keypair = read_keypair_file(get_deps_configs("local-user.json")).unwrap();
    let url = "http://localhost:8899".to_string();
    let client = RpcClient::new_with_commitment(url, CommitmentConfig::confirmed());

    let balance = client.get_balance(&user_keypair.pubkey());

    println!("balance: {} {}", user_keypair.pubkey(), balance.unwrap())
}
