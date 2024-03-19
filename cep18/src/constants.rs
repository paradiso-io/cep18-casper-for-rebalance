//! Constants used by the CEP-18 contract.
/// Name of named-key for `name`.
pub const CONTRACT_NAME: &str = "contract_name";

/// Name of named-key for `name`.
pub const NAME: &str = "name";
/// Name of named-key for `symbol`
pub const SYMBOL: &str = "symbol";
/// Name of named-key for `decimals`
pub const DECIMALS: &str = "decimals";
/// Name of dictionary-key for `balances`
pub const BALANCES: &str = "balances";
/// Name of dictionary-key for `allowances`
pub const ALLOWANCES: &str = "allowances";
/// Name of named-key for `total_supply`
pub const TOTAL_SUPPLY: &str = "total_supply";

pub const HASH_KEY_NAME_PREFIX: &str = "cep18_contract_package_";
pub const ACCESS_KEY_NAME_PREFIX: &str = "cep18_contract_package_access_";
pub const CONTRACT_NAME_PREFIX: &str = "cep18_contract_hash_";
pub const CONTRACT_VERSION_PREFIX: &str = "cep18_contract_version_";

/// Name of `name` entry point.
pub const NAME_ENTRY_POINT_NAME: &str = "name";
/// Name of `symbol` entry point.
pub const SYMBOL_ENTRY_POINT_NAME: &str = "symbol";
/// Name of `decimals` entry point.
pub const DECIMALS_ENTRY_POINT_NAME: &str = "decimals";
/// Name of `balance_of` entry point.
pub const BALANCE_OF_ENTRY_POINT_NAME: &str = "balance_of";
/// Name of `transfer` entry point.
pub const TRANSFER_ENTRY_POINT_NAME: &str = "transfer";
/// Name of `approve` entry point.
pub const APPROVE_ENTRY_POINT_NAME: &str = "approve";
/// Name of `allowance` entry point.
pub const ALLOWANCE_ENTRY_POINT_NAME: &str = "allowance";
/// Name of `transfer_from` entry point.
pub const TRANSFER_FROM_ENTRY_POINT_NAME: &str = "transfer_from";
/// Name of `total_supply` entry point.
pub const TOTAL_SUPPLY_ENTRY_POINT_NAME: &str = "total_supply";
/// Name of `mint` entry point.
pub const MINT_ENTRY_POINT_NAME: &str = "mint";
/// Name of `burn` entry point.
pub const BURN_ENTRY_POINT_NAME: &str = "burn";
/// Name of `init` entry point.
pub const INIT_ENTRY_POINT_NAME: &str = "init";
/// Name of `change_security` entry point.
pub const CHANGE_SECURITY_ENTRY_POINT_NAME: &str = "change_security";

pub const INCREASE_ALLOWANCE_ENTRY_POINT_NAME: &str = "increase_allowance";
pub const DECREASE_ALLOWANCE_ENTRY_POINT_NAME: &str = "decrease_allowance";

/// Name of `request_bridge_back` entry point.
pub const REQUEST_BRIDGE_BACK_ENTRY_POINT_NAME: &str = "request_bridge_back";
pub const CHANGE_FEE_RECEIVER_ENTRY_POINT_NAME: &str = "change_fee_receiver";
pub const CHANGE_SWAP_FEE_ENTRY_POINT_NAME: &str = "change_swap_fee";
pub const SET_SUPPORTED_CHAINS_ENTRY_POINT_NAME: &str = "set_supported_chains";
pub const SET_FEE_REQUEST_BRIDGE_BACK_ENTRY_POINT_NAME: &str = "set_fee_request_bridge_back";
pub const SET_REDEEM_TOKENS_ENTRY_POINT_NAME: &str = "set_redeem_tokens";
pub const REDEEM_TO_MULTICHAIN_TOKENS_ENTRY_POINT_NAME: &str = "redeem_to_multichain_token";

/// Name of `address` runtime argument.
pub const ADDRESS: &str = "address";
/// Name of `owner` runtime argument.
pub const OWNER: &str = "owner";
/// Name of `spender` runtime argument.
pub const SPENDER: &str = "spender";
/// Name of `amount` runtime argument.
pub const AMOUNT: &str = "amount";
/// Name of `recipient` runtime argument.
pub const RECIPIENT: &str = "recipient";
pub const PACKAGE_HASH: &str = "package_hash";
pub const EVENTS_MODE: &str = "events_mode";
pub const SECURITY_BADGES: &str = "security_badges";
pub const ADMIN_LIST: &str = "admin_list";
pub const MINTER_LIST: &str = "minter_list";
pub const BURNER_LIST: &str = "burner_list";
pub const NONE_LIST: &str = "none_list";
pub const MINT_AND_BURN_LIST: &str = "mint_and_burn_list";
pub const ENABLE_MINT_BURN: &str = "enable_mint_burn";
pub const SWAP_FEE: &str = "swap_fee";
pub const MINTIDS: &str = "mintids";
pub const MINTID: &str = "mintid";
pub const REQUEST_ID: &str = "request_id";
pub const REQUEST_MAP: &str = "request_map";
pub const FEE_RECEIVER: &str = "fee_receiver";
pub const FEE: &str = "fee";
pub const TO_CHAINID: &str = "to_chainid";
pub const RECEIVER_ADDRESS: &str = "receiver_address";
pub const ID: &str = "id";
pub const CONTRACT_OWNER_KEY_NAME: &str = "contract_owner";
pub const ARG_CONTRACT_OWNER: &str = "contract_owner";
pub const TRANSFER_OWNER_ENTRY_POINT_NAME: &str = "transfer_owner";
pub const SUPPORTED_CHAINS: &str = "supported_chains";
pub const IS_SUPPORTED: &str = "is_supported";
pub const REQUEST_INFO: &str = "request_info";
pub const REDEEM_TOKENS: &str = "redeem_tokens";
