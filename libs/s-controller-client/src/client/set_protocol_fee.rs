use moose_utils::result::Result;
use s_controller_interface::{set_protocol_fee_ix, SetProtocolFeeIxArgs, SetProtocolFeeKeys};
use solana_sdk::instruction::Instruction;

use super::SControllerClient;

impl SControllerClient {
    /// required signer: admin
    pub async fn get_set_protocol_fee_ix(
        &self,
        new_trading_protocol_fee_bps: Option<u16>,
        new_lp_protocol_fee_bps: Option<u16>,
    ) -> Result<Instruction> {
        let pool_state = self.get_pool_state().await?;

        let keys = SetProtocolFeeKeys {
            admin: pool_state.admin,
            pool_state: self.get_pool_state_pubkey(),
        };

        let args = SetProtocolFeeIxArgs {
            new_trading_protocol_fee_bps,
            new_lp_protocol_fee_bps,
        };

        let ix = set_protocol_fee_ix(keys, args)?;

        Ok(ix)
    }
}
