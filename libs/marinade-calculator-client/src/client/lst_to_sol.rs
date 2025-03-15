use generic_pool_calculator_interface::LstToSolIxArgs;
use marinade_calculator_lib::{
    marinade_lst_to_sol_ix, MarinadeSolValCalc, MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};

use moose_utils::result::Result;
use solana_sdk::instruction::Instruction;

use super::MarinadeCalculatorClient;

impl MarinadeCalculatorClient {
    /// no extra signer required
    pub async fn get_lst_to_sol_ix(&self, amount: u64) -> Result<Instruction> {
        let keys = MARINADE_LST_SOL_COMMON_INTERMEDIATE_KEYS
            .resolve::<MarinadeSolValCalc>()
            .into();

        let args = LstToSolIxArgs { amount };

        let ix = marinade_lst_to_sol_ix(keys, args)?;

        Ok(ix)
    }
}
