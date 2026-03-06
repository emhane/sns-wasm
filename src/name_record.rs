//! SNS record types: TLD, domain and subdomain.

use derive_more::{Constructor, Deref, Into};
use serde::{Deserialize, Serialize};
use solana_pubkey::Pubkey;

use crate::{
    SNSNode, SOL_TLD_ADDRESS, SOL_TLD_OWNER_ADDRESS_MAINNET, derive_domain, derive_subdomain,
    derive_tld,
};

/// SNS record address and its owner.
#[derive(Debug, Clone, Copy, Default, Constructor, Serialize, Deserialize)]
pub struct SNSNodeWithOwner {
    /// Programmatically Derived Address (PDA) of this account.
    pub pda: Pubkey,
    /// Owner of account.
    pub owner: Pubkey,
}

/// Top Level Domain (TLD) SNS record.
///
/// Specs: <https://sns.guide/domain-name/domain-tld.html>
#[derive(Debug, Clone, Copy, Default, Deref, Into)]
pub struct TLDomain(SNSNodeWithOwner);

impl TLDomain {
    /// Returns new instance with given Programmatically Derived Address (PDA) and its owner.
    pub fn new(pda: Pubkey, owner: Pubkey) -> Self {
        Self(SNSNodeWithOwner::new(pda, owner))
    }

    /// Returns new instance with given owner and deriving PDA from given class + name +
    /// [`ROOT_TLD_ADDRESS`].
    pub fn derive_new(owner: Pubkey, class: Option<&Pubkey>, name: &str) -> Self {
        let SNSNode { pda, .. } = derive_tld(class, name);
        Self(SNSNodeWithOwner::new(pda, owner))
    }

    /// Returns new instance with Mainnet owner of PDA of `.sol`.
    pub fn sol_mainnet() -> Self {
        Self(SNSNodeWithOwner::new(SOL_TLD_ADDRESS, SOL_TLD_OWNER_ADDRESS_MAINNET))
    }

    /// Returns new instance with System Program as owner of PDA of `.sol`.
    pub fn sol_devnet() -> Self {
        let owner = Pubkey::default();
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
    pub fn new(pda: Pubkey, owner: Pubkey) -> Self {
        Self(SNSNodeWithOwner::new(pda, owner))
    }

    /// Returns new instance with given owner and deriving PDA from given class + name +
    /// TLD.
    pub fn derive_new(parent: &Pubkey, owner: Pubkey, class: Option<&Pubkey>, name: &str) -> Self {
        let SNSNode { pda, .. } = derive_domain(parent, class, name);
        Self::new(pda, owner)
    }
}

/// Subdomain SNS record.
///
/// Specs: <https://sns.guide/domain-name/domain-tld.html>
#[derive(Debug, Clone, Copy, Deref, Into)]
pub struct Subdomain(SNSNodeWithOwner);

impl Subdomain {
    /// Returns new instance with given Programmatically Derived Address (PDA) and its owner.
    pub fn new(pda: Pubkey, owner: Pubkey) -> Self {
        Self(SNSNodeWithOwner::new(pda, owner))
    }

    /// Returns new instance with given owner and deriving PDA from given class + name +
    /// parent [sub]domain.
    pub fn derive_new(parent: &Pubkey, owner: Pubkey, class: Option<&Pubkey>, name: &str) -> Self {
        let SNSNode { pda, .. } = derive_subdomain(parent, class, name);
        Self::new(pda, owner)
    }
}
