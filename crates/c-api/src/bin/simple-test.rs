#[cfg(test)]
mod tests {
    use std::fs;
    use wasmi_c_api::engine::engine::WasmEngine;

    #[test]
    fn test_simple() {
        let wat_binary = fs::read("../../testdata/simple.wat").unwrap();
        let wasm_binary = wat::parse_bytes(wat_binary.as_slice()).unwrap();
        let mut wasm_engine = WasmEngine::new(None).unwrap();
        wasm_engine.set_wasm(&wasm_binary.into());
        let json_trace = wasm_engine.compute_trace().unwrap();
        println!("{:?}", json_trace);
    }

    #[test]
    fn test_greeting() {
        let wat_binary = fs::read("../../testdata/greeting.wat").unwrap();
        let wasm_binary = wat::parse_bytes(wat_binary.as_slice()).unwrap();
        let mut wasm_engine = WasmEngine::new(None).unwrap();
        wasm_engine.set_wasm(&wasm_binary.into());
        let func = |param1: i32, param2: i32| println!("param1 '{}' param2 '{}'", param1, param2);
        wasm_engine.register_host_fn(
            "_evm_return",
            func
        ).unwrap();
        let json_trace = wasm_engine.compute_trace().unwrap();
        println!("{:?}", json_trace);
    }
}

fn main() {
    println!("nothing todo");
}