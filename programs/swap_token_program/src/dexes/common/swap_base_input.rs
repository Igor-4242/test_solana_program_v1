use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct SwapBaseInput<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub input_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub output_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub input_vault: InterfaceAccount<'info, TokenAccount>,

    #[account(mut)]
    pub output_vault: InterfaceAccount<'info, TokenAccount>,

    pub input_token_mint: InterfaceAccount<'info, Mint>,
    pub input_token_program: Interface<'info, TokenInterface>,

    pub output_token_mint: InterfaceAccount<'info, Mint>,
    pub output_token_program: Interface<'info, TokenInterface>,

    #[account(mut)]
    pub pool: AccountInfo<'info>,
}
#[error_code]
pub enum ErrorCode {
    #[msg("Not implemented")]
    NotImplemented,
}

impl<'info, 'b> TryFrom<&'b mut crate::SwapToken<'info>> for SwapBaseInput<'info> {
    type Error = ErrorCode;
    fn try_from(
        value: &'b mut crate::SwapToken<'info>,
    ) -> std::result::Result<SwapBaseInput<'info>, Self::Error> {
        Err(ErrorCode::NotImplemented)
    }
}

// pub fn swap_base_input(
//     ctx: Context<SwapBaseInput>,
//     amount_in: u64,
//     minimum_amount_out: u64,
// ) -> Result<()> {
//     todo!()
// }
