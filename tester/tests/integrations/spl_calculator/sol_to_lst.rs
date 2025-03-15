use moose_utils::result::Result;
use solana_sdk::pubkey::Pubkey;
use tester::helper::instructions::spl_calculator::SplCalculator;

use crate::test_utils::{new_spl_calculator_client, TestValidator};

#[tokio::test()]
#[serial_test::serial]
async fn test_sol_to_lst() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (spl_calculator_client, initial_manager_keypair) = new_spl_calculator_client()?;

    spl_calculator_client.init_if_possible().await?;
    spl_calculator_client
        .update_last_upgrade_slot(&initial_manager_keypair)
        .await?;

    let jitosol_mint = Pubkey::from_str_const("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn");
    let jito_stake_pool = Pubkey::from_str_const("Jito4APyf642JPZPx3hGc6WWJ8zPKtRbRs4P815Awbb");
    let amount = 1_000_000_000;

    let value = spl_calculator_client
        .sol_to_lst(&jitosol_mint, &jito_stake_pool, amount)
        .await?;

    // HACK: Bypass epoch checking: ExchangeRateNotUpdatedInThisEpoch

    assert!(value.is_some());
    assert!(value.unwrap() > 0);

    Ok(())
}
