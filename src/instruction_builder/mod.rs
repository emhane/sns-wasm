//! Build instruction to interact with SNS Program.

pub mod create;

pub use create::{domain::CreateDomainInstBuilder, subdomain::CreateSubdomainInstBuilder};
