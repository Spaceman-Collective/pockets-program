use anchor_lang::prelude::*;

declare_id!("GEUwNbnu9jkRMY8GX5Ar4R11mX9vXR8UDFnKZMn5uWLJ");

#[program]
pub mod pockets_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
