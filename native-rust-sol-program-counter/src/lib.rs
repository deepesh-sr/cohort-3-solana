use std::ops::{AddAssign, SubAssign};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::Instruction,
    msg,
    pubkey::Pubkey,
    serialize_utils,
    stake::instruction,
};

#[derive(BorshDeserialize, BorshSerialize)]
enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
struct Counter {
    count: u32,
}

impl AddAssign<u32> for Counter {
    fn add_assign(&mut self, rhs: u32) {
        self.count += rhs;
    }
}

impl SubAssign<u32> for Counter {
    fn sub_assign(&mut self, rhs: u32) {
        self.count -= rhs;
    }
}

entrypoint!(counter_contract);

pub fn counter_contract(
    //whoever is calling your program will give you three things :

    // Program Id tells where the program is actually deployed.
    program_id: &Pubkey,

    // array of accounts
    account: &[AccountInfo],

    // array of u8 ( instruction)
    instruction_data: &[u8],
) -> ProgramResult {
    let mut account = next_account_info(&mut account.iter())?;

    // the program wants a specific format of data , so we are deserialing the instruction to match
    // it with the Instruction I will be giving.
    let instruction_type = InstructionType::try_from_slice(instruction_data)?;

    let mut counter_data = Counter::try_from_slice(&account.data.borrow())?;

    match instruction_type {
        InstructionType::Increment(value) => {
            counter_data += value;
        }
        InstructionType::Decrement(value) => {
            counter_data -= value;
        }
    }

    counter_data.serialize(&mut *account.data.borrow_mut())?;
    msg!("Counter updated to ,{}", counter_data);

    Ok(())
}
