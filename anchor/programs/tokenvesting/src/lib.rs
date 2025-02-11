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

  pub fn create_employee_vesting(
    ctx: Context<CreateEmployeeAccount>,
    start_time: i64,
    end_time: i64,
    total_amount: i64,
    cliff_time: i64
) -> Result<()> {

    Ok(())
}

pub fn claim_tokens(ctx: Context<ClaimTokens>, _company_name: String) -> Result<()> {
 
  Ok(())
}



}

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct CreateVestingAccount<'info> {
}

#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info> {
    
}

#[derive(Accounts)]
#[instruction(company_name: String)]
pub struct ClaimTokens<'info> {
   
}

#[account]
#[derive(InitSpace, Debug)]
pub struct VestingAccount {
}

#[account]
#[derive(InitSpace, Debug)]
pub struct EmployeeAccount {
}
