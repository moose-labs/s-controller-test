use marinade_calculator_client::client::MarinadeCalculatorClient;
use moose_utils::result::Result;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    signature::{read_keypair_file, Keypair},
};
use tester::utils::paths::get_deps_configs;

pub fn new_marinade_calculator_client() -> Result<(MarinadeCalculatorClient, Keypair)> {
    let payer = read_keypair_file(get_deps_configs("user2.json"))?;
    let url = "http://localhost:8899";

    let initial_manager_keypair =
        read_keypair_file(get_deps_configs("flat-fee-test-initial-manager-key.json"))?;

    let flat_fee_client =
        MarinadeCalculatorClient::new(payer, url.to_string(), CommitmentConfig::processed());

    Ok((flat_fee_client, initial_manager_keypair))
}
