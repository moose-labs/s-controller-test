use flat_fee_client::client::FlatFeeClient;
use moose_utils::result::Result;

#[async_trait::async_trait]
pub trait FlatFee {
    async fn is_initialized(&self) -> Result<bool>;

    async fn initialize_if_possible(&self) -> Result<bool>;

    async fn initialize(&self) -> Result<()>;
}

#[async_trait::async_trait]
impl FlatFee for FlatFeeClient {
    async fn is_initialized(&self) -> Result<bool> {
        let ret = self.get_program_state().await;

        Ok(ret.is_ok())
    }

    async fn initialize_if_possible(&self) -> Result<bool> {
        if self.is_initialized().await? {
            // return lp_token_mint when initialize
            println!("FlatFee already initialized");
            return Ok(false);
        }

        let ix = self.get_initialize_ix().await?;
        let s = self.process_instruction(ix, &vec![]).await?;

        println!("initialize: {}", s.to_string());

        Ok(true)
    }

    async fn initialize(&self) -> Result<()> {
        let ix = self.get_initialize_ix().await?;
        let s = self.process_instruction(ix, &vec![]).await?;

        println!("initialize: {}", s.to_string());

        Ok(())
    }
}
