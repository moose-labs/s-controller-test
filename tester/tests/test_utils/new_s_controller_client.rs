use moose_utils::result::Result;
use s_controller_client::client::SControllerClient;
use solana_sdk::signature::Keypair;

use super::new_s_controller_client_with_keypair;

pub fn new_s_controller_client() -> Result<(SControllerClient, Keypair)> {
    new_s_controller_client_with_keypair("local-auth.json")
}
