use moose_utils::result::Result;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use tester::helper::instructions::SController;

use crate::test_utils::{TestValidator, new_s_controller};

#[tokio::test]
#[serial_test::serial]
async fn test_set_rebalance_authority_by_admin() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller()?;

    s_controller_client.just_initialize(&admin).await?;
    let authority1 = Pubkey::new_unique();
    let authority2 = Pubkey::new_unique();
    s_controller_client
        .set_rebalance_authority_by_admin(&authority1, &admin)
        .await?;
    s_controller_client
        .set_rebalance_authority_by_admin(&authority2, &admin)
        .await?;

    let pool_state = s_controller_client.get_pool_state().await?;

    assert_eq!(pool_state.rebalance_authority, authority2);

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_set_rebalance_authority_by_authority() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller()?;

    s_controller_client.just_initialize(&admin).await?;
    let authority1 = Keypair::new();
    let authority2 = Keypair::new();
    s_controller_client
        .set_rebalance_authority_by_authority(&authority1.pubkey(), &admin)
        .await?;
    s_controller_client
        .set_rebalance_authority_by_authority(&authority2.pubkey(), &authority1)
        .await?;

    let pool_state = s_controller_client.get_pool_state().await?;

    assert_eq!(pool_state.rebalance_authority, authority2.pubkey());

    Ok(())
}
