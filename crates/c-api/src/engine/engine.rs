use ::safer_ffi::prelude::*;
use std::collections::HashMap;
use wasmi::{Config, Engine, Error, ExternType, Func, Instance, IntoFunc, Linker, Module, Store};

#[derive(Debug)]
pub struct WasmEngine {
    config: Config,
    store: Store<()>,
    engine: Engine,
    wasm_binary: Option<Vec<u8>>,
    host_fns: HashMap<String, Func>,
}

impl WasmEngine {
    pub fn new(wasm_binary: Option<Vec<u8>>) -> Result<Self, Error> {
        let mut config = Config::default();
        config.consume_fuel(false);
        let engine = Engine::new(&config);
        let store = Store::new(&engine, ());

        Ok(Self {
            config,
            store,
            engine,
            wasm_binary,
            host_fns: HashMap::new(),
        })
    }

    pub fn set_wasm(&mut self, wasm_binary: &Vec<u8>) {
        self.wasm_binary = Some(wasm_binary.clone());
    }

    pub fn compute_trace(&mut self) -> Result<String, Error> {
        let mut linker = Linker::<()>::new(&self.engine);
        let module = Module::new(self.store.engine(), self.wasm_binary.as_ref().unwrap().as_slice()).unwrap();
        for (name, func) in self.host_fns.iter() {
            linker.define("env", name.as_ref(), *func)?;
        }
        let instance = linker
            .instantiate(&mut self.store, &module)
            .unwrap()
            .start(&mut self.store)
            .unwrap();
        let mut memory_chunk_indexes: Vec<(usize, usize)> = Vec::new();
        let memory_data = self.fetch_memory_data(&instance);
        for x in instance.exports(&self.store) {
            if x.name() != "memory" {
                continue
            }
            if let ExternType::Memory(_) = x.ty(&self.store) {
                let export = instance.get_export(&self.store, x.name());
                if let Some(export) = export {
                    if let Some(memory) = export.into_memory() {
                        let mem_data = memory.data(&self.store);
                        let mut i: usize = 0;
                        let mem_data_len = mem_data.len();
                        while i < mem_data_len {
                            if mem_data[i] != 0 {
                                let start = i;
                                while i < mem_data_len && mem_data[i] != 0 { i += 1 }
                                let end = i-1;
                                memory_chunk_indexes.push((start, end));
                            }
                            i += 1;
                        }
                    }
                }
            }
        }
        for (start, end) in memory_chunk_indexes {
            self.store.tracer.global_memory(start as u32, (end-start+1) as u32, &memory_data[start..=end])
        }
        let func = instance.get_func(&self.store, "main").unwrap();
        let func = func.typed::<(), ()>(&self.store).unwrap();
        func.call(&mut self.store, ()).unwrap();
        let json_body = self.store.tracer.to_json();
        Ok(json_body)
    }

    pub fn memory_data(&mut self) -> Vec<u8> {
        let mut linker = Linker::<()>::new(&self.engine);
        let module = Module::new(self.store.engine(), self.wasm_binary.as_ref().unwrap().as_slice()).unwrap();
        for (name, func) in self.host_fns.iter() {
            linker.define("env", name.as_ref(), *func).unwrap();
        }
        let instance = linker
            .instantiate(&mut self.store, &module)
            .unwrap()
            .start(&mut self.store)
            .unwrap();
        self.fetch_memory_data(&instance)
    }

    fn fetch_memory_data(&mut self, instance: &Instance) -> Vec<u8> {
        let mut memory_data = Vec::<u8>::new();
        for x in instance.exports(&self.store) {
            if x.name() != "memory" {
                continue
            }
            if let ExternType::Memory(_) = x.ty(&self.store) {
                let export = instance.get_export(&self.store, x.name());
                if let Some(export) = export {
                    if let Some(memory) = export.into_memory() {
                        let mem_data = memory.data(&self.store);
                        memory_data = mem_data.into();
                    }
                }
            }
        }
        memory_data
    }

    pub fn register_host_fn<Params, Results>(&mut self, name: &str, func: impl IntoFunc<(), Params, Results>) -> Result<(), String> {
        if self.host_fns.contains_key(name) {
            return Err(format!("there is already fn with name: {}", &name));
        };
        let host_fn = Func::wrap(&mut self.store, func);
        self.host_fns.insert(name.to_string(), host_fn);
        Ok(())
    }
}