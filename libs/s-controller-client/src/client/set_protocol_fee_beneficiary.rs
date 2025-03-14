use moose_utils::result::Result;
use s_controller_interface::{SetProtocolFeeBeneficiaryKeys, set_protocol_fee_beneficiary_ix};
use solana_sdk::{instruction::Instruction, pubkey::Pubkey};

use super::SControllerClient;

impl SControllerClient {
    /// required signer: current_beneficiary
    pub async fn get_set_protocol_fee_beneficiary_ix(
        &self,
        new_beneficiary: &Pubkey,
    ) -> Result<Instruction> {
        let pool_state = self.get_pool_state().await?;

        let keys = SetProtocolFeeBeneficiaryKeys {
            current_beneficiary: pool_state.protocol_fee_beneficiary,
            new_beneficiary: new_beneficiary.clone(),
            pool_state: self.get_pool_state_pubkey(),
        };

        let ix = set_protocol_fee_beneficiary_ix(keys)?;

        Ok(ix)
    }
}
