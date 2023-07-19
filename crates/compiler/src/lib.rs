#![allow(dead_code)]

pub use crate::compiler::Compiler;

mod compiler;
mod drop_keep;
pub mod module;
mod opcode;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module::CompiledModule;

    fn wat2wasm(wat: &str) -> Vec<u8> {
        wat::parse_str(wat).unwrap()
    }

    #[test]
    fn main() {
        let wasm_binary = wat2wasm(
            r#"
(module
  (func $main
    global.get 0
    global.get 1
    call $add
    global.get 2
    call $add
    drop
    )
  (func $add (param $lhs i32) (param $rhs i32) (result i32)
    local.get $lhs
    local.get $rhs
    i32.add
    )
  (global (;0;) i32 (i32.const 100))
  (global (;1;) i32 (i32.const 20))
  (global (;2;) i32 (i32.const 3))
  (export "main" (func $main)))
    "#,
        );
        let mut translator = Compiler::new(&wasm_binary).unwrap();
        translator.translate().unwrap();
        let binary = translator.finalize().unwrap();
        println!("{:?}", binary);
        let module = CompiledModule::from_vec(&binary).unwrap();
        let trace = module.trace_binary();
        println!("{}", trace);
    }
}
