use anchor_lang::prelude::*;

pub mod constants;
pub mod instructions;
pub mod state;

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

    pub fn increment5(ctx: Context<IncrementFiveUpdate>) -> Result<()> {
        instructions::increment5::increment5_handler(ctx)
    }

    pub fn decrement5(ctx: Context<DecrementFiveUpdate>) -> Result<()> {
        instructions::decrement5::decrement5_handler(ctx)
    }
}
