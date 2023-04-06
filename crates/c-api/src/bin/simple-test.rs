#[cfg(test)]
mod tests {
    use std::fs;
    use wasmi_c_api::engine::engine::WasmEngine;

    #[test]
    fn test_simple_wat() {
        let wat_binary = fs::read("../../testdata/simple.wat").unwrap();
        let wasm_binary = wat::parse_bytes(wat_binary.as_slice()).unwrap();
        let mut wasm_engine = WasmEngine::new(None).unwrap();
        wasm_engine.set_wasm(&wasm_binary.into());
        let json_trace = wasm_engine.compute_trace().unwrap();
        println!("{:?}", json_trace);
    }

    #[test]
    fn test_greeting_wat() {
        let wat_binary = fs::read("../../testdata/greeting.wat").unwrap();
        let wasm_binary = wat::parse_bytes(wat_binary.as_slice()).unwrap();
        let mut wasm_engine = WasmEngine::new(None).unwrap();
        wasm_engine.set_wasm(&wasm_binary.into());
        let some_engine_id: i32 = 12;
        let wrapped_func = |engine_id: i32, param1: i32, param2: i32| {
            println!("engine_id '{}' param1 '{}' param2 '{}'", engine_id, param1, param2);
        };
        let native_func = move |param1: i32, param2: i32| {
            wrapped_func(some_engine_id, param1, param2);
        };
        wasm_engine.register_host_fn(
            "_evm_return",
            native_func
        ).unwrap();
        let json_trace = wasm_engine.compute_trace().unwrap();
        println!("{:?}", json_trace);
    }
}

fn main() {
    println!("nothing todo");
}