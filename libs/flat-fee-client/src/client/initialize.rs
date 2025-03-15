use flat_fee_interface::{initialize_ix, InitializeKeys};
use moose_utils::result::Result;
use solana_program::example_mocks::solana_sdk::system_program;
use solana_sdk::{instruction::Instruction, signer::Signer};

use super::FlatFeeClient;

impl FlatFeeClient {
    /// required signer: initial_authority
    pub async fn get_initialize_ix(&self) -> Result<Instruction> {
        let keys = InitializeKeys {
            payer: self.payer.pubkey(), // signer
            state: flat_fee_lib::program::STATE_ID,
            system_program: system_program::ID,
        };

        let ix = initialize_ix(keys)?;

        Ok(ix)
    }
}
