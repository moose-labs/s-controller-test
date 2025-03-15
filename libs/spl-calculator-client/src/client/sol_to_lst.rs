use generic_pool_calculator_interface::{SolToLstIxArgs, SolToLstKeys};
use spl_calculator_lib::{spl_sol_to_lst_ix, SplSolValCalc};

use moose_utils::result::Result;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::SplCalculatorClient;

impl SplCalculatorClient {
    /// no extra signer required
    pub async fn get_sol_to_lst_ix(
        &self,
        lst_mint: &Pubkey,
        spl_stake_pool_state: &Pubkey,
        amount: u64,
    ) -> Result<Instruction> {
        let keys = SolToLstKeys {
            lst_mint: *lst_mint,
            state: spl_calculator_lib::program::SPL_CALCULATOR_STATE_ID,
            pool_state: *spl_stake_pool_state,
            pool_program: spl_stake_pool_keys::spl_stake_pool_program::ID,
            pool_program_data: spl_stake_pool_keys::spl_stake_pool_program_progdata::ID,
        };

        let args = SolToLstIxArgs { amount };

        let ix = spl_sol_to_lst_ix(keys, args)?;

        Ok(ix)
    }
}
