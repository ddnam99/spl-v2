use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::states::spl_v2_config::*;

#[derive(Accounts)]
pub struct CreateSplV2<'info> {
    #[account(init, payer = creator, space = SplV2Config::SIZE)]
    pub config: Account<'info, SplV2Config>,

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub from_spl: Account<'info, Mint>,
    #[account(
        init, 
        payer = creator,
        seeds = [config.key().as_ref()],
        bump,
        mint::decimals = from_spl.decimals,
        mint::authority = spl_v2,
    )]
    pub spl_v2: Account<'info, Mint>,

    /// CHECK: 
    #[account(mut)]
    pub metadata: AccountInfo<'info>,
    /// CHECK:
    pub token_metadata_program: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateSplV2>, spl_v2_bump: u8, name: String, symbol: String, uri: String) -> Result<()> {
    ctx.accounts.config.creator = ctx.accounts.creator.key();
    ctx.accounts.config.from_spl = ctx.accounts.from_spl.key();
    ctx.accounts.config.spl_v2 = ctx.accounts.spl_v2.key();
    ctx.accounts.config.spl_v2_bump = spl_v2_bump;

    anchor_lang::solana_program::program::invoke_signed(
        &mpl_token_metadata::instruction::create_metadata_accounts_v2(
            ctx.accounts.token_metadata_program.key(),
            ctx.accounts.metadata.key(),
            ctx.accounts.spl_v2.key(),
            ctx.accounts.spl_v2.key(),
            ctx.accounts.creator.key(),
            ctx.accounts.creator.key(),
            name,
            symbol,
            uri,
            None,
            0,
            true,
            true,
            None,
            None,
        ),
        &[
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.spl_v2.to_account_info(),
            ctx.accounts.spl_v2.to_account_info(),
            ctx.accounts.creator.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ], 
        &[
            &[
                ctx.accounts.config.key().as_ref(),
                &[spl_v2_bump]
            ]
        ]
    )?;

    Ok(())
}