use base_client::client::Client;
use lido_calculator_interface::Lido;
use moose_utils::result::Result;
use tester::{
    helper::instructions::lido_calculator::LidoCalculator,
    test_utils::{new_lido_calculator_client, TestValidator},
};

#[tokio::test()]
#[serial_test::serial]
async fn test_update_last_upgrade_slot() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (lido_calculator_client, initial_manager_keypair) = new_lido_calculator_client()?;

    let lido: Lido = lido_calculator_client
        .get_borsh_account_data(&lido_keys::lido_state::ID)
        .await?;

    println!("xxx lido {:?}", lido.exchange_rate);

    let epoch = lido_calculator_client.get_epoch_info().await?;
    println!("xxx epoch {:?}", epoch);

    lido_calculator_client.init_if_possible().await?;
    lido_calculator_client
        .update_last_upgrade_slot(&initial_manager_keypair)
        .await?;

    let calculator_state = lido_calculator_client.get_calculator_state().await?;

    println!("lido calculator_state: {:?}", calculator_state);

    Ok(())
}
