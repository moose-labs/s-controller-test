use helper::instructions::CreateMint;
use moose_utils::result::Result;
use s_controller_client::client::SControllerClient;
use solana_sdk::{commitment_config::CommitmentConfig, signature::read_keypair_file};
use utils::paths::get_deps_configs;

mod helper;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let user_keypair = read_keypair_file(get_deps_configs("local-user.json")).unwrap();
    let url = "http://localhost:8899";

    let initial_authority_keypair = read_keypair_file(get_deps_configs(
        "s-controller-test-initial-authority-key.json",
    ))
    .unwrap();

    let s_controller_client =
        SControllerClient::new(user_keypair, url.to_string(), CommitmentConfig::processed());

    let lp_token_mint = s_controller_client
        .create_mint(&s_controller_client.get_initial_authority_pubkey(), 9)
        .await?;

    let ix = s_controller_client
        .get_initialize_ix(&lp_token_mint)
        .await?;

    let s = s_controller_client
        .process_instruction(ix, &vec![&initial_authority_keypair])
        .await?;

    println!("sig {}", s.to_string());

    Ok(())
}
