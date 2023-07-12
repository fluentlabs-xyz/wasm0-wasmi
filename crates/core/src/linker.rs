use crate::{Index, UntypedValue, WazmResult};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
};

pub type HostFn = fn(&[UntypedValue], &mut [UntypedValue]) -> WazmResult<()>;

#[derive(Default)]
pub struct Linker {
    host_functions: BTreeMap<(&'static str, &'static str), (HostFn, Index)>,
    fn_by_index: BTreeMap<Index, (&'static str, &'static str)>,
}

impl Debug for Linker {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Linker {
    pub fn new() -> Self {
        return Self {
            host_functions: BTreeMap::new(),
            fn_by_index: BTreeMap::new(),
        };
    }

    pub fn define_function(&mut self, module: &'static str, name: &'static str, func: HostFn, fn_index: Index) {
        self.host_functions.insert((module, name), (func, fn_index));
        self.fn_by_index.insert(fn_index, (module, name));
    }

    pub fn resolve_function(&self, fn_index: Index) -> Option<&HostFn> {
        self.fn_by_index
            .get(&fn_index)
            .map(|fn_key| self.host_functions.get(fn_key).map(|v| &v.0).unwrap())
    }
}
