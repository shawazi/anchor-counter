use crate::state::Counter;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct IncrementFiveUpdate<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

pub fn increment5_handler(ctx: Context<IncrementFiveUpdate>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    let previous_count = counter.count;
    counter.count += 5;
    msg!(
        "Counter incremented. Previous count: {}; New count: {}.",
        previous_count,
        counter.count
    );
    Ok(())
}
