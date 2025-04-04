use base_client::client::Client;
use marinade_keys::{marinade_program, marinade_program_progdata};
use moose_utils::result::Result;
use s_controller_client::client::SControllerClient;
use serde_json::Value;
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey, signature::Keypair, signer::Signer};

use crate::helper::decode_u64_from_return_data;

#[async_trait::async_trait]
pub trait SController {
    async fn is_initialized(&self) -> Result<bool>;

    async fn initialize_s_controller_if_possible(
        &self,
        lp_token_mint: &Pubkey,
        initial_authority_keypair: &Keypair,
    ) -> Result<bool>;

    async fn just_initialize(&self, initial_authority_keypair: &Keypair) -> Result<Pubkey>;

    async fn set_admin_if_not_match(
        &self,
        new_admin: &Pubkey,
        initial_authority_keypair: &Keypair,
    ) -> Result<()>;

    async fn disable_pool_if_possible(&self, admin: &Keypair) -> Result<()>;

    async fn enable_pool_if_possible(&self, admin: &Keypair) -> Result<()>;

    async fn set_protocol_fee(
        &self,
        new_trading_protocol_fee_bps: Option<u16>,
        new_lp_protocol_fee_bps: Option<u16>,
        admin: &Keypair,
    ) -> Result<()>;

    async fn set_pricing_program(
        &self,
        new_pricing_program: &Pubkey,
        admin: &Keypair,
    ) -> Result<()>;

    async fn set_protocol_fee_beneficiary(
        &self,
        new_protocol_fee_beneficiary: &Pubkey,
        admin: &Keypair,
    ) -> Result<()>;

    async fn set_rebalance_authority_by_admin(
        &self,
        new_rebalance_authority: &Pubkey,
        admin: &Keypair,
    ) -> Result<()>;

    async fn set_rebalance_authority_by_authority(
        &self,
        new_rebalance_authority: &Pubkey,
        authority: &Keypair,
    ) -> Result<()>;

    async fn add_disable_pool_authority(
        &self,
        new_disable_authority: &Pubkey,
        admin: &Keypair,
    ) -> Result<()>;

    async fn remove_disable_pool_authority_by_admin(
        &self,
        disable_authority: &Pubkey,
        admin: &Keypair,
    ) -> Result<()>;

    async fn remove_disable_pool_authority_by_authority(&self, authority: &Keypair) -> Result<()>;

    async fn add_lst(
        &self,
        lst_token_mint: &Pubkey,
        lst_calculator_program: &Pubkey,
        admin: &Keypair,
    ) -> Result<()>;

    async fn remove_lst(&self, lst_token_mint: &Pubkey, admin: &Keypair) -> Result<()>;

    async fn disable_lst_input(&self, lst_token_mint: &Pubkey, admin: &Keypair) -> Result<()>;

    async fn enable_lst_input(&self, lst_token_mint: &Pubkey, admin: &Keypair) -> Result<()>;

    async fn set_sol_value_calculator(
        &self,
        lst_token_mint: &Pubkey,
        sol_value_calculator_program_id: &Pubkey,
        lst_to_sol_accounts: Vec<Pubkey>,
        admin: &Keypair,
    ) -> Result<()>;

    async fn add_liquidity(
        &self,
        lst_mint_pubkey: &Pubkey,
        src_lst_token_account: &Pubkey,
        des_lp_token_account: &Pubkey,
        lst_amount: u64,
        min_lp_out: u64,
        lst_calculator_accounts: &[AccountMeta],
        pricing_program_accounts: &[AccountMeta],
    ) -> Result<()>;

    async fn add_liquidity_simulate(
        &self,
        lst_mint_pubkey: &Pubkey,
        src_lst_token_account: &Pubkey,
        des_lp_token_account: &Pubkey,
        lst_amount: u64,
        min_lp_out: u64,
        lst_calculator_accounts: &[AccountMeta],
        pricing_program_accounts: &[AccountMeta],
    ) -> Result<Option<u64>>;

    async fn remove_liquidity(
        &self,
        lst_mint_pubkey: &Pubkey,
        src_lp_token_account: &Pubkey,
        des_lst_token_account: &Pubkey,
        lp_token_amount: u64,
        min_lst_out: u64,
        lst_calculator_accounts: &[AccountMeta],
        pricing_program_accounts: &[AccountMeta],
    ) -> Result<()>;

    async fn remove_liquidity_simulate(
        &self,
        lst_mint_pubkey: &Pubkey,
        src_lp_token_account: &Pubkey,
        des_lst_token_account: &Pubkey,
        lp_token_amount: u64,
        min_lst_out: u64,
        lst_calculator_accounts: &[AccountMeta],
        pricing_program_accounts: &[AccountMeta],
    ) -> Result<Option<u64>>;
}

#[async_trait::async_trait]
impl SController for SControllerClient {
    async fn is_initialized(&self) -> Result<bool> {
        let ret = self.get_pool_state().await;

        Ok(ret.is_ok())
    }

    async fn initialize_s_controller_if_possible(
        &self,
        lp_token_mint: &Pubkey,
        initial_authority_keypair: &Keypair,
    ) -> Result<bool> {
        if self.is_initialized().await? {
            // return lp_token_mint when initialize
            println!("Pool already initialized");
            return Ok(false);
        }

        let ix = self.get_initialize_ix(&lp_token_mint).await?;
        let s = self
            .process_instruction(ix, &vec![initial_authority_keypair])
            .await?;

        println!("initialize: {}", s.to_string());

        Ok(true)
    }

    async fn just_initialize(&self, initial_authority_keypair: &Keypair) -> Result<Pubkey> {
        let lp_token_mint = self
            .create_mint(&initial_authority_keypair.pubkey(), 9)
            .await?;

        let ix = self.get_initialize_ix(&lp_token_mint).await?;
        let s = self
            .process_instruction(ix, &vec![initial_authority_keypair])
            .await?;

        println!("initialize: {}", s.to_string());

        Ok(lp_token_mint)
    }

    async fn set_admin_if_not_match(
        &self,
        new_admin: &Pubkey,
        initial_authority_keypair: &Keypair,
    ) -> Result<()> {
        let pool_state = self.get_pool_state().await?;

        if pool_state.admin.eq(new_admin) {
            println!("Set admin already set");
            return Ok(());
        }

        let ix = self.get_set_admin_ix(new_admin).await?;
        let s = self
            .process_instruction(ix, &vec![initial_authority_keypair])
            .await?;

        println!("set_admin: {}", s.to_string());

        Ok(())
    }

    async fn disable_pool_if_possible(&self, admin: &Keypair) -> Result<()> {
        let pool_state = self.get_pool_state().await?;

        if pool_state.is_disabled != 0 {
            println!("Pool already disabled");
            return Ok(());
        }

        let ix = self.get_disable_pool_ix().await?;
        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("disable_pool: {}", s.to_string());

        Ok(())
    }

    async fn enable_pool_if_possible(&self, admin: &Keypair) -> Result<()> {
        let pool_state = self.get_pool_state().await?;

        if pool_state.is_disabled == 0 {
            println!("Pool already enabled");
            return Ok(());
        }

        let ix = self.get_enable_pool_ix().await?;
        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("enable_pool: {}", s.to_string());

        Ok(())
    }

    async fn set_protocol_fee(
        &self,
        new_trading_protocol_fee_bps: Option<u16>,
        new_lp_protocol_fee_bps: Option<u16>,
        admin: &Keypair,
    ) -> Result<()> {
        let ix = self
            .get_set_protocol_fee_ix(new_trading_protocol_fee_bps, new_lp_protocol_fee_bps)
            .await?;
        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("set_protocol_fee: {}", s.to_string());

        Ok(())
    }

    async fn set_pricing_program(
        &self,
        new_pricing_program: &Pubkey,
        admin: &Keypair,
    ) -> Result<()> {
        let ix = self.get_set_pricing_program_ix(new_pricing_program).await?;
        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("set_pricing_program: {}", s.to_string());

        Ok(())
    }

    async fn set_protocol_fee_beneficiary(
        &self,
        new_protocol_fee_beneficiary: &Pubkey,
        admin: &Keypair,
    ) -> Result<()> {
        let ix = self
            .get_set_protocol_fee_beneficiary_ix(new_protocol_fee_beneficiary)
            .await?;
        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("set_protocol_fee_beneficiary: {}", s.to_string());

        Ok(())
    }

    async fn set_rebalance_authority_by_admin(
        &self,
        new_rebalance_authority: &Pubkey,
        admin: &Keypair,
    ) -> Result<()> {
        let ix = self
            .get_set_rebalance_authority_by_admin_ix(new_rebalance_authority)
            .await?;
        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("set_protocol_fee_beneficiary: {}", s.to_string());

        Ok(())
    }

    async fn set_rebalance_authority_by_authority(
        &self,
        new_rebalance_authority: &Pubkey,
        current_authority: &Keypair,
    ) -> Result<()> {
        let ix = self
            .get_set_rebalance_authority_ix(new_rebalance_authority)
            .await?;
        let s = self
            .process_instruction(ix, &vec![current_authority])
            .await?;

        println!("set_protocol_fee_beneficiary: {}", s.to_string());

        Ok(())
    }

    async fn add_disable_pool_authority(
        &self,
        new_disable_authority: &Pubkey,
        admin: &Keypair,
    ) -> Result<()> {
        let ix = self
            .get_add_disable_pool_authority_ix(new_disable_authority)
            .await?;
        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("add_disable_pool_authority: {}", s.to_string());

        Ok(())
    }

    async fn remove_disable_pool_authority_by_admin(
        &self,
        disable_authority: &Pubkey,
        admin: &Keypair,
    ) -> Result<()> {
        let ix = self
            .get_remove_disable_pool_authority_ix(disable_authority)
            .await?;
        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("remove_disable_pool_authority: {}", s.to_string());

        Ok(())
    }

    async fn remove_disable_pool_authority_by_authority(&self, authority: &Keypair) -> Result<()> {
        let ix = self
            .get_remove_disable_pool_authority_by_authority_ix(&authority.pubkey())
            .await?;
        let s = self.process_instruction(ix, &vec![authority]).await?;

        println!("remove_disable_pool_authority: {}", s.to_string());

        Ok(())
    }

    async fn add_lst(
        &self,
        lst_token_mint: &Pubkey,
        lst_calculator_program: &Pubkey,
        admin: &Keypair,
    ) -> Result<()> {
        let ix = self
            .get_add_lst_ix(lst_token_mint, lst_calculator_program)
            .await?;
        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("add_lst: {}", s.to_string());

        Ok(())
    }

    async fn remove_lst(&self, lst_token_mint: &Pubkey, admin: &Keypair) -> Result<()> {
        let ix = self.get_remove_lst_ix(lst_token_mint).await?;
        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("remove_lst: {}", s.to_string());

        Ok(())
    }

    async fn disable_lst_input(&self, lst_token_mint: &Pubkey, admin: &Keypair) -> Result<()> {
        let ix = self.get_disable_lst_input_ix(lst_token_mint).await?;
        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("disable_lst_input: {}", s.to_string());

        Ok(())
    }

    async fn enable_lst_input(&self, lst_token_mint: &Pubkey, admin: &Keypair) -> Result<()> {
        let ix = self.get_enable_lst_input_ix(lst_token_mint).await?;
        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("enable_lst_input: {}", s.to_string());

        Ok(())
    }

    async fn set_sol_value_calculator(
        &self,
        lst_token_mint: &Pubkey,
        sol_value_calculator_program_id: &Pubkey,
        lst_to_sol_accounts: Vec<Pubkey>,
        admin: &Keypair,
    ) -> Result<()> {
        let ix = self
            .get_set_sol_value_calculator_ix(
                lst_token_mint,
                sol_value_calculator_program_id,
                lst_to_sol_accounts,
            )
            .await?;

        let s = self.process_instruction(ix, &vec![admin]).await?;

        println!("set_sol_value_calculator: {}", s.to_string());

        Ok(())
    }

    async fn add_liquidity(
        &self,

        lst_mint_pubkey: &Pubkey,
        src_lst_token_account: &Pubkey,
        des_lp_token_account: &Pubkey,
        lst_amount: u64,
        min_lp_out: u64,
        lst_calculator_accounts: &[AccountMeta],
        pricing_program_accounts: &[AccountMeta],
    ) -> Result<()> {
        let ix = self
            .get_add_liquidity_ix(
                &self.payer().pubkey(),
                lst_mint_pubkey,
                src_lst_token_account,
                des_lp_token_account,
                lst_amount,
                min_lp_out,
                lst_calculator_accounts,
                pricing_program_accounts,
            )
            .await?;

        let s = self.process_instruction(ix, &vec![]).await?;

        println!("add_liquidity: {}", s.to_string());

        Ok(())
    }

    async fn add_liquidity_simulate(
        &self,
        lst_mint_pubkey: &Pubkey,
        src_lst_token_account: &Pubkey,
        des_lp_token_account: &Pubkey,
        lst_amount: u64,
        min_lp_out: u64,
        lst_calculator_accounts: &[AccountMeta],
        pricing_program_accounts: &[AccountMeta],
    ) -> Result<Option<u64>> {
        let ix = self
            .get_add_liquidity_ix(
                &self.payer().pubkey(),
                lst_mint_pubkey,
                src_lst_token_account,
                des_lp_token_account,
                lst_amount,
                min_lp_out,
                lst_calculator_accounts,
                pricing_program_accounts,
            )
            .await?;

        let result = self.simulate_instruction(ix, &vec![]).await?;

        if let Some(logs) = result.logs {
            println!("add_liquidity Log:");
            logs.iter().for_each(|l| {
                println!("♦️ {}", l);
            });
        }

        let ret_value = if let Some(inner_instructions) = result.inner_instructions {
            let pool_state = self.get_pool_state().await?;
            let lp_mint_str = pool_state.lp_token_mint.to_string();
            let mut mint_amount: Option<u64> = None;
            for txn in inner_instructions {
                for ix in txn.instructions {
                    // Convert the instruction into a serde_json::Value.
                    let ix_value: Value = serde_json::to_value(&ix).unwrap();
                    // Check if this is a mintTo instruction from the spl-token program.
                    if let Some(parsed) = ix_value.get("parsed") {
                        if parsed.get("type").and_then(|t| t.as_str()) == Some("mintTo") {
                            if let Some(info) = parsed.get("info") {
                                // Check if the mint address in the instruction matches lst_mint_pubkey.
                                if info.get("mint").and_then(|m| m.as_str()) == Some(&lp_mint_str) {
                                    if let Some(amount_str) =
                                        info.get("amount").and_then(|a| a.as_str())
                                    {
                                        match amount_str.parse::<u64>() {
                                            Ok(amount) => {
                                                println!(
                                                    "Found mintTo for mint {}: minted amount is {}",
                                                    lp_mint_str, amount
                                                );

                                                mint_amount = Some(amount);
                                            }
                                            Err(e) => {
                                                eprintln!("Error parsing minted amount: {}", e);
                                            }
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
            }
            mint_amount
        } else {
            None
        };

        if let Some(accounts) = result.accounts {
            println!("add_liquidity account:");
            accounts.iter().for_each(|a| {
                if let Some(acc) = a {
                    println!("♦️ {:?}", acc);
                }
            });
        }

        if let Some(error) = result.err {
            println!("add_liquidity error: {:?}", error);
        }

        Ok(ret_value)
    }

    async fn remove_liquidity(
        &self,
        lst_mint_pubkey: &Pubkey,
        src_lp_token_account: &Pubkey,
        des_lst_token_account: &Pubkey,
        lp_token_amount: u64,
        min_lst_out: u64,
        lst_calculator_accounts: &[AccountMeta],
        pricing_program_accounts: &[AccountMeta],
    ) -> Result<()> {
        let ix = self
            .get_remove_liquidity_ix(
                &self.payer().pubkey(),
                lst_mint_pubkey,
                src_lp_token_account,
                des_lst_token_account,
                lp_token_amount,
                min_lst_out,
                lst_calculator_accounts,
                pricing_program_accounts,
            )
            .await?;

        let s = self.process_instruction(ix, &vec![]).await?;

        println!("remove_liquidity: {}", s.to_string());

        Ok(())
    }

    async fn remove_liquidity_simulate(
        &self,
        lst_mint_pubkey: &Pubkey,
        src_lp_token_account: &Pubkey,
        des_lst_token_account: &Pubkey,
        lp_token_amount: u64,
        min_lst_out: u64,
        lst_calculator_accounts: &[AccountMeta],
        pricing_program_accounts: &[AccountMeta],
    ) -> Result<Option<u64>> {
        let ix = self
            .get_remove_liquidity_ix(
                &self.payer().pubkey(),
                lst_mint_pubkey,
                src_lp_token_account,
                des_lst_token_account,
                lp_token_amount,
                min_lst_out,
                lst_calculator_accounts,
                pricing_program_accounts,
            )
            .await?;

        let result = self.simulate_instruction(ix, &vec![]).await?;

        if let Some(logs) = result.logs {
            println!("remove_liquidity Log:");
            logs.iter().for_each(|l| {
                println!("♦️ {}", l);
            });
        }

        let ret_value = if let Some(inner_instructions) = result.inner_instructions {
            let pool_state = self.get_pool_state().await?;
            let lp_mint_str = pool_state.lp_token_mint.to_string();
            let mut burn_amount: Option<u64> = None;
            for txn in inner_instructions {
                for ix in txn.instructions {
                    // Convert the instruction into a serde_json::Value.
                    let ix_value: Value = serde_json::to_value(&ix).unwrap();
                    // Check if this is a mintTo instruction from the spl-token program.
                    if let Some(parsed) = ix_value.get("parsed") {
                        if parsed.get("type").and_then(|t| t.as_str()) == Some("burn") {
                            if let Some(info) = parsed.get("info") {
                                // Check if the mint address in the instruction matches lst_mint_pubkey.
                                if info.get("mint").and_then(|m| m.as_str()) == Some(&lp_mint_str) {
                                    if let Some(amount_str) =
                                        info.get("amount").and_then(|a| a.as_str())
                                    {
                                        match amount_str.parse::<u64>() {
                                            Ok(amount) => {
                                                println!(
                                                    "Found burn for mint {}: burnt amount is {}",
                                                    lp_mint_str, amount
                                                );

                                                burn_amount = Some(amount);
                                            }
                                            Err(e) => {
                                                eprintln!("Error parsing burnt amount: {}", e);
                                            }
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
            }
            burn_amount
        } else {
            None
        };

        if let Some(error) = result.err {
            println!("remove_liquidity error: {:?}", error);
        }

        Ok(ret_value)
    }
}
