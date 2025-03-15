use moose_utils::result::Result;
use s_controller_client::client::SControllerClient;
use solana_sdk::{pubkey::Pubkey, signer::Signer};
use spl_associated_token_account::{
    get_associated_token_address, instruction::create_associated_token_account,
};

#[async_trait::async_trait]
pub trait CreateAta {
    async fn create_ata(&mut self, mint: &Pubkey, wallet_address: &Pubkey) -> Result<Pubkey>;
}

#[async_trait::async_trait]
impl CreateAta for SControllerClient {
    async fn create_ata(&mut self, mint: &Pubkey, wallet_address: &Pubkey) -> Result<Pubkey> {
        let token_program_id = self.get_account_owner(mint).await?;
        // Derive the associated token account address
        let ata_pubkey = get_associated_token_address(wallet_address, mint);

        // Create an instruction to create the ATA
        let create_ata_instruction = create_associated_token_account(
            &self.payer.pubkey(),
            wallet_address, // Owner of the ATA
            mint,           // Mint of the token
            &token_program_id,
        );

        // Process the instruction
        self.process_instructions(&[create_ata_instruction], &vec![])
            .await?;

        Ok(ata_pubkey) // Return the derived ATA address
    }
}
