use moose_utils::result::Result;
use s_controller_interface::{set_rebalance_authority_ix, SetRebalanceAuthorityKeys};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::SControllerClient;

impl SControllerClient {
    /// required signer: admin
    pub async fn get_set_rebalance_authority_by_admin_ix(
        &self,
        new_rebalance_authority: &Pubkey,
    ) -> Result<Instruction> {
        let pool_state = self.get_pool_state().await?;

        let keys = SetRebalanceAuthorityKeys {
            signer: pool_state.admin,
            new_rebalance_authority: new_rebalance_authority.clone(),
            pool_state: self.get_pool_state_pubkey(),
        };

        let ix = set_rebalance_authority_ix(keys)?;

        Ok(ix)
    }

    /// required signer: current_rebalance_authority
    pub async fn get_set_rebalance_authority_ix(
        &self,
        new_rebalance_authority: &Pubkey,
    ) -> Result<Instruction> {
        let pool_state = self.get_pool_state().await?;

        let keys = SetRebalanceAuthorityKeys {
            signer: pool_state.rebalance_authority,
            new_rebalance_authority: new_rebalance_authority.clone(),
            pool_state: self.get_pool_state_pubkey(),
        };

        let ix = set_rebalance_authority_ix(keys)?;

        Ok(ix)
    }
}
