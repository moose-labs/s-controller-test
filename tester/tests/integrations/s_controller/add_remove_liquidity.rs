use base_client::client::Client;
use marinade_calculator_lib::marinade_sol_val_calc_account_metas;
use marinade_keys::msol;
use moose_utils::result::Result;
use solana_sdk::{instruction::AccountMeta, signer::Signer};
use tester::helper::instructions::{
    flat_fee::FlatFee, marinade_calculator::MarinadeCalculator, s_controller::SController,
};

use crate::test_utils::{
    new_flat_fee_client, new_marinade_calculator_client, new_s_controller_client_with_keypair,
    TestValidator,
};

#[tokio::test]
#[serial_test::serial]
async fn test_add_liquidity() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (marinade_calculator_client, initial_manager_keypair) = new_marinade_calculator_client()?;
    marinade_calculator_client.init_if_possible().await?;
    marinade_calculator_client
        .update_last_upgrade_slot(&initial_manager_keypair)
        .await?;

    let (s_controller_client, admin) = new_s_controller_client_with_keypair("user1.json")?;
    let user_pubkey = s_controller_client.payer().pubkey();

    s_controller_client.just_initialize(&admin).await?;

    let msol_mint = msol::ID;
    let marinade_calculator = marinade_calculator_lib::program::ID;

    s_controller_client
        .add_lst(&msol_mint, &marinade_calculator, &admin)
        .await?;

    let src_lst_token_account = s_controller_client
        .get_ata(&msol_mint, &user_pubkey)
        .await?;
    let pool_state = s_controller_client.get_pool_state().await?;
    let des_lp_token_account = s_controller_client
        .create_ata(&pool_state.lp_token_mint, &user_pubkey)
        .await?;

    let lst_add_amount = 100_000_000_000; // 100 tokens

    let lp_token_account_balance_before = s_controller_client
        .get_token_account_balance(&des_lp_token_account)
        .await?;
    let lst_token_account_balance_before = s_controller_client
        .get_token_account_balance(&src_lst_token_account)
        .await?;

    let lst_calculator_accounts = marinade_sol_val_calc_account_metas();

    let pricing_program_accounts = [AccountMeta {
        pubkey: msol_mint,
        is_signer: false,
        is_writable: false,
    }];
    s_controller_client
        .add_liquidity(
            &msol_mint,
            &src_lst_token_account,
            &des_lp_token_account,
            lst_add_amount,
            0,
            &lst_calculator_accounts,
            &pricing_program_accounts,
        )
        .await?;

    let lp_token_account_balance_after = s_controller_client
        .get_token_account_balance(&des_lp_token_account)
        .await?;
    let lst_token_account_balance_after = s_controller_client
        .get_token_account_balance(&src_lst_token_account)
        .await?;

    assert!(lp_token_account_balance_after > lp_token_account_balance_before);
    assert!(lst_token_account_balance_before > lst_token_account_balance_after);

    println!(
        "xxx BEFORE lst: {}, lp: {}",
        lst_token_account_balance_before, lp_token_account_balance_before
    );
    println!(
        "xxx AFTER  lst: {}, lp: {}",
        lst_token_account_balance_after, lp_token_account_balance_after
    );

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_add_liquidity_simulate() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (marinade_calculator_client, initial_manager_keypair) = new_marinade_calculator_client()?;
    marinade_calculator_client.init_if_possible().await?;
    marinade_calculator_client
        .update_last_upgrade_slot(&initial_manager_keypair)
        .await?;

    let (s_controller_client, admin) = new_s_controller_client_with_keypair("user1.json")?;
    let user_pubkey = s_controller_client.payer().pubkey();

    s_controller_client.just_initialize(&admin).await?;

    let msol_mint = msol::ID;
    let marinade_calculator = marinade_calculator_lib::program::ID;

    s_controller_client
        .add_lst(&msol_mint, &marinade_calculator, &admin)
        .await?;

    let src_lst_token_account = s_controller_client
        .get_ata(&msol_mint, &user_pubkey)
        .await?;
    let pool_state = s_controller_client.get_pool_state().await?;
    let des_lp_token_account = s_controller_client
        .create_ata(&pool_state.lp_token_mint, &user_pubkey)
        .await?;

    let lst_add_amount = 100_000_000_000; // 100 tokens

    let lst_calculator_accounts = marinade_sol_val_calc_account_metas();

    let pricing_program_accounts = [AccountMeta {
        pubkey: msol_mint,
        is_signer: false,
        is_writable: false,
    }];
    let ret = s_controller_client
        .add_liquidity_simulate(
            &msol_mint,
            &src_lst_token_account,
            &des_lp_token_account,
            lst_add_amount,
            0,
            &lst_calculator_accounts,
            &pricing_program_accounts,
        )
        .await?;

    println!("xxx lp mint: {}", &pool_state.lp_token_mint);
    assert!(ret.is_some());
    assert!(ret.unwrap() > 0);

    println!("xxx ret: {}", ret.unwrap());

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_remove_liquidity() -> Result<()> {
    let _validator = TestValidator::new().await?;

    // initialize flat-fee contract.
    let (flat_fee_client, _) = new_flat_fee_client()?;
    flat_fee_client.initialize().await?;

    // initialize marinade calculator contract.
    let (marinade_calculator_client, initial_manager_keypair) = new_marinade_calculator_client()?;
    marinade_calculator_client.init_if_possible().await?;
    marinade_calculator_client
        .update_last_upgrade_slot(&initial_manager_keypair)
        .await?;

    // initialize s-controller contract.
    let (s_controller_client, admin) = new_s_controller_client_with_keypair("user1.json")?;
    let user_pubkey = s_controller_client.payer().pubkey();

    s_controller_client.just_initialize(&admin).await?;

    let msol_mint = msol::ID;
    let marinade_calculator = marinade_calculator_lib::program::ID;

    s_controller_client
        .add_lst(&msol_mint, &marinade_calculator, &admin)
        .await?;

    let src_lst_token_account = s_controller_client
        .get_ata(&msol_mint, &user_pubkey)
        .await?;
    let pool_state = s_controller_client.get_pool_state().await?;
    let des_lp_token_account = s_controller_client
        .create_ata(&pool_state.lp_token_mint, &user_pubkey)
        .await?;

    let lst_add_amount = 100_000_000_000; // 100 tokens

    let lst_calculator_accounts = marinade_sol_val_calc_account_metas();

    let pricing_program_accounts = [AccountMeta {
        pubkey: msol_mint,
        is_signer: false,
        is_writable: false,
    }];
    s_controller_client
        .add_liquidity(
            &msol_mint,
            &src_lst_token_account,
            &des_lp_token_account,
            lst_add_amount,
            0,
            &lst_calculator_accounts,
            &pricing_program_accounts,
        )
        .await?;

    let lp_token_account_balance_before = s_controller_client
        .get_token_account_balance(&des_lp_token_account)
        .await?;
    let lst_token_account_balance_before = s_controller_client
        .get_token_account_balance(&src_lst_token_account)
        .await?;

    let lp_remove_amount = lp_token_account_balance_before / 2; // withdraw 50%

    let src_lp_token_account = des_lp_token_account;
    let des_lst_token_account = src_lst_token_account;

    let pricing_program_accounts_to_redeem = [
        AccountMeta {
            pubkey: msol_mint,
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: flat_fee_lib::program::STATE_ID,
            is_signer: false,
            is_writable: false,
        },
    ];

    s_controller_client
        .remove_liquidity(
            &msol_mint,
            &src_lp_token_account,
            &des_lst_token_account,
            lp_remove_amount,
            0,
            &lst_calculator_accounts,
            &pricing_program_accounts_to_redeem,
        )
        .await?;

    let lp_token_account_balance_after = s_controller_client
        .get_token_account_balance(&des_lp_token_account)
        .await?;
    let lst_token_account_balance_after = s_controller_client
        .get_token_account_balance(&src_lst_token_account)
        .await?;

    assert!(lp_token_account_balance_before > lp_token_account_balance_after);
    assert!(lst_token_account_balance_before < lst_token_account_balance_after);

    println!(
        "xxx BEFORE lst: {}, lp: {}",
        lst_token_account_balance_before, lp_token_account_balance_before
    );
    println!(
        "xxx AFTER  lst: {}, lp: {}",
        lst_token_account_balance_after, lp_token_account_balance_after
    );
    println!(
        "xxx  lp diff: {}",
        lp_token_account_balance_before - lp_token_account_balance_after
    );
    println!(
        "xxx lst diff: {}",
        lst_token_account_balance_after - lst_token_account_balance_before
    );

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_remove_liquidity_simulate() -> Result<()> {
    let _validator = TestValidator::new().await?;

    // initialize flat-fee contract.
    let (flat_fee_client, _) = new_flat_fee_client()?;
    flat_fee_client.initialize().await?;

    // initialize marinade calculator contract.
    let (marinade_calculator_client, initial_manager_keypair) = new_marinade_calculator_client()?;
    marinade_calculator_client.init_if_possible().await?;
    marinade_calculator_client
        .update_last_upgrade_slot(&initial_manager_keypair)
        .await?;

    // initialize s-controller contract.
    let (s_controller_client, admin) = new_s_controller_client_with_keypair("user1.json")?;
    let user_pubkey = s_controller_client.payer().pubkey();

    s_controller_client.just_initialize(&admin).await?;

    let msol_mint = msol::ID;
    let marinade_calculator = marinade_calculator_lib::program::ID;

    s_controller_client
        .add_lst(&msol_mint, &marinade_calculator, &admin)
        .await?;

    let src_lst_token_account = s_controller_client
        .get_ata(&msol_mint, &user_pubkey)
        .await?;
    let pool_state = s_controller_client.get_pool_state().await?;
    let des_lp_token_account = s_controller_client
        .create_ata(&pool_state.lp_token_mint, &user_pubkey)
        .await?;

    let lst_add_amount = 100_000_000_000; // 100 tokens

    let lst_calculator_accounts = marinade_sol_val_calc_account_metas();

    let pricing_program_accounts = [AccountMeta {
        pubkey: msol_mint,
        is_signer: false,
        is_writable: false,
    }];
    s_controller_client
        .add_liquidity(
            &msol_mint,
            &src_lst_token_account,
            &des_lp_token_account,
            lst_add_amount,
            0,
            &lst_calculator_accounts,
            &pricing_program_accounts,
        )
        .await?;

    let lp_token_account_balance_before = s_controller_client
        .get_token_account_balance(&des_lp_token_account)
        .await?;

    let lp_remove_amount = lp_token_account_balance_before / 2; // withdraw 50%

    let src_lp_token_account = des_lp_token_account;
    let des_lst_token_account = src_lst_token_account;

    let pricing_program_accounts_to_redeem = [
        AccountMeta {
            pubkey: msol_mint,
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: flat_fee_lib::program::STATE_ID,
            is_signer: false,
            is_writable: false,
        },
    ];

    let ret = s_controller_client
        .remove_liquidity_simulate(
            &msol_mint,
            &src_lp_token_account,
            &des_lst_token_account,
            lp_remove_amount,
            0,
            &lst_calculator_accounts,
            &pricing_program_accounts_to_redeem,
        )
        .await?;

    assert!(ret.is_some());
    assert!(ret.unwrap() > 0);

    println!("xxx ret {}", ret.unwrap());

    Ok(())
}
