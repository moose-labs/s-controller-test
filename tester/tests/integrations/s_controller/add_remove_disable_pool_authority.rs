use moose_utils::result::Result;
use solana_sdk::{signature::Keypair, signer::Signer};
use tester::helper::instructions::s_controller::SController;

use crate::test_utils::{new_s_controller_client, TestValidator};

#[tokio::test]
#[serial_test::serial]
async fn test_add_disable_pool_authority() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller_client()?;

    s_controller_client.just_initialize(&admin).await?;

    let new_authority = Keypair::new();
    s_controller_client
        .add_disable_pool_authority(&new_authority.pubkey(), &admin)
        .await?;

    let disable_authority_list = s_controller_client
        .get_disable_pool_authority_list()
        .await?;

    assert_eq!(disable_authority_list.len(), 1);
    assert_eq!(disable_authority_list[0], new_authority.pubkey());

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_remove_disable_pool_authority_by_admin() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller_client()?;

    s_controller_client.just_initialize(&admin).await?;

    let new_authority1 = Keypair::new();
    let new_authority2 = Keypair::new();
    s_controller_client
        .add_disable_pool_authority(&new_authority1.pubkey(), &admin)
        .await?;
    s_controller_client
        .add_disable_pool_authority(&new_authority2.pubkey(), &admin)
        .await?;

    s_controller_client
        .remove_disable_pool_authority_by_admin(&new_authority1.pubkey(), &admin)
        .await?;

    let disable_authority_list = s_controller_client
        .get_disable_pool_authority_list()
        .await?;

    assert_eq!(disable_authority_list.len(), 1);
    assert_eq!(disable_authority_list[0], new_authority2.pubkey());

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_remove_disable_pool_authority_by_authority() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller_client()?;

    s_controller_client.just_initialize(&admin).await?;

    let new_authority1 = Keypair::new();
    let new_authority2 = Keypair::new();
    s_controller_client
        .add_disable_pool_authority(&new_authority1.pubkey(), &admin)
        .await?;
    s_controller_client
        .add_disable_pool_authority(&new_authority2.pubkey(), &admin)
        .await?;

    s_controller_client
        .remove_disable_pool_authority_by_authority(&new_authority2)
        .await?;

    let disable_authority_list = s_controller_client
        .get_disable_pool_authority_list()
        .await?;

    assert_eq!(disable_authority_list.len(), 1);
    assert_eq!(disable_authority_list[0], new_authority1.pubkey());

    Ok(())
}
