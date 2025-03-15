use generic_pool_calculator_interface::InitKeys;
use marinade_calculator_lib::marinade_init_ix;
use moose_utils::result::Result;
use solana_sdk::{instruction::Instruction, signer::Signer, system_program};

use super::MarinadeCalculatorClient;

impl MarinadeCalculatorClient {
    /// no extra signer required
    pub async fn get_init_ix(&self) -> Result<Instruction> {
        let keys = InitKeys {
            payer: self.payer.pubkey(),
            state: marinade_calculator_lib::program::MARINADE_CALCULATOR_STATE_ID,
            system_program: system_program::ID,
        };

        let ix = marinade_init_ix(keys)?;

        Ok(ix)
    }
}
