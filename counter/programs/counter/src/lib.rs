use anchor_lang::prelude::*;

declare_id!("CNnYwPLV9GYNewi1BEz7gRX5Qh3f2EXi17o1U8GguNGm");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
