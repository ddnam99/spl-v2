use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::states::spl_v2_config::*;

#[derive(Accounts)]
pub struct SwapSpl<'info> {
    pub config: Account<'info, SplV2Config>,

    #[account(mut)]
    user: Signer<'info>,

    #[account(mut, address = config.from_spl)]
    pub from_spl: Account<'info, Mint>,

    #[account(
        mut, 
        seeds = [config.key().as_ref()],
        bump = config.spl_v2_bump,
    )]
    pub spl_v2: Account<'info, Mint>,

    #[account(mut, constraint = user_from_spl_account.mint == config.from_spl)]
    pub user_from_spl_account: Account<'info, TokenAccount>,

    #[account(mut, constraint = user_spl_v2_account.mint == spl_v2.key())]
    pub user_spl_v2_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<SwapSpl>, amount: u64) -> Result<()> {
    anchor_spl::token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Burn {
                mint: ctx.accounts.from_spl.to_account_info(),
                from: ctx.accounts.user_from_spl_account.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
    )?;

    anchor_spl::token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.spl_v2.to_account_info(),
                to: ctx.accounts.user_spl_v2_account.to_account_info(),
                authority: ctx.accounts.spl_v2.to_account_info(),
            },
            &[
                &[
                    ctx.accounts.config.key().as_ref(),
                    &[ctx.accounts.config.spl_v2_bump]
                ]
            ]
        ),
        amount
    )?;

    Ok(())
}