//! Build instruction for registering new SNS record.

pub mod domain;
pub mod subdomain;
pub use domain::CreateDomainInstBuilder;
pub use subdomain::CreateSubdomainInstBuilder;

use serde::Deserialize;
use solana_instruction::Instruction;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_rent::Rent;
use spl_name_service::{
    instruction::{NameRegistryInstruction, create},
    solana_program::program_pack::Pack as _,
    state::NameRecordHeader,
};

use crate::{SNS_PROGRAM_ID, SNSNode, SNSNodeWithOwner, from_v2_err, from_v2_instr, to_v2};

/// Builds instruction to include in transaction to register SNS record.
#[derive(Debug, Deserialize)]
pub struct CreateInstBuilder {
    payer: Pubkey,
    account: SNSNode,
    owner: Option<Pubkey>,
    class: Option<Pubkey>,
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
        create(
            to_v2(SNS_PROGRAM_ID),
            inst,
            to_v2(pda),
            to_v2(payer),
            to_v2(owner),
            class.map(to_v2),
            Some(to_v2(parent)),
            Some(to_v2(parent_owner)),
        )
        .map(from_v2_instr)
        .map_err(from_v2_err)
    }
}

/// Returns rent exemption in lamports for [`NameRecordHeader::LEN`] + given space in bytes.
pub fn calculate_rent_exemption(bytes: u32) -> u64 {
    Rent::default().minimum_balance((bytes as usize).saturating_add(NameRecordHeader::LEN))
}
