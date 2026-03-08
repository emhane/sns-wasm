//! # SNS-WASM
//!
//! Specs: <https://sns.guide/introduction.html>
//!
//! <div style="background: #fff3cd; color: #856404; padding: 15px; border: 1px solid #ffeeba;
//! border-radius: 4px;"> <strong>Warning:</strong> This crate is experimental. Only one
//! instruction is currently supported. </div>
//! Client-side SNS-WASM SDK.

#![warn(clippy::large_stack_arrays)]

pub mod constants;
pub mod instruction_builder;
pub mod name_record;
pub mod pda;

pub use constants::{
    HASH_PREFIX, ROOT_TLD_ADDRESS, SNS_PROGRAM_ID, SOL_TLD_ADDRESS, SOL_TLD_NAME_HASH,
    SOL_TLD_OWNER_ADDRESS_MAINNET,
};
pub use instruction_builder::create::calculate_rent_exemption;
pub use name_record::{Domain, Subdomain, TLDomain};
pub use pda::{SNSNode, derive_domain, derive_subdomain, derive_tld, name_hash};

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use solana_pubkey::Pubkey;
use wasm_bindgen::{JsError, JsValue, prelude::wasm_bindgen};

use crate::name_record::SNSNodeWithOwner;

/// Params to create instruction to register new SNS domain record.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateDomainCfg {
    #[serde(flatten)]
    inner: CreateSNSRecordCfgInner,
    tld: SNSNodeWithOwner,
}

/// Params to create instruction to register new SNS subdomain record.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateSubdomainCfg {
    #[serde(flatten)]
    inner: CreateSNSRecordCfgInner,
    domain: SNSNodeWithOwner,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct CreateSNSRecordCfgInner {
    payer: Pubkey,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<Pubkey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    class: Option<Pubkey>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    space: Option<u32>,
}

#[wasm_bindgen]
/// Computes instruction to register new domain from given parameters.
pub fn build_create_domain_instruction(cfg: JsValue) -> Result<JsValue, JsError> {
    let CreateDomainCfg {
        inner: CreateSNSRecordCfgInner { payer, owner, class, name, space },
        tld,
    } = from_value(cfg)?;

    let builder =
        Domain::create_instruction_builder(payer, TLDomain::new(tld.pda, tld.owner), &name)
            .owner(owner)
            .class(class)
            .space(space);
    let inst = builder.build();

    Ok(to_value(&inst)?)
}

#[wasm_bindgen]
/// Computes instruction to register new subdomain from given parameters.
pub fn build_create_subdomain_instruction(cfg: JsValue) -> Result<JsValue, JsError> {
    let CreateSubdomainCfg {
        inner: CreateSNSRecordCfgInner { payer, owner, class, name, space },
        domain,
    } = from_value(cfg)?;

    let builder =
        Subdomain::create_instruction_builder(payer, Domain::new(domain.pda, domain.owner), &name)
            .owner(owner)
            .class(class)
            .space(space);
    let inst = builder.build();

    Ok(to_value(&inst)?)
}

#[cfg(test)]
mod tests {
    use solana_instruction::Instruction;
    use solana_program_error::ProgramError;
    use solana_pubkey::pubkey;
    use wasm_bindgen_test::wasm_bindgen_test;

    use super::*;
    use crate::instruction_builder::NameRegistryInstruction;

    const BONFIDA_DOMAIN_ADDRESS: Pubkey = pubkey!("Crf8hzfthWGbGbLTVCiqRqV5MVnbpHB1L9KQMd6gsinb");

    #[wasm_bindgen_test]
    fn test_build_create_instruction() {
        // setup
        let wallet = Pubkey::new_unique();
        let name = "bonfida";
        let cfg = CreateDomainCfg {
            inner: CreateSNSRecordCfgInner {
                payer: wallet,
                name: name.to_string(),
                ..Default::default()
            },
            tld: TLDomain::sol_mainnet().into(),
        };
        let js_cfg = to_value(&cfg).expect("should serialize");
        // test
        let js_inst = build_create_domain_instruction(js_cfg).expect("should build instruction");

        let res: Result<Instruction, ProgramError> =
            from_value(js_inst).expect("should deserialize");
        let inst = res.expect("should return instruction");
        //wasm_bindgen_test::console_log!("instruction: {:#?}", inst);

        let [_, payer, pda, owner, class, parent, parent_owner] = &inst.accounts[..] else {
            unreachable!()
        };
        let NameRegistryInstruction::Create { hashed_name, lamports, space } =
            wincode::deserialize(inst.data.as_slice()).expect("should deserialize")
        else {
            unreachable!("wrong instruction")
        };

        assert_eq!(inst.program_id, SNS_PROGRAM_ID);
        assert_eq!(payer.pubkey, wallet);
        assert_eq!(pda.pubkey, BONFIDA_DOMAIN_ADDRESS);
        assert_eq!(owner.pubkey, wallet);
        assert_eq!(class.pubkey, Pubkey::default());
        assert_eq!(parent.pubkey, SOL_TLD_ADDRESS);
        assert_eq!(parent_owner.pubkey, SOL_TLD_OWNER_ADDRESS_MAINNET);
        assert_eq!(hashed_name, name_hash(name).to_bytes().to_vec());
        assert_eq!(lamports, calculate_rent_exemption(0));
        assert_eq!(space, 0)
    }
}
