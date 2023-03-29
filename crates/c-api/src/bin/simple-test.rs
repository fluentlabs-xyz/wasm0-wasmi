use std::fs;
use wasmi::{Config, Engine, Linker, Module, Store};

fn main() {
    let wasm_binary = fs::read("testdata/simple.wasm").unwrap();
    let mut config = Config::default();
    config.consume_fuel(false);
    let engine = Engine::new(&config);
    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);
    let module = Module::new(store.engine(), wasm_binary.to_vec().as_slice()).unwrap();
    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    let func = instance.get_func(&store, "main").unwrap();
    let func = func.typed::<(), ()>(&store).unwrap();
    func.call(&mut store, ()).unwrap();
    let json_body = store.tracer.to_json();
    println!("{:?}", json_body.as_bytes().to_vec())
}