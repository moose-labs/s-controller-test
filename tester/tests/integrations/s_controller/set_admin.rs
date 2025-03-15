use moose_utils::result::Result;
use solana_sdk::{signature::Keypair, signer::Signer};
use tester::helper::instructions::s_controller::SController;

use crate::test_utils::{TestValidator, new_s_controller_client};

#[tokio::test]
#[serial_test::serial]
async fn test_set_admin() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller_client()?;

    let admin_pubkey = admin.pubkey();

    s_controller_client.just_initialize(&admin).await?;

    let new_admin = Keypair::new();
    s_controller_client
        .set_admin_if_not_match(&new_admin.pubkey(), &admin)
        .await?;

    let pool_state = s_controller_client.get_pool_state().await?;

    assert_eq!(pool_state.admin, new_admin.pubkey());
    assert_eq!(pool_state.rebalance_authority, admin_pubkey);
    assert_eq!(pool_state.protocol_fee_beneficiary, admin_pubkey);

    Ok(())
}
