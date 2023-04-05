package zkwasm_wasmi

/*
#cgo CFLAGS: -I${SRCDIR}/packaged/include
#cgo LDFLAGS: -lwasmi_c_api

#cgo linux,amd64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/linux-amd64 -L${SRCDIR}/packaged/lib/linux-amd64
//#cgo linux,arm64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/linux-aarch64 -L${SRCDIR}/packaged/lib/linux-aarch64
#cgo darwin,amd64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/darwin-amd64 -L${SRCDIR}/packaged/lib/darwin-amd64
#cgo darwin,arm64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/darwin-aarch64 -L${SRCDIR}/packaged/lib/darwin-aarch64

//#include <stdint.h>
#include <stdlib.h>

typedef void (*callback_fn_p1_ret0_t)(int32_t p1);
typedef void (*callback_fn_p2_ret0_t)(int32_t p1, int32_t p2);
typedef void (*callback_fn_p3_ret0_t)(int32_t p1, int32_t p2, int32_t p3);
typedef int* (*callback_fn_t)(int* params);

void callbackHandle_cgo__evm_return(int32_t p1, int32_t p2);
void callbackHandle_cgo__evm_address(int32_t p1);
void callbackHandle_cgo_gas(int32_t p1);

#include "packaged/include/wasmi.h"
*/
import "C"

import (
	_ "embed"
	"log"
	"reflect"
	"unsafe"

	_ "github.com/wasm0/zkwasm-wasmi/packaged/include"
	_ "github.com/wasm0/zkwasm-wasmi/packaged/lib"
)

func byteArrayToRawPointer(input []byte) (*C.uchar, C.size_t) {
	var argv = make([]C.uchar, len(input))
	for i, item := range input {
		argv[i] = C.uchar(item)
	}
	return (*C.uchar)(unsafe.Pointer(&argv[0])), C.size_t(len(input))
}

func ExecuteWasmBinaryToJson(wasmBinary []byte) (traceJson []byte, err error) {
	cVec, cLen := byteArrayToRawPointer(wasmBinary)
	res := C.execute_wasm_binary_to_json(cVec, cLen)
	traceJson = C.GoBytes(unsafe.Pointer(res.ptr), C.int(res.len))
	return traceJson, nil
}

func CreateWasmEngine() (id int32, err error) {
	res := C.create_wasm_engine()
	return int32(res), nil
}

func SetWasmBinary(wasmEngineId int32, wasmBinary []byte) bool {
	cVec, cLen := byteArrayToRawPointer(wasmBinary)
	res := C.set_wasm_binary(C.int(wasmEngineId), cVec, cLen)
	return bool(res)
}

func ComputeTrace(wasmEngineId int32) (traceJson []byte, err error) {
	res := C.compute_trace(C.int(wasmEngineId))
	traceJson = C.GoBytes(unsafe.Pointer(res.ptr), C.int(res.len))
	return traceJson, nil
}

type ExecContext interface {
	//Callback interface{}
	//Context  interface{}
}

var registeredFunctions = make(map[string]ExecContext)

func registerFunction(fnName string, wrapper ExecContext) {
	if _, ok := registeredFunctions[fnName]; ok {
		log.Panicf("fn with name '%s' already registered", fnName)
	}
	registeredFunctions[fnName] = wrapper
}

func RegisterHostFn(wasmEngineId int32, fnName string, fn ExecContext) bool {
	registerFunction(fnName, fn)
	funcNameCStr := C.CString(fnName)
	defer C.free(unsafe.Pointer(funcNameCStr))
	result := false
	switch fnName {
	case "_evm_return":
		res := C.register_host_fn_p2_ret0(C.int(wasmEngineId), (*C.int8_t)(funcNameCStr), (C.callback_fn_p2_ret0_t)(C.callbackHandle_cgo__evm_return))
		result = bool(res)
	case "_evm_address":
		res := C.register_host_fn_p1_ret0(C.int(wasmEngineId), (*C.int8_t)(funcNameCStr), (C.callback_fn_p1_ret0_t)(C.callbackHandle_cgo__evm_address))
		result = bool(res)
	case "gas":
		res := C.register_host_fn_p1_ret0(C.int(wasmEngineId), (*C.int8_t)(funcNameCStr), (C.callback_fn_p1_ret0_t)(C.callbackHandle_cgo_gas))
		result = bool(res)
	default:
		log.Panicf("unsupported fnName '%s'", fnName)
	}
	return result
}

func cArrayToslice(array *C.int, len int) []int {
	var list []int
	sliceHeader := (*reflect.SliceHeader)((unsafe.Pointer(&list)))
	sliceHeader.Cap = len
	sliceHeader.Len = len
	sliceHeader.Data = uintptr(unsafe.Pointer(array))
	return list
}

//export callbackHandle_cgo__evm_return
func callbackHandle_cgo__evm_return(p1 C.int32_t, p2 C.int32_t) {
	const FN_NAME = "_evm_return"
	if execContext, ok := registeredFunctions[FN_NAME]; ok {
		if cb, ok := execContext.(func(params []int32)); ok {
			cb([]int32{int32(p1), int32(p2)})
		} else {
			log.Panicf("failed to cast FN_NAME '%s', check registered funtion (registeredFunctions)", FN_NAME)
		}
	} else {
		log.Panicf("unregistered FN_NAME '%s'", FN_NAME)
	}
}

//export callbackHandle_cgo__evm_address
func callbackHandle_cgo__evm_address(p1 C.int32_t) {
	const FN_NAME = "_evm_address"
	if execContext, ok := registeredFunctions[FN_NAME]; ok {
		if cb, ok := execContext.(func(params []int32)); ok {
			cb([]int32{int32(p1)})
		} else {
			log.Panicf("failed to cast FN_NAME '%s', check registered funtion (registeredFunctions)", FN_NAME)
		}
	} else {
		log.Panicf("unregistered FN_NAME '%s'", FN_NAME)
	}
}

//export callbackHandle_cgo_gas
func callbackHandle_cgo_gas(p1 C.int32_t) {
	const FN_NAME = "gas"
	if execContext, ok := registeredFunctions[FN_NAME]; ok {
		if cb, ok := execContext.(func(params []int32)); ok {
			cb([]int32{int32(p1)})
		} else {
			log.Panicf("failed to cast FN_NAME '%s', check registered funtion (registeredFunctions)", FN_NAME)
		}
	} else {
		log.Panicf("unregistered FN_NAME '%s'", FN_NAME)
	}
}
