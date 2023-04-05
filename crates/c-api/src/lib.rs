use std::cell::RefCell;
use std::ffi::{c_char, CStr};
use std::slice;
use safer_ffi::prelude::*;
use wasmi::{Config, Engine, Linker, Module, Store};
use crate::engine::factory::ProxyFactory;

pub mod engine;

thread_local!(static FACTORY: RefCell<ProxyFactory> = RefCell::new(ProxyFactory::new()));

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
    let mut we_id = 0;
    FACTORY.with(|pfy_cell| {
        let mut pf = pfy_cell.borrow_mut();
        let we_descr = pf.new_wasm_engine(None);
        we_id = we_descr.0;
    });
    we_id
}

#[ffi_export]
extern "C" fn set_wasm_binary(
    we_id: i32,
    wasm_binary: *mut u8,
    wasm_binary_length: usize,
) -> bool {
    let mut res = false;
    let wb = unsafe {
        slice::from_raw_parts(wasm_binary, wasm_binary_length)
    };
    FACTORY.with(|pf_cell| {
        let mut pf = pf_cell.borrow_mut();
        let we = pf.get_wasm_engine(we_id);
        match we {
            Some(we) => {
                we.borrow_mut().set_wasm(&wb.to_vec());
                res = true;
            },
            _ => ()
        }
    });
    res
}

#[ffi_export]
extern "C" fn compute_trace(
    wasm_engine_id: i32,
) -> repr_c::Vec<u8> {
    let mut res: Option<String> = None;
    FACTORY.with(|pf_cell| {
        let mut pf = pf_cell.borrow_mut();
        let we = pf.get_wasm_engine(wasm_engine_id);
        match we {
            Some(we) => {
                match we.borrow_mut().compute_trace() {
                    Ok(trace) => res = Some(trace),
                    Err(_) => res = None,
                }
            },
            _ => ()
        }
    });
    match res {
        Some(r) => repr_c::Vec::from(r.as_bytes().to_vec()),
        None => repr_c::Vec::from(Vec::new())
    }
}

#[ffi_export]
extern "C" fn register_host_fn_p1_ret0(
    wasm_engine_id: i32,
    host_fn_name_ptr: *const c_char,
    host_fn: extern "C" fn(i32) -> (),
) -> bool {
    let mut res: bool = false;
    FACTORY.with(|pf_cell| {
        let mut pf = pf_cell.borrow_mut();
        let hfn_name_bytes = unsafe {
            CStr::from_ptr(host_fn_name_ptr as *const c_char)
        };
        let hfn_name_res = hfn_name_bytes.to_str();
        match hfn_name_res {
            Ok(hfn_name) => {
                res = true;
                pf.register_host_fn_p1_ret0(wasm_engine_id, hfn_name, host_fn);
            },
            Err(e) => {
                println!("error: failed to convert host fn name slice of bytes to string: {}", e.to_string())
            }
        }
    });
    res
}

#[ffi_export]
extern "C" fn register_host_fn_p2_ret0(
    wasm_engine_id: i32,
    host_fn_name_ptr: *const c_char,
    host_fn: extern "C" fn(i32, i32) -> (),
) -> bool {
    let mut res: bool = false;
    FACTORY.with(|pf_cell| {
        let mut pf = pf_cell.borrow_mut();
        let hfn_name_bytes = unsafe {
            CStr::from_ptr(host_fn_name_ptr as *const c_char)
        };
        let hfn_name_res = hfn_name_bytes.to_str();
        match hfn_name_res {
            Ok(hfn_name) => {
                res = true;
                pf.register_host_fn_p2_ret0(wasm_engine_id, hfn_name, host_fn);
            },
            Err(e) => {
                println!("error: failed to convert host fn name slice of bytes to string: {}", e.to_string())
            }
        }
    });
    res
}