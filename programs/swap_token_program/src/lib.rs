#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

mod dexes;
declare_program!(dlmm);

declare_id!("3WeCKRYnyebodD1Hh3t6Gszcvjq1m9iQJ2a5kgFwFRbq");

#[program]
pub mod swap_token_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let _raydium_program_id = raydium_cpmm_cpi::ID;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
