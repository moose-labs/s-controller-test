use moose_utils::result::Result;
use s_controller_interface::{disable_pool_ix, enable_pool_ix, DisablePoolKeys, EnablePoolKeys};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::SControllerClient;

impl SControllerClient {
    /// required signer: admin
    pub async fn get_disable_pool_ix(&self) -> Result<Instruction> {
        let pool_state = self.get_pool_state().await?;

        let keys = DisablePoolKeys {
            signer: pool_state.admin, // admin or one of disable pool authority
            pool_state: self.get_pool_state_pubkey(),
            disable_pool_authority_list: self.get_disable_pool_authority_list_pubkey(),
        };

        let disable_pool_instruction = disable_pool_ix(keys)?;

        Ok(disable_pool_instruction)
    }

    /// required signer: disable_authority
    pub async fn get_disable_pool_by_disable_authority_ix(
        &self,
        disable_authority: &Pubkey,
    ) -> Result<Instruction> {
        let keys = DisablePoolKeys {
            signer: *disable_authority, // admin or one of disable pool authority
            pool_state: self.get_pool_state_pubkey(),
            disable_pool_authority_list: self.get_disable_pool_authority_list_pubkey(),
        };

        let disable_pool_instruction = disable_pool_ix(keys)?;

        Ok(disable_pool_instruction)
    }

    /// required signer: admin
    pub async fn get_enable_pool_ix(&self) -> Result<Instruction> {
        let pool_state_acc = self.get_pool_state().await?;

        let keys = EnablePoolKeys {
            admin: pool_state_acc.admin, // signer
            pool_state: self.get_pool_state_pubkey(),
        };

        let disable_pool_instruction = enable_pool_ix(keys)?;

        Ok(disable_pool_instruction)
    }
}
