use moose_utils::result::Result;
use tester::{
    helper::instructions::s_controller::SController,
    test_utils::{new_s_controller_client, TestValidator},
};

#[tokio::test]
#[serial_test::serial]
async fn test_disable_pool() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller_client()?;

    s_controller_client.just_initialize(&admin).await?;

    s_controller_client.disable_pool_if_possible(&admin).await?;

    let pool_state = s_controller_client.get_pool_state().await?;

    assert!(pool_state.is_disabled != 0);

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_enable_pool() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller_client()?;

    s_controller_client.just_initialize(&admin).await?;

    s_controller_client.disable_pool_if_possible(&admin).await?;
    s_controller_client.enable_pool_if_possible(&admin).await?;

    let pool_state = s_controller_client.get_pool_state().await?;

    assert!(pool_state.is_disabled == 0);

    Ok(())
}
