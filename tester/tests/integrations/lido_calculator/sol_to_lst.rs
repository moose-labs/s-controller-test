use moose_utils::result::Result;
use tester::helper::instructions::lido_calculator::LidoCalculator;

use crate::test_utils::{new_lido_calculator_client, TestValidator};

#[tokio::test()]
#[serial_test::serial]
async fn test_sol_to_lst() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (lido_calculator_client, initial_manager_keypair) = new_lido_calculator_client()?;

    lido_calculator_client.init_if_possible().await?;
    lido_calculator_client
        .update_last_upgrade_slot(&initial_manager_keypair)
        .await?;

    let amount = 1_000_000_000;

    let value = lido_calculator_client.sol_to_lst(amount).await?;

    // HACK: Bypass epoch checking: PoolNotUpdated

    assert!(value.is_some());
    assert!(value.unwrap() > 0);

    Ok(())
}
