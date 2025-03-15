use moose_utils::result::Result;
use solana_sdk::pubkey::Pubkey;

use crate::test_utils::{new_marinade_calculator_client, TestValidator};

#[tokio::test()]
#[serial_test::serial]
async fn test_calculator_state() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let marinade_calculator_client = new_marinade_calculator_client()?;

    let calculator_state = marinade_calculator_client.get_calculator_state().await?;

    println!("marinade calculator_state: {:?}", calculator_state);

    assert!(calculator_state.last_upgrade_slot >= 229946024);
    assert_eq!(
        calculator_state.manager,
        Pubkey::from_str_const("CK9cEJT7K7oRrMCcEbBQRGqHLGpxKXWnKvW7nHSDMHD1")
    );

    Ok(())
}
