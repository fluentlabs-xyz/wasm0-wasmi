use std::ffi::{CStr, CString};

use wasmi::{Config, Engine, Linker, Module, Store};

mod engine;

#[no_mangle]
extern "C" fn execute_wasm_binary_to_json(wasm_binary: CString) -> CString {
    let mut config = Config::default();
    config.consume_fuel(false);
    let engine = Engine::new(&config);
    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);
    let module = Module::new(store.engine(), wasm_binary.as_bytes()).unwrap();
    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    let func = instance.get_func(&store, "main").unwrap();
    let func = func.typed::<(), ()>(&store).unwrap();
    func.call(&mut store, ()).unwrap();
    let json_body = store.tracer.to_json();
    return CString::new(json_body.as_bytes()).unwrap();
}