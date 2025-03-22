use crate::utils::paths::get_deps_configs;
use base_client::client::Client;
use moose_utils::result::Result;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair},
};
use spl_calculator_client::client::SplCalculatorClient;

pub fn new_spl_calculator_client() -> Result<(SplCalculatorClient, Keypair)> {
    let payer = read_keypair_file(get_deps_configs("local-auth.json"))?;
    let url = "http://localhost:8899";

    let initial_manager_keypair =
        read_keypair_file(get_deps_configs("flat-fee-test-initial-manager-key.json"))?;

    let spl_calculator_client =
        SplCalculatorClient::new(payer, url.to_string(), CommitmentConfig::processed());

    Ok((spl_calculator_client, initial_manager_keypair))
}
