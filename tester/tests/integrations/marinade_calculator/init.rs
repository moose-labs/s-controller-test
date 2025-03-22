use moose_utils::result::Result;
use tester::{
    helper::instructions::marinade_calculator::MarinadeCalculator,
    test_utils::{new_marinade_calculator_client, TestValidator},
};

#[tokio::test()]
#[serial_test::serial]
async fn test_init() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (marinade_calculator_client, _) = new_marinade_calculator_client()?;

    marinade_calculator_client.init_if_possible().await?;

    let calculator_state = marinade_calculator_client.get_calculator_state().await?;

    assert_eq!(calculator_state.last_upgrade_slot, 0);
    assert_eq!(
        calculator_state.manager,
        marinade_calculator_lib::initial_manager::ID
    );

    Ok(())
}
