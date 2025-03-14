use moose_utils::result::Result;
use s_controller_client::client::SControllerClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{Keypair, read_keypair_file},
};
use tester::utils::paths::get_deps_configs;

pub fn new_s_controller() -> Result<(SControllerClient, Keypair)> {
    let payer = read_keypair_file(get_deps_configs("user1.json"))?;
    let url = "http://localhost:8899";

    let initial_authority_keypair = read_keypair_file(get_deps_configs(
        "s-controller-test-initial-authority-key.json",
    ))?;

    let s_controller_client =
        SControllerClient::new(payer, url.to_string(), CommitmentConfig::processed());

    Ok((s_controller_client, initial_authority_keypair))
}
