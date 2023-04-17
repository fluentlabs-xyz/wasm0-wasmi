use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wasmi::core::Trap;
use wasmi::OpCodeState;
use crate::engine::engine::WasmEngine;

#[derive(Debug)]
pub struct ProxyFactory {
    lock: Mutex<i32>,
    engine_id_to_wasm_engine: HashMap<i32, Arc<RefCell<WasmEngine>>>,
    // engine_id_to_logs_after_item_added_cb: HashMap<i32, Box<fn(engine_id: i32, json_trace: String)>>,
    last_engine_id: i32,
}

unsafe impl Send for ProxyFactory {}

unsafe impl Sync for ProxyFactory {}

impl<'a> ProxyFactory {
    pub fn new() -> ProxyFactory {
        ProxyFactory {
            lock: Mutex::new(0),
            engine_id_to_wasm_engine: HashMap::new(),
            // engine_id_to_logs_after_item_added_cb: HashMap::new(),
            last_engine_id: 0,
        }
    }

    pub fn new_wasm_engine(&mut self, wasm_binary: Option<Vec<u8>>) -> (i32, Option<Arc<RefCell<WasmEngine>>>) {
        let we = WasmEngine::new(wasm_binary).unwrap();
        let we = Arc::new(RefCell::new(we));
        let eid = self.get_free_engine_id();
        if let Ok(_) = self.lock.lock() {
            self.engine_id_to_wasm_engine.insert(eid, we.clone());
            return (eid, Some(we))
        } else {
            panic!("lock failed")
        }
    }

    fn get_free_engine_id(&mut self) -> i32 {
        if let Ok(_) = self.lock.lock() {
            self.last_engine_id += 1;
            return self.last_engine_id;
        } else {
            panic!("lock failed")
        }
    }

    pub fn get_wasm_engine(&mut self, engine_id: i32) -> Arc<RefCell<WasmEngine>> {
        if let Ok(_) = self.lock.lock() {
            let we = self.engine_id_to_wasm_engine.get(&engine_id);
            return we.unwrap().clone()
        } else {
            panic!("lock failed")
        }
    }

    pub fn set_wasm_binary(&mut self, engine_id: i32, wasm_binary: &Vec<u8>) {
        if let Ok(_) = self.lock.lock() {
            let we = self.engine_id_to_wasm_engine.get(&engine_id);
            match we {
                Some(we) => {
                    unsafe { (*we.as_ptr()).set_wasm(wasm_binary) };
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
        Some(trace)
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

    pub fn register_cb_on_after_item_added_to_logs(
        &mut self,
        engine_id: i32,
        cb: Box::<dyn Fn(i32, String)>
    ) {
        let we = self.get_wasm_engine(engine_id);
        let synthetic_cb = move |opcode_state: OpCodeState| {
            cb(engine_id, serde_json::to_string(&opcode_state).unwrap())
        };
        unsafe { (*we.as_ptr()).register_cb_on_after_item_added_to_logs(Box::new(synthetic_cb)) };
        /*if let Ok(_) = self.lock.lock() {
            self.engine_id_to_logs_after_item_added_cb.insert(eid, cb);
        } else {
            panic!("lock failed")
        }*/
    }

    pub fn register_host_fn_i32(
        &mut self,
        engine_id: i32,
        name: String,
        func: Box<dyn Fn(String, Vec<i32>) -> i32 + Send + Sync>,
        func_params_count: i32,
    ) -> bool {
        let res: bool;
        let func_params_count = (func_params_count + 1) as usize; // +1 for synthetic engine id param
        let we = self.get_wasm_engine(engine_id);
        if let Ok(_) = self.lock.lock() {
            let register_res: Result<(), String>;
            match func_params_count {
                1 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32| -> i32 {
                        let p = vec![engine_id];
                        func(fn_name.clone(), p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move || -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                2 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32| -> i32 {
                        let p = vec![engine_id, p1];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id, p1);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                3 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32| -> i32 {
                        let p = vec![engine_id, p1, p2];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id, p1, p2);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                4 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32, p3: i32| -> i32 {
                        let p = vec![engine_id, p1, p2, p3];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32, p3: i32| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id, p1, p2, p3);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                5 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32, p3: i32, p4: i32| -> i32 {
                        let p = vec![engine_id, p1, p2, p3, p4];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32, p3: i32, p4: i32| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id, p1, p2, p3, p4);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                6 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32| -> i32 {
                        let p = vec![engine_id, p1, p2, p3, p4, p5];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32, p3: i32, p4: i32, p5: i32| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id, p1, p2, p3, p4, p5);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                7 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32| -> i32 {
                        let p = vec![engine_id, p1, p2, p3, p4, p5, p6];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id, p1, p2, p3, p4, p5, p6);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                8 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32| -> i32 {
                        let p = vec![engine_id, p1, p2, p3, p4, p5, p6, p7];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id, p1, p2, p3, p4, p5, p6, p7);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                9 => {
                    let wrapped_func = move |fn_name: String, engine_id: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32, p8: i32| -> i32 {
                        let p = vec![engine_id, p1, p2, p3, p4, p5, p6, p7, p8];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32, p8: i32| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id, p1, p2, p3, p4, p5, p6, p7, p8);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                _ => panic!("unsupported func_params_count {}. min number is 1 means 0 params and 1 for engine_id", func_params_count)
            }
            match register_res {
                Ok(_) => { res = true; },
                Err(err) => { panic!("failed to register host fn: {}", err) }
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
        func: Box<dyn Fn(String, Vec<i64>) -> i32 + Send + Sync>,
        func_params_count: i32,
    ) -> bool {
        let res: bool;
        let func_params_count = (func_params_count + 1) as usize; // +1 for synthetic engine id param
        let we = self.get_wasm_engine(engine_id);
        if let Ok(_) = self.lock.lock() {
            let register_res: Result<(), String>;
            match func_params_count {
                1 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64| -> i32 {
                        let p = vec![engine_id];
                        func(fn_name.clone(), p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move || -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id as i64);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                2 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64| -> i32 {
                        let p = vec![engine_id, p1];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id as i64, p1);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                3 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64| -> i32 {
                        let p = vec![engine_id, p1, p2];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id as i64, p1, p2);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                4 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64, p3: i64| -> i32 {
                        let p = vec![engine_id, p1, p2, p3];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64, p3: i64| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id as i64, p1, p2, p3);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                5 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64, p3: i64, p4: i64| -> i32 {
                        let p = vec![engine_id, p1, p2, p3, p4];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64, p3: i64, p4: i64| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id as i64, p1, p2, p3, p4);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                6 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64, p3: i64, p4: i64, p5: i64| -> i32 {
                        let p = vec![engine_id, p1, p2, p3, p4, p5];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64, p3: i64, p4: i64, p5: i64| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id as i64, p1, p2, p3, p4, p5);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                7 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64, p3: i64, p4: i64, p5: i64, p6: i64| -> i32 {
                        let p = vec![engine_id, p1, p2, p3, p4, p5, p6];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64, p3: i64, p4: i64, p5: i64, p6: i64| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id as i64, p1, p2, p3, p4, p5, p6);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                8 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64, p3: i64, p4: i64, p5: i64, p6: i64, p7: i64| -> i32 {
                        let p = vec![engine_id, p1, p2, p3, p4, p5, p6, p7];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64, p3: i64, p4: i64, p5: i64, p6: i64, p7: i64| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id as i64, p1, p2, p3, p4, p5, p6, p7);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                9 => {
                    let wrapped_func = move |fn_name: String, engine_id: i64, p1: i64, p2: i64, p3: i64, p4: i64, p5: i64, p6: i64, p7: i64, p8: i64| -> i32 {
                        let p = vec![engine_id, p1, p2, p3, p4, p5, p6, p7, p8];
                        func(fn_name, p)
                    };
                    let name_cloned = name.clone();
                    let native_func = move |p1: i64, p2: i64, p3: i64, p4: i64, p5: i64, p6: i64, p7: i64, p8: i64| -> Result<(), Trap> {
                        let res = wrapped_func(name.clone(), engine_id as i64, p1, p2, p3, p4, p5, p6, p7, p8);
                        match res {
                            0 => Ok(()),
                            err_code => Err(Trap::i32_exit(err_code))
                        }
                    };
                    register_res = unsafe { (*we.as_ptr()).add_host_fn_cb(name_cloned, native_func) };
                },
                _ => panic!("unsupported func_params_count {}. min number is 1 means 0 params and 1 for engine_id", func_params_count)
            }
            match register_res {
                Ok(_) => { res = true; },
                Err(err) => { panic!("failed to register host fn: {}", err) }
            }
        } else {
            panic!("lock failed")
        }
        res
    }
}