use std::ffi::{c_char, CStr};
use std::{mem, slice};
use std::sync::Mutex;
use safer_ffi::prelude::*;
use wasmi::{Config, Engine, Linker, Module, Store};
use crate::engine::proxy_factory::ProxyFactory;

pub mod engine;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref FACTORY: Mutex<ProxyFactory> = Mutex::new(ProxyFactory::new());
}

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

#[ffi_export]
extern "C" fn create_wasm_engine() -> i32 {
    let mut id = 0;
    let descr = FACTORY.lock().unwrap().new_wasm_engine(None);
    id = descr.0;
    id
}

#[ffi_export]
extern "C" fn set_wasm_binary(
    engine_id: i32,
    wasm_binary: *mut u8,
    wasm_binary_length: usize,
) -> bool {
    let mut res = false;
    let wasm_binary = unsafe {
        slice::from_raw_parts(wasm_binary, wasm_binary_length)
    };
    FACTORY.lock().unwrap().set_wasm_binary(engine_id, &wasm_binary.to_vec());
    res
}

#[ffi_export]
extern "C" fn compute_trace(
    engine_id: i32,
) -> repr_c::Vec<u8> {
    // let mut res: Option<String> = None;
    let res = FACTORY.lock().unwrap().compute_trace(engine_id);
    match res {
        Some(r) => repr_c::Vec::from(r.lock().unwrap().as_bytes().to_vec()),
        None => repr_c::Vec::from(Vec::new())
    }
}

#[ffi_export]
extern "C" fn memory_data(
    engine_id: i32,
) -> repr_c::Vec<u8> {
    let res = FACTORY.lock().unwrap().memory_data(engine_id);
    match res {
        Some(r) => repr_c::Vec::from(r.lock().unwrap().clone()),
        None => repr_c::Vec::from(Vec::new())
    }
}

#[ffi_export]
extern "C" fn trace_memory_change(
    engine_id: i32,
    offset: u32,
    len: u32,
    data: *mut u8,
    data_length: usize,
) {
    let data = unsafe {
        slice::from_raw_parts(data, data_length)
    };
    FACTORY.lock().unwrap().trace_memory_change(engine_id, offset, len, data);
}

#[ffi_export]
extern "C" fn register_host_fn(
    engine_id: i32,
    host_fn_name_ptr: *const c_char,
    host_fn: extern "C" fn(i32, data: *mut i32, data_length: usize) -> (),
    func_params_count: i32,
) -> bool {
    let mut res: bool = false;
    let hfn_name_bytes = unsafe {
        CStr::from_ptr(host_fn_name_ptr as *const c_char)
    };
    let hfn_name = hfn_name_bytes.to_str();
    match hfn_name {
        Ok(hfn_name) => {
            res = true;
            let host_fn_wrapper =  Box::new(move |mut params: Vec<i32>| {
                let params_ptr = params.as_mut_ptr();
                let params_len = params.len();
                mem::forget(params);
                host_fn(engine_id, params_ptr, params_len);
            });
            FACTORY.lock().unwrap().register_host_fn(engine_id, hfn_name, host_fn_wrapper, func_params_count);
        },
        Err(e) => {
            panic!("failed to convert host fn name slice of bytes to string: {}", e.to_string())
        }
    }
    res
}

#[ffi_export]
extern "C" fn register_host_fn_p1_ret0(
    engine_id: i32,
    host_fn_name_ptr: *const c_char,
    host_fn: extern "C" fn(i32, i32) -> (),
) -> bool {
    let mut res: bool = false;
    let hfn_name_bytes = unsafe {
        CStr::from_ptr(host_fn_name_ptr as *const c_char)
    };
    let hfn_name_res = hfn_name_bytes.to_str();
    match hfn_name_res {
        Ok(hfn_name) => {
            res = true;
            FACTORY.lock().unwrap().register_host_fn_p1_ret0(engine_id, hfn_name, host_fn);
        },
        Err(e) => {
            panic!("failed to convert host fn name slice of bytes to string: {}", e.to_string())
        }
    }
    res
}

#[ffi_export]
extern "C" fn register_host_fn_p2_ret0(
    engine_id: i32,
    host_fn_name_ptr: *const c_char,
    host_fn: extern "C" fn(i32, i32, i32) -> (),
) -> bool {
    let mut res: bool = false;
    let hfn_name_bytes = unsafe {
        CStr::from_ptr(host_fn_name_ptr as *const c_char)
    };
    let hfn_name_res = hfn_name_bytes.to_str();
    match hfn_name_res {
        Ok(hfn_name) => {
            res = true;
            FACTORY.lock().unwrap().register_host_fn_p2_ret0(engine_id, hfn_name, host_fn);
        },
        Err(e) => {
            panic!("error: failed to convert host fn name slice of bytes to string: {}", e.to_string())
        }
    }
    res
}