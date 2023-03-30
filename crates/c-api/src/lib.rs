use std::slice;
use safer_ffi::prelude::*;
use wasmi::{Config, Engine, Linker, Module, Store};

mod engine;

#[ffi_export]
extern "C" fn execute_wasm_binary_to_json(
    wasm_binary: *mut u8,
    wasm_binary_length: usize
) -> repr_c::Vec<u8> {
    let wasm_binary = unsafe {
        slice::from_raw_parts(wasm_binary, wasm_binary_length)
    };
    // return wasm_binary;
    let mut config = Config::default();
    config.consume_fuel(false);
    let engine = Engine::new(&config);
    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);
    let module = Module::new(store.engine(), wasm_binary).unwrap();
    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    let func = instance.get_func(&store, "main").unwrap();
    let func = func.typed::<(), ()>(&store).unwrap();
    func.call(&mut store, ()).unwrap();
    let json_body = store.tracer.to_json();
    repr_c::Vec::from(json_body.as_bytes().to_vec())
}