use base_client::client::Client;
use moose_utils::result::Result;
use s_controller_interface::{
    add_lst_ix, remove_lst_ix, AddLstKeys, RemoveLstIxArgs, RemoveLstKeys,
};
use s_controller_lib::{
    find_pool_reserves_address, find_protocol_fee_accumulator_address, find_protocol_fee_address,
    try_find_lst_mint_on_list, FindLstPdaAtaKeys,
};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer, system_program};

use super::SControllerClient;

impl SControllerClient {
    /// required signer: admin
    pub async fn get_add_lst_ix(
        &self,
        lst_mint_pubkey: &Pubkey,
        sol_value_calculator_pubkey: &Pubkey,
    ) -> Result<Instruction> {
        let lst_token_program = self.get_account_owner(lst_mint_pubkey).await?;
        let pool_state = self.get_pool_state().await?;
        let (pool_reserves_pubkey, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
            lst_mint: *lst_mint_pubkey,
            token_program: lst_token_program,
        });
        let (protocol_fee_accumulator_pubkey, _) =
            find_protocol_fee_accumulator_address(FindLstPdaAtaKeys {
                lst_mint: *lst_mint_pubkey,
                token_program: lst_token_program,
            });

        let (protocol_fee_accumulator_auth_pubkey, _) =
            find_protocol_fee_address(self.get_program_id());

        let keys = AddLstKeys {
            admin: pool_state.admin,    // signer
            payer: self.payer().pubkey(), // signer
            lst_mint: *lst_mint_pubkey,
            pool_reserves: pool_reserves_pubkey,
            protocol_fee_accumulator: protocol_fee_accumulator_pubkey,
            protocol_fee_accumulator_auth: protocol_fee_accumulator_auth_pubkey,
            sol_value_calculator: *sol_value_calculator_pubkey,
            pool_state: self.get_pool_state_pubkey(),
            lst_state_list: self.get_lst_state_list_pubkey(),
            associated_token_program: spl_associated_token_account::ID,
            system_program: system_program::ID,
            lst_token_program,
        };

        let add_lst_instruction = add_lst_ix(keys)?;

        Ok(add_lst_instruction)
    }

    /// required signer: admin
    pub async fn get_remove_lst_ix(&self, lst_mint_pubkey: &Pubkey) -> Result<Instruction> {
        let lst_token_program = self.get_account_owner(lst_mint_pubkey).await?;
        let lst_state_list = self.get_lst_state_list().await?;
        let (lst_index, _) =
            try_find_lst_mint_on_list(*lst_mint_pubkey, lst_state_list.as_slice())?;

        let pool_state = self.get_pool_state().await?;
        let (pool_reserves_pubkey, _) = find_pool_reserves_address(FindLstPdaAtaKeys {
            lst_mint: *lst_mint_pubkey,
            token_program: lst_token_program,
        });
        let (protocol_fee_accumulator_pubkey, _) =
            find_protocol_fee_accumulator_address(FindLstPdaAtaKeys {
                lst_mint: *lst_mint_pubkey,
                token_program: lst_token_program,
            });

        let (protocol_fee_accumulator_auth_pubkey, _) =
            find_protocol_fee_address(self.get_program_id());

        let remove_lst_instruction = remove_lst_ix(
            RemoveLstKeys {
                lst_mint: *lst_mint_pubkey,
                admin: pool_state.admin,
                pool_reserves: pool_reserves_pubkey,
                protocol_fee_accumulator: protocol_fee_accumulator_pubkey,
                protocol_fee_accumulator_auth: protocol_fee_accumulator_auth_pubkey,
                pool_state: self.get_pool_state_pubkey(),
                lst_state_list: self.get_lst_state_list_pubkey(),
                lst_token_program,
                refund_rent_to: self.payer().pubkey(),
            },
            RemoveLstIxArgs {
                lst_index: lst_index as u32,
            },
        )?;

        Ok(remove_lst_instruction)
    }
}
