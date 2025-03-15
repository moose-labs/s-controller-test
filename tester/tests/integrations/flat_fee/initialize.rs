use moose_utils::result::Result;
use solana_sdk::signer::Signer;
use tester::helper::instructions::flat_fee::FlatFee;

use crate::test_utils::{new_flat_fee_client, TestValidator};

#[tokio::test()]
#[serial_test::serial]
async fn test_initialize() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (flat_fee_client, initial_manager) = new_flat_fee_client()?;
    let initial_manager_pubkey = initial_manager.pubkey();

    flat_fee_client.initialize().await?;

    let program_state = flat_fee_client.get_program_state().await?;

    assert_eq!(program_state.lp_withdrawal_fee_bps, 5);
    assert_eq!(program_state.manager, initial_manager_pubkey);

    Ok(())
}
