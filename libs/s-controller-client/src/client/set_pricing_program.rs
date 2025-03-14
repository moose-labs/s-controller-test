use moose_utils::result::Result;
use s_controller_interface::{SetPricingProgramKeys, set_pricing_program_ix};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::SControllerClient;

impl SControllerClient {
    /// required signer: admin
    pub async fn get_set_pricing_program_ix(
        &self,
        new_pricing_program: &Pubkey,
    ) -> Result<Instruction> {
        let pool_state = self.get_pool_state().await?;

        let keys = SetPricingProgramKeys {
            admin: pool_state.admin,
            new_pricing_program: new_pricing_program.clone(),
            pool_state: self.get_pool_state_pubkey(),
        };

        let ix = set_pricing_program_ix(keys)?;

        Ok(ix)
    }
}
