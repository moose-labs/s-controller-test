use std::time::Duration;

use base_client::client::Client;
use borsh::BorshDeserialize;
use flat_fee_interface::ProgramState;
use solana_client::nonblocking::rpc_client::RpcClient;

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

pub struct FlatFeeClient {
    rpc_client: RpcClient,
    pub payer: Keypair,
}

impl Client for FlatFeeClient {
    fn rpc_client(&self) -> &RpcClient {
        &self.rpc_client
    }

    fn payer(&self) -> &Keypair {
        &self.payer
    }

    fn from_parts(rpc_client: RpcClient, payer: Keypair) -> Self {
        Self { rpc_client, payer }
    }
}

impl FlatFeeClient {
    pub async fn get_program_state(&self) -> Result<ProgramState> {
        let program_state = self
            .get_borsh_account_data(&self.get_program_state_pubkey())
            .await?;

        Ok(program_state)
    }

    pub fn get_program_id(&self) -> Pubkey {
        flat_fee_lib::program::ID
    }

    pub fn get_program_state_pubkey(&self) -> Pubkey {
        flat_fee_lib::program::STATE_ID
    }

    pub fn get_initial_manager_pubkey(&self) -> Pubkey {
        flat_fee_lib::initial_constants::initial_manager::ID
    }
}
