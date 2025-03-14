use moose_utils::result::Result;
use solana_sdk::pubkey::Pubkey;
use tester::helper::instructions::SController;

use crate::test_utils::{TestValidator, new_s_controller};

#[tokio::test]
#[serial_test::serial]
async fn test_set_protocol_fee_beneficiary() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller()?;

    s_controller_client.just_initialize(&admin).await?;

    let new_protocol_fee_beneficiary = Pubkey::new_unique();
    s_controller_client
        .set_protocol_fee_beneficiary(&new_protocol_fee_beneficiary, &admin)
        .await?;

    let pool_state = s_controller_client.get_pool_state().await?;

    assert_eq!(
        pool_state.protocol_fee_beneficiary,
        new_protocol_fee_beneficiary
    );

    Ok(())
}
