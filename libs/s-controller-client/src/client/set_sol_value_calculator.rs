use moose_utils::result::Result;
use s_controller_lib::{
    SetSolValueCalculatorByMintFreeArgs, set_sol_value_calculator_ix_by_mint_full,
};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use super::SControllerClient;

impl SControllerClient {
    /// sol_value_calculator: The LST's SOL value calculator program to set to..
    /// lst_to_sol_accounts: Account suffix slice to call LstToSol for the given LST, excluding the program ID and mint.
    /// required signer: admin
    pub async fn get_set_sol_value_calculator_ix(
        &self,
        lst_mint_pubkey: &Pubkey,
        sol_value_calculator_program_id: &Pubkey,
        lst_to_sol_accounts: Vec<Pubkey>,
    ) -> Result<Instruction> {
        let pool_state_acc = self.get_account(&self.get_pool_state_pubkey()).await?;
        let lst_state_list_acc = self.get_account(&&self.get_lst_state_list_pubkey()).await?;
        let lst_mint_acc = self.get_account(lst_mint_pubkey).await?;

        let args = &SetSolValueCalculatorByMintFreeArgs {
            pool_state: pool_state_acc,
            lst_state_list: lst_state_list_acc,
            lst_mint: KeyedAccount {
                pubkey: *lst_mint_pubkey,
                account: lst_mint_acc,
            },
        };

        let sol_value_calculator_accounts: Vec<AccountMeta> = std::iter::once(AccountMeta {
            pubkey: *lst_mint_pubkey,
            is_signer: false,
            is_writable: false,
        })
        .chain(lst_to_sol_accounts.into_iter().map(|pubkey| AccountMeta {
            pubkey,
            is_signer: false,
            is_writable: false,
        }))
        .collect();

        let ix = set_sol_value_calculator_ix_by_mint_full(
            args,
            &sol_value_calculator_accounts,
            *sol_value_calculator_program_id,
        )?;

        Ok(ix)
    }
}
