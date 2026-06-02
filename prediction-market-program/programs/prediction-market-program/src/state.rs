use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    #[max_len(20)]
    pub market_id: String,
    pub admin: Pubkey,
    pub market_vault: Pubkey,
    pub resolved: bool,
    pub resolution_time: i64,
    pub outcome: Option<u8>,
    pub bump: u8,
}
