use anchor_lang::prelude::*;

use crate::{errors::*, state::Amm};

#[derive(Accounts)]
#[instruction(id: Pubkey, fee: u16)]
pub struct CreateAmm<'info> {
    #[account(
        init,
        payer = payer,
        space = Amm::LEN,
        seeds = [
            id.as_ref()
        ],
        bump,
        constraint = fee < 10000 @ AmmError::InvalidFee,
    )]
    pub amm: Account<'info, Amm>,

    /// The admin of the AMM
    /// CHECK: Read only, delegatable creation
    pub admin: AccountInfo<'info>,

    /// The account paying for all rents
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Solana ecosystem accounts
    pub system_program: Program<'info, System>,
}


impl<'info> CreateAmm<'info> {
    pub fn create_amm(
    
        &mut self, 
        id: Pubkey, 
        fee: u16
    ) -> Result<()> {
            let amm = &mut self.amm;
            amm.id = id;
            amm.admin = self.admin.key();
            amm.fee = fee;
    
            Ok(())
        }
    }