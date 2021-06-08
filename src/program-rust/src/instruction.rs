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
}
