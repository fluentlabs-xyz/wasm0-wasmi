#![warn(
    clippy::cast_lossless,
    clippy::missing_errors_doc,
    clippy::used_underscore_binding,
    clippy::redundant_closure_for_method_calls,
    clippy::type_repetition_in_bounds,
    clippy::inconsistent_struct_constructor,
    clippy::default_trait_access,
    clippy::map_unwrap_or,
    clippy::items_after_statements
)]
#![allow(dead_code)]

mod host_error;
mod nan_preserving_float;
mod trap;
mod units;
mod untyped;
mod value;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;

pub use self::{
    host_error::HostError,
    nan_preserving_float::{F32, F64},
    trap::{Trap, TrapCode},
    units::Pages,
    untyped::{DecodeUntypedSlice, EncodeUntypedSlice, UntypedError, UntypedValue},
    value::{
        ArithmeticOps,
        ExtendInto,
        Float,
        Integer,
        LittleEndianConvert,
        LoadInto,
        SignExtendFrom,
        StoreFrom,
        TruncateSaturateInto,
        TryTruncateInto,
        ValueType,
        WrapInto,
    },
};

mod encoding;
mod host_call;
mod linker;
mod meta;
mod module;
mod opcode;
mod utils;

pub use self::{encoding::*, host_call::*, linker::*, meta::*, module::*, opcode::*, untyped::*, utils::*};
use std::{fmt, fmt::Display};

use std::io::Cursor;

#[derive(Debug)]
pub enum WazmError {
    TranslationError,
    MissingEntrypoint,
    NotSupportedOpcode,
    MissingFunction,
    NotSupportedImport,
    NotSupportedMemory(&'static str),
    ParseError(&'static str),
    OutOfBuffer,
    ReachedUnreachable,
    IllegalOpcode(u8),
    ImpossibleJump,
    InternalError(&'static str),
    MemoryOverflow,
    EmptyBytecode,
}

impl Display for WazmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WazmError::TranslationError => write!(f, "translation error"),
            WazmError::MissingEntrypoint => write!(f, "missing entrypoint"),
            WazmError::NotSupportedOpcode => write!(f, "not supported opcode"),
            WazmError::MissingFunction => write!(f, "missing function"),
            WazmError::NotSupportedImport => write!(f, "not supported import"),
            WazmError::NotSupportedMemory(err) => write!(f, "not supported memory ({})", err),
            WazmError::ParseError(err) => write!(f, "parse error ({})", err),
            WazmError::OutOfBuffer => write!(f, "out of buffer"),
            WazmError::ReachedUnreachable => write!(f, "reached unreachable"),
            WazmError::IllegalOpcode(code) => write!(f, "illegal opcode ({})", code),
            WazmError::ImpossibleJump => write!(f, "impossible jump"),
            WazmError::InternalError(err) => write!(f, "internal error ({})", err),
            WazmError::MemoryOverflow => write!(f, "memory overflow"),
            WazmError::EmptyBytecode => write!(f, "empty bytecode"),
        }
    }
}

pub type WazmResult<T> = Result<T, WazmError>;

pub trait BinaryFormat<'a> {
    type SelfType;

    fn write_binary(&self, sink: &mut Vec<u8>) -> WazmResult<()>;
    fn read_binary(sink: &mut Cursor<&'a [u8]>) -> WazmResult<Self::SelfType>;
}

pub const MAX_MEMORY_PAGES: u32 = 512;
pub const MAX_MEMORY_SIZE: u32 = 512 * 0x10000;
