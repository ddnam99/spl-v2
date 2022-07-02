use anchor_lang::prelude::*;

#[account]
pub struct SplV2Config {
    pub creator: Pubkey,
    pub from_spl: Pubkey,
    pub spl_v2: Pubkey,
    pub spl_v2_bump: u8,
}

impl SplV2Config {
    pub const SIZE: usize = 8 + 32 + 32 + 32 + 1;
}
