#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
// use anchor_spl::token::{Mint, TokenAccount};

mod dexes;

declare_program!(meteora_dlmm);
// declare_program!(raydium_amm_v3);
// declare_program!(raydium_cp_swap);

declare_id!("3WeCKRYnyebodD1Hh3t6Gszcvjq1m9iQJ2a5kgFwFRbq");

#[program]
pub mod swap_token_program {
    use super::*;

    pub fn swap_token(
        ctx: Context<SwapToken>,
        token_mint: Pubkey,
        quote_mint: Pubkey,
        amount_in: u64,
        min_amount_out: u64,
    ) -> Result<()> {
        let swap_result = dexes::raydium_cp_swap_cpi::swap_base_input(
            Context::new(
                ctx.program_id,
                &mut dexes::raydium_cp_swap_cpi::SwapBaseInput::get_default(),
                // dexes::raydium_cp_swap_cpi::SwapBaseInput::derive(&mut ctx.accounts, quote_mint, token_mint)?,
                ctx.remaining_accounts,
                dexes::raydium_cp_swap_cpi::SwapBaseInputBumps::default(),
            ),
            amount_in,
            min_amount_out,
        );

        // dexes::meteora_cpi::swap_base_input(ctx, amount_in, min_amount_out)
        // dexes::raydium_cp_swap_cpi::swap_base_input(ctx, amount_in, minimum_amount_out)

        swap_result
    }
}

#[derive(Accounts)]
pub struct SwapToken<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid pool for the given mints")]
    InvalidPool,

    #[msg("Invalid token account")]
    InvalidTokenAccount,

    #[msg("Invalid vault account")]
    InvalidVault,

    #[msg("No pool found on either DEX")]
    NoPoolFound,
}
