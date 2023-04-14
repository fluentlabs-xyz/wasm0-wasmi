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
        assert_eq!(json_trace, "{\"global_memory\":[],\"logs\":[{\"pc\":0,\"source_pc\":47,\"name\":\"const\",\"opcode\":65,\"params\":[100]},{\"pc\":1,\"source_pc\":50,\"name\":\"const\",\"opcode\":65,\"params\":[20],\"stack\":[100]},{\"pc\":2,\"source_pc\":52,\"name\":\"const\",\"opcode\":65,\"params\":[3],\"stack\":[100,20]},{\"pc\":3,\"source_pc\":54,\"name\":\"i32_add\",\"opcode\":106,\"stack\":[100,20,3]},{\"pc\":4,\"source_pc\":55,\"name\":\"i32_add\",\"opcode\":106,\"stack\":[100,23]},{\"pc\":5,\"source_pc\":56,\"name\":\"drop\",\"opcode\":26,\"stack\":[123]},{\"pc\":6,\"source_pc\":57,\"name\":\"return\",\"opcode\":11}],\"fn_metas\":[{\"fn_index\":0,\"max_stack_height\":3,\"num_locals\":0}]}");
    }

    #[test]
    fn test_greeting_wat_i32() {
        let wat_binary = fs::read("../../testdata/greeting_i32.wat").unwrap();
        let wasm_binary = wat::parse_bytes(wat_binary.as_slice()).unwrap();
        let mut wasm_engine = WasmEngine::new(None).unwrap();
        let some_engine_id: i32 = 12;
        let wrapped_func = |engine_id: i32, param1: i32, param2: i32| {
            println!("engine_id '{}' param1 '{}' param2 '{}'", engine_id, param1, param2);
        };
        let func = move |param1: i32, param2: i32| {
            wrapped_func(some_engine_id, param1, param2);
        };
        wasm_engine.add_host_fn_cb(
            "_evm_return".to_string(),
            func
        ).unwrap();
        wasm_engine.set_wasm(&wasm_binary.into());
        let json_trace = wasm_engine.compute_trace().unwrap();
        println!("{:?}", json_trace);
        assert_eq!(json_trace, "{\"global_memory\":[{\"offset\":1048576,\"len\":12,\"data\":\"48656c6c6f2c20576f726c64\"}],\"logs\":[{\"pc\":0,\"source_pc\":127,\"name\":\"const\",\"opcode\":65,\"params\":[1048576]},{\"pc\":1,\"source_pc\":132,\"name\":\"const\",\"opcode\":65,\"params\":[12],\"stack\":[1048576]},{\"pc\":2,\"source_pc\":134,\"name\":\"call\",\"opcode\":16,\"params\":[0],\"stack\":[1048576,12]},{\"pc\":3,\"source_pc\":136,\"name\":\"return\",\"opcode\":11}],\"fn_metas\":[{\"fn_index\":1,\"max_stack_height\":2,\"num_locals\":0}]}");
    }

    #[test]
    fn test_greeting_wat_i64() {
        let wat_binary = fs::read("../../testdata/greeting_i64.wat").unwrap();
        let wasm_binary = wat::parse_bytes(wat_binary.as_slice()).unwrap();
        let mut wasm_engine = WasmEngine::new(None).unwrap();
        let some_engine_id: i32 = 12;
        let wrapped_func = |engine_id: i32, param1: i64, param2: i64| {
            println!("engine_id '{}' param1 '{}' param2 '{}'", engine_id, param1, param2);
        };
        let func = move |param1: i64, param2: i64| {
            wrapped_func(some_engine_id, param1, param2);
        };
        wasm_engine.add_host_fn_cb(
            "_evm_return".to_string(),
            func
        ).unwrap();
        wasm_engine.set_wasm(&wasm_binary.into());
        let json_trace = wasm_engine.compute_trace().unwrap();
        println!("{:?}", json_trace);
        assert_eq!(json_trace, "{\"global_memory\":[{\"offset\":1048576,\"len\":12,\"data\":\"48656c6c6f2c20576f726c64\"}],\"logs\":[{\"pc\":0,\"source_pc\":127,\"name\":\"const\",\"opcode\":66,\"params\":[1048576]},{\"pc\":1,\"source_pc\":132,\"name\":\"const\",\"opcode\":66,\"params\":[12],\"stack\":[1048576]},{\"pc\":2,\"source_pc\":134,\"name\":\"call\",\"opcode\":16,\"params\":[0],\"stack\":[1048576,12]},{\"pc\":3,\"source_pc\":136,\"name\":\"return\",\"opcode\":11}],\"fn_metas\":[{\"fn_index\":1,\"max_stack_height\":2,\"num_locals\":0}]}");
    }
}

fn main() {
    println!("nothing todo");
}