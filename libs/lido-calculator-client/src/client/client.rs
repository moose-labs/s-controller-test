use std::time::Duration;

use borsh::BorshDeserialize;
use generic_pool_calculator_interface::CalculatorState;
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
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::VersionedTransaction,
};

use moose_utils::{result::Result, sorted_signers::SortedSigners};

pub struct LidoCalculatorClient {
    rpc_client: RpcClient,
    pub payer: Keypair,
}

impl LidoCalculatorClient {
    pub fn new(payer: Keypair, url: String, commitment_config: CommitmentConfig) -> Self {
        let timeout = Duration::from_secs(30);
        let confirm_transaction_initial_timeout = Duration::from_secs(10);

        Self {
            rpc_client: RpcClient::new_with_timeouts_and_commitment(
                url,
                timeout,
                commitment_config,
                confirm_transaction_initial_timeout,
            ),
            payer: payer.insecure_clone(),
        }
    }

    pub async fn process_instruction(
        &self,
        instruction: Instruction,
        signers: &Vec<&Keypair>,
    ) -> Result<Signature> {
        self.process_instructions(&[instruction], signers).await
    }

    pub async fn process_instructions(
        &self,
        instructions: &[Instruction],
        signers: &[&Keypair], // accept a slice for more flexibility
    ) -> Result<Signature> {
        // Create the compute budget instruction
        let compute_units_ix =
            compute_budget::ComputeBudgetInstruction::set_compute_unit_limit(2_000_000);

        // Combine the compute budget instruction with the provided instructions
        let mut all_instructions = Vec::with_capacity(instructions.len() + 1);
        all_instructions.push(compute_units_ix);
        all_instructions.extend_from_slice(instructions);

        let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;

        // Combine the payer with the provided signers if the payer is not already included.
        // Adjust this as needed if your callers already include the payer.
        let mut signers_with_payer = vec![&self.payer];
        signers_with_payer.extend_from_slice(signers);

        // Create a versioned transaction.
        let compiled_message = Message::try_compile(
            &self.payer.pubkey(),
            &all_instructions,
            &[], // TODO: add option to apply address_lookup
            recent_blockhash,
        )?;
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(compiled_message),
            &SortedSigners(&signers_with_payer), // use the combined list of signers
        )?;

        let signature = self.rpc_client.send_and_confirm_transaction(&tx).await?;
        Ok(signature)
    }

    pub async fn simulate_instruction(
        &self,
        instruction: Instruction,
        signers: &Vec<&Keypair>,
    ) -> Result<RpcSimulateTransactionResult> {
        self.simulate_instructions(&[instruction], signers).await
    }

    pub async fn simulate_instructions(
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

        let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;

        // Combine the payer with the provided signers if the payer is not already included.
        // Adjust this as needed if your callers already include the payer.
        let mut signers_with_payer = vec![&self.payer];
        signers_with_payer.extend_from_slice(signers);

        // Create a versioned transaction.
        let compiled_message = Message::try_compile(
            &self.payer.pubkey(),
            &all_instructions,
            &[], // TODO: add option to apply address_lookup
            recent_blockhash,
        )?;
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(compiled_message),
            &SortedSigners(&signers_with_payer), // use the combined list of signers
        )?;

        let response = self.rpc_client.simulate_transaction(&tx).await?;
        Ok(response.value)
    }

    pub async fn airdrop(&self, to: &Pubkey, lamports: u64) -> Result<Signature> {
        let s = self.rpc_client.request_airdrop(to, lamports).await?;
        Ok(s)
    }

    pub async fn get_balance(&self, pubkey: &Pubkey) -> Result<u64> {
        let balance = self.rpc_client.get_balance(pubkey).await?;
        Ok(balance)
    }

    pub async fn get_account(&self, pubkey: &Pubkey) -> Result<Account> {
        let account = self.rpc_client.get_account(pubkey).await?;
        Ok(account)
    }

    pub async fn get_epoch_info(&self) -> Result<EpochInfo> {
        let epoch = self.rpc_client.get_epoch_info().await?;
        Ok(epoch)
    }

    pub async fn get_account_owner(&self, pubkey: &Pubkey) -> Result<Pubkey> {
        let account = self.get_account(pubkey).await?;
        Ok(account.owner)
    }

    pub async fn get_borsh_account_data<T: BorshDeserialize>(&self, pubkey: &Pubkey) -> Result<T> {
        let account = self.get_account(pubkey).await?;

        Ok(T::deserialize(&mut &account.data[..])?)
    }

    pub async fn get_calculator_state(&self) -> Result<CalculatorState> {
        let program_state = self
            .get_borsh_account_data(&self.get_lido_calculator_state_pubkey())
            .await?;

        Ok(program_state)
    }

    pub fn get_program_id(&self) -> Pubkey {
        lido_calculator_lib::program::ID
    }

    pub fn get_lido_calculator_state_pubkey(&self) -> Pubkey {
        lido_calculator_lib::program::LIDO_CALCULATOR_STATE_ID
    }
}
