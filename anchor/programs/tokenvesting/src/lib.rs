#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

#[program]
pub mod tokenvesting {
    use super::*;

    pub fn create_vesting_account(
      ctx: Context<CreateVestingAccount>,
      company_name: String
  ) -> Result<()> {
      
      Ok(())
  }

}

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount<'info> {
}

#[account]
#[derive(InitSpace, Debug)]
pub struct VestingAccount {
}
