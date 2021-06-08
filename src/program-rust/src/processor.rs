use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    //program_pack::{IsInitialized, Pack},
    program_pack::Pack,
    pubkey::Pubkey,
    //sysvar::{rent::Rent, Sysvar},
};

//use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::{burn, mint_to, transfer};
use spl_token::state::Account as TokenAccount;
//use spl_token::ID;

use crate::{error::ExchangeError, instruction::ExchangeInstruction, state::Escrow};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = ExchangeInstruction::unpack(instruction_data)?;

        match instruction {
            ExchangeInstruction::Deposit { amount } => {
                msg!("Instruction: Deposit");
                Self::process_deposit(accounts, amount, program_id)
            }
            ExchangeInstruction::Withdraw { amount } => {
                msg!("Instruction: Withdraw");
                Self::process_withdraw(accounts, amount, program_id)
            }
        }
    }

    fn process_deposit(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        msg!("Divvy program entrypoint");

        // Iterating accounts is safer then indexing
        let accounts_iter = &mut accounts.iter();

        // Get the account to say hello to
        let account = next_account_info(accounts_iter)?;
        let mint = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        //let token_owner = next_account_info(accounts_iter)?;
        let token_account = next_account_info(accounts_iter)?;
        let pda_account = next_account_info(accounts_iter)?;
        let user_account = next_account_info(accounts_iter)?;
        let hp_usdt_account = next_account_info(accounts_iter)?;

        msg!("Amount is {} ", amount);
        //let (_pda, bump_seed) = Pubkey::find_program_address(&[b"divvyexchange"], program_id);
        let transfer_instruction = transfer(
            &token_program.key,
            &user_account.key,
            &hp_usdt_account.key,
            &account.key,
            &[&account.key],
            amount.clone(),
        )?;
        msg!("Calling the token program to transfer tokens...");
        invoke(
            &transfer_instruction,
            &[
                user_account.clone(),
                hp_usdt_account.clone(),
                account.clone(),
                token_program.clone(),
            ],
        )?;

        msg!("Creating mint instruction");
        let mint_ix = mint_to(
            &token_program.key,
            &mint.key,
            &token_account.key,
            &pda_account.key,
            &[&pda_account.key],
            amount * 10000000,
        )?;

        invoke_signed(
            &mint_ix,
            &[mint.clone(), token_account.clone(), pda_account.clone()],
            &[&[&b"divvyexchange"[..], &[254]]],
        )?;

        Ok(())
    }

    fn process_withdraw(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        // Iterating accounts is safer then indexing
        let accounts_iter = &mut accounts.iter();

        // Get the account to say hello to
        let account = next_account_info(accounts_iter)?;
        let mint = next_account_info(accounts_iter)?;
        let token_program = next_account_info(accounts_iter)?;
        //let token_owner = next_account_info(accounts_iter)?;
        let token_account = next_account_info(accounts_iter)?;
        let pda_account = next_account_info(accounts_iter)?;
        let user_account = next_account_info(accounts_iter)?;
        let hp_usdt_account = next_account_info(accounts_iter)?;

        //Burn the transfers
        let burn_tx = burn(
            &token_program.key,
            &token_account.key,
            &mint.key,
            &account.key,
            &[&account.key],
            amount * 10000000,
        )?;

        invoke(
            &burn_tx,
            &[
                token_program.clone(),
                token_account.clone(),
                mint.clone(),
                account.clone(),
            ],
        )?;

        //Transfer Withdraw
        let transfer_instruction = transfer(
            &token_program.key,
            &hp_usdt_account.key,
            &user_account.key,
            &pda_account.key,
            &[&pda_account.key],
            amount.clone(),
        )?;
        msg!("Calling the token program to transfer tokens...");
        invoke_signed(
            &transfer_instruction,
            &[
                hp_usdt_account.clone(),
                user_account.clone(),
                pda_account.clone(),
                token_program.clone(),
            ],
            &[&[&b"divvyexchange"[..], &[254]]],
        )?;

        Ok(())
    }
}
