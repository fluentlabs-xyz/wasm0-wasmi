use std::ffi::{c_char, CStr};
use std::{mem, slice};
use safer_ffi::prelude::*;
use wasmi::{Config, Engine, Linker, Module, Store};
use crate::engine::proxy_factory::ProxyFactory;

pub mod engine;

static mut FACTORY: once_cell::sync::Lazy<ProxyFactory> = once_cell::sync::Lazy::new(|| ProxyFactory::new());

#[ffi_export]
extern "C" fn execute_wasm_binary_to_json(
    wasm_binary: *mut u8,
    wasm_binary_length: usize
) -> repr_c::Vec<u8> {
    let wasm_binary = unsafe {
        slice::from_raw_parts(wasm_binary, wasm_binary_length)
    };
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
    let descr = unsafe {FACTORY.new_wasm_engine(None)};
    descr.0
}

#[ffi_export]
extern "C" fn set_wasm_binary(
    engine_id: i32,
    wasm_binary: *mut u8,
    wasm_binary_length: usize,
) {
    let wasm_binary = unsafe {
        slice::from_raw_parts(wasm_binary, wasm_binary_length)
    };
    unsafe {FACTORY.set_wasm_binary(engine_id, &wasm_binary.to_vec())};
}

#[ffi_export]
extern "C" fn compute_result(
    engine_id: i32,
) -> i32 {
    let res = unsafe {FACTORY.compute_result(engine_id)};
    match res {
        Some(r) => r,
        None => 0,
    }
}

#[ffi_export]
extern "C" fn dump_trace(
    engine_id: i32,
) -> repr_c::Vec<u8> {
    let res = unsafe {FACTORY.dump_trace(engine_id)};
    match res {
        Some(r) => repr_c::Vec::from(r.as_bytes().to_vec()),
        None => repr_c::Vec::from(Vec::new())
    }
}

#[ffi_export]
extern "C" fn compute_trace(
    engine_id: i32,
) -> repr_c::Vec<u8> {
    let res = unsafe {FACTORY.compute_trace(engine_id)};
    match res {
        Some(r) => repr_c::Vec::from(r.as_bytes().to_vec()),
        None => repr_c::Vec::from(Vec::new())
    }
}

#[ffi_export]
extern "C" fn memory_data(
    engine_id: i32,
) -> repr_c::Vec<u8> {
    let res = unsafe {FACTORY.memory_data(engine_id)};
    match res {
        Some(r) => repr_c::Vec::from(r.to_vec()),
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
    unsafe {FACTORY.trace_memory_change(engine_id, offset, len, data)};
}

#[ffi_export]
extern "C" fn register_cb_on_after_item_added_to_logs(
    engine_id: i32,
    cb: extern "C" fn(engine_id: i32, json_trace: *const i8, json_trace_len: usize) -> (),
) {
    let cb_wrapper =  move |engine_id: i32, json_trace: String| {
        let json_trace_c_string = unsafe {CStr::from_bytes_with_nul_unchecked(json_trace.as_bytes())};
        cb(engine_id, json_trace_c_string.as_ptr() as *const i8, json_trace.len());
        mem::forget(json_trace_c_string);
    };
    unsafe {FACTORY.register_cb_on_after_item_added_to_logs(engine_id, Box::new(cb_wrapper))};
}

#[ffi_export]
extern "C" fn register_host_fn_i32(
    engine_id: i32,
    host_fn_name_ptr: *const c_char,
    host_fn: extern "C" fn(engine_id: i32, fn_name: *const i8, fn_name_len: usize, data: *mut i32, data_length: usize) -> i32,
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
            let host_fn_wrapper =  Box::new(move |host_fn_name: String, mut params: Vec<i32>| -> i32 {
                let params_mut_ptr = params.as_mut_ptr();
                let params_len = params.len();
                mem::forget(params);
                let hfn_name_c_string = unsafe {CStr::from_bytes_with_nul_unchecked(host_fn_name.as_bytes())};
                let res = host_fn(engine_id, hfn_name_c_string.as_ptr() as *const i8, host_fn_name.len(), params_mut_ptr, params_len);
                mem::forget(host_fn_name);
                res
            });
            unsafe {FACTORY.register_host_fn_i32(engine_id, hfn_name.to_string(), host_fn_wrapper, func_params_count)};
        },
        Err(e) => {
            panic!("failed to convert host fn name slice of bytes to string: {}", e.to_string())
        }
    }
    res
}

#[ffi_export]
extern "C" fn register_host_fn_i64(
    engine_id: i32,
    host_fn_name_ptr: *const c_char,
    host_fn: extern "C" fn(engine_id: i32, fn_name: *const i8, fn_name_len: usize, data: *mut i64, data_len: usize) -> i32,
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
            let host_fn_wrapper =  Box::new(move |host_fn_name: String, mut params: Vec<i64>| {
                let params_mut_ptr = params.as_mut_ptr();
                let params_len = params.len();
                mem::forget(params);
                let hfn_name_c_string = unsafe {CStr::from_bytes_with_nul_unchecked(host_fn_name.as_bytes())};
                let res = host_fn(engine_id, hfn_name_c_string.as_ptr() as *const i8, host_fn_name.len(), params_mut_ptr, params_len);
                mem::forget(host_fn_name);
                res
            });
            unsafe {FACTORY.register_host_fn_i64(engine_id, hfn_name.to_string(), host_fn_wrapper, func_params_count)};
        },
        Err(e) => {
            panic!("failed to convert host fn name slice of bytes to string: {}", e.to_string())
        }
    }
    res
}
