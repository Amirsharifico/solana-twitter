use anchor_lang::prelude::*;

declare_id!("Ca1Aifp4bnnNscpy7fSvucPkQ3rTzHx6vQ2GTD99kcRf");

#[program]
pub mod solana_twitter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
