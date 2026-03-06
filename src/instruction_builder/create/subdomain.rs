//! Build instruction to create new SNS subdomain.

use solana_instruction::Instruction;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{CreateInstBuilder, Domain, SNSNodeWithOwner, derive_subdomain};

/// Builds instruction to register subdomain.
#[derive(Debug)]
pub struct CreateSubdomainInstBuilder {
    payer: Pubkey,
    domain: SNSNodeWithOwner,
    owner: Option<Pubkey>,
    class: Option<Pubkey>,
    name: String,
    space: Option<u32>,
}

impl CreateSubdomainInstBuilder {
    /// Returns new builder for given payer, parent SNS record and user defined name.
    ///
    /// Reserves no extra space for user data like URLs and social media handles. See
    /// [space](Self::space).
    pub fn new(payer: Pubkey, domain: Domain, name: String) -> Self {
        Self { domain: domain.into(), payer, owner: None, class: None, name, space: None }
    }

    /// Sets owner to register for SNS record.
    pub fn owner(mut self, owner: Pubkey) -> Self {
        self.owner = Some(owner);
        self
    }

    /// Sets class to derive SNS record address from.
    pub fn class(mut self, class: Pubkey) -> Self {
        self.class = Some(class);
        self
    }

    /// Sets additional blockchain space to allocate for SNS record other than minimum reserved for
    /// [`NameRecordHeader::LEN`].
    ///
    /// Registering domains at <https://v1.sns.id> allocates 10 KB extra.
    pub fn space(mut self, space: u32) -> Self {
        self.space = Some(space);
        self
    }

    /// Builds instruction to input into transaction to create new SNS domain.
    pub fn build(self) -> Result<Instruction, ProgramError> {
        let Self { domain: parent, payer, owner, class, name, space } = self;

        // derive subdomain PDA
        let account = derive_subdomain(&parent.pda, class.as_ref(), &name);

        let builder = CreateInstBuilder { payer, account, parent, owner, class, space };

        builder.build()
    }
}
