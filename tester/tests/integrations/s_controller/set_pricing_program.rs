use moose_utils::result::Result;
use solana_sdk::pubkey::Pubkey;
use tester::{
    helper::instructions::s_controller::SController,
    test_utils::{new_s_controller_client, TestValidator},
};

#[tokio::test]
#[serial_test::serial]
async fn test_set_pricing_program() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller_client()?;

    s_controller_client.just_initialize(&admin).await?;

    let new_pricing_program = Pubkey::from_str_const("f1tUoNEKrDp1oeGn4zxr7bh41eN6VcfHjfrL3ZqQday");
    s_controller_client
        .set_pricing_program(&new_pricing_program, &admin)
        .await?;

    let pool_state = s_controller_client.get_pool_state().await?;

    assert_eq!(pool_state.pricing_program, new_pricing_program);

    Ok(())
}
