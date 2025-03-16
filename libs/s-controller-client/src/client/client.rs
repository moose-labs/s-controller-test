use std::time::Duration;

use base_client::client::Client;
use borsh::BorshDeserialize;
use s_controller_interface::{LstState, PoolState};
use s_controller_lib::{try_disable_pool_authority_list, try_lst_state_list};
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

pub struct SControllerClient {
    rpc_client: RpcClient,
    pub payer: Keypair,
}

impl Client for SControllerClient {
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

impl SControllerClient {
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

    pub async fn get_disable_pool_authority_list(&self) -> Result<Vec<Pubkey>> {
        let account = self
            .get_account(&&self.get_disable_pool_authority_list_pubkey())
            .await?;
        let disable_pool_authority_list = try_disable_pool_authority_list(&account.data)?;

        // Convert the slice to a Vec
        Ok(disable_pool_authority_list.to_vec())
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
