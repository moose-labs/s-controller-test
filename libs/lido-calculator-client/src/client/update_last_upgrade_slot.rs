use generic_pool_calculator_interface::UpdateLastUpgradeSlotKeys;
use lido_calculator_lib::lido_update_last_upgrade_slot_ix;
use moose_utils::result::Result;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::LidoCalculatorClient;

impl LidoCalculatorClient {
    /// required signer: manager
    pub async fn get_update_last_upgrade_slot_ix(&self, manager: &Pubkey) -> Result<Instruction> {
        let keys = UpdateLastUpgradeSlotKeys {
            manager: *manager,
            state: lido_calculator_lib::program::LIDO_CALCULATOR_STATE_ID,
            pool_program: lido_keys::lido_program::ID,
            pool_program_data: lido_keys::lido_program_progdata::ID,
        };

        let ix = lido_update_last_upgrade_slot_ix(keys)?;

        Ok(ix)
    }
}
