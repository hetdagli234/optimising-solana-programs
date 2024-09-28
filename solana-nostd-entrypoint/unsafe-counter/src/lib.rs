use solana_nostd_entrypoint::{
    basic_panic_impl, entrypoint_nostd, noalloc_allocator,
    solana_program::{
        entrypoint::ProgramResult, log, program_error::ProgramError, pubkey::Pubkey, system_program,
    },
    InstructionC, NoStdAccountInfo,
};

entrypoint_nostd!(process_instruction, 32);

pub const ID: Pubkey = solana_nostd_entrypoint::solana_program::pubkey!(
    "EgB1zom79Ek4LkvJjafbkUMTwDK9sZQKEzNnrNFHpHHz"
);

noalloc_allocator!();
basic_panic_impl!();

const ACCOUNT_DATA_LEN: usize = 8; // 8 bytes for u64 counter

/*
 * Program Entrypoint
 * ------------------
 * Entrypoint receives:
 * - program_id: The public key of the program's account
 * - accounts: An array of accounts required for the instruction
 * - instruction_data: A byte array containing the instruction data
 *
 * Instruction data format:
 * ------------------------
 * | Bit 0 | Bits 1-7 |
 * |-------|----------|
 * |  0/1  |  Unused  |
 * 
 * 0: Initialize
 * 1: Increment
 */
#[inline(always)]
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[NoStdAccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    // Use the least significant bit to determine the instruction
    match instruction_data[0] & 1 {
        0 => initialize(accounts),
        1 => increment(accounts),
        _ => unreachable!(),
    }
}

/*
 * Initialize Function
 * -------------------
 * This function initializes a new counter account.
 * 
 * Account structure:
 * ------------------
 * 1. Payer account (signer, writable)
 * 2. Counter account (writable)
 * 3. System program
 *
 * Memory layout of instruction_data:
 * -----------------------------------------
 * | Bytes    | Content                     |
 * |----------|----------------------------|
 * | 0-3      | Instruction discriminator  |
 * | 4-11     | Required lamports (u64)    |
 * | 12-19    | Space (u64)                |
 * | 20-51    | Program ID                 |
 * | 52-55    | Unused                     |
 */
#[inline(always)]
fn initialize(accounts: &[NoStdAccountInfo]) -> ProgramResult {

    let [payer, counter, system_program] = match accounts {
        [payer, counter, system_program, ..] => [payer, counter, system_program],
        _ => return Err(ProgramError::NotEnoughAccountKeys),
    };

    if counter.key() == &system_program::ID {
        return Err(ProgramError::InvalidAccountData);
    }
    
    let rent = solana_program::rent::Rent::default();
    let required_lamports = rent.minimum_balance(ACCOUNT_DATA_LEN);

    let mut instruction_data = [0u8; 56];
    instruction_data[4..12].copy_from_slice(&required_lamports.to_le_bytes());
    instruction_data[12..20].copy_from_slice(&(ACCOUNT_DATA_LEN as u64).to_le_bytes());
    instruction_data[20..52].copy_from_slice(ID.as_ref());

    let instruction_accounts = [
        payer.to_meta_c(),
        counter.to_meta_c(),
    ];

    let instruction = InstructionC {
        program_id: &system_program::ID,
        accounts: instruction_accounts.as_ptr(),
        accounts_len: instruction_accounts.len() as u64,
        data: instruction_data.as_ptr(),
        data_len: instruction_data.len() as u64,
    };

    let infos = [payer.to_info_c(), counter.to_info_c()];

    // Invoke system program to create account
    #[cfg(target_os = "solana")]
    unsafe {
        solana_program::syscalls::sol_invoke_signed_c(
            &instruction as *const InstructionC as *const u8,
            infos.as_ptr() as *const u8,
            infos.len() as u64,
            std::ptr::null(),
            0,
        );
    }

    // Initialize counter to 0
    let mut counter_data = counter.try_borrow_mut_data().ok_or(ProgramError::AccountBorrowFailed)?;
    counter_data[..8].copy_from_slice(&0u64.to_le_bytes());

    Ok(())
}

/*
 * Increment Function
 * ------------------
 * This function increments the counter in the counter account.
 *
 * Account structure:
 * ------------------
 * 1. Counter account (writable)
 * 2. Payer account (signer)
 *
 * Counter account data layout:
 * ----------------------------
 * | Bytes | Content        |
 * |-------|----------------|
 * | 0-7   | Counter (u64)  |
 */
#[inline(always)]
fn increment(accounts: &[NoStdAccountInfo]) -> ProgramResult {

    let [counter, payer] = match accounts {
        [counter, payer, ..] => [counter, payer],
        _ => return Err(ProgramError::NotEnoughAccountKeys),
    };

    if !payer.is_signer() || counter.owner() != &ID {
        return Err(ProgramError::IllegalOwner);
    }

    let mut counter_data = counter.try_borrow_mut_data().ok_or(ProgramError::AccountBorrowFailed)?;

    if counter_data.len() != 8 {
        return Err(ProgramError::UninitializedAccount);
    }

    let mut value = u64::from_le_bytes(counter_data[..8].try_into().unwrap());
    value += 1;
    counter_data[..8].copy_from_slice(&value.to_le_bytes());

    Ok(())
}