use generic_pool_calculator_interface::SolToLstIxArgs;
use lido_calculator_lib::{
    lido_sol_to_lst_ix, LidoSolValCalc, LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS,
};

use moose_utils::result::Result;
use solana_sdk::instruction::Instruction;

use super::LidoCalculatorClient;

impl LidoCalculatorClient {
    /// no extra signer required
    pub async fn get_sol_to_lst_ix(&self, amount: u64) -> Result<Instruction> {
        let keys = LIDO_LST_SOL_COMMON_INTERMEDIATE_KEYS
            .resolve::<LidoSolValCalc>()
            .into();

        let args = SolToLstIxArgs { amount };

        let ix = lido_sol_to_lst_ix(keys, args)?;

        Ok(ix)
    }
}
