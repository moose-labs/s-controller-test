use crate::utils::paths::get_deps_configs;
use base_client::client::Client;
use lido_calculator_client::client::LidoCalculatorClient;
use moose_utils::result::Result;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair},
};

pub fn new_lido_calculator_client() -> Result<(LidoCalculatorClient, Keypair)> {
    let payer = read_keypair_file(get_deps_configs("local-auth.json"))?;
    let url = "http://localhost:8899";

    let initial_manager_keypair =
        read_keypair_file(get_deps_configs("flat-fee-test-initial-manager-key.json"))?;

    let lido_calculator_client =
        LidoCalculatorClient::new(payer, url.to_string(), CommitmentConfig::processed());

    Ok((lido_calculator_client, initial_manager_keypair))
}
