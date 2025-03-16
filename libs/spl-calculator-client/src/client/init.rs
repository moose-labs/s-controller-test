use base_client::client::Client;
use generic_pool_calculator_interface::InitKeys;
use moose_utils::result::Result;
use solana_sdk::{instruction::Instruction, signer::Signer, system_program};
use spl_calculator_lib::spl_init_ix;

use super::SplCalculatorClient;

impl SplCalculatorClient {
    /// no extra signer required
    pub async fn get_init_ix(&self) -> Result<Instruction> {
        let keys = InitKeys {
            payer: self.payer().pubkey(),
            state: spl_calculator_lib::program::SPL_CALCULATOR_STATE_ID,
            system_program: system_program::ID,
        };

        let ix = spl_init_ix(keys)?;

        Ok(ix)
    }
}
