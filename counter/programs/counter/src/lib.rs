use anchor_lang::prelude::*;
 
declare_id!("37oUa3WkeqwnFxSCqyMnpC3CfTSwtvyJxnwYQc3u6U7C");

#[program]
pub mod counter {
    use super::*;
 
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        Ok(())
    }
 
    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        //Not doing checked_add, wrapping add or any overflow checks
        //to keep it simple
        counter.count += 1;
        Ok(())
    }
}
 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
 
#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}
 
#[account]
pub struct Counter {
    pub count: u64,
}