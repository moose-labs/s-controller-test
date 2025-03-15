use generic_pool_calculator_interface::UpdateLastUpgradeSlotKeys;
use marinade_calculator_lib::marinade_update_last_upgrade_slot_ix;
use moose_utils::result::Result;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::MarinadeCalculatorClient;

impl MarinadeCalculatorClient {
    /// required signer: initial_authority
    pub async fn get_update_last_upgrade_slot_ix(&self, manager: &Pubkey) -> Result<Instruction> {
        let keys = UpdateLastUpgradeSlotKeys {
            manager: *manager,
            state: marinade_calculator_lib::program::MARINADE_CALCULATOR_STATE_ID,
            pool_program: marinade_keys::marinade_program::ID,
            pool_program_data: marinade_keys::marinade_program_progdata::ID,
        };

        let ix = marinade_update_last_upgrade_slot_ix(keys)?;

        Ok(ix)
    }
}
