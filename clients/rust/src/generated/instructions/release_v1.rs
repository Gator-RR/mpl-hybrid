//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

#[cfg(feature = "anchor")]
use anchor_lang::prelude::{AnchorDeserialize, AnchorSerialize};
#[cfg(not(feature = "anchor"))]
use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct ReleaseV1 {
    pub owner: solana_program::pubkey::Pubkey,

    pub authority: solana_program::pubkey::Pubkey,

    pub escrow: solana_program::pubkey::Pubkey,

    pub asset: solana_program::pubkey::Pubkey,

    pub collection: solana_program::pubkey::Pubkey,

    pub user_token_account: solana_program::pubkey::Pubkey,

    pub escrow_token_account: solana_program::pubkey::Pubkey,

    pub token: solana_program::pubkey::Pubkey,

    pub fee_token_account: solana_program::pubkey::Pubkey,

    pub fee_sol_account: solana_program::pubkey::Pubkey,

    pub fee_project_account: solana_program::pubkey::Pubkey,

    pub recent_blockhashes: solana_program::pubkey::Pubkey,

    pub mpl_core: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub associated_token_program: solana_program::pubkey::Pubkey,
}

impl ReleaseV1 {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(16 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.owner, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.escrow,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.asset, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.collection,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.escrow_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.fee_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.fee_sol_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.fee_project_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.recent_blockhashes,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.mpl_core,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.associated_token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = ReleaseV1InstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::MPL_HYBRID_ID,
            accounts,
            data,
        }
    }
}

#[cfg_attr(not(feature = "anchor"), derive(BorshSerialize, BorshDeserialize))]
#[cfg_attr(feature = "anchor", derive(AnchorSerialize, AnchorDeserialize))]
pub struct ReleaseV1InstructionData {
    discriminator: [u8; 8],
}

impl ReleaseV1InstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [86, 208, 216, 30, 127, 65, 71, 80],
        }
    }
}

/// Instruction builder for `ReleaseV1`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` owner
///   1. `[writable]` authority
///   2. `[writable]` escrow
///   3. `[writable]` asset
///   4. `[writable]` collection
///   5. `[writable]` user_token_account
///   6. `[writable]` escrow_token_account
///   7. `[]` token
///   8. `[writable]` fee_token_account
///   9. `[writable, optional]` fee_sol_account (default to `GjF4LqmEhV33riVyAwHwiEeAHx4XXFn2yMY3fmMigoP3`)
///   10. `[writable]` fee_project_account
///   11. `[optional]` recent_blockhashes (default to `SysvarS1otHashes111111111111111111111111111`)
///   12. `[optional]` mpl_core (default to `CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d`)
///   13. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   14. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   15. `[optional]` associated_token_program (default to `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`)
#[derive(Default)]
pub struct ReleaseV1Builder {
    owner: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    escrow: Option<solana_program::pubkey::Pubkey>,
    asset: Option<solana_program::pubkey::Pubkey>,
    collection: Option<solana_program::pubkey::Pubkey>,
    user_token_account: Option<solana_program::pubkey::Pubkey>,
    escrow_token_account: Option<solana_program::pubkey::Pubkey>,
    token: Option<solana_program::pubkey::Pubkey>,
    fee_token_account: Option<solana_program::pubkey::Pubkey>,
    fee_sol_account: Option<solana_program::pubkey::Pubkey>,
    fee_project_account: Option<solana_program::pubkey::Pubkey>,
    recent_blockhashes: Option<solana_program::pubkey::Pubkey>,
    mpl_core: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    associated_token_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ReleaseV1Builder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.owner = Some(owner);
        self
    }
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    #[inline(always)]
    pub fn escrow(&mut self, escrow: solana_program::pubkey::Pubkey) -> &mut Self {
        self.escrow = Some(escrow);
        self
    }
    #[inline(always)]
    pub fn asset(&mut self, asset: solana_program::pubkey::Pubkey) -> &mut Self {
        self.asset = Some(asset);
        self
    }
    #[inline(always)]
    pub fn collection(&mut self, collection: solana_program::pubkey::Pubkey) -> &mut Self {
        self.collection = Some(collection);
        self
    }
    #[inline(always)]
    pub fn user_token_account(
        &mut self,
        user_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.user_token_account = Some(user_token_account);
        self
    }
    #[inline(always)]
    pub fn escrow_token_account(
        &mut self,
        escrow_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.escrow_token_account = Some(escrow_token_account);
        self
    }
    #[inline(always)]
    pub fn token(&mut self, token: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token = Some(token);
        self
    }
    #[inline(always)]
    pub fn fee_token_account(
        &mut self,
        fee_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.fee_token_account = Some(fee_token_account);
        self
    }
    /// `[optional account, default to 'GjF4LqmEhV33riVyAwHwiEeAHx4XXFn2yMY3fmMigoP3']`
    #[inline(always)]
    pub fn fee_sol_account(
        &mut self,
        fee_sol_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.fee_sol_account = Some(fee_sol_account);
        self
    }
    #[inline(always)]
    pub fn fee_project_account(
        &mut self,
        fee_project_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.fee_project_account = Some(fee_project_account);
        self
    }
    /// `[optional account, default to 'SysvarS1otHashes111111111111111111111111111']`
    #[inline(always)]
    pub fn recent_blockhashes(
        &mut self,
        recent_blockhashes: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.recent_blockhashes = Some(recent_blockhashes);
        self
    }
    /// `[optional account, default to 'CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d']`
    #[inline(always)]
    pub fn mpl_core(&mut self, mpl_core: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mpl_core = Some(mpl_core);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// `[optional account, default to 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL']`
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.associated_token_program = Some(associated_token_program);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = ReleaseV1 {
            owner: self.owner.expect("owner is not set"),
            authority: self.authority.expect("authority is not set"),
            escrow: self.escrow.expect("escrow is not set"),
            asset: self.asset.expect("asset is not set"),
            collection: self.collection.expect("collection is not set"),
            user_token_account: self
                .user_token_account
                .expect("user_token_account is not set"),
            escrow_token_account: self
                .escrow_token_account
                .expect("escrow_token_account is not set"),
            token: self.token.expect("token is not set"),
            fee_token_account: self
                .fee_token_account
                .expect("fee_token_account is not set"),
            fee_sol_account: self.fee_sol_account.unwrap_or(solana_program::pubkey!(
                "GjF4LqmEhV33riVyAwHwiEeAHx4XXFn2yMY3fmMigoP3"
            )),
            fee_project_account: self
                .fee_project_account
                .expect("fee_project_account is not set"),
            recent_blockhashes: self.recent_blockhashes.unwrap_or(solana_program::pubkey!(
                "SysvarS1otHashes111111111111111111111111111"
            )),
            mpl_core: self.mpl_core.unwrap_or(solana_program::pubkey!(
                "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
            )),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
            associated_token_program: self.associated_token_program.unwrap_or(
                solana_program::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            ),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `release_v1` CPI accounts.
pub struct ReleaseV1CpiAccounts<'a, 'b> {
    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub escrow: &'b solana_program::account_info::AccountInfo<'a>,

    pub asset: &'b solana_program::account_info::AccountInfo<'a>,

    pub collection: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub escrow_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token: &'b solana_program::account_info::AccountInfo<'a>,

    pub fee_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub fee_sol_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub fee_project_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub recent_blockhashes: &'b solana_program::account_info::AccountInfo<'a>,

    pub mpl_core: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `release_v1` CPI instruction.
pub struct ReleaseV1Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub escrow: &'b solana_program::account_info::AccountInfo<'a>,

    pub asset: &'b solana_program::account_info::AccountInfo<'a>,

    pub collection: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub escrow_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub token: &'b solana_program::account_info::AccountInfo<'a>,

    pub fee_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub fee_sol_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub fee_project_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub recent_blockhashes: &'b solana_program::account_info::AccountInfo<'a>,

    pub mpl_core: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> ReleaseV1Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ReleaseV1CpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            owner: accounts.owner,
            authority: accounts.authority,
            escrow: accounts.escrow,
            asset: accounts.asset,
            collection: accounts.collection,
            user_token_account: accounts.user_token_account,
            escrow_token_account: accounts.escrow_token_account,
            token: accounts.token,
            fee_token_account: accounts.fee_token_account,
            fee_sol_account: accounts.fee_sol_account,
            fee_project_account: accounts.fee_project_account,
            recent_blockhashes: accounts.recent_blockhashes,
            mpl_core: accounts.mpl_core,
            system_program: accounts.system_program,
            token_program: accounts.token_program,
            associated_token_program: accounts.associated_token_program,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(16 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.owner.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.escrow.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.asset.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.collection.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.escrow_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.fee_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.fee_sol_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.fee_project_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.recent_blockhashes.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.mpl_core.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.associated_token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = ReleaseV1InstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::MPL_HYBRID_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(16 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.owner.clone());
        account_infos.push(self.authority.clone());
        account_infos.push(self.escrow.clone());
        account_infos.push(self.asset.clone());
        account_infos.push(self.collection.clone());
        account_infos.push(self.user_token_account.clone());
        account_infos.push(self.escrow_token_account.clone());
        account_infos.push(self.token.clone());
        account_infos.push(self.fee_token_account.clone());
        account_infos.push(self.fee_sol_account.clone());
        account_infos.push(self.fee_project_account.clone());
        account_infos.push(self.recent_blockhashes.clone());
        account_infos.push(self.mpl_core.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.associated_token_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `ReleaseV1` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` owner
///   1. `[writable]` authority
///   2. `[writable]` escrow
///   3. `[writable]` asset
///   4. `[writable]` collection
///   5. `[writable]` user_token_account
///   6. `[writable]` escrow_token_account
///   7. `[]` token
///   8. `[writable]` fee_token_account
///   9. `[writable]` fee_sol_account
///   10. `[writable]` fee_project_account
///   11. `[]` recent_blockhashes
///   12. `[]` mpl_core
///   13. `[]` system_program
///   14. `[]` token_program
///   15. `[]` associated_token_program
pub struct ReleaseV1CpiBuilder<'a, 'b> {
    instruction: Box<ReleaseV1CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ReleaseV1CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ReleaseV1CpiBuilderInstruction {
            __program: program,
            owner: None,
            authority: None,
            escrow: None,
            asset: None,
            collection: None,
            user_token_account: None,
            escrow_token_account: None,
            token: None,
            fee_token_account: None,
            fee_sol_account: None,
            fee_project_account: None,
            recent_blockhashes: None,
            mpl_core: None,
            system_program: None,
            token_program: None,
            associated_token_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn owner(&mut self, owner: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.owner = Some(owner);
        self
    }
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    #[inline(always)]
    pub fn escrow(
        &mut self,
        escrow: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.escrow = Some(escrow);
        self
    }
    #[inline(always)]
    pub fn asset(&mut self, asset: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.asset = Some(asset);
        self
    }
    #[inline(always)]
    pub fn collection(
        &mut self,
        collection: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.collection = Some(collection);
        self
    }
    #[inline(always)]
    pub fn user_token_account(
        &mut self,
        user_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token_account = Some(user_token_account);
        self
    }
    #[inline(always)]
    pub fn escrow_token_account(
        &mut self,
        escrow_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.escrow_token_account = Some(escrow_token_account);
        self
    }
    #[inline(always)]
    pub fn token(&mut self, token: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.token = Some(token);
        self
    }
    #[inline(always)]
    pub fn fee_token_account(
        &mut self,
        fee_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.fee_token_account = Some(fee_token_account);
        self
    }
    #[inline(always)]
    pub fn fee_sol_account(
        &mut self,
        fee_sol_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.fee_sol_account = Some(fee_sol_account);
        self
    }
    #[inline(always)]
    pub fn fee_project_account(
        &mut self,
        fee_project_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.fee_project_account = Some(fee_project_account);
        self
    }
    #[inline(always)]
    pub fn recent_blockhashes(
        &mut self,
        recent_blockhashes: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.recent_blockhashes = Some(recent_blockhashes);
        self
    }
    #[inline(always)]
    pub fn mpl_core(
        &mut self,
        mpl_core: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.mpl_core = Some(mpl_core);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn associated_token_program(
        &mut self,
        associated_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.associated_token_program = Some(associated_token_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = ReleaseV1Cpi {
            __program: self.instruction.__program,

            owner: self.instruction.owner.expect("owner is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            escrow: self.instruction.escrow.expect("escrow is not set"),

            asset: self.instruction.asset.expect("asset is not set"),

            collection: self.instruction.collection.expect("collection is not set"),

            user_token_account: self
                .instruction
                .user_token_account
                .expect("user_token_account is not set"),

            escrow_token_account: self
                .instruction
                .escrow_token_account
                .expect("escrow_token_account is not set"),

            token: self.instruction.token.expect("token is not set"),

            fee_token_account: self
                .instruction
                .fee_token_account
                .expect("fee_token_account is not set"),

            fee_sol_account: self
                .instruction
                .fee_sol_account
                .expect("fee_sol_account is not set"),

            fee_project_account: self
                .instruction
                .fee_project_account
                .expect("fee_project_account is not set"),

            recent_blockhashes: self
                .instruction
                .recent_blockhashes
                .expect("recent_blockhashes is not set"),

            mpl_core: self.instruction.mpl_core.expect("mpl_core is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),

            associated_token_program: self
                .instruction
                .associated_token_program
                .expect("associated_token_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct ReleaseV1CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    escrow: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    asset: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collection: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    escrow_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    fee_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    fee_sol_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    fee_project_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    recent_blockhashes: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mpl_core: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    associated_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
