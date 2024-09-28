# `solana-nostd-entrypoint`

The entrypoint function in `solana_program` is grossly inefficient. With an empty `process_instruction` function, it uses upwards of 8000 bpf instructions when the program receives 32 non-duplicate accounts. We use a new `NoStdAccountInfo` struct whose layout is consistent with that in the vm input memory region; unlike the usual entrypoint, it reads everything with no copies and no allocations.

This crate also includes a simple reference program that invokes another program. See `example_program/lib.rs`:

```rust
use solana_nostd_entrypoint::{
    basic_panic_impl, entrypoint_nostd, noalloc_allocator,
    solana_program::{
        entrypoint::ProgramResult, log, program_error::ProgramError, pubkey::Pubkey, system_program,
    },
    InstructionC, NoStdAccountInfo,
};

entrypoint_nostd!(process_instruction, 32);

pub const ID: Pubkey = solana_nostd_entrypoint::solana_program::pubkey!(
    "EWUt9PAjn26zCUALRRt56Gutaj52Bpb8ifbf7GZX3h1k"
);

noalloc_allocator!();
basic_panic_impl!();

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[NoStdAccountInfo],
    _data: &[u8],
) -> ProgramResult {
    log::sol_log("nostd");

    // Unpack accounts
    let [user, config, _rem @ ..] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Transfer has discriminant 2_u32 (little endian), followed u64 lamport amount
    let mut instruction_data = [0; 12];
    instruction_data[0] = 2;
    instruction_data[4..12].copy_from_slice(&100_000_000_u64.to_le_bytes());

    // Instruction accounts are are from, to
    let instruction_accounts = [user.to_meta_c(), config.to_meta_c()];

    // Build instruction expected by sol_invoke_signed_c
    let instruction = InstructionC {
        program_id: &system_program::ID,
        accounts: instruction_accounts.as_ptr(),
        accounts_len: instruction_accounts.len() as u64,
        data: instruction_data.as_ptr(),
        data_len: instruction_data.len() as u64,
    };

    // Get infos and seeds
    let infos = [user.to_info_c(), config.to_info_c()];
    let seeds: &[&[&[u8]]] = &[];

    // Invoke system program
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

    // For clippy
    #[cfg(not(target_os = "solana"))]
    core::hint::black_box(&(&instruction, &infos, &seeds));

    Ok(())
}
```
