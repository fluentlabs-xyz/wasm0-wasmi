use std::cell::RefCell;

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use wasmi_core::UntypedValue;

use crate::engine::bytecode::Instruction;
use crate::engine::opcode::OpCode;

#[derive(Debug)]
pub struct MemoryState {
    pub offset: u32,
    pub len: u32,
    pub data: Vec<u8>,
}

impl Serialize for MemoryState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("MemoryState", 3)?;
        s.serialize_field("offset", &self.offset)?;
        s.serialize_field("len", &self.len)?;
        s.serialize_field("data", &hex::encode(&self.data))?;
        s.end()
    }
}

#[derive(Debug)]
pub struct OpCodeState {
    pub program_counter: u32,
    pub opcode: OpCode,
    pub memory_changes: Vec<MemoryState>,
    pub stack: Vec<u64>,
}

impl Serialize for OpCodeState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("OpCodeState", 4)?;
        s.serialize_field("pc", &self.program_counter)?;
        s.serialize_field("opcode", self.opcode.name())?;
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

#[derive(Default, Debug)]
pub struct Tracer {
    global_memory: Vec<MemoryState>,
    logs: Vec<OpCodeState>,
    memory_changes: RefCell<Vec<MemoryState>>,
}

impl Serialize for Tracer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut s = serializer.serialize_struct("Tracer", 1)?;
        s.serialize_field("global_memory", &self.global_memory)?;
        s.serialize_field("logs", &self.logs)?;
        s.end()
    }
}

impl Tracer {
    pub fn global_memory(
        &mut self,
        offset: u32,
        len: u32,
        memory: &[u8],
    ) {
        self.global_memory.push(MemoryState {
            offset,
            len,
            data: Vec::from(memory),
        });
    }

    pub fn pre_opcode_state(
        &mut self,
        program_counter: u32,
        opcode: Instruction,
        stack: Vec<UntypedValue>,
    ) {
        let memory_changes = self.memory_changes.replace(Vec::new());
        let stack = stack
            .iter()
            .map(|v| v.to_bits())
            .collect();
        self.logs.push(OpCodeState {
            program_counter,
            opcode: OpCode(opcode),
            memory_changes,
            stack,
        });
    }

    pub fn memory_change(
        &mut self,
        offset: u32,
        len: u32,
        memory: &[u8],
    ) {
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