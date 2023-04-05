use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;
use crate::engine::engine::WasmEngine;

#[derive(Debug)]
pub struct ProxyFactory {
    mx: Mutex<i32>,
    hm: HashMap<i32, Rc<RefCell<WasmEngine>>>,
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

    pub fn new_wasm_engine(&mut self, wasm_binary: Option<Vec<u8>>) -> (i32, Option<Rc<RefCell<WasmEngine>>>) {
        let we = WasmEngine::new(wasm_binary).unwrap();
        let we = Rc::new(RefCell::new(we));
        if let Ok(_) = self.mx.lock() {
            self.lv += 1;
            self.hm.insert(self.lv, we.clone());
        }
        (self.lv, Some(we))
    }

    pub fn get_wasm_engine(&mut self, engine_id: i32) -> Option<Rc<RefCell<WasmEngine>>> {
        if let Ok(_) = self.mx.lock() {
            let we = self.hm.get(&engine_id);
            return we.cloned()
        }
        None
    }

    pub fn register_host_fn_p1_ret0(
        &mut self,
        engine_id: i32,
        name: &str,
        func: extern "C" fn(i32) -> ()
    ) -> bool {
        let func = move |p1: i32| { func(p1); };
        let mut res = false;
        let we = self.get_wasm_engine(engine_id);
        match we {
            Some(we) => {
                let register_res = we.borrow_mut().register_host_fn(name, func);
                match register_res {
                    Ok(_) => { res = true; },
                    Err(e) => {println!("error: failed to register host fn: {}", e)}
                }
            },
            None => ()
        }
        res
    }

    pub fn register_host_fn_p2_ret0(
        &mut self,
        engine_id: i32,
        name: &str,
        func: extern "C" fn(i32, i32) -> ()
    ) -> bool {
        let func = move |p1: i32, p2: i32| { func(p1, p2); };
        let mut res = false;
        let we = self.get_wasm_engine(engine_id);
        match we {
            Some(we) => {
                let register_res = we.borrow_mut().register_host_fn(name, func);
                match register_res {
                    Ok(_) => { res = true; },
                    Err(e) => {println!("error: failed to register host fn: {}", e)}
                }
            },
            None => ()
        }
        res
    }
}