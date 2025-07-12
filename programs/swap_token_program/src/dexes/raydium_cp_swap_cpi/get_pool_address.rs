use anchor_lang::prelude::*;

// Raydium AMM v4 program ID
// https://solscan.io/account/675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
const RAYDIUM_PROGRAM_ID: &Pubkey = &pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

pub fn get_pool_address<'info>(token_a: Pubkey, token_b: Pubkey) -> Result<Option<Pubkey>> {
    // let (token_0, token_1) = if token_a < token_b {
    //     check_address(token_a, token_b)
    // } else {
    //     check_address(token_b, token_a)
    // };

    match (
        check_address(token_b, token_a),
        check_address(token_a, token_b),
    ) {
        (Ok(Some(pool_address)), Ok(None)) => Ok(Some(pool_address)),
        (Ok(None), Ok(Some(pool_address))) => Ok(Some(pool_address)),
        _ => Ok(None),
    }
}

fn check_address(token_0: Pubkey, token_1: Pubkey) -> Result<Option<Pubkey>> {
    let (pool_address, _bump) = Pubkey::find_program_address(
        &[b"amm", token_0.as_ref(), token_1.as_ref()],
        &RAYDIUM_PROGRAM_ID,
    );

    // let pool_account_info = pool_account.to_account_info();
    // if pool_account_info.key() == pool_address && pool_account_info.owner == &RAYDIUM_PROGRAM_ID {
    //     Ok(Some(pool_address))
    // } else {
    //     Ok(None)
    // }
    Ok(None)
}
