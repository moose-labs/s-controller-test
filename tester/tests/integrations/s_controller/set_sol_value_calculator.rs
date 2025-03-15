use marinade_keys::{marinade_program, marinade_program_progdata, marinade_state, msol};
use moose_utils::result::Result;
use tester::helper::instructions::{
    marinade_calculator::MarinadeCalculator, s_controller::SController,
};

use crate::test_utils::{new_marinade_calculator_client, new_s_controller_client, TestValidator};

#[tokio::test]
#[serial_test::serial]
async fn test_set_sol_value_calculator() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller_client()?;
    let (marinade_calculator_client, manager) = new_marinade_calculator_client()?;

    s_controller_client.just_initialize(&admin).await?;
    marinade_calculator_client.init_if_possible().await?;
    marinade_calculator_client
        .update_last_upgrade_slot(&manager)
        .await?;

    let msol_mint = msol::ID;
    let lido_calculator = lido_calculator_lib::program::ID;
    let marinade_calculator = marinade_calculator_lib::program::ID;

    s_controller_client
        .add_lst(&msol_mint, &lido_calculator, &admin)
        .await?;

    let lst_state_list = s_controller_client.get_lst_state_list().await?;
    assert_eq!(lst_state_list[0].sol_value_calculator, lido_calculator);

    s_controller_client
        .set_sol_value_calculator(
            &msol_mint,
            &marinade_calculator,
            [
                // without lst
                marinade_calculator_lib::program::MARINADE_CALCULATOR_STATE_ID, // The CalculatorState PDA
                marinade_state::ID,            // The main stake pool state account
                marinade_program::ID,          // The stake pool program
                marinade_program_progdata::ID, // The stake pool program executable data
            ]
            .to_vec(),
            &admin,
        )
        .await?;

    let lst_state_list = s_controller_client.get_lst_state_list().await?;

    assert_eq!(lst_state_list[0].sol_value, 0);
    assert_eq!(lst_state_list[0].sol_value_calculator, marinade_calculator);
    Ok(())
}
