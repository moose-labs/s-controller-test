use generic_pool_calculator_interface::{SetManagerKeys, SolToLstIxArgs};
use lido_calculator_lib::{
    lido_set_manager_ix, lido_sol_to_lst_ix, LidoSolValCalc, LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};

use moose_utils::result::Result;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::LidoCalculatorClient;

impl LidoCalculatorClient {
    /// required signer: manager
    pub async fn get_set_manager_ix(&self, new_manager: &Pubkey) -> Result<Instruction> {
        let calculator_state = self.get_calculator_state().await?;
        let keys = SetManagerKeys {
            manager: calculator_state.manager,
            new_manager: *new_manager,
            state: lido_calculator_lib::program::LIDO_CALCULATOR_STATE_ID,
        };

        let ix = lido_set_manager_ix(keys)?;

        Ok(ix)
    }
}
