use anchor_lang::prelude::*;

declare_id!("3FJig81MorewpefvAyhEHJppLPDikSqnyQibGw7Cyf1B");

#[program]
pub mod starter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
