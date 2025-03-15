use moose_utils::result::Result;
use s_controller_interface::{initialize_ix, InitializeKeys};
use solana_program::example_mocks::solana_sdk::system_program;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signer::Signer};

use super::SControllerClient;

impl SControllerClient {
    /// required signer: initial_authority
    /// authority of the lp_token_mint must be initial_authority
    pub async fn get_initialize_ix(&self, lp_token_mint: &Pubkey) -> Result<Instruction> {
        let lp_token_program = self.get_account_owner(lp_token_mint).await?;

        let keys = InitializeKeys {
            payer: self.payer.pubkey(),                     // signer
            authority: self.get_initial_authority_pubkey(), // signer
            pool_state: self.get_pool_state_pubkey(),
            lp_token_mint: lp_token_mint.clone(),
            lp_token_program,
            system_program: system_program::ID,
        };

        let ix = initialize_ix(keys)?;

        Ok(ix)
    }
}
