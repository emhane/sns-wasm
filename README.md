# SNS-WASM
Client-side SDK for interacting with Solana Name Service Program.

---

> [!WARNING]
> **Experimental Release**: This crate is in early development. 
> Currently, only **[Create Instruction]** implemented. 

<!-- snips: src/lib.rs#example -->
```rust
use solana_address::address;
use solana_instruction::Instruction;
use solana_program_error::ProgramError;
use wasm_bindgen_test::wasm_bindgen_test;

use super::*;
use crate::instruction_builder::NameRegistryInstruction;

const BONFIDA_DOMAIN_ADDRESS: Address =
    address!("Crf8hzfthWGbGbLTVCiqRqV5MVnbpHB1L9KQMd6gsinb");
const PAYER: Address = address!("1113eKEmP3gmGaNeoKSVoYwPpyfTmrmizMbi1TqGj2");
const NAME: &str = "bonfida";

#[wasm_bindgen_test]
fn test_build_create_instruction() {
    // setup
    let cfg = CreateDomainCfg {
        inner: CreateSNSRecordCfgInner {
            payer: PAYER,
            name: NAME.to_string(),
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
    assert_eq!(payer.pubkey, PAYER);
    assert_eq!(pda.pubkey, BONFIDA_DOMAIN_ADDRESS);
    assert_eq!(owner.pubkey, PAYER);
    assert_eq!(class.pubkey, Address::default());
    assert_eq!(parent.pubkey, SOL_TLD_ADDRESS);
    assert_eq!(parent_owner.pubkey, SOL_TLD_OWNER_ADDRESS_MAINNET);
    assert_eq!(hashed_name, name_hash(NAME).to_bytes().to_vec());
    assert_eq!(lamports, calculate_rent_exemption(0));
    assert_eq!(space, 0)
}
```