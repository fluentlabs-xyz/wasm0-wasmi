use std::cell::RefCell;
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::engine::engine::WasmEngine;

#[derive(Debug)]
pub struct ProxyFactory {
    mx: Mutex<i32>,
    hm: HashMap<i32, Arc<Mutex<WasmEngine>>>,
    lv: i32,
}

impl<'a> ProxyFactory {
    pub fn new() -> ProxyFactory {
        ProxyFactory {
            mx: Mutex::new(0),
            hm: HashMap::new(),
            lv: 0,
        }
    }

    pub fn new_wasm_engine(&mut self, wasm_binary: Option<Vec<u8>>) -> (i32, Option<Arc<Mutex<WasmEngine>>>) {
        let we = WasmEngine::new(wasm_binary).unwrap();
        let we = Arc::new(Mutex::new(we));
        if let Ok(_) = self.mx.lock() {
            self.lv += 1;
            self.hm.insert(self.lv, we.clone());
            return (self.lv, Some(we))
        }
        panic!("new_wasm_engine: failed to acquire lock")
    }

    pub fn try_get_wasm_engine(&mut self, engine_id: i32) -> Option<Arc<Mutex<WasmEngine>>> {
        if let Ok(_) = self.mx.lock() {
            let we = self.hm.get(&engine_id);
            return we.cloned()
        }
        panic!("try_get_wasm_engine: failed to acquire lock")
    }

    pub fn get_wasm_engine(&mut self, engine_id: i32) -> Arc<Mutex<WasmEngine>> {
        let we = self.try_get_wasm_engine(engine_id);
        match we {
            Some(we) => {
                we
            },
            _ => panic!("get_wasm_engine: failed to acquire lock")
        }
    }

    pub fn set_wasm_binary(&mut self, engine_id: i32, wasm_binary: &Vec<u8>) -> Option<Arc<Mutex<WasmEngine>>> {
        if let Ok(_) = self.mx.lock() {
            let we = self.hm.get(&engine_id);
            match we {
                Some(we) => {
                    we.lock().unwrap().set_wasm(wasm_binary);
                },
                _ => panic!("set_wasm_binary: engine id {} not found", engine_id)
            }

            return we.cloned()
        }
        panic!("set_wasm_binary: failed to acquire lock")
    }

    pub fn compute_trace(&mut self, engine_id: i32) -> Option<Arc<Mutex<String>>> {
        if let Ok(_) = self.mx.lock() {
            let we = self.hm.get(&engine_id);
            let trace: String;
            match we {
                Some(we) => {
                    trace = we.lock().unwrap().compute_trace().unwrap();
                },
                _ => panic!("set_wasm_binary: engine id {} not found", engine_id)
            }

            return Some(Arc::new(Mutex::new(trace)))
        }
        panic!("set_wasm_binary: failed to acquire lock")
    }

    pub fn memory_data(&mut self, engine_id: i32) -> Option<Arc<Mutex<Vec<u8>>>> {
        let we = self.get_wasm_engine(engine_id);
        if let Ok(_) = self.mx.lock() {
            let data = we.lock().unwrap().memory_data();

            return Some(Arc::new(Mutex::new(data)))
        }
        panic!("memory_data: failed to acquire lock")
    }

    pub fn trace_memory_change(&mut self, engine_id: i32, offset: u32, len: u32, data: &[u8]) {
        let we = self.get_wasm_engine(engine_id);
        if let Ok(_) = self.mx.lock() {
            we.lock().unwrap().trace_memory_change(offset, len, data)
        }
        panic!("trace_memory_change: failed to acquire lock")
    }

    pub fn register_host_fn(
        &mut self,
        engine_id: i32,
        name: &str,
        func: Box<dyn Fn(Vec<i32>) -> () + Send + Sync>,
        func_params_count: i32,
    ) -> bool {
        let res: bool;
        let we = self.try_get_wasm_engine(engine_id);
        match we {
            Some(we) => {
                let register_res: Result<(), String>;
                println!("RUST: register_host_fn '{}' params count {}", name, func_params_count);
                match func_params_count {
                    1 => {
                        let wrapped_func = move |engine_id: i32| {
                            let p = vec![engine_id];
                            if p.len() != func_params_count as usize {
                                panic!("wrapped_func expected params count {} got {}", func_params_count, p.len());
                            }
                            println!("wrapped_func: {:?}", p);
                            func(p);
                        };
                        let native_func = move || { wrapped_func(engine_id); };
                        register_res = we.lock().unwrap().register_host_fn(name, native_func);
                    },
                    2 => {
                        let wrapped_func = move |engine_id: i32, p1: i32| {
                            let p = vec![engine_id, p1];
                            if p.len() != func_params_count as usize {
                                panic!("wrapped_func expected params count {} got {}", func_params_count, p.len());
                            }
                            println!("wrapped_func: {:?}", p);
                            func(p);
                        };
                        let native_func = move |p1: i32| { wrapped_func(engine_id, p1) };
                        register_res = we.lock().unwrap().register_host_fn(name, native_func);
                    },
                    3 => {
                        let wrapped_func = move |engine_id: i32, p1: i32, p2: i32| {
                            let p = vec![engine_id, p1, p2];
                            if p.len() != func_params_count as usize {
                                panic!("wrapped_func expected params count {} got {}", func_params_count, p.len());
                            }
                            println!("wrapped_func: {:?}", p);
                            func(p);
                        };
                        let native_func = move |p1: i32, p2: i32| { wrapped_func(engine_id, p1, p2) };
                        register_res = we.lock().unwrap().register_host_fn(name, native_func);
                    },
                    _ => panic!("unsupported func_params_count {}. min number is 1 means 0 params and 1 for engine_id", func_params_count)
                }
                match register_res {
                    Ok(_) => { res = true; },
                    Err(e) => {panic!("failed to register host fn: {}", e)}
                }
            },
            None => panic!("register_host_fn: engine with id {} not found", engine_id)
        }
        res
    }

    pub fn register_host_fn_p1_ret0(
        &mut self,
        engine_id: i32,
        name: &str,
        func: extern "C" fn(i32, i32) -> ()
    ) -> bool {
        let wrapped_func = move |engine_id: i32, p1: i32| { func(engine_id, p1); };
        let native_func = move |p1: i32| { wrapped_func(engine_id, p1) };
        let res: bool;
        let we = self.try_get_wasm_engine(engine_id);
        match we {
            Some(we) => {
                let register_res = we.lock().unwrap().register_host_fn(name, native_func);
                match register_res {
                    Ok(_) => { res = true; },
                    Err(e) => {panic!("failed to register host fn: {}", e)}
                }
            },
            None => panic!("register_host_fn_p1_ret0: engine with id {} not found", engine_id)
        }
        res
    }

    pub fn register_host_fn_p2_ret0(
        &mut self,
        engine_id: i32,
        name: &str,
        func: extern "C" fn(i32, i32, i32) -> ()
    ) -> bool {
        let wrapped_func = move |engine_id: i32, p1: i32, p2: i32| { func(engine_id, p1, p2); };
        let native_func = move |p1: i32, p2: i32| { wrapped_func(engine_id, p1, p2) };
        let res: bool;
        let we = self.try_get_wasm_engine(engine_id);
        match we {
            Some(we) => {
                let register_res = we.lock().unwrap().register_host_fn(name, native_func);
                match register_res {
                    Ok(_) => { res = true; },
                    Err(e) => {panic!("failed to register host fn: {}", e)}
                }
            },
            None => panic!("register_host_fn_p2_ret0: engine with id {} not found", engine_id)
        }
        res
    }
}