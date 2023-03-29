(module
 (memory $0 1)
 (export "memory" (memory $0))
 (export "main" (func $main))
 (func $main (; 0 ;) (result)
  (i32.const 100)
  (i32.const 20)
  (i32.const 3)
  (i32.add)
  (i32.add)
  (drop)
 )
)

