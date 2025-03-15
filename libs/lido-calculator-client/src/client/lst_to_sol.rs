use generic_pool_calculator_interface::LstToSolIxArgs;
use lido_calculator_lib::{
    lido_lst_to_sol_ix, LidoSolValCalc, LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};

use moose_utils::result::Result;
use solana_sdk::instruction::Instruction;

use super::LidoCalculatorClient;

impl LidoCalculatorClient {
    /// no extra signer required
    pub async fn get_lst_to_sol_ix(&self, amount: u64) -> Result<Instruction> {
        let keys = LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS
            .resolve::<LidoSolValCalc>()
            .into();

        let args = LstToSolIxArgs { amount };

        let ix = lido_lst_to_sol_ix(keys, args)?;

        Ok(ix)
    }
}
