use crate::state::UserAccount;
use anchor_lang::prelude::*;
use ephemeral_vrf_sdk::rnd::random_u64;

#[derive(Accounts)]
pub struct ConsumeRandomness<'info> {
    /// This check ensure that the vrf_program_identity (which is a PDA) is a singer
    /// enforcing the callback is executed by the VRF program trough CPI
    #[account(address = ephemeral_vrf_sdk::consts::VRF_PROGRAM_IDENTITY)]
    pub vrf_program_identity: Signer<'info>,
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
}

impl<'info> ConsumeRandomness<'info> {
    pub fn consume(&mut self, randomness: [u8; 32]) -> Result<()> {
        let random_value = random_u64(&randomness);
        let user_account = &mut self.user_account;
        user_account.data = random_value; // Update the user account with the random value
        Ok(())
    }
}
