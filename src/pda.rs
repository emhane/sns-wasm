//! Generate programmatically derived addresses (PDAs).

use derive_more::Constructor;
use serde::Deserialize;
use sha2::{Digest as _, Sha256};
use solana_hash::Hash;
use solana_pubkey::Pubkey;
use spl_name_service::state::{HASH_PREFIX, get_seeds_and_key};

use crate::{ROOT_TLD_ADDRESS, SNS_PROGRAM_ID, SOL_TLD_ADDRESS, SOL_TLD_NAME_HASH, from_v2, to_v2};

/// Programmatically Derived Address (PDA) of SNS record, and its hashed name (created as input to
/// derivation process).
#[derive(Debug, Clone, Copy, Constructor, Deserialize)]
pub struct SNSNode {
    /// PDA of SNS record. Derived from hashed user given name, parent PDA and
    /// class address.
    ///
    /// See [`derive_tld`], [`derive_domain`] and [`derive_subdomain`].
    pub pda: Pubkey,
    /// Sha256 of [`HASH_PREFIX`] + user given name.
    pub hashed_name: Hash,
}

impl SNSNode {
    /// Returns TLD `.sol`.
    pub fn sol() -> Self {
        Self::new(SOL_TLD_ADDRESS, SOL_TLD_NAME_HASH)
    }
}

// todo: make nice hash builder type to avoid format!()

/// Returns the `sha256` hashed concatenation of [`HASH_PREFIX`] and given name.
///
/// - Top level domains (TLDs) require '.' prefix, e.g. pass `.sol`
/// - Subdomains require '\0' prefix, e.g. pass `code.blush.sol`
pub fn hashed_name(name: &str) -> Hash {
    let mut hasher = Sha256::new();

    hasher.update(HASH_PREFIX.as_bytes());
    hasher.update(name.as_bytes());

    let hash = hasher.finalize();

    Hash::new_from_array(hash.into())
}

/// Returns PDA of TLD with given name and [`ROOT_TLD_ADDRESS`] as parent.
///
/// Adds '.' prefix to given name.
///
/// Spec: <https://sns.guide/domain-name/domain-tld.html>
pub fn derive_tld(class: Option<&Pubkey>, name: &str) -> SNSNode {
    let dot_name = format!(".{name}");
    let hashed_tld_name = hashed_name(&dot_name);

    let (tld, _) = get_seeds_and_key(
        &to_v2(SNS_PROGRAM_ID),
        hashed_tld_name.to_bytes().to_vec(),
        class.map(|c| to_v2(*c)).as_ref(),
        Some(&to_v2(ROOT_TLD_ADDRESS)),
    );

    SNSNode::new(from_v2(tld), hashed_tld_name)
}

/// Returns PDA of domain with given name and TLD.
///
/// Spec: <https://sns.guide/domain-name/domain-tld.html>
pub fn derive_domain(tld: &Pubkey, class: Option<&Pubkey>, name: &str) -> SNSNode {
    let hashed_name = hashed_name(name);
    let (domain, _) = get_seeds_and_key(
        &to_v2(SNS_PROGRAM_ID),
        hashed_name.to_bytes().to_vec(),
        class.map(|c| to_v2(*c)).as_ref(),
        Some(&to_v2(*tld)),
    );

    SNSNode::new(from_v2(domain), hashed_name)
}

/// Returns PDA of subdomain with given name and parent.
///
/// Adds '\0' prefix to given name.
///
/// <https://sns.guide/domain-name/domain-tld.html>
pub fn derive_subdomain(parent: &Pubkey, class: Option<&Pubkey>, name: &str) -> SNSNode {
    let name_dot = format!("\0{name}");
    let hashed_subdomain_name = hashed_name(&name_dot);

    let (subdomain, _) = get_seeds_and_key(
        &to_v2(SNS_PROGRAM_ID),
        hashed_subdomain_name.to_bytes().to_vec(),
        class.map(|c| to_v2(*c)).as_ref(),
        Some(&to_v2(*parent)),
    );

    SNSNode::new(from_v2(subdomain), hashed_subdomain_name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SOL_TLD_ADDRESS;
    use solana_pubkey::pubkey;

    const BONFIDA_DOMAIN_ADDRESS: Pubkey = pubkey!("Crf8hzfthWGbGbLTVCiqRqV5MVnbpHB1L9KQMd6gsinb");
    const DEX_BONFIDA_SUBDOMAIN_ADDRESS: Pubkey =
        pubkey!("HoFfFXqFHAC8RP3duuQNzag1ieUwJRBv1HtRNiWFq4Qu");

    #[test]
    fn test_sol_hashed_name() {
        let hash = hashed_name(".sol");
        assert_eq!(hash, SOL_TLD_NAME_HASH)
    }

    #[test]
    fn test_derive_sol() {
        let SNSNode { pda, .. } = derive_tld(None, "sol");
        assert_eq!(pda, SOL_TLD_ADDRESS)
    }

    #[test]
    fn test_derive_domain() {
        let SNSNode { pda, .. } = derive_domain(&SOL_TLD_ADDRESS, None, "bonfida");
        assert_eq!(pda, BONFIDA_DOMAIN_ADDRESS);
    }

    #[test]
    fn test_derive_subdomain() {
        let SNSNode { pda, .. } = derive_subdomain(&BONFIDA_DOMAIN_ADDRESS, None, "dex");
        assert_eq!(pda, DEX_BONFIDA_SUBDOMAIN_ADDRESS);
    }
}
