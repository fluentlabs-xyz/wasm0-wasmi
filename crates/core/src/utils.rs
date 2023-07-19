use crate::{OpCode, UntypedValue};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct JumpDest(pub i32);

impl From<i32> for JumpDest {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl JumpDest {
    pub fn neg(&self) -> JumpDest {
        Self(-self.0)
    }

    /// Creates a [`BranchOffset`] from the given raw `i32` value.
    pub fn from_i32(value: i32) -> Self {
        Self(value)
    }

    /// Creates an uninitalized [`BranchOffset`].
    pub fn uninit() -> Self {
        Self(0)
    }

    /// Creates an initialized [`BranchOffset`] from `src` to `dst`.
    pub fn init(src: u32, dst: u32) -> Self {
        let src = src as i32;
        let dst = dst as i32;
        Self(dst - src)
    }

    /// Returns `true` if the [`BranchOffset`] has been initialized.
    pub fn is_init(self) -> bool {
        self.0 != 0
    }

    /// Returns the `i32` representation of the [`BranchOffset`].
    pub fn into_i32(self) -> i32 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Offset(pub u32);

impl From<u32> for Offset {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Offset {
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Ord, PartialOrd)]
pub struct Index(pub u32);

impl From<u32> for Index {
    fn from(value: u32) -> Self {
        Index(value)
    }
}

impl Index {
    pub fn into_inner(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Fuel(pub u64);

impl From<u64> for Fuel {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct DropKeep {
    pub drop: u32,
    pub keep: u32,
}

impl DropKeep {
    pub fn new(drop: u32, keep: u32) -> Self {
        Self { drop, keep }
    }

    pub fn none() -> Self {
        Self { drop: 0, keep: 0 }
    }

    pub fn non_empty(&self) -> bool {
        self.drop + self.keep > 0
    }

    pub fn drop(&self) -> u32 {
        self.drop
    }

    pub fn keep(&self) -> u32 {
        self.keep
    }
}

#[derive(Default, Clone)]
pub struct InstructionSet(pub Vec<OpCode>);

macro_rules! impl_opcode {
    ($name:ident, $opcode:ident($into:ident)) => {
        pub fn $name<I: Into<$into>>(&mut self, value: I) {
            self.0.push(OpCode::$opcode(value.into()));
        }
    };
    ($name:ident, $opcode:ident($into:ident, $into2:ident)) => {
        pub fn $name<I: Into<$into>, J: Into<$into2>>(&mut self, value: I, value2: J) {
            self.0.push(OpCode::$opcode(value.into(), value2.into()));
        }
    };
    ($name:ident, $opcode:ident) => {
        pub fn $name(&mut self) {
            self.0.push(OpCode::$opcode);
        }
    };
}

impl From<Vec<OpCode>> for InstructionSet {
    fn from(value: Vec<OpCode>) -> Self {
        Self(value)
    }
}

impl InstructionSet {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, opcode: OpCode) -> u32 {
        let opcode_pos = self.len();
        self.0.push(opcode);
        opcode_pos
    }

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    impl_opcode!(op_unreachable, Unreachable);
    impl_opcode!(op_consume_fuel, ConsumeFuel(Fuel));
    impl_opcode!(op_drop, Drop);
    impl_opcode!(op_select, Select);
    impl_opcode!(op_local_get, LocalGet(Index));
    impl_opcode!(op_local_set, LocalSet(Index));
    impl_opcode!(op_local_tee, LocalTee(Index));
    impl_opcode!(op_br, Br(BranchParams));
    impl_opcode!(op_br_if_eqz, BrIfEqz(BranchParams));
    impl_opcode!(op_br_if_nez, BrIfNez(BranchParams));
    impl_opcode!(op_br_table, BrTable(Index));
    impl_opcode!(op_return, Return(DropKeep));
    impl_opcode!(op_return_call_indirect, ReturnCallIndirect(Index, DropKeep));
    impl_opcode!(op_call, Call(Index));
    impl_opcode!(op_call_indirect, CallIndirect(Index));
    impl_opcode!(op_global_get, GlobalGet(Index));
    impl_opcode!(op_global_set, GlobalSet(Index));
    // add more opcodes
    impl_opcode!(op_i32_const, I32Const(UntypedValue));
    impl_opcode!(op_i64_const, I64Const(UntypedValue));

    pub fn extend<I: Into<InstructionSet>>(&mut self, with: I) {
        self.0.extend(Into::<InstructionSet>::into(with).0);
    }

    pub fn finalize(&mut self) -> Vec<OpCode> {
        self.0.clone()
    }
}

/// A branching target.
///
/// This also specifies how many values on the stack
/// need to be dropped and kept in order to maintain
/// value stack integrity.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct BranchParams {
    /// The branching offset.
    ///
    /// How much instruction pointer is offset upon taking the branch.
    pub offset: JumpDest,
    /// How many values on the stack need to be dropped and kept.
    pub drop_keep: DropKeep,
}

impl From<i32> for BranchParams {
    fn from(value: i32) -> Self {
        BranchParams::new(JumpDest::from(value), DropKeep::none())
    }
}

impl BranchParams {
    /// Creates new [`BranchParams`].
    pub fn new(offset: JumpDest, drop_keep: DropKeep) -> Self {
        Self { offset, drop_keep }
    }

    /// Returns `true` if the [`BranchParams`] have been initialized already.
    fn is_init(&self) -> bool {
        self.offset.is_init()
    }

    /// Initializes the [`BranchParams`] with a proper [`BranchOffset`].
    ///
    /// # Panics
    ///
    /// - If the [`BranchParams`] have already been initialized.
    /// - If the given [`BranchOffset`] is not properly initialized.
    pub fn init(&mut self, offset: JumpDest) {
        assert!(offset.is_init());
        assert!(!self.is_init());
        self.offset = offset;
    }

    /// Returns the branching offset.
    pub fn offset(self) -> JumpDest {
        self.offset
    }

    /// Returns the amount of stack values to drop and keep upon taking the branch.
    pub fn drop_keep(self) -> DropKeep {
        self.drop_keep
    }
}
