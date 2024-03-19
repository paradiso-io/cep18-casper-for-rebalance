//! Contains definition of the entry points.
use alloc::{boxed::Box, string::String, vec, vec::Vec};

use casper_types::{
    CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key, Parameter,
    U256,
};

use crate::constants::*;

/// Returns the `name` entry point.
pub fn name() -> EntryPoint {
    EntryPoint::new(
        String::from(NAME_ENTRY_POINT_NAME),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `symbol` entry point.
pub fn symbol() -> EntryPoint {
    EntryPoint::new(
        String::from(SYMBOL_ENTRY_POINT_NAME),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `transfer_from` entry point.
pub fn transfer_from() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_FROM_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OWNER, Key::cl_type()),
            Parameter::new(RECIPIENT, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `allowance` entry point.
pub fn allowance() -> EntryPoint {
    EntryPoint::new(
        String::from(ALLOWANCE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OWNER, Key::cl_type()),
            Parameter::new(SPENDER, Key::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `approve` entry point.
pub fn approve() -> EntryPoint {
    EntryPoint::new(
        String::from(APPROVE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(SPENDER, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `increase_allowance` entry point.
pub fn increase_allowance() -> EntryPoint {
    EntryPoint::new(
        String::from(INCREASE_ALLOWANCE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(SPENDER, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `decrease_allowance` entry point.
pub fn decrease_allowance() -> EntryPoint {
    EntryPoint::new(
        String::from(DECREASE_ALLOWANCE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(SPENDER, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `transfer` entry point.
pub fn transfer() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_ENTRY_POINT_NAME),
        vec![
            Parameter::new(RECIPIENT, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `balance_of` entry point.
pub fn balance_of() -> EntryPoint {
    EntryPoint::new(
        String::from(BALANCE_OF_ENTRY_POINT_NAME),
        vec![Parameter::new(ADDRESS, Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `total_supply` entry point.
pub fn total_supply() -> EntryPoint {
    EntryPoint::new(
        String::from(TOTAL_SUPPLY_ENTRY_POINT_NAME),
        Vec::new(),
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `decimals` entry point.
pub fn decimals() -> EntryPoint {
    EntryPoint::new(
        String::from(DECIMALS_ENTRY_POINT_NAME),
        Vec::new(),
        u8::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `burn` entry point.
pub fn burn() -> EntryPoint {
    EntryPoint::new(
        String::from(BURN_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OWNER, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `mint` entry point.
pub fn mint() -> EntryPoint {
    EntryPoint::new(
        String::from(MINT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(RECIPIENT, Key::cl_type()),
            Parameter::new(AMOUNT, U256::cl_type()),
            Parameter::new(SWAP_FEE, U256::cl_type()),
            Parameter::new(MINTID, String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `request_bridge_back` entry point.
pub fn request_bridge_back() -> EntryPoint {
    EntryPoint::new(
        String::from(REQUEST_BRIDGE_BACK_ENTRY_POINT_NAME),
        vec![
            Parameter::new(AMOUNT, U256::cl_type()),
            Parameter::new(FEE, U256::cl_type()),
            Parameter::new(TO_CHAINID, U256::cl_type()),
            Parameter::new(ID, String::cl_type()),
            Parameter::new(RECEIVER_ADDRESS, String::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `change_fee_receiver` entry point.
pub fn change_fee_receiver() -> EntryPoint {
    EntryPoint::new(
        String::from(CHANGE_FEE_RECEIVER_ENTRY_POINT_NAME),
        vec![Parameter::new(FEE_RECEIVER, Key::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `change_fee_receiver` entry point.
pub fn change_swap_fee() -> EntryPoint {
    EntryPoint::new(
        String::from(CHANGE_SWAP_FEE_ENTRY_POINT_NAME),
        vec![Parameter::new(SWAP_FEE, U256::cl_type())],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `set_supported_chains` entry point.
pub fn set_supported_chains() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_SUPPORTED_CHAINS_ENTRY_POINT_NAME),
        vec![
            Parameter::new(SUPPORTED_CHAINS, CLType::List(Box::new(CLType::U256))),
            Parameter::new(IS_SUPPORTED, CLType::Bool),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `set_fee_request_bridge_back` entry point.
pub fn set_fee_request_bridge_back() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_FEE_REQUEST_BRIDGE_BACK_ENTRY_POINT_NAME),
        vec![
            Parameter::new(REQUEST_ID, CLType::U256),
            Parameter::new(FEE, CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `set_fee_request_bridge_back` entry point.
pub fn set_redeen_tokens() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_REDEEM_TOKENS_ENTRY_POINT_NAME),
        vec![
            Parameter::new(REDEEM_TOKENS, CLType::List(Box::new(CLType::Key))),
            Parameter::new(IS_SUPPORTED, CLType::Bool),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `set_fee_request_bridge_back` entry point.
pub fn redeem_to_multichain_token() -> EntryPoint {
    EntryPoint::new(
        String::from(REDEEM_TO_MULTICHAIN_TOKENS_ENTRY_POINT_NAME),
        vec![
            Parameter::new("token_package_hash", CLType::Key),
            Parameter::new(AMOUNT, CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `change_security` entry point.
pub fn change_security() -> EntryPoint {
    EntryPoint::new(
        String::from(CHANGE_SECURITY_ENTRY_POINT_NAME),
        vec![
            // Optional Arguments (can be added or omitted when calling):
            /*
            - "admin_list" : Vec<Key>
            - "mint_and_burn_list" : Vec<Key>
            - "minter_list" : Vec<Key>
            - "burner_list" : Vec<Key>
            - "none_list" : Vec<Key>
            */
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn re_initialize_event_entrypoint() -> EntryPoint {
    EntryPoint::new(
        String::from("re_initialize_event"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `init` entry point.
pub fn init() -> EntryPoint {
    EntryPoint::new(
        String::from(INIT_ENTRY_POINT_NAME),
        Vec::new(),
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of CEP-18 token entry points.
pub fn generate_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(init());
    entry_points.add_entry_point(name());
    entry_points.add_entry_point(symbol());
    entry_points.add_entry_point(decimals());
    entry_points.add_entry_point(total_supply());
    entry_points.add_entry_point(balance_of());
    entry_points.add_entry_point(transfer());
    entry_points.add_entry_point(approve());
    entry_points.add_entry_point(allowance());
    entry_points.add_entry_point(decrease_allowance());
    entry_points.add_entry_point(increase_allowance());
    entry_points.add_entry_point(transfer_from());
    entry_points.add_entry_point(change_security());
    entry_points.add_entry_point(burn());
    entry_points.add_entry_point(mint());
    entry_points.add_entry_point(request_bridge_back());
    entry_points.add_entry_point(change_fee_receiver());
    entry_points.add_entry_point(change_swap_fee());
    entry_points.add_entry_point(set_supported_chains());
    entry_points.add_entry_point(set_fee_request_bridge_back());
    entry_points.add_entry_point(re_initialize_event_entrypoint());
    entry_points.add_entry_point(set_redeen_tokens());
    entry_points.add_entry_point(redeem_to_multichain_token());
    entry_points
}
