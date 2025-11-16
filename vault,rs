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
