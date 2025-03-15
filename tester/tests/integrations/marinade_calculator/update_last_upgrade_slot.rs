use moose_utils::result::Result;
use tester::helper::instructions::marinade_calculator::MarinadeCalculator;

use crate::test_utils::{new_marinade_calculator_client, TestValidator};

#[tokio::test()]
#[serial_test::serial]
async fn test_calculator_state() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (marinade_calculator_client, initial_manager_keypair) = new_marinade_calculator_client()?;

    println!("xx {}", marinade_calculator_lib::initial_manager::ID);

    marinade_calculator_client.init_if_possible().await?;
    marinade_calculator_client
        .update_last_upgrade_slot(&initial_manager_keypair)
        .await?;

    let calculator_state = marinade_calculator_client.get_calculator_state().await?;

    println!("marinade calculator_state: {:?}", calculator_state);

    Ok(())
}
