use crate::{Index, WazmError, WazmResult};
use lazy_static::lazy_static;
use std::collections::HashMap;

// EVM-compatible host functions (starts with 0xEE00)
pub const IMPORT_EVM_STOP: Index = Index(0xEE01);
pub const IMPORT_EVM_RETURN: Index = Index(0xEE02);
pub const IMPORT_EVM_KECCAK256: Index = Index(0xEE03);
pub const IMPORT_EVM_ADDRESS: Index = Index(0xEE04);
pub const IMPORT_EVM_BALANCE: Index = Index(0xEE05);
pub const IMPORT_EVM_ORIGIN: Index = Index(0xEE06);
pub const IMPORT_EVM_CALLER: Index = Index(0xEE07);
pub const IMPORT_EVM_CALLVALUE: Index = Index(0xEE08);
pub const IMPORT_EVM_CALLDATALOAD: Index = Index(0xEE09);
pub const IMPORT_EVM_CALLDATASIZE: Index = Index(0xEE0A);
pub const IMPORT_EVM_CALLDATACOPY: Index = Index(0xEE0B);
pub const IMPORT_EVM_CODESIZE: Index = Index(0xEE0C);
pub const IMPORT_EVM_CODECOPY: Index = Index(0xEE0D);
pub const IMPORT_EVM_GASPRICE: Index = Index(0xEE0E);
pub const IMPORT_EVM_EXTCODESIZE: Index = Index(0xEE0F);
pub const IMPORT_EVM_EXTCODECOPY: Index = Index(0xEE10);
pub const IMPORT_EVM_EXTCODEHASH: Index = Index(0xEE11);
pub const IMPORT_EVM_RETURNDATASIZE: Index = Index(0xEE12);
pub const IMPORT_EVM_RETURNDATACOPY: Index = Index(0xEE13);
pub const IMPORT_EVM_BLOCKHASH: Index = Index(0xEE14);
pub const IMPORT_EVM_COINBASE: Index = Index(0xEE15);
pub const IMPORT_EVM_TIMESTAMP: Index = Index(0xEE16);
pub const IMPORT_EVM_NUMBER: Index = Index(0xEE17);
pub const IMPORT_EVM_DIFFICULTY: Index = Index(0xEE18);
pub const IMPORT_EVM_GASLIMIT: Index = Index(0xEE19);
pub const IMPORT_EVM_CHAINID: Index = Index(0xEE1A);
pub const IMPORT_EVM_BASEFEE: Index = Index(0xEE1B);
pub const IMPORT_EVM_SLOAD: Index = Index(0xEE1C);
pub const IMPORT_EVM_SSTORE: Index = Index(0xEE1D);
pub const IMPORT_EVM_LOG0: Index = Index(0xEE1E);
pub const IMPORT_EVM_LOG1: Index = Index(0xEE1F);
pub const IMPORT_EVM_LOG2: Index = Index(0xEE20);
pub const IMPORT_EVM_LOG3: Index = Index(0xEE21);
pub const IMPORT_EVM_LOG4: Index = Index(0xEE22);
pub const IMPORT_EVM_CREATE: Index = Index(0xEE23);
pub const IMPORT_EVM_CALL: Index = Index(0xEE24);
pub const IMPORT_EVM_CALLCODE: Index = Index(0xEE25);
pub const IMPORT_EVM_DELEGATECALL: Index = Index(0xEE26);
pub const IMPORT_EVM_CREATE2: Index = Index(0xEE27);
pub const IMPORT_EVM_STATICCALL: Index = Index(0xEE28);
pub const IMPORT_EVM_REVERT: Index = Index(0xEE29);
pub const IMPORT_EVM_SELFDESTRUCT: Index = Index(0xEE2A);

lazy_static! {
    static ref WAZM_CIRCUITS: HashMap<(&'static str, &'static str), Index> = HashMap::from([
        (("env", "_evm_stop"), IMPORT_EVM_STOP),
        (("env", "_evm_return"), IMPORT_EVM_RETURN),
        (("env", "_evm_keccak256"), IMPORT_EVM_KECCAK256),
        (("env", "_evm_address"), IMPORT_EVM_ADDRESS),
        (("env", "_evm_balance"), IMPORT_EVM_BALANCE),
        (("env", "_evm_origin"), IMPORT_EVM_ORIGIN),
        (("env", "_evm_caller"), IMPORT_EVM_CALLER),
        (("env", "_evm_callvalue"), IMPORT_EVM_CALLVALUE),
        (("env", "_evm_calldataload"), IMPORT_EVM_CALLDATALOAD),
        (("env", "_evm_calldatasize"), IMPORT_EVM_CALLDATASIZE),
        (("env", "_evm_calldatacopy"), IMPORT_EVM_CALLDATACOPY),
        (("env", "_evm_codesize"), IMPORT_EVM_CODESIZE),
        (("env", "_evm_codecopy"), IMPORT_EVM_CODECOPY),
        (("env", "_evm_gasprice"), IMPORT_EVM_GASPRICE),
        (("env", "_evm_extcodesize"), IMPORT_EVM_EXTCODESIZE),
        (("env", "_evm_extcodecopy"), IMPORT_EVM_EXTCODECOPY),
        (("env", "_evm_extcodehash"), IMPORT_EVM_EXTCODEHASH),
        (("env", "_evm_returndatasize"), IMPORT_EVM_RETURNDATASIZE),
        (("env", "_evm_returndatacopy"), IMPORT_EVM_RETURNDATACOPY),
        (("env", "_evm_blockhash"), IMPORT_EVM_BLOCKHASH),
        (("env", "_evm_coinbase"), IMPORT_EVM_COINBASE),
        (("env", "_evm_timestamp"), IMPORT_EVM_TIMESTAMP),
        (("env", "_evm_number"), IMPORT_EVM_NUMBER),
        (("env", "_evm_difficulty"), IMPORT_EVM_DIFFICULTY),
        (("env", "_evm_gaslimit"), IMPORT_EVM_GASLIMIT),
        (("env", "_evm_chainid"), IMPORT_EVM_CHAINID),
        (("env", "_evm_basefee"), IMPORT_EVM_BASEFEE),
        (("env", "_evm_sload"), IMPORT_EVM_SLOAD),
        (("env", "_evm_sstore"), IMPORT_EVM_SSTORE),
        (("env", "_evm_log0"), IMPORT_EVM_LOG0),
        (("env", "_evm_log1"), IMPORT_EVM_LOG1),
        (("env", "_evm_log2"), IMPORT_EVM_LOG2),
        (("env", "_evm_log3"), IMPORT_EVM_LOG3),
        (("env", "_evm_log4"), IMPORT_EVM_LOG4),
        (("env", "_evm_create"), IMPORT_EVM_CREATE),
        (("env", "_evm_call"), IMPORT_EVM_CALL),
        (("env", "_evm_callcode"), IMPORT_EVM_CALLCODE),
        (("env", "_evm_delegatecall"), IMPORT_EVM_DELEGATECALL),
        (("env", "_evm_create2"), IMPORT_EVM_CREATE2),
        (("env", "_evm_staticcall"), IMPORT_EVM_STATICCALL),
        (("env", "_evm_revert"), IMPORT_EVM_REVERT),
        (("env", "_evm_selfdestruct"), IMPORT_EVM_SELFDESTRUCT),
    ]);
}

pub fn resolve_host_call(module: &str, name: &str) -> WazmResult<Index> {
    Ok(*WAZM_CIRCUITS
        .get(&(module, name))
        .ok_or(WazmError::NotSupportedImport)?)
}
