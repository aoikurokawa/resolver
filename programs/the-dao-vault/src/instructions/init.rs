use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken, Create},
    token::{Mint, Token, TokenAccount},
};
use port_anchor_adaptor::PortReserve;

use std::convert::Into;

use crate::{adapters::SolendReserve, state::*};

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct InitBumpSeeds {
    authority: u8,
    reserve: u8,
    lp_mint: u8,
    solend_lp: u8,
    port_lp: u8,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
pub struct VaultConfigArg {
    pub deposit_cap: u64,
    pub fee_carry_bps: u32,
    pub fee_mgmt_bps: u32,
    pub referral_fee_pct: u8,
    pub allocation_cap_pct: u8,
    pub rebalance_mode: RebalanceMode,
    pub strategy_type: StrategyType,
}

#[derive(Accounts)]
#[instruction(bumps: InitBumpSeeds)]
pub struct Initialize<'info> {
    /// Vault state account
    #[account(zero)]
    pub vault: Box<Account<'info, Vault>>,

    /// Authority that the vault uses for lp token mints/burns and transfers to/from downstream assets
    #[account(
        mut,
        seeds = [vault.key().as_ref(), b"authority".as_ref()],
        bump = bumps.authority,
    )]
    pub vault_authority: AccountInfo<'info>,

    /// Mint for vault lp token
    #[account(
        init,
        payer = payer,
        seeds = [vault.key().as_ref(), b"lp_mint".as_ref()],
        bump,
        mint::authority = vault_authority,
        mint::decimals = reserve_token_mint.decimals,
    )]
    pub lp_token_mint: Box<Account<'info, Mint>>,

    /// Token account for vault reserve tokens
    #[account(
        init,
        payer = payer,
        seeds = [vault.key().as_ref(), reserve_token_mint.key().as_ref()],
        bump,
        token::authority = vault_authority,
        token::mint = reserve_token_mint,
    )]
    pub vault_reserve_token: Box<Account<'info, TokenAccount>>,

    /// Token account for the vault's solend lp tokens
    #[account(
        init,
        payer = payer,
        seeds = [vault.key().as_ref(), solend_lp_token_mint.key().as_ref()],
        bump,
        token::authority = vault_authority,
        token::mint = solend_lp_token_mint,
    )]
    pub vault_solend_lp_token: Box<Account<'info, TokenAccount>>,

    /// Token account for the vault's port lp tokens
    #[account(
        init,
        payer = payer,
        seeds = [vault.key().as_ref(), port_lp_token_mint.key().as_ref()],
        bump,
        token::authority = vault_authority,
        token::mint = port_lp_token_mint,
    )]
    pub vault_port_lp_token: Box<Account<'info, TokenAccount>>,

    /// Mint of the token that the vault accepts and stores
    pub reserve_token_mint: Box<Account<'info, Mint>>,

    /// Mint of the solend lp token
    pub solend_lp_token_mint: AccountInfo<'info>,

    /// Mint of the port lp token
    pub port_lp_token_mint: AccountInfo<'info>,

    pub solend_reserve: Box<Account<'info, SolendReserve>>,

    pub port_reserve: Box<Account<'info, PortReserve>>,

    /// Token account that receives the primary ratio of fees from the vault
    /// denominated in vault lp tokens
    #[account(mut)]
    pub fee_receiver: AccountInfo<'info>,

    /// Token account that receives the secondary ratio of fees from the vault
    /// denominated in vault lp tokens
    pub referral_fee_receiver: AccountInfo<'info>,

    /// Owner of the referral fee reciever token account
    pub referral_fee_owner: AccountInfo<'info>,

    /// Account that pays for above account inits
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Owner of the vault
    /// Only this account can call restricted instructions
    /// Acts as authority of the fee receiver account
    pub owner: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}


