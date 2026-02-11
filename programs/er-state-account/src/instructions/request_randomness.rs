use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::anchor::vrf;
use ephemeral_vrf_sdk::instructions::{create_request_randomness_ix, RequestRandomnessParams};
use ephemeral_vrf_sdk::types::SerializableAccountMeta;

use crate::{instruction::ConsumeRandomness, state::UserAccount, ID};

#[vrf] // VRF functionality macro
#[derive(Accounts)]
pub struct RequestRandomness<'info> {
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    /// CHECK: The oracle queue
    #[account(
        mut,
        // address = ephemeral_vrf_sdk::consts::DEFAULT_QUEUE,
    )]
    pub oracle_queue: AccountInfo<'info>,
}

impl<'info> RequestRandomness<'info> {
    // Request randomness from the VRF oracle
    pub fn request(&mut self, client_seed: u8) -> Result<()> {
        // Create the VRF request instruction
        let ix = create_request_randomness_ix(RequestRandomnessParams {
            payer: self.user.key(),
            oracle_queue: self.oracle_queue.key(),
            callback_program_id: ID, // Program ID for the callback
            callback_discriminator: ConsumeRandomness::DISCRIMINATOR.to_vec(), // IDs which callback function to call
            caller_seed: [client_seed; 32],

            // Accounts required by the callback function
            accounts_metas: Some(vec![SerializableAccountMeta {
                pubkey: self.user.key(),
                is_signer: false,
                is_writable: true,
            }]),
            ..Default::default()
        });

        // Execute the CPI to VRF program
        self.invoke_signed_vrf(&self.user.to_account_info(), &ix)?;

        Ok(())
    }
}
