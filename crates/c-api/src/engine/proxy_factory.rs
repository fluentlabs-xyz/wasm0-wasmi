use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::engine::engine::WasmEngine;

#[derive(Debug)]
pub struct ProxyFactory {
    lock: Mutex<i32>,
    hm: HashMap<i32, Arc<RefCell<WasmEngine>>>,
    lv: i32,
}

unsafe impl Send for ProxyFactory {}
unsafe impl Sync for ProxyFactory {}

impl<'a> ProxyFactory {
    pub fn new() -> ProxyFactory {
        ProxyFactory {
            lock: Mutex::new(0),
            hm: HashMap::new(),
            lv: 0,
        }
    }

    pub fn new_wasm_engine(&mut self, wasm_binary: Option<Vec<u8>>) -> (i32, Option<Arc<RefCell<WasmEngine>>>) {
        let we = WasmEngine::new(wasm_binary).unwrap();
        let we = Arc::new(RefCell::new(we));
        if let Ok(_) = self.lock.lock() {
            self.lv += 1;
            self.hm.insert(self.lv, we.clone());
            return (self.lv, Some(we))
        } else {
            panic!("lock failed")
        }
    }

    pub fn get_wasm_engine(&mut self, engine_id: i32) -> Arc<RefCell<WasmEngine>> {
        if let Ok(_) = self.lock.lock() {
            let we = self.hm.get(&engine_id);
            return we.unwrap().clone()
        } else {
            panic!("lock failed")
        }
    }

    pub fn set_wasm_binary(&mut self, engine_id: i32, wasm_binary: &Vec<u8>) {
        if let Ok(_) = self.lock.lock() {
            let we = self.hm.get(&engine_id);
            match we {
                Some(we) => {
                    unsafe { (*we.as_ptr()).set_wasm(wasm_binary)};
                },
                _ => panic!("engine id {} not found", engine_id)
            }
        } else {
            panic!("lock failed")
        }
    }

    pub fn compute_trace(&mut self, engine_id: i32) -> Option<String> {
        let we = self.get_wasm_engine(engine_id);
        let trace: String = unsafe { (*we.as_ptr()).compute_trace().unwrap() };

        return Some(trace);
    }

    pub fn memory_data(&mut self, engine_id: i32) -> Option<Vec<u8>> {
        let we = self.get_wasm_engine(engine_id);
        let data = unsafe { (*we.as_ptr()).memory_data() };
        Some(data)
    }

    pub fn trace_memory_change(&mut self, engine_id: i32, offset: u32, len: u32, data: &[u8]) {

        let we = self.get_wasm_engine(engine_id);
        unsafe { (*we.as_ptr()).trace_memory_change(offset, len, data) };
    }

    pub fn register_host_fn_i32(
        &mut self,
        engine_id: i32,
        name: String,
        func: Box<dyn Fn(String, Vec<i32>) -> () + Send + Sync>,
        func_params_count: i32,
    ) -> bool {
        let res: bool;
        let func_params_count = func_params_count as usize;
        let we = self.get_wasm_engine(engine_id);
        if let Ok(_) = self.lock.lock() {
            let register_res: Result<(), String>;
            match func_params_count {
                1 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32| {
                        let p = vec![engine_id];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name.clone(), p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move || {
                        wrapped_func(name.clone(), engine_id);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                2 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32| {
                        let p = vec![engine_id, p1];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32| {
                        wrapped_func(name.clone(), engine_id, p1);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                3 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32| {
                        let p = vec![engine_id, p1, p2];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32| {
                        wrapped_func(name.clone(), engine_id, p1, p2);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                4 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32, p3: i32| {
                        let p = vec![engine_id, p1, p2, p3];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32, p3: i32| {
                        wrapped_func(name.clone(), engine_id, p1, p2, p3);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                5 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32, p3: i32, p4: i32| {
                        let p = vec![engine_id, p1, p2, p3, p4];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32, p3: i32, p4: i32| {
                        wrapped_func(name.clone(), engine_id, p1, p2, p3, p4);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                6 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32| {
                        let p = vec![engine_id, p1, p2, p3, p4, p5];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32, p3: i32, p4: i32, p5: i32| {
                        wrapped_func(name.clone(), engine_id, p1, p2, p3, p4, p5);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                7 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32| {
                        let p = vec![engine_id, p1, p2, p3, p4, p5, p6];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32| {
                        wrapped_func(name.clone(), engine_id, p1, p2, p3, p4, p5, p6);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                8 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32| {
                        let p = vec![engine_id, p1, p2, p3, p4, p5, p6, p7];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32| {
                        wrapped_func(name.clone(), engine_id, p1, p2, p3, p4, p5, p6, p7);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                9 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32, p8: i32| {
                        let p = vec![engine_id, p1, p2, p3, p4, p5, p6, p7, p8];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32, p8: i32| {
                        wrapped_func(name.clone(), engine_id, p1, p2, p3, p4, p5, p6, p7, p8);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                _ => panic!("unsupported func_params_count {}. min number is 1 means 0 params and 1 for engine_id", func_params_count)
            }
            match register_res {
                Ok(_) => { res = true; },
                Err(e) => { panic!("failed to register host fn: {}", e) }
            }
        } else {
            panic!("lock failed")
        }
        res
    }

    pub fn register_host_fn_i64(
        &mut self,
        engine_id: i32,
        name: String,
        func: Box<dyn Fn(String, Vec<i64>) -> () + Send + Sync>,
        func_params_count: i32,
    ) -> bool {
        let res: bool;
        let func_params_count = func_params_count as usize;
        let we = self.get_wasm_engine(engine_id);
        if let Ok(_) = self.lock.lock() {
            let register_res: Result<(), String>;
            match func_params_count {
                1 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64| {
                        let p = vec![engine_id];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name.clone(), p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move || {
                        wrapped_func(name.clone(), engine_id as i64);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                2 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64| {
                        let p = vec![engine_id, p1];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64| {
                        wrapped_func(name.clone(),engine_id as i64, p1);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                3 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64| {
                        let p = vec![engine_id, p1, p2];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64| {
                        wrapped_func(name.clone(), engine_id as i64, p1, p2);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                4 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64, p3: i64| {
                        let p = vec![engine_id, p1, p2, p3];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64, p3: i64| {
                        wrapped_func(name.clone(), engine_id as i64, p1, p2, p3);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                5 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64, p3: i64, p4: i64| {
                        let p = vec![engine_id, p1, p2, p3, p4];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64, p3: i64, p4: i64| {
                        wrapped_func(name.clone(), engine_id as i64, p1, p2, p3, p4);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                6 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64, p3: i64, p4: i64, p5: i64| {
                        let p = vec![engine_id, p1, p2, p3, p4, p5];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64, p3: i64, p4: i64, p5: i64| {
                        wrapped_func(name.clone(), engine_id as i64, p1, p2, p3, p4, p5);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                7 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64, p3: i64, p4: i64, p5: i64, p6: i64| {
                        let p = vec![engine_id, p1, p2, p3, p4, p5, p6];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64, p3: i64, p4: i64, p5: i64, p6: i64| {
                        wrapped_func(name.clone(), engine_id as i64, p1, p2, p3, p4, p5, p6);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                8 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64, p3: i64, p4: i64, p5: i64, p6: i64, p7: i64| {
                        let p = vec![engine_id, p1, p2, p3, p4, p5, p6, p7];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64, p3: i64, p4: i64, p5: i64, p6: i64, p7: i64| {
                        wrapped_func(name.clone(), engine_id as i64, p1, p2, p3, p4, p5, p6, p7);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                9 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64, p3: i64, p4: i64, p5: i64, p6: i64, p7: i64, p8: i64| {
                        let p = vec![engine_id, p1, p2, p3, p4, p5, p6, p7, p8];
                        if p.len() != func_params_count {
                            panic!("expected params count {} got {}", func_params_count, p.len());
                        }
                        func(fn_name, p);
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64, p3: i64, p4: i64, p5: i64, p6: i64, p7: i64, p8: i64| {
                        wrapped_func(name.clone(), engine_id as i64, p1, p2, p3, p4, p5, p6, p7, p8);
                    };
                    register_res = unsafe { (*we.as_ptr()).register_host_fn(name_cloned, native_func) };
                },
                _ => panic!("unsupported func_params_count {}. min number is 1 means 0 params and 1 for engine_id", func_params_count)
            }
            match register_res {
                Ok(_) => { res = true; },
                Err(e) => { panic!("failed to register host fn: {}", e) }
            }
        } else {
            panic!("lock failed")
        }
        res
    }
}