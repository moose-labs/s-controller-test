use generic_pool_calculator_interface::{SetManagerKeys, SolToLstIxArgs};
use spl_calculator_lib::{spl_set_manager_ix, spl_sol_to_lst_ix, SplSolValCalc};

use moose_utils::result::Result;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::SplCalculatorClient;

impl SplCalculatorClient {
    /// required signer: manager
    pub async fn get_set_manager_ix(&self, new_manager: &Pubkey) -> Result<Instruction> {
        let calculator_state = self.get_calculator_state().await?;
        let keys = SetManagerKeys {
            manager: calculator_state.manager,
            new_manager: *new_manager,
            state: spl_calculator_lib::program::SPL_CALCULATOR_STATE_ID,
        };

        let ix = spl_set_manager_ix(keys)?;

        Ok(ix)
    }
}
