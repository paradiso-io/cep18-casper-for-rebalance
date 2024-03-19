use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{
    runtime_args, ApiError, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
};

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
fn should_redeem() {
    let mint_amount = U256::from(1000000000);

    let (mut builder, TestContext { cep18_token, .. }) = setup();
    println!("a");
    println!("{}", *DEFAULT_ACCOUNT_ADDR);
    // upgrade
    let args = runtime_args! {
        ARG_NAME => TOKEN_NAME,
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS_SIX,
        ARG_TOTAL_SUPPLY => U256::from("1000000000000"),
        EVENTS_MODE => 1_u8,
        ENABLE_MINT_BURN =>1_u8,
        ADMIN_LIST => vec![Key::from(*DEFAULT_ACCOUNT_ADDR)],
        MINTER_LIST => vec![Key::from(*DEFAULT_ACCOUNT_ADDR)],
        SWAP_FEE => U256::from(0),
        FEE_RECEIVER => TOKEN_OWNER_ADDRESS_1,
        SUPPORTED_CHAINS => vec![U256::from(97),U256::from(43113)],
        "re_initialize_event" => true
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

    // deploy multichain token
    let args = runtime_args! {
        ARG_NAME => "MultiChainToken",
        ARG_SYMBOL => TOKEN_SYMBOL,
        ARG_DECIMALS => TOKEN_DECIMALS_SIX,
        ARG_TOTAL_SUPPLY => U256::from("1000000000000"),
        EVENTS_MODE => 1_u8,
        ENABLE_MINT_BURN =>1_u8,
        ADMIN_LIST => vec![Key::from(*DEFAULT_ACCOUNT_ADDR)],
        MINTER_LIST => vec![Key::from(*DEFAULT_ACCOUNT_ADDR)],
        SWAP_FEE => U256::from(0),
        FEE_RECEIVER => TOKEN_OWNER_ADDRESS_1,
        SUPPORTED_CHAINS => vec![U256::from(97),U256::from(43113)],
        "re_initialize_event" => true

    };
    deploy_cep18(&mut builder, args);
    println!("done deploy upgrage");

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let old_token_pk_hash = account
        .named_keys()
        .get("cep18_contract_package_CasperTest")
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have contract package hash");

    println!("{:?}", &Key::from(old_token_pk_hash).to_formatted_string());

    let multichain_token = account
        .named_keys()
        .get("cep18_contract_hash_MultiChainToken")
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let multichain_token_pk_hash = account
        .named_keys()
        .get("cep18_contract_package_MultiChainToken")
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have contract package hash");

    // set redeem enable

    let set_redeem_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        multichain_token,
        "set_redeem_tokens",
        runtime_args! {
            "redeem_tokens" => vec![Key::from(old_token_pk_hash)],
            "is_supported"=> true

        },
    )
    .build();

    builder.exec(set_redeem_request).expect_success().commit();

    println!("done set support redeem");

    // approve

    let amount_redeem = U256::from(100000000);
    let approve_redeem_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep18_token,
        "approve",
        runtime_args! {
            "spender" => Key::from(multichain_token_pk_hash),
            "amount"=> amount_redeem

        },
    )
    .build();

    builder
        .exec(approve_redeem_request)
        .expect_success()
        .commit();

    println!("before redeem");

    let old_balance_before = cep18_check_balance_of(
        &mut builder,
        &cep18_token,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );

    println!("Old token: balance default acc  {:?}", old_balance_before);
    let new_balance_before = cep18_check_balance_of(
        &mut builder,
        &multichain_token,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );

    println!(
        "Multichain token: balance default acc  {:?}",
        new_balance_before
    );

    let redeem_request = ExecuteRequestBuilder::contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        multichain_token,
        "redeem_to_multichain_token",
        runtime_args! {
            "amount" => amount_redeem,
            "token_package_hash" => Key::from(old_token_pk_hash),
        },
    )
    .build();

    builder.exec(redeem_request).expect_success().commit();

    let old_balance_after = cep18_check_balance_of(
        &mut builder,
        &cep18_token,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );

    println!("Old token: balance default acc  {:?}", old_balance_after);
    let new_balance_after = cep18_check_balance_of(
        &mut builder,
        &multichain_token,
        Key::Account(*DEFAULT_ACCOUNT_ADDR),
    );

    println!(
        "Multichain token: balance default acc  {:?}",
        new_balance_after
    );

    assert_eq!(
        old_balance_after,
        old_balance_before.checked_sub(amount_redeem).unwrap()
    );
    assert_eq!(
        new_balance_after,
        new_balance_before.checked_add(amount_redeem).unwrap()
    );

    println!("Done set fee");
}
