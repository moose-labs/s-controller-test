use base_client::client::Client;
use moose_utils::result::Result;
use s_controller_lib::{
    disable_lst_input_ix_by_mint_full, enable_lst_input_ix_by_mint_full,
    DisableEnableLstInputByMintFreeArgs,
};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::SControllerClient;

impl SControllerClient {
    /// required signer: admin
    pub async fn get_disable_lst_input_ix(&self, lst_mint_pubkey: &Pubkey) -> Result<Instruction> {
        let pool_state_acc = self.get_account(&self.get_pool_state_pubkey()).await?;
        let lst_state_list_acc = self.get_account(&&self.get_lst_state_list_pubkey()).await?;

        let args = DisableEnableLstInputByMintFreeArgs {
            lst_mint: *lst_mint_pubkey,
            pool_state: pool_state_acc,
            lst_state_list: lst_state_list_acc,
        };

        let disable_lst_input_instruction = disable_lst_input_ix_by_mint_full(&args)?;

        Ok(disable_lst_input_instruction)
    }

    /// required signer: admin
    pub async fn get_enable_lst_input_ix(&self, lst_mint_pubkey: &Pubkey) -> Result<Instruction> {
        let pool_state_acc = self.get_account(&self.get_pool_state_pubkey()).await?;
        let lst_state_list_acc = self.get_account(&&self.get_lst_state_list_pubkey()).await?;

        let args = DisableEnableLstInputByMintFreeArgs {
            lst_mint: *lst_mint_pubkey,
            pool_state: pool_state_acc,
            lst_state_list: lst_state_list_acc,
        };

        let disable_lst_input_instruction = enable_lst_input_ix_by_mint_full(&args)?;

        Ok(disable_lst_input_instruction)
    }
}
