#![no_std]
#![no_main]

// Fork from release v1.0.2
extern crate alloc;

mod allowances;
mod balances;
pub mod constants;
pub mod entry_points;
mod error;
mod events;
mod modalities;
mod redeem;
mod utils;
use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use allowances::{get_allowances_uref, read_allowance_from, write_allowance_to};
use balances::{get_balances_uref, read_balance_from, transfer_balance, write_balance_to};
use casper_contract::{
    contract_api::{
        runtime::{self, get_caller, get_key, get_named_arg, put_key, revert},
        storage::{self, dictionary_put},
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys, runtime_args, CLValue, ContractHash, ContractPackageHash, Key,
    RuntimeArgs, U256,
};
use entry_points::generate_entry_points;

use constants::*;
pub use error::Cep18Error;
use events::{
    init_events, Burn, ChangeSecurity, DecreaseAllowance, Event, IncreaseAllowance, Mint,
    ParadisoMint, RegisterBridgeBack, RequestBridgeBack, SetAllowance, SetFeeBridgeBack, Transfer,
    TransferFrom,
};
use utils::*;

#[no_mangle]
pub extern "C" fn name() {
    runtime::ret(CLValue::from_t(utils::read_from::<String>(NAME)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol() {
    runtime::ret(CLValue::from_t(utils::read_from::<String>(SYMBOL)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals() {
    runtime::ret(CLValue::from_t(utils::read_from::<u8>(DECIMALS)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    runtime::ret(CLValue::from_t(utils::read_from::<U256>(TOTAL_SUPPLY)).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Key = runtime::get_named_arg(ADDRESS);
    let balances_uref = get_balances_uref();
    let balance = balances::read_balance_from(balances_uref, address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn allowance() {
    let spender: Key = runtime::get_named_arg(SPENDER);
    let owner: Key = runtime::get_named_arg(OWNER);
    let allowances_uref = get_allowances_uref();
    let val: U256 = read_allowance_from(allowances_uref, owner, spender);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn approve() {
    let owner = utils::get_immediate_caller_address().unwrap_or_revert();
    let spender: Key = runtime::get_named_arg(SPENDER);
    if spender == owner {
        revert(Cep18Error::CannotTargetSelfUser);
    }
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let allowances_uref = get_allowances_uref();
    write_allowance_to(allowances_uref, owner, spender, amount);
    events::record_event_dictionary(Event::SetAllowance(SetAllowance {
        owner,
        spender,
        allowance: amount,
    }))
}

#[no_mangle]
pub extern "C" fn decrease_allowance() {
    let owner = utils::get_immediate_caller_address().unwrap_or_revert();
    let spender: Key = runtime::get_named_arg(SPENDER);
    if spender == owner {
        revert(Cep18Error::CannotTargetSelfUser);
    }
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let allowances_uref = get_allowances_uref();
    let current_allowance = read_allowance_from(allowances_uref, owner, spender);
    let new_allowance = current_allowance.saturating_sub(amount);
    write_allowance_to(allowances_uref, owner, spender, new_allowance);
    events::record_event_dictionary(Event::DecreaseAllowance(DecreaseAllowance {
        owner,
        spender,
        decr_by: amount,
        allowance: new_allowance,
    }))
}

#[no_mangle]
pub extern "C" fn increase_allowance() {
    let owner = utils::get_immediate_caller_address().unwrap_or_revert();
    let spender: Key = runtime::get_named_arg(SPENDER);
    if spender == owner {
        revert(Cep18Error::CannotTargetSelfUser);
    }
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let allowances_uref = get_allowances_uref();
    let current_allowance = read_allowance_from(allowances_uref, owner, spender);
    let new_allowance = current_allowance.saturating_add(amount);
    write_allowance_to(allowances_uref, owner, spender, new_allowance);
    events::record_event_dictionary(Event::IncreaseAllowance(IncreaseAllowance {
        owner,
        spender,
        allowance: new_allowance,
        inc_by: amount,
    }))
}

#[no_mangle]
pub extern "C" fn transfer() {
    let sender = utils::get_immediate_caller_address().unwrap_or_revert();
    let recipient: Key = runtime::get_named_arg(RECIPIENT);
    if sender == recipient {
        revert(Cep18Error::CannotTargetSelfUser);
    }
    let amount: U256 = runtime::get_named_arg(AMOUNT);

    transfer_balance(sender, recipient, amount).unwrap_or_revert();
    events::record_event_dictionary(Event::Transfer(Transfer {
        sender,
        recipient,
        amount,
    }))
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let spender = utils::get_immediate_caller_address().unwrap_or_revert();
    let recipient: Key = runtime::get_named_arg(RECIPIENT);
    let owner: Key = runtime::get_named_arg(OWNER);
    if owner == recipient {
        revert(Cep18Error::CannotTargetSelfUser);
    }
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    if amount.is_zero() {
        return;
    }

    let allowances_uref = get_allowances_uref();
    let spender_allowance: U256 = read_allowance_from(allowances_uref, owner, spender);
    let new_spender_allowance = spender_allowance
        .checked_sub(amount)
        .ok_or(Cep18Error::InsufficientAllowance)
        .unwrap_or_revert();

    transfer_balance(owner, recipient, amount).unwrap_or_revert();
    write_allowance_to(allowances_uref, owner, spender, new_spender_allowance);
    events::record_event_dictionary(Event::TransferFrom(TransferFrom {
        spender,
        owner,
        recipient,
        amount,
    }))
}

#[no_mangle]
pub extern "C" fn mint() {
    if 0 == read_from::<u8>(ENABLE_MINT_BURN) {
        revert(Cep18Error::MintBurnDisabled);
    }

    sec_check(vec![SecurityBadge::Minter]);
    // sec_check(vec![SecurityBadge::Admin, SecurityBadge::Minter]);

    let recipient: Key = runtime::get_named_arg(RECIPIENT);
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let swap_fee_in: U256 = runtime::get_named_arg(SWAP_FEE);
    let mintid: String = runtime::get_named_arg(MINTID);
    let mintid_value = read_mintids(mintid.clone());
    if mintid_value > 0 {
        runtime::revert(Cep18Error::AlreadyMint);
    }
    save_mintids(mintid.clone());
    let swap_fee = read_swap_fee();
    if swap_fee != swap_fee_in {
        runtime::revert(Cep18Error::InvalidFee);
    }
    if amount < swap_fee {
        runtime::revert(Cep18Error::MintTooLow);
    }
    _mint(recipient, swap_fee, amount);

    events::record_event_dictionary(Event::ParadisoMint(ParadisoMint {
        recipient,
        amount,
        mintid,
    }));
}

fn _mint(recipient: Key, swap_fee: U256, amount: U256) {
    let balances_uref = get_balances_uref();
    let total_supply_uref = get_total_supply_uref();
    let mut new_balance = {
        let balance = read_balance_from(balances_uref, recipient);
        balance
            .checked_add(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    new_balance = new_balance
        .checked_sub(swap_fee)
        .ok_or(Cep18Error::Overflow)
        .unwrap_or_revert();

    let new_total_supply = {
        let total_supply: U256 = read_total_supply_from(total_supply_uref);
        total_supply
            .checked_add(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    let fee_receiver = read_fee_receiver();
    let new_dev_balance = {
        let balance = read_balance_from(balances_uref, fee_receiver);
        balance
            .checked_add(swap_fee)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    write_balance_to(balances_uref, fee_receiver, new_dev_balance);
    write_balance_to(balances_uref, recipient, new_balance);
    write_total_supply_to(total_supply_uref, new_total_supply);
    events::record_event_dictionary(Event::Mint(Mint { recipient, amount }))
}

#[no_mangle]
pub extern "C" fn request_bridge_back() {
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    let fee: U256 = runtime::get_named_arg(FEE);
    let to_chainid: U256 = runtime::get_named_arg(TO_CHAINID);
    let id: String = runtime::get_named_arg(ID);
    let receiver_address: String = runtime::get_named_arg(RECEIVER_ADDRESS);
    require(
        check_supported_chain(to_chainid),
        Cep18Error::UnsupportedChainIdForRequestBridgeBack,
    );
    if fee != read_swap_fee() {
        runtime::revert(Cep18Error::InvalidFee);
    }
    if id.chars().count() != 64 {
        runtime::revert(Cep18Error::RequestIdIllFormatted);
    }
    if hex::decode(&id).is_err() {
        runtime::revert(Cep18Error::RequestIdIllFormatted);
    }

    //read request map
    let request_map_result = read_request_map(id.clone());
    if request_map_result != U256::zero() {
        runtime::revert(Cep18Error::RequestIdExist);
    }
    //check whether id is used
    let val = read_request_id();
    let next_index = val + U256::one();

    save_request_id(next_index);
    save_request_map(id, val);
    let _owner = utils::get_immediate_caller_address().unwrap_or_revert();

    // save request info
    let request_info = utils::RequestBridgeBackInfo {
        owner: _owner,
        amount,
        fee,
        index: val,
        receiver_address: receiver_address.clone(),
        to_chainid,
    };
    let _request_amount_after_fee = {
        amount
            .checked_sub(fee)
            .ok_or(Cep18Error::RequestAmountTooLow)
            .unwrap_or_revert()
    };

    // transfer token to contract
    transfer_balance(_owner, get_self_key(), amount).unwrap_or_revert();
    events::record_event_dictionary(Event::Transfer(Transfer {
        sender: _owner,
        recipient: get_self_key(),
        amount,
    }));

    save_request_info(val, &request_info);

    // emit event

    events::record_event_dictionary(Event::RegisterBridgeBack(RegisterBridgeBack {
        owner: _owner,
        amount,
        index: val,
        receiver_address,
        to_chainid,
    }))
}

#[no_mangle]
pub extern "C" fn re_initialize_event() {
    sec_check(vec![SecurityBadge::Admin]);
    runtime::remove_key("__events_length");
    runtime::remove_key("__events_schema");
    runtime::remove_key("__events");
    runtime::remove_key("__events_ces_version");
    init_events();
}

#[no_mangle]
pub extern "C" fn set_fee_request_bridge_back() {
    sec_check(vec![SecurityBadge::Admin]);
    let request_id: U256 = runtime::get_named_arg(REQUEST_ID);
    let fee: U256 = runtime::get_named_arg(FEE);
    let request_info = read_request_info(request_id);
    let total_fee = fee + request_info.fee;
    let request_amount = request_info.amount;

    let request_amount_after_fee = {
        request_amount
            .checked_sub(total_fee)
            .ok_or(Cep18Error::RequestAmountTooLow)
            .unwrap_or_revert()
    };
    // //transfer fee to dev
    transfer_balance(get_self_key(), read_fee_receiver(), total_fee).unwrap_or_revert();
    events::record_event_dictionary(Event::Transfer(Transfer {
        sender: get_self_key(),
        recipient: read_fee_receiver(),
        amount: total_fee,
    }));

    // burn remaining
    burn_token(get_self_key(), request_amount_after_fee);
    events::record_event_dictionary(Event::RequestBridgeBack(RequestBridgeBack {
        owner: request_info.owner,
        amount: request_amount_after_fee,
        fee: total_fee,
        index: request_id,
        receiver_address: request_info.receiver_address,
        to_chainid: request_info.to_chainid,
    }));

    events::record_event_dictionary(Event::SetFeeBridgeBack(SetFeeBridgeBack {
        request_id,
        fee,
    }));
}

#[no_mangle]
pub extern "C" fn burn() {
    if 0 == read_from::<u8>(ENABLE_MINT_BURN) {
        revert(Cep18Error::MintBurnDisabled);
    }
    let owner: Key = runtime::get_named_arg(OWNER);
    if owner != get_immediate_caller_address().unwrap_or_revert() {
        revert(Cep18Error::InvalidBurnTarget);
    }
    let amount: U256 = runtime::get_named_arg(AMOUNT);
    burn_token(owner, amount);
}

pub(crate) fn burn_token(owner: Key, amount: U256) {
    let balances_uref = get_balances_uref();
    let total_supply_uref = get_total_supply_uref();
    let new_balance = {
        let balance = read_balance_from(balances_uref, owner);
        balance
            .checked_sub(amount)
            .ok_or(Cep18Error::InsufficientBalance)
            .unwrap_or_revert()
    };
    let new_total_supply = {
        let total_supply = read_total_supply_from(total_supply_uref);
        total_supply
            .checked_sub(amount)
            .ok_or(Cep18Error::Overflow)
            .unwrap_or_revert()
    };
    write_balance_to(balances_uref, owner, new_balance);
    write_total_supply_to(total_supply_uref, new_total_supply);
    events::record_event_dictionary(Event::Burn(Burn { owner, amount }));
}

/// Initiates the contracts states. Only used by the installer call,
/// later calls will cause it to revert.
#[no_mangle]
pub extern "C" fn init() {
    if get_key(ALLOWANCES).is_some() {
        revert(Cep18Error::AlreadyInitialized);
    }
    let package_hash = get_named_arg::<Key>(PACKAGE_HASH);
    put_key(PACKAGE_HASH, package_hash);
    storage::new_dictionary(ALLOWANCES).unwrap_or_revert();
    let balances_uref = storage::new_dictionary(BALANCES).unwrap_or_revert();
    let initial_supply = runtime::get_named_arg(TOTAL_SUPPLY);
    // DTO- rebalancing adding
    storage::new_dictionary(MINTIDS).unwrap_or_revert();
    storage::new_dictionary(REQUEST_MAP).unwrap_or_revert();
    storage::new_dictionary(REQUEST_INFO).unwrap_or_revert();
    storage::new_dictionary(REDEEM_TOKENS).unwrap_or_revert();

    let supported_chains_dict = storage::new_dictionary(SUPPORTED_CHAINS).unwrap_or_revert();
    let caller = get_caller();
    write_balance_to(balances_uref, caller.into(), initial_supply);
    let security_badges_dict = storage::new_dictionary(SECURITY_BADGES).unwrap_or_revert();
    dictionary_put(
        security_badges_dict,
        &make_dictionary_item_key_for_account(Key::from(get_caller())),
        SecurityBadge::Admin,
    );

    let admin_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(ADMIN_LIST, Cep18Error::InvalidAdminList);
    let minter_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(MINTER_LIST, Cep18Error::InvalidMinterList);

    init_events();

    if let Some(minter_list) = minter_list {
        for minter in minter_list {
            dictionary_put(
                security_badges_dict,
                &make_dictionary_item_key_for_account(minter),
                SecurityBadge::Minter,
            );
        }
    }

    if let Some(admin_list) = admin_list {
        for admin in admin_list {
            dictionary_put(
                security_badges_dict,
                &make_dictionary_item_key_for_account(admin),
                SecurityBadge::Admin,
            );
        }
    }
    // set supported chains
    let supported_chains: Vec<U256> = get_named_arg(SUPPORTED_CHAINS);
    for chain in supported_chains {
        dictionary_put(supported_chains_dict, &chain.to_string(), true);
    }
}

/// Admin EntryPoint to manipulate the security access granted to users.
/// One user can only possess one access group badge.
/// Change strength: None > Admin > Minter
/// Change strength meaning by example: If user is added to both Minter and Admin they will be an
/// Admin, also if a user is added to Admin and None then they will be removed from having rights.
/// Beware: do not remove the last Admin because that will lock out all admin functionality.
#[no_mangle]
pub extern "C" fn change_security() {
    if 0 == read_from::<u8>(ENABLE_MINT_BURN) {
        revert(Cep18Error::MintBurnDisabled);
    }
    sec_check(vec![SecurityBadge::Admin]);
    let admin_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(ADMIN_LIST, Cep18Error::InvalidAdminList);
    let minter_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(MINTER_LIST, Cep18Error::InvalidMinterList);
    let none_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(NONE_LIST, Cep18Error::InvalidNoneList);

    let mut badge_map: BTreeMap<Key, SecurityBadge> = BTreeMap::new();
    if let Some(minter_list) = minter_list {
        for account_key in minter_list {
            badge_map.insert(account_key, SecurityBadge::Minter);
        }
    }
    if let Some(admin_list) = admin_list {
        for account_key in admin_list {
            badge_map.insert(account_key, SecurityBadge::Admin);
        }
    }
    if let Some(none_list) = none_list {
        for account_key in none_list {
            badge_map.insert(account_key, SecurityBadge::None);
        }
    }

    let caller = get_immediate_caller_address().unwrap_or_revert();
    badge_map.remove(&caller);

    utils::change_sec_badge(&badge_map);
    events::record_event_dictionary(Event::ChangeSecurity(ChangeSecurity {
        admin: utils::get_immediate_caller_address().unwrap_or_revert(),
        sec_change_map: badge_map,
    }));
}

#[no_mangle]
pub extern "C" fn change_fee_receiver() {
    sec_check(vec![SecurityBadge::Admin]);
    let fee_receiver: Key = runtime::get_named_arg(FEE_RECEIVER);
    save_fee_receiver(fee_receiver);
}
#[no_mangle]
pub extern "C" fn change_swap_fee() {
    sec_check(vec![SecurityBadge::Admin]);
    let swap_fee: U256 = runtime::get_named_arg(SWAP_FEE);
    save_swap_fee(swap_fee);
}

#[no_mangle]
pub extern "C" fn set_supported_chains() {
    sec_check(vec![SecurityBadge::Admin]);
    let supported_chains: Vec<U256> = runtime::get_named_arg(SUPPORTED_CHAINS);
    let is_supported: bool = runtime::get_named_arg(IS_SUPPORTED);
    for chain in supported_chains {
        write_dictionary_value_from_key(SUPPORTED_CHAINS, &chain.to_string(), is_supported);
    }
}

#[no_mangle]
pub extern "C" fn migrate() {}

pub fn install_contract() {
    let name: String = runtime::get_named_arg(NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL);
    let decimals: u8 = runtime::get_named_arg(DECIMALS);
    let total_supply: U256 = runtime::get_named_arg(TOTAL_SUPPLY);
    let events_mode: u8 =
        utils::get_optional_named_arg_with_user_errors(EVENTS_MODE, Cep18Error::InvalidEventsMode)
            .unwrap_or(0u8);
    let admin_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(ADMIN_LIST, Cep18Error::InvalidAdminList);
    let minter_list: Option<Vec<Key>> =
        utils::get_optional_named_arg_with_user_errors(MINTER_LIST, Cep18Error::InvalidMinterList);

    let enable_mint_burn: u8 = utils::get_optional_named_arg_with_user_errors(
        ENABLE_MINT_BURN,
        Cep18Error::InvalidEnableMBFlag,
    )
    .unwrap_or(0);
    // Paradiso rebalance added
    let swap_fee: U256 = runtime::get_named_arg(SWAP_FEE);
    let fee_receiver: Key = runtime::get_named_arg(FEE_RECEIVER);
    let supported_chains: Vec<U256> = runtime::get_named_arg(SUPPORTED_CHAINS);

    let mut named_keys = NamedKeys::new();
    named_keys.insert(NAME.to_string(), storage::new_uref(name.clone()).into());
    named_keys.insert(SYMBOL.to_string(), storage::new_uref(symbol).into());
    named_keys.insert(DECIMALS.to_string(), storage::new_uref(decimals).into());
    named_keys.insert(
        TOTAL_SUPPLY.to_string(),
        storage::new_uref(total_supply).into(),
    );
    named_keys.insert(
        EVENTS_MODE.to_string(),
        storage::new_uref(events_mode).into(),
    );
    named_keys.insert(
        ENABLE_MINT_BURN.to_string(),
        storage::new_uref(enable_mint_burn).into(),
    );
    named_keys.insert(SWAP_FEE.to_string(), storage::new_uref(swap_fee).into());
    named_keys.insert(
        REQUEST_ID.to_string(),
        storage::new_uref(U256::zero()).into(),
    );
    named_keys.insert(
        FEE_RECEIVER.to_string(),
        storage::new_uref(fee_receiver).into(),
    );

    let entry_points = generate_entry_points();

    let hash_key_name = format!("{HASH_KEY_NAME_PREFIX}{name}");

    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(hash_key_name.clone()),
        Some(format!("{ACCESS_KEY_NAME_PREFIX}{name}")),
    );
    let package_hash = runtime::get_key(&hash_key_name).unwrap_or_revert();

    // Store contract_hash and contract_version under the keys CONTRACT_NAME and CONTRACT_VERSION
    runtime::put_key(
        &format!("{CONTRACT_NAME_PREFIX}{name}"),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{CONTRACT_VERSION_PREFIX}{name}"),
        storage::new_uref(contract_version).into(),
    );
    // Call contract to initialize it
    let mut init_args = runtime_args! {TOTAL_SUPPLY => total_supply, PACKAGE_HASH => package_hash, SUPPORTED_CHAINS => supported_chains};

    if let Some(admin_list) = admin_list {
        init_args.insert(ADMIN_LIST, admin_list).unwrap_or_revert();
    }
    if let Some(minter_list) = minter_list {
        init_args
            .insert(MINTER_LIST, minter_list)
            .unwrap_or_revert();
    }

    runtime::call_contract::<()>(contract_hash, INIT_ENTRY_POINT_NAME, init_args);
}

#[no_mangle]
pub extern "C" fn call() {
    // contract_name should be the name of cep18 token
    let name: String = runtime::get_named_arg(NAME);
    let hash_key_name = format!("{HASH_KEY_NAME_PREFIX}{name}");

    if !runtime::has_key(&hash_key_name) {
        // install contract
        install_contract()
    } else {
        // upgrade contract
        // let hash_key_name = format!("{HASH_KEY_NAME_PREFIX}{name}");
        let package_hash: ContractPackageHash = runtime::get_key(&hash_key_name)
            .unwrap_or_revert()
            .into_hash()
            .unwrap()
            .into();
        let old_contract_hash: ContractHash =
            runtime::get_key(&format!("{CONTRACT_NAME_PREFIX}{name}"))
                .unwrap_or_revert()
                .into_hash()
                .unwrap()
                .into();
        let (contract_hash, contract_version) =
            storage::add_contract_version(package_hash, generate_entry_points(), NamedKeys::new());

        let re_initialize_event: bool = runtime::get_named_arg("re_initialize_event");
        if re_initialize_event {
            let _: () = runtime::call_versioned_contract(
                package_hash,
                None,
                "re_initialize_event",
                runtime_args! {},
            );
        }

        storage::disable_contract_version(package_hash, old_contract_hash).unwrap_or_revert();

        runtime::put_key(
            &format!("{CONTRACT_NAME_PREFIX}{name}"),
            contract_hash.into(),
        );
        runtime::put_key(
            &format!("{CONTRACT_VERSION_PREFIX}{name}"),
            storage::new_uref(contract_version).into(),
        );
    }
}
