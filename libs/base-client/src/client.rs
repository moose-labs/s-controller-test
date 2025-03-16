use std::time::Duration;

use borsh::BorshDeserialize;
use moose_utils::{result::Result, sorted_signers::SortedSigners};
use solana_client::{
    nonblocking::rpc_client::RpcClient, rpc_response::RpcSimulateTransactionResult,
};
use solana_sdk::{
    account::Account,
    commitment_config::CommitmentConfig,
    compute_budget,
    epoch_info::EpochInfo,
    instruction::Instruction,
    message::{v0::Message, VersionedMessage},
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    rent::Rent,
    signature::{Keypair, Signature},
    signer::Signer,
    system_instruction::create_account,
    transaction::VersionedTransaction,
};
use spl_associated_token_account::{
    get_associated_token_address_with_program_id, instruction::create_associated_token_account,
};
use spl_token::instruction::initialize_mint;

#[async_trait::async_trait]
pub trait Client {
    fn rpc_client(&self) -> &RpcClient;
    fn payer(&self) -> &Keypair;

    // Default constructor. This assumes the implementing type has these fields.
    fn new(payer: Keypair, url: String, commitment_config: CommitmentConfig) -> Self
    where
        Self: Sized,
    {
        let timeout = Duration::from_secs(30);
        let confirm_transaction_initial_timeout = Duration::from_secs(10);
        Self::from_parts(
            RpcClient::new_with_timeouts_and_commitment(
                url,
                timeout,
                commitment_config,
                confirm_transaction_initial_timeout,
            ),
            payer.insecure_clone(),
        )
    }

    // Helper function to create an instance from the parts.
    fn from_parts(rpc_client: RpcClient, payer: Keypair) -> Self;

    async fn process_instruction(
        &self,
        instruction: Instruction,
        signers: &Vec<&Keypair>,
    ) -> Result<Signature> {
        self.process_instructions(&[instruction], signers).await
    }

    async fn process_instructions(
        &self,
        instructions: &[Instruction],
        signers: &[&Keypair], // accept a slice for more flexibility
    ) -> Result<Signature> {
        // Create the compute budget instruction.
        let compute_units_ix =
            compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(2_000_000);

        // Combine the compute budget instruction with the provided instructions.
        let mut all_instructions = Vec::with_capacity(instructions.len() + 1);
        all_instructions.push(compute_units_ix);
        all_instructions.extend_from_slice(instructions);

        let recent_blockhash = self.rpc_client().get_latest_blockhash().await?;

        // Combine the payer with the provided signers.
        let mut signers_with_payer = vec![self.payer()];
        signers_with_payer.extend_from_slice(signers);

        // Create a versioned transaction.
        let compiled_message = Message::try_compile(
            &self.payer().pubkey(),
            &all_instructions,
            &[], // TODO: add option to apply address_lookup
            recent_blockhash,
        )?;
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(compiled_message),
            &SortedSigners(&signers_with_payer),
        )?;

        let signature = self.rpc_client().send_and_confirm_transaction(&tx).await?;
        Ok(signature)
    }

    async fn simulate_instruction(
        &self,
        instruction: Instruction,
        signers: &Vec<&Keypair>,
    ) -> Result<RpcSimulateTransactionResult> {
        self.simulate_instructions(&[instruction], signers).await
    }

    async fn simulate_instructions(
        &self,
        instructions: &[Instruction],
        signers: &[&Keypair], // accept a slice for more flexibility
    ) -> Result<RpcSimulateTransactionResult> {
        // Create the compute budget instruction
        let compute_units_ix =
            compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(2_000_000);

        // Combine the compute budget instruction with the provided instructions
        let mut all_instructions = Vec::with_capacity(instructions.len() + 1);
        all_instructions.push(compute_units_ix);
        all_instructions.extend_from_slice(instructions);

        let recent_blockhash = self.rpc_client().get_latest_blockhash().await?;

        // Combine the payer with the provided signers if the payer is not already included.
        // Adjust this as needed if your callers already include the payer.
        let mut signers_with_payer = vec![self.payer()];
        signers_with_payer.extend_from_slice(signers);

        // Create a versioned transaction.
        let compiled_message = Message::try_compile(
            &self.payer().pubkey(),
            &all_instructions,
            &[], // TODO: add option to apply address_lookup
            recent_blockhash,
        )?;
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(compiled_message),
            &SortedSigners(&signers_with_payer), // use the combined list of signers
        )?;

        let response = self.rpc_client().simulate_transaction(&tx).await?;
        Ok(response.value)
    }

    async fn get_ata(&self, mint: &Pubkey, wallet_address: &Pubkey) -> Result<Pubkey> {
        let token_program_id = self.get_account_owner(mint).await?;
        let ata_pubkey =
            get_associated_token_address_with_program_id(wallet_address, mint, &token_program_id);

        Ok(ata_pubkey)
    }

    async fn create_ata(&self, mint: &Pubkey, wallet_address: &Pubkey) -> Result<Pubkey> {
        let token_program_id = self.get_account_owner(mint).await?;
        // Derive the associated token account address
        let ata_pubkey =
            get_associated_token_address_with_program_id(wallet_address, mint, &token_program_id);

        // Create an instruction to create the ATA
        let create_ata_instruction = create_associated_token_account(
            &self.payer().pubkey(),
            wallet_address, // Owner of the ATA
            mint,           // Mint of the token
            &token_program_id,
        );

        // Process the instruction
        self.process_instructions(&[create_ata_instruction], &vec![])
            .await?;

        Ok(ata_pubkey) // Return the derived ATA address
    }

    async fn create_mint(&self, authority: &Pubkey, decimals: u8) -> Result<Pubkey> {
        let mint_keypair = Keypair::new();

        let create_account_instruction = create_account(
            &self.payer().pubkey(),
            &mint_keypair.pubkey(),
            Rent::default().minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN as u64,
            &spl_token::ID,
        );

        let initialize_mint_instruction = initialize_mint(
            &spl_token::ID,
            &mint_keypair.pubkey(),
            authority,
            Some(authority),
            decimals,
        )?;

        self.process_instructions(
            &[create_account_instruction, initialize_mint_instruction],
            &vec![&mint_keypair],
        )
        .await?;

        Ok(mint_keypair.pubkey())
    }

    async fn airdrop(&self, to: &Pubkey, lamports: u64) -> Result<Signature> {
        let s = self.rpc_client().request_airdrop(to, lamports).await?;
        Ok(s)
    }

    async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        let balance = self.rpc_client().get_balance(pubkey).await?;
        Ok(balance)
    }

    async fn get_account(&self, pubkey: &Pubkey) -> Result<Account> {
        let account = self.rpc_client().get_account(pubkey).await?;
        Ok(account)
    }

    async fn get_epoch_info(&self) -> Result<EpochInfo> {
        let epoch = self.rpc_client().get_epoch_info().await?;
        Ok(epoch)
    }

    async fn get_account_owner(&self, pubkey: &Pubkey) -> Result<Pubkey> {
        let account = self.get_account(pubkey).await?;
        Ok(account.owner)
    }

    async fn get_borsh_account_data<T: BorshDeserialize>(&self, pubkey: &Pubkey) -> Result<T> {
        let account = self.get_account(pubkey).await?;

        Ok(T::deserialize(&mut &account.data[..])?)
    }

    async fn get_packed_account_data<T: Pack + IsInitialized>(&self, pubkey: &Pubkey) -> Result<T> {
        let account = self.get_account(pubkey).await?;

        let ret = T::unpack(&account.data[..])?;

        Ok(ret)
    }

    async fn get_token_account_balance(&self, token_account: &Pubkey) -> Result<u64> {
        let account: spl_token::state::Account =
            self.get_packed_account_data(token_account).await?;

        Ok(account.amount)
    }
}
