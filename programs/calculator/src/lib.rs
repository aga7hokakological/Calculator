use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod calculator {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let values = &mut ctx.accounts.values;
        // values.num1 = 0;
        // values.num2 = 0;
        values.result = 0;

        Ok(())
    }

    pub fn add(ctx: Context<Add>, n1: i64, n2: i64) -> ProgramResult {
        let values = &mut ctx.accounts.values;
        // values.num1 = n1;
        // values.num2 = n2;
        values.result = n1 + n2;

        println!("The result is: {}", values.result);
        
        Ok(())
    }

    pub fn sub(ctx: Context<Sub>, n1: i64, n2: i64) -> ProgramResult {
        let values = &mut ctx.accounts.values;
        // values.num1 = n1;
        // values.num2 = n2;
        values.result = n1 - n2;

        println!("The result is: {}", values.result);
        
        Ok(())
    }

    pub fn mul(ctx: Context<Mul>, n1: i64, n2: i64) -> ProgramResult {
        let values = &mut ctx.accounts.values;
        // values.num1 = n1;
        // values.num2 = n2;
        values.result = n1 * n2;

        println!("The result is: {}", values.result);
        
        Ok(())
    }

    pub fn div(ctx: Context<Div>, n1: i64, n2: i64) -> ProgramResult {
        let values = &mut ctx.accounts.values;
        // values.num1 = n1;
        // values.num2 = n2;
        values.result = n1 / n2;
        values.remainder = n1 % n2;

        println!("The result is: {}", values.result);
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 64 + 64 + 64 + 64)]
    pub values: Account<'info, Values>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub values: Account<'info, Values>,
}

#[derive(Accounts)]
pub struct Sub<'info> {
    #[account(mut)]
    pub values: Account<'info, Values>,
}

#[derive(Accounts)]
pub struct Mul<'info> {
    #[account(mut)]
    pub values: Account<'info, Values>,
}

#[derive(Accounts)]
pub struct Div<'info> {
    #[account(mut)]
    pub values: Account<'info, Values>,
}

#[account] 
pub struct Values {
    // num1: i64,
    // num2: i64,
    result: i64,
    remainder: i64
}