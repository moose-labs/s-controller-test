use base_client::client::Client;
use moose_utils::result::Result;
use tester::helper::instructions::spl_calculator::SplCalculator;

use crate::test_utils::{new_spl_calculator_client, TestValidator};

#[tokio::test()]
#[serial_test::serial]
async fn test_init() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (spl_calculator_client, _) = new_spl_calculator_client()?;

    spl_calculator_client.init_if_possible().await?;

    let calculator_state = spl_calculator_client.get_calculator_state().await?;

    assert_eq!(calculator_state.last_upgrade_slot, 0);
    assert_eq!(
        calculator_state.manager,
        spl_calculator_lib::initial_manager::ID
    );

    // println!("xxx lido {:?}", lido.exchange_rate);

    let epoch = spl_calculator_client.get_epoch_info().await?;
    println!("xxx epoch {:?}", epoch);

    Ok(())
}
