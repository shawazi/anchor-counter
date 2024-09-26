use crate::state::Counter;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct DecrementFiveUpdate<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

pub fn decrement5_handler(ctx: Context<DecrementFiveUpdate>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    let previous_count = counter.count;
    counter.count -= 5;
    msg!(
        "Counter decremented. Previous count: {}; New count: {}.",
        previous_count,
        counter.count
    );
    Ok(())
}
