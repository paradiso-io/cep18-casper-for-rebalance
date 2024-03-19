use crate::Event;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use casper_contract::{
    contract_api::{
        self,
        runtime::{self, revert},
        storage::{self, dictionary_get, dictionary_put},
    },
    ext_ffi,
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    api_error,
    bytesrepr::{self, FromBytes, ToBytes},
    runtime_args,
    system::CallStackElement,
    ApiError, CLTyped, ContractPackageHash, Key, RuntimeArgs, URef, U256,
};
use core::convert::TryInto;

use crate::{
    constants::*,
    error::Cep18Error,
    get_dictionary_value_from_key, get_immediate_caller_address,
    make_dictionary_item_key_for_contract, sec_check, write_dictionary_value_from_key,
    SecurityBadge, _mint,
    events::{self, Redeem},
};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

#[no_mangle]
pub extern "C" fn set_redeen_tokens() {
    sec_check(vec![SecurityBadge::Admin]);
    let tokens: Vec<Key> = runtime::get_named_arg(REDEEM_TOKENS);
    let is_supported: bool = runtime::get_named_arg(IS_SUPPORTED);
    for token in tokens {
        let token_dict_key = make_dictionary_item_key_for_contract(token);
        write_dictionary_value_from_key(REDEEM_TOKENS, &token_dict_key, is_supported);
    }
}

pub fn check_redeem_support(token: Key) {
    let token_dict_key = make_dictionary_item_key_for_contract(token);
    if !get_dictionary_value_from_key(REDEEM_TOKENS, &token_dict_key).unwrap_or(false) {
        revert(Cep18Error::NotSupportedToken)
    }
}

#[no_mangle]
pub extern "C" fn redeem_to_multichain_token() {
    let token_package_hash: Key = runtime::get_named_arg("token_package_hash");
    let amount: U256 = runtime::get_named_arg("amount");
    let caller = get_immediate_caller_address().unwrap_or_revert();
    check_redeem_support(token_package_hash);
    // burn old version token
    runtime::call_versioned_contract::<()>(
        token_package_hash.into_hash().unwrap_or_revert().into(),
        None,
        "burn",
        runtime_args! {
            "owner" => caller,
            "amount" => amount
        },
    );

    // mint equivalent multichain token amount
    _mint(caller, U256::zero(), amount);

    // emit event redeem
    events::record_event_dictionary(Event::Redeem(Redeem {
        owner: caller,
        amount,
        old_token_package_hash: token_package_hash,
    }))
}
