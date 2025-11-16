use anchor_lang::prelude::*;

declare_id!("RaidCodeVaultID");

#[program]
pub mod raidcode_vault {
    use super::*;
    pub fn init_bounty(ctx: Context<InitBounty>, amount: u64) -> Result<()> {
        ctx.accounts.vault.amount = amount; // Bounty pool for PRs
        Ok(())
    }
    pub fn payout_pr(ctx: Context<PayoutPR>, pr_id: String) -> Result<()> {
        // Verify GitHub PR via oracle, transfer $RAID
        Ok(())
    }
}

#[account]
pub struct Vault { pub amount: u64; }
use anchor_lang::prelude::*;
use chainlink_solana as chainlink; // Add to Cargo.toml

#[program]
pub mod raidcode_oracle {
  use super::*;
  pub fn burn_on_stars(ctx: Context<BurnOnStars>, stars: u64) -> Result<()> {
    // Chainlink oracle feeds GitHub stars (off-chain job)
    require!(stars > 1000, ErrorCode::LowStars); // Threshold for burn
    let burn_amount = stars / 10000; // 0.1% burn per 1k stars
    // SPL transfer to dead address
    Ok(())
  }
}

#[account]
pub struct StarsFeed { pub current_stars: u64; }

#[error_code]
pub enum ErrorCode { #[msg("Stars too low for burn")] LowStars, }
