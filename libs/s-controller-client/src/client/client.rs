use borsh::BorshDeserialize;
use s_controller_interface::{LstState, PoolState};
use s_controller_lib::try_lst_state_list;
use solana_client::nonblocking::rpc_client::RpcClient;

use solana_sdk::{
    account::Account,
    commitment_config::CommitmentConfig,
    compute_budget,
    instruction::Instruction,
    message::{VersionedMessage, v0::Message},
    pubkey::Pubkey,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::VersionedTransaction,
};

use moose_utils::result::Result;

use crate::utils::sorted_signers::SortedSigners;

pub struct SControllerClient {
    rpc_client: RpcClient,
    pub payer: Keypair,
}

impl SControllerClient {
    pub fn new(payer: Keypair, url: String, commitment_config: CommitmentConfig) -> Self {
        Self {
            rpc_client: RpcClient::new_with_commitment(url, commitment_config),
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

    pub async fn get_account(&self, pubkey: &Pubkey) -> Result<Account> {
        let account = self.rpc_client.get_account(pubkey).await?;
        Ok(account)
    }

    pub async fn get_account_owner(&self, pubkey: &Pubkey) -> Result<Pubkey> {
        let account = self.get_account(pubkey).await?;
        Ok(account.owner)
    }

    pub async fn get_borsh_account_data<T: BorshDeserialize>(&self, pubkey: &Pubkey) -> Result<T> {
        let account = self.get_account(pubkey).await?;

        Ok(T::deserialize(&mut &account.data[..])?)
    }

    pub async fn get_pool_state(&self) -> Result<PoolState> {
        let pool_state = self
            .get_borsh_account_data(&self.get_pool_state_pubkey())
            .await?;

        Ok(pool_state)
    }

    pub async fn get_lst_state_list(&self) -> Result<Vec<LstState>> {
        let account = self.get_account(&self.get_lst_state_list_pubkey()).await?;
        let lst_state_slice = try_lst_state_list(&account.data)?;

        // Convert the slice to a Vec
        Ok(lst_state_slice.to_vec())
    }

    pub fn get_program_id(&self) -> Pubkey {
        s_controller_lib::program::ID
    }

    pub fn get_pool_state_pubkey(&self) -> Pubkey {
        s_controller_lib::program::POOL_STATE_ID
    }

    pub fn get_lst_state_list_pubkey(&self) -> Pubkey {
        s_controller_lib::program::LST_STATE_LIST_ID
    }

    pub fn get_disable_pool_authority_list_pubkey(&self) -> Pubkey {
        s_controller_lib::program::DISABLE_POOL_AUTHORITY_LIST_ID
    }

    pub fn get_initial_authority_pubkey(&self) -> Pubkey {
        s_controller_lib::initial_authority::ID
    }
}
