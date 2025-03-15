use generic_pool_calculator_interface::LstToSolIxArgs;
use spl_calculator_lib::{
    spl_lst_to_sol_ix, SplSolValCalc, SPL_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};

use moose_utils::result::Result;
use solana_sdk::instruction::Instruction;

use super::SplCalculatorClient;

impl SplCalculatorClient {
    /// no extra signer required
    pub async fn get_lst_to_sol_ix(&self, amount: u64) -> Result<Instruction> {
        let keys = SPL_LST_SOL_COMMON_INTERMEDIATE_KEYS
            .resolve::<SplSolValCalc>()
            .into();

        let args = LstToSolIxArgs { amount };

        let ix = spl_lst_to_sol_ix(keys, args)?;

        Ok(ix)
    }
}
