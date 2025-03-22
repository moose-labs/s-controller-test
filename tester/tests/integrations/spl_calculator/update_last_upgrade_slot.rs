use moose_utils::result::Result;
use tester::{
    helper::instructions::spl_calculator::SplCalculator,
    test_utils::{new_spl_calculator_client, TestValidator},
};

#[tokio::test()]
#[serial_test::serial]
async fn test_update_last_upgrade_slot() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (spl_calculator_client, initial_manager_keypair) = new_spl_calculator_client()?;

    spl_calculator_client.init_if_possible().await?;
    spl_calculator_client
        .update_last_upgrade_slot(&initial_manager_keypair)
        .await?;

    let calculator_state = spl_calculator_client.get_calculator_state().await?;

    println!("spl calculator_state: {:?}", calculator_state);

    Ok(())
}
