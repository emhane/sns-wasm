//! Build instruction to create new SNS domain.

use solana_instruction::Instruction;
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;

use crate::{CreateInstBuilder, TLDomain, derive_domain};

/// Builds instruction to register domain.
#[derive(Debug)]
pub struct CreateDomainInstBuilder {
    payer: Pubkey,
    tld: TLDomain,
    owner: Option<Pubkey>,
    class: Option<Pubkey>,
    name: String,
    space: Option<u32>,
}

impl CreateDomainInstBuilder {
    /// Returns new builder for given payer, parent SNS record and user defined name.
    ///
    /// Reserves no extra space for user data like URLs and social media handles. See
    /// [space](Self::space).
    pub fn new(payer: Pubkey, tld: TLDomain, name: String) -> Self {
        Self { payer, tld, owner: None, class: None, name, space: None }
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
        let Self { payer, tld: parent, owner, class, name, space } = self;

        let account = derive_domain(&parent.pda, class.as_ref(), &name);

        let builder =
            CreateInstBuilder { payer, account, parent: parent.into(), owner, class, space };

        builder.build()
    }
}
