use anchor_lang::solana_program::account_info::AccountInfo;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::solana_program::pubkey::Pubkey;
pub use anchor_lang::prelude::*;
// use anchor_lang::{Accounts, CpiContext};
use anchor_lang::Result;
use anchor_lang::{context::CpiContext, Accounts};
pub use spl_associated_token_account::{get_associated_token_address, ID};

// pub use spl_associated_token_account::{get_associated_token_address, ID};

pub fn create<'info>(ctx: CpiContext<'_, '_, '_, 'info, Create<'info>>) -> ProgramResult {
    let ix = spl_associated_token_account::create_associated_token_account(
        ctx.accounts.payer.key,
        ctx.accounts.authority.key,
        ctx.accounts.mint.key,
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            ctx.accounts.payer,
            ctx.accounts.associated_token,
            ctx.accounts.authority,
            ctx.accounts.mint,
            ctx.accounts.system_program,
            ctx.accounts.token_program,
            ctx.accounts.rent,
        ],
        ctx.signer_seeds,
    )
}

#[derive(Accounts)]
pub struct Create<'info> {
    /// CHECK:
    pub payer: AccountInfo<'info>,
    /// CHECK:
    pub associated_token: AccountInfo<'info>,
    /// CHECK:
    pub authority: AccountInfo<'info>,
    /// CHECK:
    pub mint: AccountInfo<'info>,
    /// CHECK:
    pub system_program: AccountInfo<'info>,
    /// CHECK:
    pub token_program: AccountInfo<'info>,
    /// CHECK:
    pub rent: AccountInfo<'info>,
}

#[derive(Clone)]
pub struct AssociatedToken;

impl anchor_lang::AccountDeserialize for AssociatedToken {
    fn try_deserialize(buf: &mut &[u8]) -> Result<Self> {
        AssociatedToken::try_deserialize_unchecked(buf)
    }

    fn try_deserialize_unchecked(_buf: &mut &[u8]) -> Result<Self> {
        Ok(AssociatedToken)
    }
}

impl anchor_lang::Id for AssociatedToken {
    fn id() -> Pubkey {
        ID
    }
}
