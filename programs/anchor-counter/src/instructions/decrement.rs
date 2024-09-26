use crate::state::Counter;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DecrementUpdate<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

pub fn decrement_handler(ctx: Context<DecrementUpdate>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    let previous_count = counter.count;
    counter.count -= 1;
    msg!(
        "Counter decremented. Previous count: {}; New count: {}.",
        previous_count,
        counter.count
    );
    Ok(())
}
