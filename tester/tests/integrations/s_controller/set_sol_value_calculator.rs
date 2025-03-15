use marinade_keys::{marinade_program, marinade_program_progdata, marinade_state, msol};
use moose_utils::result::Result;
use tester::helper::instructions::s_controller::SController;

use crate::test_utils::{TestValidator, new_s_controller_client};

#[tokio::test]
#[serial_test::serial]
async fn test_set_sol_value_calculator() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller_client()?;

    s_controller_client.just_initialize(&admin).await?;

    let msol_mint = msol::ID;
    let marinade_calculator = marinade_calculator_lib::program::ID;

    s_controller_client
        .add_lst(&msol_mint, &marinade_calculator, &admin)
        .await?;

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
    Ok(())
}
