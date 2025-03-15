use generic_pool_calculator_lib::utils::read_stake_pool_progdata_meta;
use moose_utils::result::Result;
use solana_sdk::pubkey::Pubkey;
use tester::helper::instructions::marinade_calculator::MarinadeCalculator;

use crate::test_utils::{new_marinade_calculator_client, TestValidator};

#[tokio::test()]
#[serial_test::serial]
async fn test_sol_to_lst() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (marinade_calculator_client, initial_manager_keypair) = new_marinade_calculator_client()?;

    marinade_calculator_client.init_if_possible().await?;
    marinade_calculator_client
        .update_last_upgrade_slot(&initial_manager_keypair)
        .await?;

    let account = marinade_calculator_client
        .get_account(&Pubkey::from_str_const(
            "4PQH9YmfuKrVyZaibkLYpJZPv2FPaybhq2GAuBcWMSBf",
        ))
        .await?;

    let meta = read_stake_pool_progdata_meta(account)?;

    println!("meta: slot: {}, upgrade authority: {:?}", meta.0, meta.1);

    let amount = 1_000_000_000;

    marinade_calculator_client.sol_to_lst(amount).await?;

    Ok(())
}
