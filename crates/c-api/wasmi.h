#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

#ifdef __cplusplus
extern "C" {
#endif

struct ResultStruct {
  uintptr_t len;
  unsigned char *data;
  int exit_code;
} inject_into_utf8_wat_or_binary_wasm_external(
    unsigned const char *utf8_wat_or_binary_wasm_bytes_ptr_c,
    uintptr_t utf8_wat_or_binary_wasm_bytes_length_c,
    int inject_type,
    int inject_gas_type,
    int instruction_cost,
    int memory_grow_cost,
    int call_per_local_cost,
    int stack_limit,
    int return_format
);

#ifdef __cplusplus
} // extern "C"
#endif
