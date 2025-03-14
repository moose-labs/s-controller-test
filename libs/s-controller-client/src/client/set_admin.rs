use moose_utils::result::Result;
use s_controller_interface::{SetAdminKeys, set_admin_ix};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::SControllerClient;

impl SControllerClient {
    /// required signer: current_admin
    pub async fn get_set_admin_ix(&self, new_admin: &Pubkey) -> Result<Instruction> {
        let pool_state = self.get_pool_state().await?;

        let keys = SetAdminKeys {
            current_admin: pool_state.admin,
            new_admin: new_admin.clone(),
            pool_state: self.get_pool_state_pubkey(),
        };

        let ix = set_admin_ix(keys)?;

        Ok(ix)
    }
}
