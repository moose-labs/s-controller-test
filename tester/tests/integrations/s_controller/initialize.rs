use base_client::client::Client;
use moose_utils::result::Result;
use solana_sdk::signer::Signer;
use tester::helper::instructions::s_controller::SController;

use crate::test_utils::{new_s_controller_client, TestValidator};

#[tokio::test()]
#[serial_test::serial]
async fn test_initialize() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, initial_authority_keypair) = new_s_controller_client()?;
    let initial_authority_pubkey = initial_authority_keypair.pubkey();

    let lp_token_mint = s_controller_client
        .create_mint(&initial_authority_pubkey, 9)
        .await?;

    s_controller_client
        .initialize_s_controller_if_possible(&lp_token_mint, &initial_authority_keypair)
        .await?;

    let pool_state = s_controller_client.get_pool_state().await?;

    assert_eq!(pool_state.total_sol_value, 0);
    assert_eq!(pool_state.trading_protocol_fee_bps, 1_000); // DEFAULT_TRADING_PROTOCOL_FEE_BPS (10%)
    assert_eq!(pool_state.lp_protocol_fee_bps, 1_000); // DEFAULT_LP_PROTOCOL_FEE_BPS (10%)
    assert_eq!(pool_state.version, 1); // CURRENT_PROGRAM_VERS
    assert_eq!(pool_state.is_disabled, 0);
    assert_eq!(pool_state.is_rebalancing, 0);
    assert_eq!(pool_state.admin, initial_authority_pubkey); // initial_authority::ID
    assert_eq!(pool_state.rebalance_authority, initial_authority_pubkey); // initial_authority::ID
    assert_eq!(
        pool_state.protocol_fee_beneficiary,
        initial_authority_pubkey
    ); // initial_authority::ID
    assert_eq!(pool_state.pricing_program, flat_fee_lib::program::ID); // DEFAULT_PRICING_PROGRAM
    assert_eq!(pool_state.lp_token_mint, lp_token_mint);
    assert_eq!(pool_state.padding, [0; 1]); // don't care

    Ok(())
}
