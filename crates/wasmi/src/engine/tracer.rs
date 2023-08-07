use core::fmt::{Debug, Formatter};
use std::cell::RefCell;
use std::collections::BTreeMap;

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use wasmi_core::UntypedValue;

use crate::engine::bytecode::{InstrMeta, Instruction};
use crate::engine::opcode::OpCode;
use crate::Extern;

#[derive(Debug, Clone)]
pub struct MemoryState {
    pub offset: u32,
    pub len: u32,
    pub data: Vec<u8>,
}

impl Serialize for MemoryState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MemoryState", 3)?;
        s.serialize_field("offset", &self.offset)?;
        s.serialize_field("len", &self.len)?;
        s.serialize_field("data", &hex::encode(&self.data))?;
        s.end()
    }
}

#[derive(Debug, Clone)]
pub struct OpCodeState {
    pub program_counter: u32,
    pub opcode: OpCode,
    pub memory_changes: Vec<MemoryState>,
    pub stack: Vec<u64>,
    pub source_pc: u32,
    pub code: u16,
}

impl Serialize for OpCodeState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("OpCodeState", 9)?;
        s.serialize_field("pc", &self.program_counter)?;
        s.serialize_field("source_pc", &self.source_pc)?;
        s.serialize_field("name", self.opcode.name())?;
        s.serialize_field("opcode", &self.code)?;
        if let Some(drop_keep) = self.opcode.drop_keep() {
            s.serialize_field("stack_drop", &drop_keep.drop())?;
            s.serialize_field("stack_keep", &drop_keep.keep())?;
        }
        if let Some(params) = self.opcode.params() {
            s.serialize_field("params", &params)?;
        }
        if self.memory_changes.len() > 0 {
            s.serialize_field("memory_changes", &self.memory_changes)?;
        }
        if self.stack.len() > 0 {
            s.serialize_field("stack", &self.stack)?;
        }
        s.end()
    }
}

#[derive(Debug)]
pub struct FunctionMeta {
    pub fn_index: u32,
    pub max_stack_height: u32,
    pub num_locals: u32,
    pub fn_name: String,
}

impl Serialize for FunctionMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("FunctionMeta", 3)?;
        s.serialize_field("fn_index", &self.fn_index)?;
        s.serialize_field("max_stack_height", &self.max_stack_height)?;
        s.serialize_field("num_locals", &self.num_locals)?;
        s.serialize_field("fn_name", &self.fn_name)?;
        s.end()
    }
}

#[derive(Debug)]
pub struct GlobalVariable {
    pub index: u32,
    pub value: u64,
}

impl Serialize for GlobalVariable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("GlobalVariable", 3)?;
        s.serialize_field("index", &self.index)?;
        s.serialize_field("value", &self.value)?;
        s.end()
    }
}

#[derive(Default)]
pub struct Tracer {
    global_memory: Vec<MemoryState>,
    logs: Vec<OpCodeState>,
    cb_on_after_item_added_to_logs: Option<Box<dyn Fn(OpCodeState)>>,
    memory_changes: RefCell<Vec<MemoryState>>,
    fns_meta: Vec<FunctionMeta>,
    global_variables: Vec<GlobalVariable>,
    extern_names: BTreeMap<u32, String>,
}

impl Debug for Tracer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "global_memory: {:?}; logs: {:?}; memory_changes: {:?}; fns_meta: {:?}",
            self.global_memory, self.logs, self.memory_changes, self.fns_meta
        )
    }
}

impl Serialize for Tracer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Tracer", 3)?;
        s.serialize_field("global_memory", &self.global_memory)?;
        s.serialize_field("logs", &self.logs)?;
        s.serialize_field("global_variables", &self.global_variables)?;
        s.serialize_field("fn_metas", &self.fns_meta)?;
        s.end()
    }
}

impl Tracer {
    pub fn global_memory(&mut self, offset: u32, len: u32, memory: &[u8]) {
        self.global_memory.push(MemoryState {
            offset,
            len,
            data: Vec::from(memory),
        });
    }

    pub fn get_last_pc(&self) -> Option<u32> {
        self.logs.last().map(|opcode| opcode.source_pc)
    }

    pub fn register_extern(&mut self, ex: Extern, name: &Box<str>, entity_index: u32) {
        match ex {
            Extern::Global(_) => {}
            Extern::Table(_) => {}
            Extern::Memory(_) => {}
            Extern::Func(_) => {
                self.extern_names.insert(entity_index, name.to_string());
            }
        }
    }

    pub fn pre_opcode_state(
        &mut self,
        program_counter: u32,
        opcode: Instruction,
        stack: Vec<UntypedValue>,
        meta: &InstrMeta,
    ) {
        let memory_changes = self.memory_changes.replace(Vec::new());
        let stack = stack.iter().map(|v| v.to_bits()).collect();
        let opcode_state = OpCodeState {
            program_counter,
            opcode: OpCode(opcode),
            memory_changes,
            stack,
            source_pc: meta.source_pc(),
            code: meta.opcode(),
        };
        self.logs.push(opcode_state.clone());
        if let Some(cb) = &self.cb_on_after_item_added_to_logs {
            cb(opcode_state)
        }
    }

    pub fn set_cb_on_after_item_added_to_logs(&mut self, cb: Box<dyn Fn(OpCodeState)>) {
        self.cb_on_after_item_added_to_logs = Some(cb);
    }

    pub fn reset_cb_on_after_item_added_to_logs(&mut self) {
        self.cb_on_after_item_added_to_logs = None;
    }

    pub fn function_call(
        &mut self,
        fn_index: u32,
        max_stack_height: usize,
        num_locals: usize,
        fn_name: String,
    ) {
        let resolved_name = self.extern_names.get(&fn_index).unwrap_or(&fn_name);
        self.fns_meta.push(FunctionMeta {
            fn_index,
            max_stack_height: max_stack_height as u32,
            num_locals: num_locals as u32,
            fn_name: resolved_name.clone(),
        })
    }

    pub fn global_variable(&mut self, value: UntypedValue, index: u32) {
        self.global_variables.push(GlobalVariable {
            value: value.to_bits(),
            index,
        })
    }

    pub fn memory_change(&mut self, offset: u32, len: u32, memory: &[u8]) {
        self.memory_changes.borrow_mut().push(MemoryState {
            offset,
            len,
            data: Vec::from(memory),
        });
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
