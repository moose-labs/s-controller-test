use moose_utils::result::Result;
use s_controller_client::client::SControllerClient;
use solana_sdk::{
    program_pack::Pack, pubkey::Pubkey, rent::Rent, signature::Keypair, signer::Signer,
    system_instruction::create_account,
};
use spl_token::instruction::initialize_mint;

#[async_trait::async_trait]
pub trait CreateMint {
    async fn create_mint(&self, initial_authority_pubkey: &Pubkey, decimals: u8) -> Result<Pubkey>;
}

#[async_trait::async_trait]
impl CreateMint for SControllerClient {
    async fn create_mint(&self, initial_authority_pubkey: &Pubkey, decimals: u8) -> Result<Pubkey> {
        let mint_keypair = Keypair::new();

        let create_account_instruction = create_account(
            &self.payer.pubkey(),
            &mint_keypair.pubkey(),
            Rent::default().minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN as u64,
            &spl_token::ID,
        );

        let initialize_mint_instruction = initialize_mint(
            &spl_token::ID,
            &mint_keypair.pubkey(),
            initial_authority_pubkey,
            Some(initial_authority_pubkey),
            decimals,
        )?;

        self.process_instructions(
            &[create_account_instruction, initialize_mint_instruction],
            &vec![&mint_keypair],
        )
        .await?;

        Ok(mint_keypair.pubkey())
    }
}
