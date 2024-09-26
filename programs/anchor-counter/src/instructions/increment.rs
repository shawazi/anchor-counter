use crate::state::Counter;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct IncrementUpdate<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

pub fn increment_handler(ctx: Context<IncrementUpdate>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    let previous_count = counter.count;
    counter.count += 1;
    msg!(
        "Counter incremented. Previous count: {}; New count: {}.",
        previous_count,
        counter.count
    );
    Ok(())
}
