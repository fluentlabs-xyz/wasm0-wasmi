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

typedef void (*callback_fn_p1_ret0_t)(int32_t engine_id, int32_t p1);
typedef void (*callback_fn_p2_ret0_t)(int32_t engine_id, int32_t p1, int32_t p2);
typedef void (*callback_fn_p3_ret0_t)(int32_t engine_id, int32_t p1, int32_t p2, int32_t p3);
typedef void (*callback_fn_t)(int32_t engine_id, char* fn_name, int32_t* data, int32_t data_len);

void callbackHandle_cgo(int32_t engine_id, char* fn_name, int32_t* data, int32_t data_len);
void callbackHandle_cgo__evm_return_args(int32_t engine_id, int32_t* data, int32_t data_len);
void callbackHandle_cgo__evm_return(int32_t engine_id, int32_t p1, int32_t p2);
void callbackHandle_cgo__evm_address(int32_t engine_id, int32_t p1);
void callbackHandle_cgo_gas(int32_t engine_id, int32_t p1);

#include "packaged/include/wasmi.h"
*/
import "C"

import (
	_ "embed"
	"fmt"
	"log"
	"reflect"
	"sync"
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

type WasmEnginesPool struct {
	// engine_id -> WasmEngine
	pool     map[int32]*WasmEngine
	poolLock sync.Mutex
}

func NewWasmEnginesPool() *WasmEnginesPool {
	return &WasmEnginesPool{
		pool: make(map[int32]*WasmEngine),
	}
}

func (wep *WasmEnginesPool) Add(id int32, engine *WasmEngine) bool {
	wep.poolLock.Lock()
	defer wep.poolLock.Unlock()

	if _, ok := wep.pool[id]; ok {
		return false
	}
	wep.pool[id] = engine

	return true
}

func (wep *WasmEnginesPool) Get(id int32) *WasmEngine {
	wep.poolLock.Lock()
	defer wep.poolLock.Unlock()

	if we, ok := wep.pool[id]; ok {
		return we
	}
	return nil
}

var wasmEnginesPool = NewWasmEnginesPool()

type WasmEngine struct {
	id             int32
	execContexts   map[string]ExecContext
	registeredLock sync.Mutex
}

type ExecContext interface {
	//Callback interface{}
	//Context  interface{}
}

func createWasmEngine() (id int32, err error) {
	engine_id := C.create_wasm_engine()
	engineId := int32(engine_id)
	return engineId, nil
}

func NewWasmEngine() *WasmEngine {
	id, _ := createWasmEngine()
	entity := &WasmEngine{
		id:           id,
		execContexts: make(map[string]ExecContext),
	}
	ok := wasmEnginesPool.Add(id, entity)
	if !ok {
		log.Panicf("tried to register wasm engine with existing id %d\n", id)
	}
	return entity
}

func (we *WasmEngine) SetWasmBinary(wasmBinary []byte) bool {
	cVec, cLen := byteArrayToRawPointer(wasmBinary)
	res := C.set_wasm_binary(C.int(we.id), cVec, cLen)
	return bool(res)
}

func (we *WasmEngine) ComputeTrace() (traceJson []byte, err error) {
	res := C.compute_trace(C.int(we.id))
	traceJson = C.GoBytes(unsafe.Pointer(res.ptr), C.int(res.len))
	return traceJson, nil
}

func (we *WasmEngine) MemoryData() (data []byte, err error) {
	res := C.memory_data(C.int(we.id))
	data = C.GoBytes(unsafe.Pointer(res.ptr), C.int(res.len))
	return data, nil
}

func (we *WasmEngine) TraceMemoryChange(offset, len uint32, data []byte) (err error) {
	cVec, cLen := byteArrayToRawPointer(data)
	C.trace_memory_change(C.int(we.id), C.uint32_t(offset), C.uint32_t(len), cVec, cLen)
	return nil
}

func (we *WasmEngine) register(name string, execContext ExecContext) {
	we.registeredLock.Lock()
	defer we.registeredLock.Unlock()

	if _, ok := we.execContexts[name]; ok {
		log.Panicf("name '%s' already occupied\n", name)
	}
	we.execContexts[name] = execContext
}

func (we *WasmEngine) getRegistered(name string) ExecContext {
	we.registeredLock.Lock()
	defer we.registeredLock.Unlock()

	found, ok := we.execContexts[name]
	if !ok {
		log.Panicf("nothing registered for name '%s'\n", name)
	}
	return found
}

func (we *WasmEngine) RegisterHostFn2(fnName string, paramsCount int, fn ExecContext) bool {
	we.register(fnName, fn)
	funcNameCStr := C.CString(fnName)
	defer C.free(unsafe.Pointer(funcNameCStr))
	result := false
	fmt.Printf("RegisterHostFn2 fnName '%s' paramsCount %d\n", fnName, paramsCount)
	//switch fnName {
	//case "_evm_return":
	res := C.register_host_fn(C.int(we.id), (*C.int8_t)(funcNameCStr), (C.callback_fn_t)(C.callbackHandle_cgo), C.int32_t(paramsCount))
	result = bool(res)
	//default:
	//	log.Panicf("unsupported fnName '%s'\n", fnName)
	//}
	return result
}

func (we *WasmEngine) RegisterHostFn(fnName string, fn ExecContext) bool {
	we.register(fnName, fn)
	funcNameCStr := C.CString(fnName)
	defer C.free(unsafe.Pointer(funcNameCStr))
	result := false
	switch fnName {
	case "_evm_return":
		res := C.register_host_fn_p2_ret0(C.int(we.id), (*C.int8_t)(funcNameCStr), (C.callback_fn_p2_ret0_t)(C.callbackHandle_cgo__evm_return))
		result = bool(res)
	default:
		log.Panicf("unsupported fnName '%s'\n", fnName)
	}
	return result
}

func cArrayToSlice(array *C.int32_t, len C.int) []int32 {
	var list []int32
	sliceHeader := (*reflect.SliceHeader)(unsafe.Pointer(&list))
	sliceHeader.Cap = int(len)
	sliceHeader.Len = int(len)
	sliceHeader.Data = uintptr(unsafe.Pointer(array))
	return list
}

func cArrayToString(array *C.char, len C.int) string {
	var list []byte
	sliceHeader := (*reflect.SliceHeader)(unsafe.Pointer(&list))
	sliceHeader.Cap = int(len)
	sliceHeader.Len = int(len)
	sliceHeader.Data = uintptr(unsafe.Pointer(array))
	fmt.Printf("cArrayToString '%v'\n", list)
	return string(list)
}

func cCharPtrToString(p *C.char) string {
	s := C.GoString(p)
	C.free(unsafe.Pointer(p))
	fmt.Printf("cCharPtrToString '%v'\n", s)
	return s
}

//export callbackHandle_cgo__evm_return
func callbackHandle_cgo__evm_return(engine_id C.int32_t, p1 C.int32_t, p2 C.int32_t) {
	const FN_NAME = "_evm_return"
	engineId := int32(engine_id)
	wasmEngine := wasmEnginesPool.Get(engineId)
	if wasmEngine == nil {
		log.Panicf("not existing wasm engine id %d", engineId)
	}
	execContext := wasmEngine.getRegistered(FN_NAME)
	if cb, ok := execContext.(func(params []int32)); ok {
		cb([]int32{int32(p1), int32(p2)})
	} else {
		log.Panicf("failed to cast FN_NAME '%s', check registered funtion (registeredFunctions)\n", FN_NAME)
	}
}

//export callbackHandle_cgo__evm_return_args
func callbackHandle_cgo__evm_return_args(engine_id C.int32_t, data *C.int32_t, data_len C.int32_t) {
	const FN_NAME = "_evm_return"
	args := cArrayToSlice(data, data_len)
	engineId := int32(engine_id)
	wasmEngine := wasmEnginesPool.Get(engineId)
	if wasmEngine == nil {
		log.Panicf("wasm engine id %d doesn't exist", engineId)
	}
	execContext := wasmEngine.getRegistered(FN_NAME)
	if cb, ok := execContext.(func(params []int32)); ok {
		fmt.Printf("calling cb for '%s': %v\n", FN_NAME, args[1:])
		cb(args[1:])
	} else {
		log.Panicf("failed to cast FN_NAME '%s', check registered funtion (registeredFunctions)\n", FN_NAME)
	}
}

//export callbackHandle_cgo
func callbackHandle_cgo(engine_id C.int32_t, fn_name *C.char, data *C.int32_t, data_len C.int32_t) {
	//const FN_NAME = "_evm_return"
	engineId := int32(engine_id)
	fnName := cCharPtrToString(fn_name)
	args := cArrayToSlice(data, data_len)
	fmt.Printf("callbackHandle_cgo engineId %d fnName '%s' args %v\n", engineId, fnName, args)
	wasmEngine := wasmEnginesPool.Get(engineId)
	if wasmEngine == nil {
		log.Panicf("wasm engine id %d doesn't exist", engineId)
	}
	execContext := wasmEngine.getRegistered(fnName)
	if cb, ok := execContext.(func(params []int32)); ok {
		fmt.Printf("calling cb for '%s': %v\n", fnName, args[1:])
		cb(args[1:])
	} else {
		log.Panicf("failed to cast fnName '%s', check registered function\n", fnName)
	}
}

//export callbackHandle_cgo__evm_address
func callbackHandle_cgo__evm_address(engine_id C.int32_t, p1 C.int32_t) {
	const FN_NAME = "_evm_address"
	engineId := int32(engine_id)
	wasmEngine := wasmEnginesPool.Get(engineId)
	if wasmEngine == nil {
		log.Panicf("not existing wasm engine id %d", engineId)
	}
	execContext := wasmEngine.getRegistered(FN_NAME)
	if cb, ok := execContext.(func(params []int32)); ok {
		cb([]int32{int32(p1)})
	} else {
		log.Panicf("failed to cast FN_NAME '%s', check registered funtion (registeredFunctions)\n", FN_NAME)
	}
}

//export callbackHandle_cgo_gas
func callbackHandle_cgo_gas(engine_id C.int32_t, p1 C.int32_t) {
	const FN_NAME = "gas"
	engineId := int32(engine_id)
	wasmEngine := wasmEnginesPool.Get(engineId)
	if wasmEngine == nil {
		log.Panicf("not existing wasm engine id %d", engineId)
	}
	execContext := wasmEngine.getRegistered(FN_NAME)
	if cb, ok := execContext.(func(params []int32)); ok {
		cb([]int32{int32(p1)})
	} else {
		log.Panicf("failed to cast FN_NAME '%s', check registered funtion (registeredFunctions)", FN_NAME)
	}
}
