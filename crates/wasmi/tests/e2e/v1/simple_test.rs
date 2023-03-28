//! Tests to check if wasmi's fuel metering works as intended.

use std::fmt::Debug;
use wasmi::{Config, Engine, Func, Linker, Module, Store};
use wasmi_core::{Trap};

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
