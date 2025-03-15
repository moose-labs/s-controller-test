use marinade_keys::msol;
use moose_utils::result::Result;
use tester::helper::instructions::s_controller::SController;

use crate::test_utils::{TestValidator, new_s_controller_client};

#[tokio::test]
#[serial_test::serial]
async fn test_disable_lst_input() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller_client()?;

    s_controller_client.just_initialize(&admin).await?;

    let msol_mint = msol::ID;
    let marinade_calculator = marinade_calculator_lib::program::ID;

    s_controller_client
        .add_lst(&msol_mint, &marinade_calculator, &admin)
        .await?;

    s_controller_client
        .disable_lst_input(&msol_mint, &admin)
        .await?;

    let lst_state_list = s_controller_client.get_lst_state_list().await?;

    assert_eq!(lst_state_list.len(), 1);
    assert_eq!(lst_state_list[0].is_input_disabled, 1);

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_enable_lst_input_first_one() -> Result<()> {
    let _validator: TestValidator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller_client()?;

    s_controller_client.just_initialize(&admin).await?;

    let msol_mint = msol::ID;
    let marinade_calculator = marinade_calculator_lib::program::ID;
    let stsol_mint = lido_keys::stsol::ID;
    let lido_calculator = lido_calculator_lib::program::ID;

    s_controller_client
        .add_lst(&msol_mint, &marinade_calculator, &admin)
        .await?;
    s_controller_client
        .add_lst(&stsol_mint, &lido_calculator, &admin)
        .await?;

    s_controller_client
        .disable_lst_input(&msol_mint, &admin)
        .await?;
    s_controller_client
        .enable_lst_input(&msol_mint, &admin)
        .await?;

    let lst_state_list = s_controller_client.get_lst_state_list().await?;

    assert_eq!(lst_state_list.len(), 2);
    assert_eq!(lst_state_list[0].mint, msol_mint);
    assert_eq!(lst_state_list[0].is_input_disabled, 0);

    Ok(())
}

#[tokio::test]
#[serial_test::serial]
async fn test_enable_lst_input_second_one() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller_client()?;

    s_controller_client.just_initialize(&admin).await?;

    let msol_mint = msol::ID;
    let marinade_calculator = marinade_calculator_lib::program::ID;
    let stsol_mint = lido_keys::stsol::ID;
    let lido_calculator = lido_calculator_lib::program::ID;

    s_controller_client
        .add_lst(&msol_mint, &marinade_calculator, &admin)
        .await?;
    s_controller_client
        .add_lst(&stsol_mint, &lido_calculator, &admin)
        .await?;

    s_controller_client
        .disable_lst_input(&stsol_mint, &admin)
        .await?;
    s_controller_client
        .enable_lst_input(&stsol_mint, &admin)
        .await?;

    let lst_state_list = s_controller_client.get_lst_state_list().await?;

    assert_eq!(lst_state_list.len(), 2);
    assert_eq!(lst_state_list[1].mint, stsol_mint);
    assert_eq!(lst_state_list[1].is_input_disabled, 0);

    Ok(())
}
