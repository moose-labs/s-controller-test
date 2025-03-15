use lido_calculator_client::client::LidoCalculatorClient;
use moose_utils::result::Result;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::helper::decode_u64_from_return_data;

#[async_trait::async_trait]
pub trait LidoCalculator {
    async fn is_initialized(&self) -> Result<bool>;

    async fn init_if_possible(&self) -> Result<bool>;

    async fn update_last_upgrade_slot(&self, manager: &Keypair) -> Result<()>;

    async fn set_manager(&self, new_manager: &Pubkey, manager: &Keypair) -> Result<()>;

    async fn sol_to_lst(&self, amount: u64) -> Result<Option<u64>>;

    async fn lst_to_sol(&self, amount: u64) -> Result<Option<u64>>;
}

#[async_trait::async_trait]
impl LidoCalculator for LidoCalculatorClient {
    async fn is_initialized(&self) -> Result<bool> {
        let ret = self.get_calculator_state().await;

        Ok(ret.is_ok())
    }

    async fn init_if_possible(&self) -> Result<bool> {
        if self.is_initialized().await? {
            // return lp_token_mint when initialize
            println!("Lido calculator already initialized");
            return Ok(false);
        }

        let ix = self.get_init_ix().await?;

        let s = self.process_instruction(ix, &vec![]).await?;

        println!("init: {}", s.to_string());

        Ok(true)
    }

    async fn update_last_upgrade_slot(&self, manager: &Keypair) -> Result<()> {
        let ix = self
            .get_update_last_upgrade_slot_ix(&manager.pubkey())
            .await?;

        let s = self.process_instruction(ix, &vec![manager]).await?;

        println!("update_last_upgrade_slot: {}", s.to_string());

        Ok(())
    }

    async fn set_manager(&self, new_manager: &Pubkey, manager: &Keypair) -> Result<()> {
        let ix = self.get_set_manager_ix(new_manager).await?;

        let s = self.process_instruction(ix, &vec![manager]).await?;

        println!("set_manager: {}", s.to_string());

        Ok(())
    }

    async fn sol_to_lst(&self, amount: u64) -> Result<Option<u64>> {
        let ix = self.get_sol_to_lst_ix(amount).await?;

        let result = self.simulate_instruction(ix, &vec![]).await?;

        if let Some(logs) = result.logs {
            println!("sol_to_lst Log:");
            logs.iter().for_each(|l| {
                println!("{}", l);
            });
        }

        let ret_value = {
            if let Some(return_data) = result.return_data {
                let decoded_value = decode_u64_from_return_data(return_data.data.0.as_str());
                println!(
                    "sol_to_lst return data: {:?}, {}",
                    return_data.data, decoded_value
                );
                Some(decoded_value)
            } else {
                None
            }
        };

        if let Some(error) = result.err {
            println!("sol_to_lst error: {:?}", error);
        }

        Ok(ret_value)
    }

    async fn lst_to_sol(&self, amount: u64) -> Result<Option<u64>> {
        let ix = self.get_lst_to_sol_ix(amount).await?;

        let result = self.simulate_instruction(ix, &vec![]).await?;

        if let Some(logs) = result.logs {
            println!("lst_to_sol Log:");
            logs.iter().for_each(|l| {
                println!("{}", l);
            });
        }

        let ret_value = {
            if let Some(return_data) = result.return_data {
                let decoded_value = decode_u64_from_return_data(return_data.data.0.as_str());
                println!(
                    "lst_to_sol return data: {:?}, {}",
                    return_data.data, decoded_value
                );
                Some(decoded_value)
            } else {
                None
            }
        };
        if let Some(error) = result.err {
            println!("lst_to_sol error: {:?}", error);
        }

        Ok(ret_value)
    }
}
