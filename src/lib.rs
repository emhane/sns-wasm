//! Client-side SNS-WASM SDK.

pub mod constants;
pub mod instruction_builder;
pub mod name_record;
pub mod pda;

pub use constants::{
    ROOT_TLD_ADDRESS, SNS_PROGRAM_ID, SOL_TLD_ADDRESS, SOL_TLD_NAME_HASH,
    SOL_TLD_OWNER_ADDRESS_MAINNET,
};
pub use instruction_builder::create::{
    CreateDomainInstBuilder, CreateInstBuilder, CreateSubdomainInstBuilder,
};
pub use name_record::{Domain, SNSNodeWithOwner, Subdomain, TLDomain};
pub use pda::{SNSNode, derive_domain, derive_subdomain, derive_tld, hashed_name};

use solana_instruction::{AccountMeta, Instruction};
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use wasm_bindgen::{JsError, JsValue, prelude::wasm_bindgen};

#[wasm_bindgen]
/// Computes instruction to register new SNS record from given parameters.
pub fn build_create_instruction(cfg: JsValue) -> Result<JsValue, JsError> {
    let builder: CreateInstBuilder = serde_wasm_bindgen::from_value(cfg)?;
    let inst = builder.build();

    Ok(serde_wasm_bindgen::to_value(&inst)?)
}

// todo: remove in favour of copying over funcs from archived spl_name_service
pub(crate) const fn to_v2(pk: Pubkey) -> spl_name_service::solana_program::pubkey::Pubkey {
    spl_name_service::solana_program::pubkey::Pubkey::new_from_array(pk.to_bytes())
}

// todo: remove in favour of copying over funcs from archived spl_name_service
pub(crate) const fn from_v2(pk: spl_name_service::solana_program::pubkey::Pubkey) -> Pubkey {
    Pubkey::new_from_array(pk.to_bytes())
}

// todo: remove in favour of copying over funcs from archived spl_name_service
pub(crate) fn from_v2_instr(
    instr: spl_name_service::solana_program::instruction::Instruction,
) -> Instruction {
    let spl_name_service::solana_program::instruction::Instruction { program_id, accounts, data } =
        instr;

    Instruction {
        program_id: from_v2(program_id),
        accounts: accounts.into_iter().map(from_v2_acc).collect::<Vec<_>>(),
        data,
    }
}

// todo: remove in favour of copying over funcs from archived spl_name_service
pub(crate) const fn from_v2_acc(
    acc: spl_name_service::solana_program::instruction::AccountMeta,
) -> AccountMeta {
    let spl_name_service::solana_program::instruction::AccountMeta {
        pubkey,
        is_signer,
        is_writable,
    } = acc;
    AccountMeta { pubkey: from_v2(pubkey), is_signer, is_writable }
}

// todo: remove in favour of copying over funcs from archived spl_name_service
pub(crate) fn from_v2_err(
    err: spl_name_service::solana_program::program_error::ProgramError,
) -> ProgramError {
    use spl_name_service::solana_program::program_error::ProgramError::*;
    match err {
        Custom(e) => ProgramError::Custom(e),
        InvalidArgument => ProgramError::InvalidArgument,
        InvalidInstructionData => ProgramError::InvalidInstructionData,
        InvalidAccountData => ProgramError::InvalidAccountData,
        AccountDataTooSmall => ProgramError::AccountDataTooSmall,
        InsufficientFunds => ProgramError::InsufficientFunds,
        IncorrectProgramId => ProgramError::IncorrectProgramId,
        MissingRequiredSignature => ProgramError::MissingRequiredSignature,
        AccountAlreadyInitialized => ProgramError::AccountAlreadyInitialized,
        UninitializedAccount => ProgramError::UninitializedAccount,
        NotEnoughAccountKeys => ProgramError::NotEnoughAccountKeys,
        AccountBorrowFailed => ProgramError::AccountBorrowFailed,
        MaxSeedLengthExceeded => ProgramError::MaxSeedLengthExceeded,
        InvalidSeeds => ProgramError::InvalidSeeds,
        BorshIoError(..) => ProgramError::BorshIoError,
        AccountNotRentExempt => ProgramError::AccountNotRentExempt,
        UnsupportedSysvar => ProgramError::UnsupportedSysvar,
        IllegalOwner => ProgramError::IllegalOwner,
        MaxAccountsDataAllocationsExceeded => ProgramError::MaxAccountsDataAllocationsExceeded,
        InvalidRealloc => ProgramError::InvalidRealloc,
        MaxInstructionTraceLengthExceeded => ProgramError::MaxInstructionTraceLengthExceeded,
        BuiltinProgramsMustConsumeComputeUnits => {
            ProgramError::BuiltinProgramsMustConsumeComputeUnits
        }
        InvalidAccountOwner => ProgramError::InvalidAccountOwner,
        ArithmeticOverflow => ProgramError::ArithmeticOverflow,
    }
}
