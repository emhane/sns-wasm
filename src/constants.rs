//! Solana public keys involved in SNS.

use solana_address::address;
use solana_hash::Hash;
use solana_pubkey::{Pubkey, pubkey};

/// Solana Name Service (SNS) Program ID.
///
/// <https://explorer.solana.com/address/namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX>
pub const SNS_PROGRAM_ID: Pubkey = address!("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");

/// Root SNS node PDA.
///
/// Specs: <https://sns.guide/domain-name/domain-tld.html>
pub const ROOT_TLD_ADDRESS: Pubkey = pubkey!("ZoAhWEqTVqHVqupYmEanDobY7dee5YKbQox9BNASZzU");

/// TLD `.sol`. A PDA derived from [`ROOT_TLD_ADDRESS`] and [`SNS_PROGRAM_ID`] (see
/// [`derive_tld`](crate::derive_tld).
///
/// Bonfida Root Domain Account is public name of address on Mainnet.
///
/// Specs: <https://sns.guide/domain-name/domain-tld.html>
pub const SOL_TLD_ADDRESS: Pubkey = pubkey!("58PwtjSDuFHuUkYjH9BYnnQKHfwo9reZhC2zMJv9JPkx");

/// Bonfida Reverse Lookup Class is owner of `.sol` ([`SOL_TLD_ADDRESS`]) on Mainnet.
///
/// Note: System Program owns [`SOL_TLD_ADDRESS`] on Devnet.
pub const SOL_TLD_OWNER_ADDRESS_MAINNET: Pubkey =
    pubkey!("33m47vH6Eav6jr5Ry86XjhRft2jRBLDnDgPSHoquXi2Z");

/// Owner of SNS root [`ROOT_TLD_ADDRESS`] on Mainnet is wallet address.
pub const ROOT_TLD_OWNER_ADDRESS_MAINNET: Pubkey =
    pubkey!("3Wnd5Df69KitZfUoPYZU438eFRNwGHkhLnSAWL65PxJX");

/// Hashed `.sol` TLD name is `sha256` hash of [`HASH_PREFIX`] + `.sol`.
pub const SOL_TLD_NAME_HASH: Hash = Hash::new_from_array([
    232, 184, 39, 63, 48, 202, 46, 190, 215, 38, 192, 99, 147, 180, 70, 162, 246, 19, 112, 11, 204,
    167, 171, 81, 170, 134, 183, 94, 166, 73, 107, 132,
]);

/// Prefix concatenated with user given name before hashing, in address derivation process. See
/// [`derive_tld`](crate::derive_tld), [`derive_domain`](crate::derive_domain) and
/// [`derive_subdomain`](crate::derive_subdomain).
pub const HASH_PREFIX: &str = "SPL Name Service";

/// Minimum length of SNS record is length of header, 96 bytes, containing:
///
/// - parent PDA
/// - owner address
/// - class address
pub const SNS_RECORD_HEADER_BYTE_LEN: usize = 96;
