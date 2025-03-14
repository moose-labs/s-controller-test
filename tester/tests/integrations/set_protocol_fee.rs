use moose_utils::result::Result;
use tester::helper::instructions::SController;

use crate::test_utils::{TestValidator, new_s_controller};

#[tokio::test]
#[serial_test::serial]
async fn test_set_protocol_fee() -> Result<()> {
    let _validator = TestValidator::new().await?;

    let (s_controller_client, admin) = new_s_controller()?;

    s_controller_client.just_initialize(&admin).await?;

    s_controller_client
        .set_protocol_fee(Some(5_000), Some(6_000), &admin)
        .await?;

    let pool_state = s_controller_client.get_pool_state().await?;

    assert_eq!(pool_state.trading_protocol_fee_bps, 5_000);
    assert_eq!(pool_state.lp_protocol_fee_bps, 6_000);

    Ok(())
}
