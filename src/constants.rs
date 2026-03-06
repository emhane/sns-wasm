//! Solana public keys involved in SNS.

use solana_hash::Hash;
use solana_pubkey::{Pubkey, pubkey};

/// Solana Name Service (SNS) Program ID.
///
/// <https://explorer.solana.com/address/namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX>
pub const SNS_PROGRAM_ID: Pubkey = pubkey!("namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX");

/// Root SNS node. Address is owned by Name Service Program ([`SNS_PROGRAM_ID`]).
///
/// Specs: <https://sns.guide/domain-name/domain-tld.html>
pub const ROOT_TLD_ADDRESS: Pubkey = pubkey!("ZoAhWEqTVqHVqupYmEanDobY7dee5YKbQox9BNASZzU");

/// TLD `.sol`. A PDA derived from [`ROOT_TLD_ADDRESS`] and [`SNS_PROGRAM_ID`] (see
/// [`derive_tld`](crate::derive_tld). Address is owned by Name Service Program
/// ([`SNS_PROGRAM_ID`]).
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

/// Hashed `.sol` TLD name is `sha256` hash of
/// [`HASH_PREFIX`](spl_name_service::state::HASH_PREFIX) + `.sol`.
pub const SOL_TLD_NAME_HASH: Hash = Hash::new_from_array([
    232, 184, 39, 63, 48, 202, 46, 190, 215, 38, 192, 99, 147, 180, 70, 162, 246, 19, 112, 11, 204,
    167, 171, 81, 170, 134, 183, 94, 166, 73, 107, 132,
]);
