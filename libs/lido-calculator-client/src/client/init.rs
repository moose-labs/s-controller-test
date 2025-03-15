use generic_pool_calculator_interface::InitKeys;
use lido_calculator_lib::lido_init_ix;
use moose_utils::result::Result;
use solana_sdk::{instruction::Instruction, signer::Signer, system_program};

use super::LidoCalculatorClient;

impl LidoCalculatorClient {
    /// no extra signer required
    pub async fn get_init_ix(&self) -> Result<Instruction> {
        let keys = InitKeys {
            payer: self.payer.pubkey(),
            state: lido_calculator_lib::program::LIDO_CALCULATOR_STATE_ID,
            system_program: system_program::ID,
        };

        let ix = lido_init_ix(keys)?;

        Ok(ix)
    }
}
