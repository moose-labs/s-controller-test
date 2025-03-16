use base_client::client::Client;
use moose_utils::result::Result;
use s_controller_client::client::SControllerClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair},
};
use tester::utils::paths::get_deps_configs;

pub fn new_s_controller_client_with_keypair(
    key_file: &str,
) -> Result<(SControllerClient, Keypair)> {
    let payer = read_keypair_file(get_deps_configs(key_file))?;
    let url = "http://localhost:8899";

    let initial_authority_keypair = read_keypair_file(get_deps_configs(
        "s-controller-test-initial-authority-key.json",
    ))?;

    let s_controller_client =
        SControllerClient::new(payer, url.to_string(), CommitmentConfig::processed());

    Ok((s_controller_client, initial_authority_keypair))
}
