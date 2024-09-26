use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod constants;

use instructions::*;

declare_id!("5jthknB5s52QJVqV8mqvfP66qyo1mNzKMXiF4xFQPeDt");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn increment(ctx: Context<IncrementUpdate>) -> Result<()> {
        instructions::increment::increment_handler(ctx)
    }

    pub fn decrement(ctx: Context<DecrementUpdate>) -> Result<()> {
        instructions::decrement::decrement_handler(ctx)
    }
}

