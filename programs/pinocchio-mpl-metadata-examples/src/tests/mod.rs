#[cfg(test)]
mod test_cases {
    use mollusk_svm::{program, result::Check, Mollusk};

    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount, WritableAccount},
        instruction::{AccountMeta, Instruction},
        program_option::COption,
        program_pack::Pack,
        pubkey::Pubkey,
    };

    #[test]
    #[ignore = "working"]
    fn initialize_mint() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
            "22222222222222222222222222222222222222222222",
        ));

        let mut mollusk = Mollusk::new(
            &program_id,
            "../../target/deploy/pinocchio_mpl_metadata_examples",
        );

        mollusk.add_program(
            &mpl_token_metadata::ID,
            "src/tests/metadata",
            &mollusk_svm::program::loader_keys::LOADER_V3,
        );
        let (token_program, token_program_account) = (
            spl_token::ID,
            program::create_program_account_loader_v3(&spl_token::ID),
        );

        let (rent_sysvar, rent_sysvar_account) = (
            solana_sdk::sysvar::rent::ID,
            program::create_program_account_loader_v3(&solana_sdk::sysvar::rent::ID),
        );

        // Accounts
        let mint = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();

        // Data
        let data = [vec![0], vec![6], mint_authority.to_bytes().to_vec()].concat();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(mint, false),
                AccountMeta::new_readonly(rent_sysvar, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let mint_lamports = mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN);

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::Some(mint_authority), // 32 + 4 = 36
                supply: 0,                                     // 8
                decimals: 6,                                   // 1
                is_initialized: true,                          // 1
                freeze_authority: COption::Some(mint_authority), // 32 + 4 = 36
            },
            mint_account.data_as_mut_slice(),
        )
        .unwrap();

        let checks = vec![
            Check::success(),
            Check::account(&mint).data(mint_account.data()).build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (
                    mint,
                    AccountSharedData::new(
                        mint_lamports,
                        spl_token::state::Mint::LEN,
                        &spl_token::ID,
                    ),
                ),
                (rent_sysvar, rent_sysvar_account),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    // #[test]
    // #[ignore = "working"]
    // fn initialize_account() {
    //     let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
    //         "22222222222222222222222222222222222222222222",
    //     ));

    //     let mut mollusk = Mollusk::new(
    //         &program_id,
    //         "../../target/deploy/pinocchio_mpl_metadata_examples",
    //     );

    //     mollusk.add_program(
    //         &spl_token::ID,
    //         "src/tests/metadata",
    //         &mollusk_svm::program::loader_keys::LOADER_V3,
    //     );
    //     let (token_program, token_program_account) = (
    //         spl_token::ID,
    //         program::create_program_account_loader_v3(&spl_token::ID),
    //     );

    //     let (rent_sysvar, rent_sysvar_account) = (
    //         solana_sdk::sysvar::rent::ID,
    //         program::create_program_account_loader_v3(&solana_sdk::sysvar::rent::ID),
    //     );

    //     // Accounts
    //     let token = Pubkey::new_unique();
    //     let mint = Pubkey::new_unique();
    //     let owner = Pubkey::new_unique();

    //     // Data
    //     let data = [1];

    //     let mut mint_account = AccountSharedData::new(
    //         mollusk
    //             .sysvars
    //             .rent
    //             .minimum_balance(spl_token::state::Mint::LEN),
    //         spl_token::state::Mint::LEN,
    //         &token_program,
    //     );
    //     solana_sdk::program_pack::Pack::pack(
    //         spl_token::state::Mint {
    //             mint_authority: COption::None,
    //             supply: 100_000_000_000,
    //             decimals: 6,
    //             is_initialized: true,
    //             freeze_authority: COption::None,
    //         },
    //         mint_account.data_as_mut_slice(),
    //     )
    //     .unwrap();

    //     let mut token_account = AccountSharedData::new(
    //         mollusk
    //             .sysvars
    //             .rent
    //             .minimum_balance(spl_token::state::Account::LEN),
    //         spl_token::state::Account::LEN,
    //         &token_program,
    //     );
    //     solana_sdk::program_pack::Pack::pack(
    //         spl_token::state::Account {
    //             mint,
    //             owner,
    //             amount: 0,
    //             delegate: COption::None,
    //             state: AccountState::Initialized,
    //             is_native: COption::None,
    //             delegated_amount: 0,
    //             close_authority: COption::None,
    //         },
    //         token_account.data_as_mut_slice(),
    //     )
    //     .unwrap();

    //     // Instruction
    //     let instruction = Instruction::new_with_bytes(
    //         program_id,
    //         &data,
    //         vec![
    //             AccountMeta::new(token, false),
    //             AccountMeta::new_readonly(mint, false),
    //             AccountMeta::new_readonly(owner, false),
    //             AccountMeta::new_readonly(rent_sysvar, false),
    //             AccountMeta::new_readonly(token_program, false),
    //         ],
    //     );

    //     let check = [
    //         Check::success(),
    //         Check::account(&token).data(token_account.data()).build(),
    //     ];

    //     let token_lamports = mollusk
    //         .sysvars
    //         .rent
    //         .minimum_balance(spl_token::state::Account::LEN);

    //     mollusk.process_and_validate_instruction(
    //         &instruction,
    //         &vec![
    //             (
    //                 token,
    //                 AccountSharedData::new(
    //                     token_lamports,
    //                     spl_token::state::Account::LEN,
    //                     &spl_token::ID,
    //                 ),
    //             ),
    //             (mint, mint_account),
    //             (
    //                 owner,
    //                 AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
    //             ),
    //             (rent_sysvar, rent_sysvar_account),
    //             (token_program, token_program_account),
    //         ],
    //         &check,
    //     );
    // }

    // #[test]
    // #[ignore = "working"]
    // fn transfer() {
    //     let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
    //         "22222222222222222222222222222222222222222222",
    //     ));

    //     let mut mollusk = Mollusk::new(
    //         &program_id,
    //         "../../target/deploy/pinocchio_mpl_metadata_examples",
    //     );

    //     mollusk.add_program(
    //         &spl_token::ID,
    //         "src/tests/metadata",
    //         &mollusk_svm::program::loader_keys::LOADER_V3,
    //     );
    //     let (token_program, token_program_account) = (
    //         spl_token::ID,
    //         program::create_program_account_loader_v3(&spl_token::ID),
    //     );

    //     // Accounts
    //     let from = Pubkey::new_unique();
    //     let mint = Pubkey::new_unique();
    //     let to = Pubkey::new_unique();
    //     let authority = Pubkey::new_unique();

    //     // Data
    //     let data = [vec![3], 1_000_000u64.to_le_bytes().to_vec()].concat();

    //     let mut from_account = AccountSharedData::new(
    //         mollusk
    //             .sysvars
    //             .rent
    //             .minimum_balance(spl_token::state::Account::LEN),
    //         spl_token::state::Account::LEN,
    //         &token_program,
    //     );
    //     solana_sdk::program_pack::Pack::pack(
    //         spl_token::state::Account {
    //             mint,
    //             owner: authority,
    //             amount: 1_000_000,
    //             delegate: COption::None,
    //             state: AccountState::Initialized,
    //             is_native: COption::None,
    //             delegated_amount: 0,
    //             close_authority: COption::None,
    //         },
    //         from_account.data_as_mut_slice(),
    //     )
    //     .unwrap();

    //     let mut to_account = AccountSharedData::new(
    //         mollusk
    //             .sysvars
    //             .rent
    //             .minimum_balance(spl_token::state::Account::LEN),
    //         spl_token::state::Account::LEN,
    //         &token_program,
    //     );
    //     solana_sdk::program_pack::Pack::pack(
    //         spl_token::state::Account {
    //             mint,
    //             owner: Pubkey::default(),
    //             amount: 0,
    //             delegate: COption::None,
    //             state: AccountState::Initialized,
    //             is_native: COption::None,
    //             delegated_amount: 0,
    //             close_authority: COption::None,
    //         },
    //         to_account.data_as_mut_slice(),
    //     )
    //     .unwrap();

    //     let mut new_from_account_data = AccountSharedData::new(
    //         mollusk
    //             .sysvars
    //             .rent
    //             .minimum_balance(spl_token::state::Account::LEN),
    //         spl_token::state::Account::LEN,
    //         &token_program,
    //     );
    //     solana_sdk::program_pack::Pack::pack(
    //         spl_token::state::Account {
    //             mint,
    //             owner: authority,
    //             amount: 0,
    //             delegate: COption::None,
    //             state: AccountState::Initialized,
    //             is_native: COption::None,
    //             delegated_amount: 0,
    //             close_authority: COption::None,
    //         },
    //         new_from_account_data.data_as_mut_slice(),
    //     )
    //     .unwrap();

    //     let mut new_to_account_data = AccountSharedData::new(
    //         mollusk
    //             .sysvars
    //             .rent
    //             .minimum_balance(spl_token::state::Account::LEN),
    //         spl_token::state::Account::LEN,
    //         &token_program,
    //     );
    //     solana_sdk::program_pack::Pack::pack(
    //         spl_token::state::Account {
    //             mint,
    //             owner: Pubkey::default(),
    //             amount: 1_000_000,
    //             delegate: COption::None,
    //             state: AccountState::Initialized,
    //             is_native: COption::None,
    //             delegated_amount: 0,
    //             close_authority: COption::None,
    //         },
    //         new_to_account_data.data_as_mut_slice(),
    //     )
    //     .unwrap();

    //     // Instruction
    //     let instruction = Instruction::new_with_bytes(
    //         program_id,
    //         &data,
    //         vec![
    //             AccountMeta::new(from, false),
    //             AccountMeta::new(to, false),
    //             AccountMeta::new_readonly(authority, true),
    //             AccountMeta::new_readonly(token_program, false),
    //         ],
    //     );

    //     let check = [
    //         Check::success(),
    //         Check::account(&from)
    //             .data(new_from_account_data.data())
    //             .build(),
    //         Check::account(&to).data(new_to_account_data.data()).build(),
    //     ];

    //     mollusk.process_and_validate_instruction(
    //         &instruction,
    //         &vec![
    //             (from, from_account),
    //             (to, to_account),
    //             (
    //                 authority,
    //                 AccountSharedData::new(1_000_000_000, 0, &Pubkey::default()),
    //             ),
    //             (token_program, token_program_account),
    //         ],
    //         &check,
    //     );
    // }
}
