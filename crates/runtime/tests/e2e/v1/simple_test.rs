//! Tests to check if runtime's fuel metering works as intended.

use std::fmt::Debug;
use wazm_core::Trap;
use wazm_wasmi::{Config, Engine, Func, Linker, Module, Store};

/// Setup [`Engine`] and [`Store`] for fuel metering.
fn test_setup() -> (Store<()>, Linker<()>) {
    let mut config = Config::default();
    config.consume_fuel(false);
    let engine = Engine::new(&config);
    let store = Store::new(&engine, ());
    let linker = Linker::new(&engine);
    (store, linker)
}

/// Converts the `wat` string source into `wasm` encoded byte.
fn wat2wasm(wat: &str) -> Vec<u8> {
    wat::parse_str(wat).unwrap()
}

/// Compiles the `wasm` encoded bytes into a [`Module`].
///
/// # Panics
///
/// If an error occurred upon module compilation, validation or translation.
fn create_module(store: &Store<()>, bytes: &[u8]) -> Module {
    Module::new(store.engine(), bytes).unwrap()
}

/// Setup [`Store`] and [`Instance`] for fuel metering.
fn default_test_setup(wasm: &[u8]) -> (Store<()>, Func) {
    let (mut store, linker) = test_setup();
    let module = create_module(&store, wasm);
    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    let func = instance.get_func(&store, "test").unwrap();
    (store, func)
}

/// Asserts the the call was successful.
///
/// # Note
///
/// We just check if the call succeeded, not if the results are correct.
/// That is to be determined by another kind of test.
fn assert_success<T>(call_result: Result<T, Trap>)
where
    T: Debug,
{
    assert!(call_result.is_ok());
}

#[test]
fn simple_i32_add() {
    let wasm = wat2wasm(
        r#"
        (module
            (func (export "test") (param $a i32) (param $b i32) (result i32)
                (i32.add
                    (local.get $a)
                    (local.get $b)
                )
            )
        )
    "#,
    );
    let (mut store, func) = default_test_setup(&wasm);
    let func = func.typed::<(i32, i32), i32>(&store).unwrap();
    assert_success(func.call(&mut store, (1, 2)));
    println!("{:?}", store.tracer);
    let json_body = store.tracer.to_json();
    println!("{:?}", json_body);
}

#[test]
fn test_global_data() {
    let wasm = wat2wasm(
        r#"
(module
  (type (;0;) (func))
  (func (;1;) (type 0)
    i32.const 0
    drop)
  (memory (;0;) 1)
  (export "test" (func 0))
  (export "memory" (memory 0))
  (data (;0;) (i32.const 0) "\aa\bb\cc\dd\ee\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"))
    "#,
    );
    let (mut store, func) = default_test_setup(&wasm);
    let func = func.typed::<(), ()>(&store).unwrap();
    assert_success(func.call(&mut store, ()));
    println!("{:?}", store.tracer);
    let json_body = store.tracer.to_json();
    println!("{:?}", json_body);
}

#[test]
fn test_global_variable() {
    let wasm = wat2wasm(
        r#"
(module
  (type (;0;) (func))
  (func (;0;) (type 0)
    global.get 0
    drop)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 127))
  (export "test" (func 0))
  (export "memory" (memory 0)))
    "#,
    );
    let (mut store, func) = default_test_setup(&wasm);
    let func = func.typed::<(), ()>(&store).unwrap();
    assert_success(func.call(&mut store, ()));
    println!("{:?}", store.tracer);
    let json_body = store.tracer.to_json();
    println!("{:?}", json_body);
}

#[test]
fn function_with_local_variables() {
    let wasm = wat2wasm(
        r#"
(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32) (result i32)))
  (func (;0;) (type 1) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    i32.add
    local.set 2
    i32.const 0
    local.tee 2
    return)
  (func (;1;) (type 0)
    i32.const 100
    i32.const 20
    call 0
    drop)
  (memory (;0;) 1)
  (export "test" (func 1))
  (export "memory" (memory 0)))
    "#,
    );
    let (mut store, func) = default_test_setup(&wasm);
    let func = func.typed::<(), ()>(&store).unwrap();
    assert_success(func.call(&mut store, ()));
    println!("{:?}", store.tracer);
    let json_body = store.tracer.to_json();
    println!("{:?}", json_body);
}

#[test]
fn function_with_call() {
    let wasm = wat2wasm(
        r#"
(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32 i32 i32 i32 i32 i32 i32)))
  (type (;2;) (func (param i32 i32)))
  (import "env" "_evm_call" (func (;0;) (type 1)))
  (import "env" "_evm_return" (func (;1;) (type 2)))
  (func (;2;) (type 0)
    i32.const 0
    i32.const 0
    i32.const 32
    i32.const 64
    i32.const 320
    i32.const 0
    i32.const 32
    i32.const 64
    call 0
    i32.const 0
    i32.const 0
    i32.const 32
    i32.const 64
    i32.const 320
    i32.const 0
    i32.const 32
    i32.const 64
    call 0
    i32.const 0
    i32.const 0
    call 1)
  (memory (;0;) 1)
  (export "main" (func 2))
  (export "memory" (memory 0))
  (data (;0;) (i32.const 0) "\00\00\00\00\00\00\00\00\00\00\00\00\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\ff\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\0d\e0\b6\b3\a7d\00\00\00"))
    "#,
    );
    let (mut store, func) = default_test_setup(&wasm);
    let func = func.typed::<(), ()>(&store).unwrap();
    assert_success(func.call(&mut store, ()));
    println!("{:?}", store.tracer);
    let json_body = store.tracer.to_json();
    println!("{:?}", json_body);
}
