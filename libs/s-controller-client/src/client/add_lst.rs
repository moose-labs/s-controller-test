use moose_utils::result::Result;
use s_controller_interface::{AddLstKeys, add_lst_ix};
use s_controller_lib::{
    FindLstPdaAtaKeys, find_pool_reserves_address, find_protocol_fee_accumulator_address,
    find_protocol_fee_address,
};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer, system_program};

use super::SControllerClient;

impl SControllerClient {
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
            admin: pool_state.admin,
            payer: self.payer.pubkey(),
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
}
