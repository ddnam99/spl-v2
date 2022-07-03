use anchor_lang::prelude::*;
use instructions::*;

mod instructions;
mod states;

declare_id!("ASau1EwV3BS1QgrhNtfw9Zwv5TitMFp93fdzmXu8o66u");

#[program]
pub mod spl_v2 {
    use crate::instructions::create_spl_v2::CreateSplV2;

    use super::*;

    pub fn create_spl_v2(
        ctx: Context<CreateSplV2>,
        spl_v2_bump: u8,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        return instructions::create_spl_v2::handler(ctx, spl_v2_bump, name, symbol, uri);
    }

    pub fn update_spl_v2_metadata(
        ctx: Context<UpdateSplV2Metadata>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        return instructions::update_spl_v2_metadata::handler(ctx, name, symbol, uri);
    }

    pub fn swap_spl(ctx: Context<SwapSpl>, amount: u64) -> Result<()> {
        return instructions::swap_spl::handler(ctx, amount);
    }
}
