(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func))
  (import "env" "_evm_return" (func (;0;) (type 0)))
  (func (;1;) (type 1)
    i32.const 1048576
    i32.const 12
    call 0)
  (memory (;0;) 17)
  (global (;0;) (mut i32) (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048588))
  (global (;2;) i32 (i32.const 1048592))
  (export "memory" (memory 0))
  (export "main" (func 1))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data (;0;) (i32.const 1048576) "Hello, World"))
