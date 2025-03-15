use generic_pool_calculator_interface::UpdateLastUpgradeSlotKeys;
use moose_utils::result::Result;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};
use spl_calculator_lib::spl_update_last_upgrade_slot_ix;

use super::SplCalculatorClient;

impl SplCalculatorClient {
    /// required signer: manager
    pub async fn get_update_last_upgrade_slot_ix(&self, manager: &Pubkey) -> Result<Instruction> {
        let keys = UpdateLastUpgradeSlotKeys {
            manager: *manager,
            state: spl_calculator_lib::program::SPL_CALCULATOR_STATE_ID,
            pool_program: spl_stake_pool_keys::spl_stake_pool_program::ID,
            pool_program_data: spl_stake_pool_keys::spl_stake_pool_program_progdata::ID,
        };

        let ix = spl_update_last_upgrade_slot_ix(keys)?;

        Ok(ix)
    }
}
