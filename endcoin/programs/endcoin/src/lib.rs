use anchor_lang::prelude::*;

pub use errors::*;
pub mod errors;
pub use state::*;
pub mod state;

pub use ammconfig::*;
pub mod ammconfig;

pub use contexts::*;
pub mod contexts;
declare_id!("Gm7fXnKwxt8pic7Ww98BDJRsf6tABmzeEpoRQiaceriu");

#[program]
pub mod endcoin {
    use super::*;
    

    pub fn init(ctx: Context<Init>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)?;
        Ok(())
    }
    

    // pub fn create_endcoin_metadata(ctx: Context<Init>, name: String, symbol: String, uri: String) -> Result<()> {
    //     ctx.accounts.create_endcoin_metadata();
    //     Ok(())
    // }

    // pub fn emit(ctx: Context<MintTokens>) -> Result<()> {

    //     Ok(())
    // }
}



#[derive(Accounts)]
pub struct Withdraw{}

#[derive(Accounts)]
pub struct Swap{}
