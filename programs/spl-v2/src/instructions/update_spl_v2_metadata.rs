use anchor_lang::prelude::*;
use mpl_token_metadata::state::DataV2;

#[derive(Accounts)]
pub struct UpdateSplV2Metadata<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: 
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// CHECK:
    pub token_metadata_program: AccountInfo<'info>,
}

pub fn handler(
    ctx: Context<UpdateSplV2Metadata>,
    name: String, 
    symbol: String, 
    uri: String,
) -> Result<()> {
    anchor_lang::solana_program::program::invoke(
        &mpl_token_metadata::instruction::update_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.authority.key(),
            None,
            Some(DataV2 {
                name,
                symbol,
                uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None
            }),
            None,
            None
        ),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info()
        ]
    )?;

    Ok(())
}