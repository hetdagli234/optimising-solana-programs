use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    program::invoke,
    sysvar::Sysvar,
};
use std::mem::size_of;

// Define the state struct
struct Counter {
    count: u64,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instruction_data
        .get(0)
        .ok_or(ProgramError::InvalidInstructionData)?;

    match instruction {
        0 => initialize(program_id, accounts),
        1 => increment(accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn initialize(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let counter_account = next_account_info(account_info_iter)?;
    let user = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if counter_account.owner != program_id {
        let rent = Rent::get()?;
        let space = size_of::<Counter>();
        let rent_lamports = rent.minimum_balance(space);

        invoke(
            &system_instruction::create_account(
                user.key,
                counter_account.key,
                rent_lamports,
                space as u64,
                program_id,
            ),
            &[user.clone(), counter_account.clone(), system_program.clone()],
        )?;
    }

    let mut counter_data = Counter { count: 0 };
    counter_data.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;

    Ok(())
}

fn increment(accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let counter_account = next_account_info(account_info_iter)?;
    let user = next_account_info(account_info_iter)?;

    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut counter_data = Counter::deserialize(&counter_account.data.borrow())?;

    //Not doing checked_add, wrapping add or any overflow checks to keep it simple
    counter_data.count += 1;
    counter_data.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;

    Ok(())
}

impl Counter {
    fn serialize(&self, data: &mut [u8]) -> ProgramResult {
        if data.len() < size_of::<Self>() {
            return Err(ProgramError::AccountDataTooSmall);
        }
        
        //First 8 bytes is the count
        data[..8].copy_from_slice(&self.count.to_le_bytes());
        Ok(())
    }

    fn deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() < size_of::<Self>() {
            return Err(ProgramError::AccountDataTooSmall);
        }

        //First 8 bytes is the count
        let count = u64::from_le_bytes(data[..8].try_into().unwrap());
        Ok(Self { count })
    }
}
