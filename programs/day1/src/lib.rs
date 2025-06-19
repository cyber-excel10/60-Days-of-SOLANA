use anchor_lang::prelude::*;

declare_id!("Csu7ycw6jzrkKym847XSL8J8ZEiWZ1o5W7j1GWhngcpf");

#[program]
pub mod day1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello World!");  // **** NEW LINE HERE ****
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
