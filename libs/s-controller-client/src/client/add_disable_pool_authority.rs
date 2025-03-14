use moose_utils::result::Result;
use s_controller_interface::{AddDisablePoolAuthorityKeys, add_disable_pool_authority_ix};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer, system_program};

use super::SControllerClient;

impl SControllerClient {
    /// required signer: admin
    pub async fn get_add_disable_pool_authority_ix(
        &self,
        new_authority: &Pubkey,
    ) -> Result<Instruction> {
        let pool_state = self.get_pool_state().await?;

        let keys = AddDisablePoolAuthorityKeys {
            payer: self.payer.pubkey(), // signer
            admin: pool_state.admin,    // signer
            pool_state: self.get_pool_state_pubkey(),
            new_authority: *new_authority,
            disable_pool_authority_list: self.get_disable_pool_authority_list_pubkey(),
            system_program: system_program::ID,
        };

        let add_disable_pool_authority_instruction = add_disable_pool_authority_ix(keys)?;

        Ok(add_disable_pool_authority_instruction)
    }
}
