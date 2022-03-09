// use anchor_lang::prelude::*;

// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// #[program]
// pub mod create_icury_token {
//     use super::*;

//     pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

#[cfg(feature = "associated_token")]
pub mod associated_token;

#[cfg(feature = "mint")]
pub mod mint;

#[cfg(feature = "token")]
pub mod token;
