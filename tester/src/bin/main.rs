use moose_utils::result::Result;
use s_controller_client::client::SControllerClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, signature::read_keypair_file, signer::Signer,
};
use tester::{
    helper::instructions::s_controller::{CreateMint, SController},
    utils::paths::get_deps_configs,
};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let payer = read_keypair_file(get_deps_configs("user1.json")).unwrap();
    let admin = read_keypair_file(get_deps_configs("admin.json")).unwrap();
    let url = "http://localhost:8899";

    let initial_authority_keypair = read_keypair_file(get_deps_configs(
        "s-controller-test-initial-authority-key.json",
    ))
    .unwrap();

    let s_controller_client =
        SControllerClient::new(payer, url.to_string(), CommitmentConfig::processed());

    let lp_token_mint = s_controller_client
        .create_mint(&s_controller_client.get_initial_authority_pubkey(), 9)
        .await?;

    s_controller_client
        .initialize_s_controller_if_possible(&lp_token_mint, &initial_authority_keypair)
        .await?;

    s_controller_client
        .set_admin_if_not_match(&admin.pubkey(), &initial_authority_keypair)
        .await?;

    s_controller_client.disable_pool_if_possible(&admin).await?;
    s_controller_client.disable_pool_if_possible(&admin).await?;
    s_controller_client.enable_pool_if_possible(&admin).await?;
    s_controller_client.enable_pool_if_possible(&admin).await?;

    Ok(())
}
