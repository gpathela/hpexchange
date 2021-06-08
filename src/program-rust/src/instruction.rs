use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::ExchangeError::InvalidInstruction;

pub enum ExchangeInstruction {
    /// Starts the deposit by getting the amount that need to be transffered to the account
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow
    Deposit {
        /// The amount party A expects to receive of token Y
        amount: u64,
    },
    /// Withdraw
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person taking the trade    
    Withdraw {
        /// the amount the taker expects to be paid in the other token, as a u64 because that's the max possible supply of a token
        amount: u64,
    },

    //Init Bet
    Initbet {
        amount: u64,
        odds: u64,
    },
    //Settle Bet
    Settle {
        user_won: bool,
    },
}

impl ExchangeInstruction {
    /// Unpacks a byte buffer into a [ExchangeInstruction](enum.ExchangeInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        //let (bump, _rest1) = input.split_last().ok_or(InvalidInstruction)?;
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => Self::Deposit {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Withdraw {
                amount: Self::unpack_amount(rest)?,
            },
            2 => Self::Initbet {
                amount: Self::unpack_amount(rest)?,
                odds: Self::unpack_odds(rest)?,
            },
            3 => Self::Settle {
                user_won: Self::unpack_result(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }

    fn unpack_odds(input: &[u8]) -> Result<u64, ProgramError> {
        let odds = input
            .get(8..16)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(odds)
    }

    fn unpack_result(input: &[u8]) -> Result<bool, ProgramError> {
        let (tag, _rest) = input.split_last().ok_or(InvalidInstruction)?;

        if *tag == 1 {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
