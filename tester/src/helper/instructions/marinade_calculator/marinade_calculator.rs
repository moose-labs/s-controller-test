use marinade_calculator_client::client::MarinadeCalculatorClient;
use moose_utils::result::Result;
use solana_sdk::{signature::Keypair, signer::Signer};

#[async_trait::async_trait]
pub trait MarinadeCalculator {
    async fn is_initialized(&self) -> Result<bool>;

    async fn update_last_upgrade_slot(&self, manager: &Keypair) -> Result<()>;

    async fn sol_to_lst(&self, amount: u64) -> Result<()>;
}

#[async_trait::async_trait]
impl MarinadeCalculator for MarinadeCalculatorClient {
    async fn is_initialized(&self) -> Result<bool> {
        let ret = self.get_calculator_state().await;

        Ok(ret.is_ok())
    }

    async fn update_last_upgrade_slot(&self, manager: &Keypair) -> Result<()> {
        let ix = self
            .get_update_last_upgrade_slot_ix(&manager.pubkey())
            .await?;

        let s = self.process_instruction(ix, &vec![manager]).await?;

        println!("update_last_upgrade_slot: {}", s.to_string());

        Ok(())
    }

    async fn sol_to_lst(&self, amount: u64) -> Result<()> {
        let ix = self.get_sol_to_lst_ix(amount).await?;

        let result = self.simulate_instruction(ix, &vec![]).await?;

        if let Some(logs) = result.logs {
            println!("sol_to_lst Log:");
            logs.iter().for_each(|l| {
                println!("{}", l);
            });
        }

        if let Some(return_data) = result.return_data {
            println!("sol_to_lst return data: {:?}", return_data.data);
        }

        if let Some(error) = result.err {
            println!("sol_to_lst error: {:?}", error);
        }

        Ok(())
    }
}
