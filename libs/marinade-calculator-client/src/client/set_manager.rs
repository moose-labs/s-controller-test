use generic_pool_calculator_interface::{SetManagerKeys, SolToLstIxArgs};
use marinade_calculator_lib::{
    marinade_set_manager_ix, marinade_sol_to_lst_ix, MarinadeSolValCalc,
    MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};

use moose_utils::result::Result;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::MarinadeCalculatorClient;

impl MarinadeCalculatorClient {
    /// required signer: manager
    pub async fn get_set_manager_ix(&self, new_manager: &Pubkey) -> Result<Instruction> {
        let calculator_state = self.get_calculator_state().await?;
        let keys = SetManagerKeys {
            manager: calculator_state.manager,
            new_manager: *new_manager,
            state: marinade_calculator_lib::program::MARINADE_CALCULATOR_STATE_ID,
        };

        let ix = marinade_set_manager_ix(keys)?;

        Ok(ix)
    }
}
