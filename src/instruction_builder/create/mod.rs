//! Build instruction for registering new SNS record.

pub mod domain;
pub mod subdomain;

use solana_address::Address;
use solana_instruction::{AccountMeta, Instruction};
use solana_program_error::ProgramError;
use solana_rent::Rent;

use crate::{
    SNS_PROGRAM_ID, SNSNode, constants::SNS_RECORD_HEADER_BYTE_LEN,
    instruction_builder::NameRegistryInstruction, name_record::SNSNodeWithOwner,
};

/// Builds instruction to include in transaction to register SNS record.
#[derive(Debug, Clone, Copy)]
pub struct CreateInstBuilder {
    payer: Address,
    account: SNSNode,
    owner: Option<Address>,
    class: Option<Address>,
    parent: SNSNodeWithOwner,
    space: Option<u32>,
}

impl CreateInstBuilder {
    /// Builds SNS Program CREATE instruction.
    pub fn build(self) -> Result<Instruction, ProgramError> {
        let Self {
            payer,
            account: SNSNode { pda, hashed_name },
            owner,
            class,
            parent: SNSNodeWithOwner { pda: parent, owner: parent_owner },
            space,
        } = self;

        let owner = owner.unwrap_or(payer);
        let hashed_name = hashed_name.to_bytes().to_vec();
        let space = space.unwrap_or_default();
        let lamports = calculate_rent_exemption(space);

        let inst = NameRegistryInstruction::Create { hashed_name, lamports, space };

        // instruction to register new sns node
        create(SNS_PROGRAM_ID, &inst, pda, payer, owner, class, Some(parent), Some(parent_owner))
    }
}

/// Returns rent exemption in lamports for [`SNS_RECORD_HEADER_BYTE_LEN`] + given space in bytes.
pub fn calculate_rent_exemption(bytes: u32) -> u64 {
    Rent::default().minimum_balance((bytes as usize).saturating_add(SNS_RECORD_HEADER_BYTE_LEN))
}

/// Code ported from archived <https://github.com/solana-labs/solana-program-library>
#[allow(clippy::too_many_arguments)]
pub fn create(
    name_service_program_id: Address,
    instruction_data: &NameRegistryInstruction,
    name_account_key: Address,
    payer_key: Address,
    name_owner: Address,
    name_class_opt: Option<Address>,
    name_parent_opt: Option<Address>,
    name_parent_owner_opt: Option<Address>,
) -> Result<Instruction, ProgramError> {
    let data: Vec<u8> = wincode::serialize(&instruction_data).unwrap();
    let mut accounts = vec![
        AccountMeta::new_readonly(Address::default(), false), // system program aka null address
        AccountMeta::new(payer_key, true),
        AccountMeta::new(name_account_key, false),
        AccountMeta::new_readonly(name_owner, false),
    ];
    if let Some(name_class) = name_class_opt {
        accounts.push(AccountMeta::new_readonly(name_class, true));
    } else {
        accounts.push(AccountMeta::new_readonly(Address::default(), false));
    }
    if let Some(name_parent) = name_parent_opt {
        accounts.push(AccountMeta::new_readonly(name_parent, false));
    } else {
        accounts.push(AccountMeta::new_readonly(Address::default(), false));
    }
    if let Some(key) = name_parent_owner_opt {
        accounts.push(AccountMeta::new_readonly(key, true));
    }

    Ok(Instruction { program_id: name_service_program_id, accounts, data })
}
