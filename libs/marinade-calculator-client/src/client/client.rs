use std::time::Duration;

use base_client::client::Client;
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

pub struct MarinadeCalculatorClient {
    rpc_client: RpcClient,
    pub payer: Keypair,
}

impl Client for MarinadeCalculatorClient {
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

impl MarinadeCalculatorClient {
    pub async fn get_calculator_state(&self) -> Result<CalculatorState> {
        let program_state = self
            .get_borsh_account_data(&self.get_marinade_calculator_state_pubkey())
            .await?;

        Ok(program_state)
    }

    pub fn get_program_id(&self) -> Pubkey {
        marinade_calculator_lib::program::ID
    }

    pub fn get_marinade_calculator_state_pubkey(&self) -> Pubkey {
        marinade_calculator_lib::program::MARINADE_CALCULATOR_STATE_ID
    }
}
