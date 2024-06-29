use anchor_lang::prelude::*;

declare_id!("CpUcoQLHSBuhPFFBZkMcdPaegaScAm9bAvUheYdo9e1f");

#[program]
pub mod intro_to_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn instruction_one(ctx: Context<InstructionAccounts>, instruction_data: u64) -> Result<()> {
        Ok(())
    }
}

#[account]
pub struct AccountStruct {
    // Define the fields of AccountStruct here
}

#[derive(Accounts)]
pub struct InstructionAccounts<'info> {
    #[account(init, payer = user, space = 8 + 8, bumps = [account_name])]
    pub account_name: Account<'info, AccountStruct>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub struct Initialize {}
