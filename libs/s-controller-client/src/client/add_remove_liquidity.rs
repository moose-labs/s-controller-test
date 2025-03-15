use moose_utils::result::Result;
use s_controller_lib::{
    add_liquidity_ix_by_mint_full, remove_liquidity_ix_by_mint_full, AddLiquidityByMintFreeArgs,
    AddLiquidityIxAmts, AddRemoveLiquidityAccountSuffixes, RemoveLiquidityByMintFreeArgs,
    RemoveLiquidityIxAmts,
};
use solana_readonly_account::sdk::KeyedAccount;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

use super::SControllerClient;

impl SControllerClient {
    /// required signer: liquidity_provider
    pub async fn get_add_liquidity_ix(
        &self,
        liquidity_provider: &Pubkey,
        lst_mint_pubkey: &Pubkey,
        src_lst_token_account: &Pubkey,
        des_lp_token_account: &Pubkey,
        lst_amount: u64,
        min_lp_out: u64,
        lst_calculator_accounts: &[AccountMeta],
        pricing_program_accounts: &[AccountMeta],
    ) -> Result<Instruction> {
        let pool_state_acc = self.get_account(&self.get_pool_state_pubkey()).await?;
        let lst_state_list_acc = self.get_account(&self.get_lst_state_list_pubkey()).await?;
        let lst_mint_acc = self.get_account(lst_mint_pubkey).await?;

        let args = AddLiquidityByMintFreeArgs {
            signer: *liquidity_provider,
            src_lst_acc: *src_lst_token_account,
            dst_lp_acc: *des_lp_token_account,
            pool_state: pool_state_acc,
            lst_state_list: lst_state_list_acc,
            lst_mint: KeyedAccount {
                pubkey: *lst_mint_pubkey,
                account: lst_mint_acc,
            },
        };

        let amts = AddLiquidityIxAmts {
            lst_amount,
            min_lp_out,
        };

        let account_suffixes = AddRemoveLiquidityAccountSuffixes {
            lst_calculator_accounts,
            pricing_program_price_lp_accounts: pricing_program_accounts,
        };

        let add_liquidity_instruction =
            add_liquidity_ix_by_mint_full(args, amts, account_suffixes)?;

        Ok(add_liquidity_instruction)
    }

    /// required signer: liquidity_provider
    pub async fn get_remove_liquidity_ix(
        &self,
        liquidity_provider: &Pubkey,
        lst_mint_pubkey: &Pubkey,
        src_lp_token_account: &Pubkey,
        des_lst_token_account: &Pubkey,
        lp_token_amount: u64,
        min_lst_out: u64,
        lst_calculator_accounts: &[AccountMeta],
        pricing_program_accounts: &[AccountMeta],
    ) -> Result<Instruction> {
        let pool_state_acc = self.get_account(&self.get_pool_state_pubkey()).await?;
        let lst_state_list_acc = self.get_account(&self.get_lst_state_list_pubkey()).await?;
        let lst_mint_acc = self.get_account(lst_mint_pubkey).await?;

        let args = RemoveLiquidityByMintFreeArgs {
            signer: *liquidity_provider,
            src_lp_acc: *src_lp_token_account,
            dst_lst_acc: *des_lst_token_account,
            pool_state: pool_state_acc,
            lst_state_list: lst_state_list_acc,
            lst_mint: KeyedAccount {
                pubkey: *lst_mint_pubkey,
                account: lst_mint_acc,
            },
        };

        let amts = RemoveLiquidityIxAmts {
            lp_token_amount,
            min_lst_out,
        };

        let account_suffixes = AddRemoveLiquidityAccountSuffixes {
            lst_calculator_accounts,
            pricing_program_price_lp_accounts: pricing_program_accounts,
        };

        let remove_liquidity_instruction =
            remove_liquidity_ix_by_mint_full(args, amts, account_suffixes)?;

        Ok(remove_liquidity_instruction)
    }
}
