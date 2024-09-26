use anchor_lang::prelude::*;

declare_id!("7YkAh5yHbLK4uZSxjGYPsG14VUuDD6RQbK6k4k3Ji62g");
#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let mut counter = ctx.accounts.counter.load_init()?;
        counter.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        let mut counter = ctx.accounts.counter.load_mut()?;
        counter.count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + std::mem::size_of::<CounterData>())]
    pub counter: AccountLoader<'info, CounterData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: AccountLoader<'info, CounterData>,
    pub user: Signer<'info>,
}

#[account(zero_copy)]
pub struct CounterData {
    pub count: u64,
}