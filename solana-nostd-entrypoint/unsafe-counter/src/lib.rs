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

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[NoStdAccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    log::sol_log("Counter program entry");

    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    match instruction_data[0] {
        0 => initialize(accounts),
        1 => increment(accounts),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

fn initialize(accounts: &[NoStdAccountInfo]) -> ProgramResult {
    log::sol_log("Initialize");

    let [payer, counter, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if counter.key() == &system_program::ID {
        return Err(ProgramError::InvalidAccountData);
    }
    
    // Calculate required lamports
    let rent = solana_program::rent::Rent::default();
    let required_lamports = rent.minimum_balance(ACCOUNT_DATA_LEN);

    // Create account instruction data
    let mut create_instruction_data = [0; 56];
    create_instruction_data[0..4].copy_from_slice(&[0, 0, 0, 0]); // CreateAccount instruction
    create_instruction_data[4..12].copy_from_slice(&required_lamports.to_le_bytes());
    create_instruction_data[12..20].copy_from_slice(&(ACCOUNT_DATA_LEN as u64).to_le_bytes());
    create_instruction_data[20..52].copy_from_slice(ID.as_ref());

    // Instruction accounts
    let instruction_accounts = [
        payer.to_meta_c(),
        counter.to_meta_c(),
    ];

    // Build instruction
    let instruction = InstructionC {
        program_id: &system_program::ID,
        accounts: instruction_accounts.as_ptr(),
        accounts_len: instruction_accounts.len() as u64,
        data: create_instruction_data.as_ptr(),
        data_len: create_instruction_data.len() as u64,
    };

    // Get account infos
    let infos = [payer.to_info_c(), counter.to_info_c()];
    let seeds: &[&[&[u8]]] = &[];

    // Invoke system program to create account
    #[cfg(target_os = "solana")]
    unsafe {
        solana_program::syscalls::sol_invoke_signed_c(
            &instruction as *const InstructionC as *const u8,
            infos.as_ptr() as *const u8,
            infos.len() as u64,
            seeds.as_ptr() as *const u8,
            seeds.len() as u64,
        );
    }

    // Initialize counter to 0
    let mut counter_data = counter.try_borrow_mut_data().ok_or(ProgramError::AccountBorrowFailed)?;
    counter_data[..8].copy_from_slice(&0_u64.to_le_bytes());

    Ok(())
}

fn increment(accounts: &[NoStdAccountInfo]) -> ProgramResult {
    log::sol_log("Increment");

    let [counter, payer] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut counter_data = counter.try_borrow_mut_data().ok_or(ProgramError::AccountBorrowFailed)?;
    let mut value = u64::from_le_bytes(counter_data[..8].try_into().unwrap());
    value = value.wrapping_add(1);
    counter_data[..8].copy_from_slice(&value.to_le_bytes());

    Ok(())
}