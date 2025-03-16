use base_client::client::Client;
use moose_utils::result::Result;
use s_controller_interface::{
    add_disable_pool_authority_ix, remove_disable_pool_authority_ix, AddDisablePoolAuthorityKeys,
    RemoveDisablePoolAuthorityIxArgs, RemoveDisablePoolAuthorityKeys,
};
use s_controller_lib::try_find_element_in_list;
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
            payer: self.payer().pubkey(), // signer
            admin: pool_state.admin,    // signer
            pool_state: self.get_pool_state_pubkey(),
            new_authority: *new_authority,
            disable_pool_authority_list: self.get_disable_pool_authority_list_pubkey(),
            system_program: system_program::ID,
        };

        let add_disable_pool_authority_instruction = add_disable_pool_authority_ix(keys)?;

        Ok(add_disable_pool_authority_instruction)
    }

    /// required signer: admin
    pub async fn get_remove_disable_pool_authority_ix(
        &self,
        authority: &Pubkey,
    ) -> Result<Instruction> {
        let pool_state = self.get_pool_state().await?;
        let disable_authority_list = self.get_disable_pool_authority_list().await?;
        let (index, _) =
            try_find_element_in_list(*authority, disable_authority_list.as_slice()).unwrap();

        let keys = RemoveDisablePoolAuthorityKeys {
            refund_rent_to: self.payer().pubkey(),
            signer: pool_state.admin,
            authority: *authority,
            pool_state: self.get_pool_state_pubkey(),
            disable_pool_authority_list: self.get_disable_pool_authority_list_pubkey(),
        };

        let args = RemoveDisablePoolAuthorityIxArgs {
            index: index as u32,
        };

        let add_disable_pool_authority_instruction = remove_disable_pool_authority_ix(keys, args)?;

        Ok(add_disable_pool_authority_instruction)
    }

    /// required signer: authority
    pub async fn get_remove_disable_pool_authority_by_authority_ix(
        &self,
        authority: &Pubkey,
    ) -> Result<Instruction> {
        let disable_authority_list = self.get_disable_pool_authority_list().await?;
        let (index, _) =
            try_find_element_in_list(*authority, disable_authority_list.as_slice()).unwrap();

        let keys = RemoveDisablePoolAuthorityKeys {
            refund_rent_to: self.payer().pubkey(),
            signer: *authority,
            authority: *authority,
            pool_state: self.get_pool_state_pubkey(),
            disable_pool_authority_list: self.get_disable_pool_authority_list_pubkey(),
        };

        let args = RemoveDisablePoolAuthorityIxArgs {
            index: index as u32,
        };

        let add_disable_pool_authority_instruction = remove_disable_pool_authority_ix(keys, args)?;
        Ok(add_disable_pool_authority_instruction)
    }
}
