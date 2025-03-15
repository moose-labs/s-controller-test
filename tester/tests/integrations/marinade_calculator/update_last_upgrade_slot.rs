use moose_utils::result::Result;
use solana_sdk::signature::Keypair;
use tester::helper::instructions::marinade_calculator::MarinadeCalculator;

use crate::test_utils::{new_marinade_calculator_client, TestValidator};

#[tokio::test()]
#[serial_test::serial]
async fn test_calculator_state() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let marinade_calculator_client = new_marinade_calculator_client()?;

    let manager = Keypair::new();

    marinade_calculator_client
        .update_last_upgrade_slot(&manager)
        .await?;

    Ok(())
}
