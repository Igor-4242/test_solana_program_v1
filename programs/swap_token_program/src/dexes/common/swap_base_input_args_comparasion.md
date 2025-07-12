common_swap_base_input_arguments
   `payer` - who pays for swap
   `fee_payer` - who pays for fees
   `destination_wallet` - where to send funds
   +
   `token_mint`: Pubkey,
   `quote_mint`: Pubkey,
   `amount_in`: u64,
   `min_amount_out`: u64,


raydium
    amm_config: Box<Account<'info, AmmConfig>>,
    authority: UncheckedAccount<'info>,
    cp_swap_program: Program<'info, RaydiumCpmm>,
    `get_or_init_and_get_ata(quote_mint)` - input_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    `quote_mint` - input_token_mint: Box<InterfaceAccount<'info, Mint>>,
    input_token_program: Interface<'info, TokenInterface>,
    input_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    observation_state: AccountLoader<'info, ObservationState>,
    `get_or_init_and_get_ata(token_mint)` - output_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    `token_mint` - output_token_mint: Box<InterfaceAccount<'info, Mint>>,
    output_token_program: Interface<'info, TokenInterface>,
    `destination_wallet` - output_vault: Box<InterfaceAccount<'info, TokenAccount>>,
    `payer`: Signer<'info>,
    pool_state: AccountLoader<'info, PoolState>,
    +
    `amount_in`: u64,
    `min_amount_out`: u64,


meteora
    bin_array_bitmap_extension: Option<UncheckedAccount<'info>>,
    dlmm_program: UncheckedAccount<'info>,
    event_authority: UncheckedAccount<'info>,
    `fee_payer` - host_fee_in: Option<UncheckedAccount<'info>>,
    lb_pair: UncheckedAccount<'info>,
    oracle: UncheckedAccount<'info>,
    reserve_x: UncheckedAccount<'info>,
    reserve_y: UncheckedAccount<'info>,
    `quote_mint` - token_x_mint: UncheckedAccount<'info>,
    token_x_program: UncheckedAccount<'info>,
    `token_mint` - token_y_mint: UncheckedAccount<'info>,
    token_y_program: UncheckedAccount<'info>,
    `get_or_init_and_get_ata(quote_mint)` - user_token_in: UncheckedAccount<'info>,
    `get_or_init_and_get_ata(token_mint)` - user_token_out: UncheckedAccount<'info>,
    `payer` - user: Signer<'info>,
    +
    `amount_in`: u64,
    `min_amount_out`: u64,
