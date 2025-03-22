use moose_utils::result::Result;
use solana_sdk::{signature::Keypair, signer::Signer};
use tester::{
    helper::instructions::lido_calculator::LidoCalculator,
    test_utils::{new_lido_calculator_client, TestValidator},
};

#[tokio::test()]
#[serial_test::serial]
async fn test_set_manager() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (lido_calculator_client, initial_manager_keypair) = new_lido_calculator_client()?;
    lido_calculator_client.init_if_possible().await?;

    let manager1 = Keypair::new();
    let manager2 = Keypair::new();

    lido_calculator_client
        .set_manager(&manager1.pubkey(), &initial_manager_keypair)
        .await?;

    lido_calculator_client
        .set_manager(&manager2.pubkey(), &manager1)
        .await?;

    let calculator_state = lido_calculator_client.get_calculator_state().await?;

    assert_eq!(calculator_state.manager, manager2.pubkey());

    Ok(())
}
