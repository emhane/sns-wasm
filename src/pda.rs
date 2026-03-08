//! Generate programmatically derived addresses (PDAs).

use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use solana_address::Address;
use solana_hash::Hash;

use crate::{HASH_PREFIX, ROOT_TLD_ADDRESS, SNS_PROGRAM_ID, SOL_TLD_ADDRESS, SOL_TLD_NAME_HASH};

/// Programmatically Derived Address (PDA) of SNS record, and its hashed name (created as input to
/// derivation process).
#[derive(Debug, Clone, Copy, Constructor, Serialize, Deserialize)]
pub struct SNSNode {
    /// PDA of SNS record. Derived from hashed user given name, parent PDA and
    /// class address.
    ///
    /// See [`derive_tld`], [`derive_domain`] and [`derive_subdomain`].
    pub pda: Address,
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
pub fn name_hash(name: &str) -> Hash {
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
pub fn derive_tld(class: Option<&Address>, name: &str) -> SNSNode {
    let dot_name = format!(".{name}");
    let hashed_tld_name = name_hash(&dot_name);

    let (tld, _) = get_seeds_and_key(
        &SNS_PROGRAM_ID,
        hashed_tld_name.to_bytes().to_vec(),
        class,
        Some(&ROOT_TLD_ADDRESS),
    );

    SNSNode::new(tld, hashed_tld_name)
}

/// Returns PDA of domain with given name and TLD.
///
/// Spec: <https://sns.guide/domain-name/domain-tld.html>
pub fn derive_domain(tld: &Address, class: Option<&Address>, name: &str) -> SNSNode {
    let hashed_name = name_hash(name);
    let (domain, _) =
        get_seeds_and_key(&SNS_PROGRAM_ID, hashed_name.to_bytes().to_vec(), class, Some(tld));

    SNSNode::new(domain, hashed_name)
}

/// Returns PDA of subdomain with given name and parent.
///
/// Adds '\0' prefix to given name.
///
/// <https://sns.guide/domain-name/domain-tld.html>
pub fn derive_subdomain(parent: &Address, class: Option<&Address>, name: &str) -> SNSNode {
    let name_dot = format!("\0{name}");
    let hashed_subdomain_name = name_hash(&name_dot);

    let (subdomain, _) = get_seeds_and_key(
        &SNS_PROGRAM_ID,
        hashed_subdomain_name.to_bytes().to_vec(),
        class,
        Some(parent),
    );

    SNSNode::new(subdomain, hashed_subdomain_name)
}

/// Code ported from archived <https://github.com/solana-labs/solana-program-library>
pub fn get_seeds_and_key(
    program_id: &Address,
    hashed_name: Vec<u8>, // Hashing is done off-chain
    name_class_opt: Option<&Address>,
    parent_name_address_opt: Option<&Address>,
) -> (Address, Vec<u8>) {
    let mut seeds_vec: Vec<u8> = hashed_name;

    let name_class = name_class_opt.cloned().unwrap_or_default();

    for b in name_class.to_bytes() {
        seeds_vec.push(b);
    }

    let parent_name_address = parent_name_address_opt.cloned().unwrap_or_default();

    for b in parent_name_address.to_bytes() {
        seeds_vec.push(b);
    }

    let (name_account_key, bump) =
        Address::find_program_address(&seeds_vec.chunks(32).collect::<Vec<&[u8]>>(), program_id);
    seeds_vec.push(bump);

    (name_account_key, seeds_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SOL_TLD_ADDRESS;
    use solana_address::address;

    const BONFIDA_DOMAIN_ADDRESS: Address =
        address!("Crf8hzfthWGbGbLTVCiqRqV5MVnbpHB1L9KQMd6gsinb");
    const DEX_BONFIDA_SUBDOMAIN_ADDRESS: Address =
        address!("HoFfFXqFHAC8RP3duuQNzag1ieUwJRBv1HtRNiWFq4Qu");

    #[test]
    fn test_sol_hashed_name() {
        let hash = name_hash(".sol");
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
