//! SNS record types: TLD, domain and subdomain.

use derive_more::{Constructor, Deref, Into};
use serde::{Deserialize, Serialize};
use solana_address::Address;

use crate::{
    SNSNode, SOL_TLD_ADDRESS, SOL_TLD_OWNER_ADDRESS_MAINNET, derive_domain, derive_subdomain,
    derive_tld,
    instruction_builder::{CreateDomainInstBuilder, CreateSubdomainInstBuilder},
};

/// SNS record address and its owner.
#[derive(Debug, Clone, Copy, Default, Constructor, Serialize, Deserialize)]
pub struct SNSNodeWithOwner {
    /// Programmatically Derived Address (PDA) of this account.
    #[serde(rename = "tld", alias = "domain")]
    pub pda: Address,
    /// Owner of account.
    #[serde(rename = "tldOwner", alias = "domainOwner")]
    pub owner: Address,
}

/// Top Level Domain (TLD) SNS record.
///
/// Specs: <https://sns.guide/domain-name/domain-tld.html>
#[derive(Debug, Clone, Copy, Default, Deref, Into)]
pub struct TLDomain(SNSNodeWithOwner);

impl TLDomain {
    /// Returns new instance with given Programmatically Derived Address (PDA) and its owner.
    pub fn new(pda: Address, owner: Address) -> Self {
        Self(SNSNodeWithOwner::new(pda, owner))
    }

    /// Returns new instance with given owner and deriving PDA from given class + name +
    /// [`ROOT_TLD_ADDRESS`].
    pub fn derive_new(owner: Address, class: Option<&Address>, name: &str) -> Self {
        let SNSNode { pda, .. } = derive_tld(class, name);
        Self(SNSNodeWithOwner::new(pda, owner))
    }

    /// Returns new instance with Mainnet owner of PDA of `.sol`.
    pub fn sol_mainnet() -> Self {
        Self(SNSNodeWithOwner::new(SOL_TLD_ADDRESS, SOL_TLD_OWNER_ADDRESS_MAINNET))
    }

    /// Returns new instance with System Program as owner of PDA of `.sol`.
    pub fn sol_devnet() -> Self {
        let owner = Address::default();
        Self(SNSNodeWithOwner::new(SOL_TLD_ADDRESS, owner))
    }
}

/// Domain SNS record.
///
/// Specs: <https://sns.guide/domain-name/domain-tld.html>
#[derive(Debug, Clone, Copy, Deref, Into)]
pub struct Domain(SNSNodeWithOwner);

impl Domain {
    /// Returns new instance with given Programmatically Derived Address (PDA) and its owner.
    pub fn new(pda: Address, owner: Address) -> Self {
        Self(SNSNodeWithOwner::new(pda, owner))
    }

    /// Returns new instance with given owner and deriving PDA from given class + name +
    /// TLD.
    pub fn derive_new(
        parent: &Address,
        owner: Address,
        class: Option<&Address>,
        name: &str,
    ) -> Self {
        let SNSNode { pda, .. } = derive_domain(parent, class, name);
        Self::new(pda, owner)
    }

    /// Returns builder for create domain instruction.
    pub fn create_instruction_builder(
        payer: Address,
        tld: TLDomain,
        name: &str,
    ) -> CreateDomainInstBuilder {
        CreateDomainInstBuilder::new(payer, tld, String::from(name))
    }
}

/// Subdomain SNS record.
///
/// Specs: <https://sns.guide/domain-name/domain-tld.html>
#[derive(Debug, Clone, Copy, Deref, Into)]
pub struct Subdomain(SNSNodeWithOwner);

impl Subdomain {
    /// Returns new instance with given Programmatically Derived Address (PDA) and its owner.
    pub fn new(pda: Address, owner: Address) -> Self {
        Self(SNSNodeWithOwner::new(pda, owner))
    }

    /// Returns new instance with given owner and deriving PDA from given class + name +
    /// parent [sub]domain.
    pub fn derive_new(
        parent: &Address,
        owner: Address,
        class: Option<&Address>,
        name: &str,
    ) -> Self {
        let SNSNode { pda, .. } = derive_subdomain(parent, class, name);
        Self::new(pda, owner)
    }

    /// Returns builder for create subdomain instruction.
    pub fn create_instruction_builder(
        payer: Address,
        domain: Domain,
        name: &str,
    ) -> CreateSubdomainInstBuilder {
        CreateSubdomainInstBuilder::new(payer, domain, String::from(name))
    }
}
