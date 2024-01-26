use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, ApiError, Key, RuntimeArgs, U256};

use crate::utility::{
    constants::*,
    installer_request_builders::{
        cep18_check_balance_of, cep18_check_total_supply, deploy_cep18, setup, setup_with_args,
        TestContext,
    },
};

use casper_execution_engine::core::{
    engine_state::Error as CoreError, execution::Error as ExecError,
};

#[test]
fn test_mint_and_burn_tokens() {
    let mint_amount = U256::one();

    let (mut builder, TestContext { cep18_token, .. }) = setup();
    println!("a");
    println!("{}", *DEFAULT_ACCOUNT_ADDR);
    // upgrade
    let args = runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        EVENTS_MODE => 1_u8,
        ENABLE_MINT_BURN =>1_u8,
        ADMIN_LIST => vec![Key::from(*DEFAULT_ACCOUNT_ADDR)],
        MINTER_LIST => vec![Key::from(*DEFAULT_ACCOUNT_ADDR)],
        SWAP_FEE => U256::from(0),
        FEE_RECEIVER => TOKEN_OWNER_ADDRESS_1,
        SUPPORTED_CHAINS => vec![U256::from(97),U256::from(43113)]

    };
    deploy_cep18(&mut builder, args);
    println!("done deploy upgrage");

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {RECIPIENT => TOKEN_OWNER_ADDRESS_1, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_1), SWAP_FEE => U256::zero(), MINTID => "123".to_string()},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();
    let mint_request_2 = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {RECIPIENT => TOKEN_OWNER_ADDRESS_2, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_2),SWAP_FEE => U256::zero(), MINTID => "1234".to_string()},
    )
    .build();
    builder.exec(mint_request_2).expect_success().commit();
    println!(
        "mint gas {:?}",
        builder.last_exec_gas_cost().value().as_u128() / 1_000_000_000_u128
    );

    assert_eq!(
        cep18_check_balance_of(
            &mut builder,
            &cep18_token,
            Key::Account(*DEFAULT_ACCOUNT_ADDR)
        ),
        U256::from(TOKEN_TOTAL_SUPPLY),
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_1),
        U256::from(TOKEN_OWNER_AMOUNT_1)
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_2),
        U256::from(TOKEN_OWNER_AMOUNT_2)
    );
    let total_supply_before_mint = cep18_check_total_supply(&mut builder, &cep18_token);

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {
            RECIPIENT => TOKEN_OWNER_ADDRESS_1,
            ARG_AMOUNT => mint_amount,
            SWAP_FEE => U256::zero(),
            MINTID => "0x703ed6891a882d9d4821367b3ab62ebdebaa87303c6d1642874212e7519a81eb-43113-96945816564243-83-0x6fa6fa85d692f6956064c398c3918a4bff2c1de3-43113".to_string()

        },
    )
    .build();

    builder.exec(mint_request).expect_success().commit();

    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_1),
        U256::from(TOKEN_OWNER_AMOUNT_1) + mint_amount,
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_2),
        U256::from(TOKEN_OWNER_AMOUNT_2)
    );

    let total_supply_after_mint = cep18_check_total_supply(&mut builder, &cep18_token);
    assert_eq!(
        total_supply_after_mint,
        total_supply_before_mint + mint_amount,
    );
    let total_supply_before_burn = total_supply_after_mint;

    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_BURN,
        runtime_args! {
            ARG_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(burn_request).expect_success().commit();

    assert_eq!(
        cep18_check_balance_of(
            &mut builder,
            &cep18_token,
            Key::Account(*DEFAULT_ACCOUNT_ADDR)
        ),
        U256::from(999999999),
    );
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_2),
        U256::from(TOKEN_OWNER_AMOUNT_2)
    );
    let total_supply_after_burn = cep18_check_total_supply(&mut builder, &cep18_token);
    assert_eq!(
        total_supply_after_burn,
        total_supply_before_burn - mint_amount,
    );

    assert_eq!(total_supply_after_burn, total_supply_before_mint);
    println!("before request back");
    let request_bridge_back = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        "request_bridge_back",
        runtime_args! {
            "amount" => mint_amount,
            "fee" => U256::zero(),
            "to_chainid" => U256::from(97),
            "id" => "636363".to_string(),
            "receiver_address"=> "0x000000000".to_string()

        },
    )
    .build();

    builder.exec(request_bridge_back).expect_success().commit();
    println!("Done request_bridge_back");
    let set_supported_chains = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        "set_supported_chains",
        runtime_args! {
            "supported_chains" => vec![U256::from(1), U256::from(97)],
            "is_supported"=> false

        },
    )
    .build();

    builder.exec(set_supported_chains).expect_success().commit();
    println!("Done set_supported_chains")
}

#[test]
fn test_should_not_mint_above_limits() {
    let mint_amount = U256::MAX;

    let (mut builder, TestContext { cep18_token, .. }) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        "enable_mint_burn" => true,
    });

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {OWNER => TOKEN_OWNER_ADDRESS_1, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_1)},
    )
    .build();
    builder.exec(mint_request).expect_success().commit();
    let mint_request_2 = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {OWNER => TOKEN_OWNER_ADDRESS_2, AMOUNT => U256::from(TOKEN_OWNER_AMOUNT_2)},
    )
    .build();
    builder.exec(mint_request_2).expect_success().commit();
    assert_eq!(
        cep18_check_balance_of(&mut builder, &cep18_token, TOKEN_OWNER_ADDRESS_1),
        U256::from(TOKEN_OWNER_AMOUNT_1)
    );

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {
            ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_OVERFLOW),
        "{:?}",
        error
    );
}

#[test]
fn test_should_not_burn_above_balance() {
    let (mut builder, TestContext { cep18_token, .. }) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        "enable_mint_burn" => true,
    });

    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_BURN,
        runtime_args! {
            ARG_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_AMOUNT => U256::from(TOKEN_TOTAL_SUPPLY)+1,
        },
    )
    .build();

    builder.exec(burn_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == ERROR_INSUFFICIENT_BALANCE),
        "{:?}",
        error
    );
}

#[test]
fn test_should_not_mint_or_burn_with_entrypoint_disabled() {
    let mint_amount = U256::one();

    let (mut builder, TestContext { cep18_token, .. }) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        ENABLE_MINT_BURN => false,
    });

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {
            ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60016),
        "{:?}",
        error
    );

    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_BURN,
        runtime_args! {
            ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(burn_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60016),
        "{:?}",
        error
    );
}

#[test]
fn test_security_no_rights() {
    let mint_amount = U256::one();

    let (mut builder, TestContext { cep18_token, .. }) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        ENABLE_MINT_BURN => true,
    });

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {
            ARG_OWNER => Key::Account(*ACCOUNT_1_ADDR),
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60010),
        "{:?}",
        error
    );

    let passing_admin_mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {
            ARG_OWNER => Key::Account(*ACCOUNT_1_ADDR),
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder
        .exec(passing_admin_mint_request)
        .expect_success()
        .commit();

    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        cep18_token,
        METHOD_BURN,
        runtime_args! {
            ARG_OWNER => Key::Account(*ACCOUNT_1_ADDR),
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(burn_request).expect_success().commit();
}

#[test]
fn test_security_minter_rights() {
    let mint_amount = U256::one();

    let (mut builder, TestContext { cep18_token, .. }) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        ENABLE_MINT_BURN => true,
        MINTER_LIST => vec![Key::Account(*ACCOUNT_1_ADDR)]
    });

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {
            ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).commit().expect_success();
}

#[test]
fn test_security_burner_rights() {
    let mint_amount = U256::one();

    let (mut builder, TestContext { cep18_token, .. }) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        ENABLE_MINT_BURN => true,
    });

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {
            ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60010),
        "{:?}",
        error
    );

    // mint by admin
    let working_mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {
            ARG_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(working_mint_request).commit().expect_success();

    // any user can burn
    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_BURN,
        runtime_args! {
            ARG_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(burn_request).commit().expect_success();
}

#[test]
fn test_change_security() {
    let mint_amount = U256::one();

    let (mut builder, TestContext { cep18_token, .. }) = setup_with_args(runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS,
        ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
        ENABLE_MINT_BURN => true,
        ADMIN_LIST => vec![Key::Account(*ACCOUNT_1_ADDR)]
    });

    let change_security_request = ExecuteRequestBuilder::contract_call_by_hash(
        *ACCOUNT_1_ADDR,
        cep18_token,
        CHANGE_SECURITY,
        runtime_args! {
            NONE_LIST => vec![Key::Account(*DEFAULT_ACCOUNT_ADDR)],
        },
    )
    .build();

    builder
        .exec(change_security_request)
        .commit()
        .expect_success();

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        METHOD_MINT,
        runtime_args! {
            ARG_OWNER => TOKEN_OWNER_ADDRESS_1,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).commit();

    let error = builder.get_error().expect("should have error");
    assert!(
        matches!(error, CoreError::Exec(ExecError::Revert(ApiError::User(user_error))) if user_error == 60010),
        "{:?}",
        error
    );
}
