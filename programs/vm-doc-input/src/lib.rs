use anchor_lang::prelude::*;

declare_id!("8FBdK1WvpGhnkgMVspcD1ocdZdVJhkYTBUGqtkSUvG1z");

#[program]
pub mod vm_doc_input {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
