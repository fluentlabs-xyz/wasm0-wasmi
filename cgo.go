package zkwasm_wasmi

/*
#cgo CFLAGS: -I${SRCDIR}/packaged/include
#cgo LDFLAGS: -lwasmi_c_api

#cgo linux,amd64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/linux-amd64 -L${SRCDIR}/packaged/lib/linux-amd64
#cgo linux,arm64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/linux-aarch64 -L${SRCDIR}/packaged/lib/linux-aarch64
#cgo darwin,amd64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/darwin-amd64 -L${SRCDIR}/packaged/lib/darwin-amd64
#cgo darwin,arm64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/darwin-aarch64 -L${SRCDIR}/packaged/lib/darwin-aarch64

#include <stdint.h>
#include <stdlib.h>

typedef void (*callback_fn_json_trace)(int32_t engine_id, char* json_trace, int32_t json_trace_len);
typedef int32_t (*callback_fn_i32_t)(int32_t engine_id, char* fn_name, int32_t fn_name_len, int32_t* data, int32_t data_len);
typedef int32_t (*callback_fn_i64_t)(int32_t engine_id, char* fn_name, int32_t fn_name_len, int64_t* data, int32_t data_len);

void callbackHandle_cgo_on_item_added_to_logs(int32_t engine_id, char* json_trace, int32_t json_trace_len);
int32_t callbackHandle_cgo_i32(int32_t engine_id, char* fn_name, int32_t fn_name_len, int32_t* data, int32_t data_len);
int32_t callbackHandle_cgo_i64(int32_t engine_id, char* fn_name, int32_t fn_name_len, int64_t* data, int32_t data_len);

#include "packaged/include/wasmi.h"
*/
import "C"

import (
	_ "embed"
	"errors"
	"log"
	"reflect"
	"strconv"
	"strings"
	"sync"
	"unsafe"

	_ "github.com/wasm0/zkwasm-wasmi/packaged/include"
	_ "github.com/wasm0/zkwasm-wasmi/packaged/lib"
)

type ComputeTraceErrorCode int32

const (
	ComputeTraceErrorCodeOk ComputeTraceErrorCode = iota
	ComputeTraceErrorCodeOutOfGas
	ComputeTraceErrorCodeExecutionReverted
	ComputeTraceErrorCodeStopToken
	ComputeTraceErrorCodeUnknown
)

var (
	ErrorOutOfGas = errors.New("out of gas")
	ErrorExecutionReverted  = errors.New("execution reverted")
	ErrorStopToken = errors.New("stop token")
	ErrorUnknown = errors.New("unknown")
)

func ComputeTraceErrorFromInt32(code int32) error {
	c := ComputeTraceErrorCode(code)
	switch c {
	case ComputeTraceErrorCodeOutOfGas:
		return ErrorOutOfGas
	case ComputeTraceErrorCodeExecutionReverted:
		return ErrorExecutionReverted
	case ComputeTraceErrorCodeStopToken:
		return ErrorStopToken
	}
	return ErrorUnknown
}

func byteArrayToRawPointer(input []byte) (*C.uchar, C.size_t) {
	var capacity = len(input)
	if capacity == 0 {
		capacity = 1
	}
	var argv = make([]C.uchar, capacity)
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

type Callback interface{}

type WasmEngine struct {
	id                             int32
	execContexts                   map[string]Callback
	execContextsMutex              sync.Mutex
	onAfterItemAddedToLogsCallback Callback
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
		execContexts: make(map[string]Callback),
	}
	ok := wasmEnginesPool.Add(id, entity)
	if !ok {
		log.Panicf("tried to register wasm engine with existing id %d\n", id)
	}
	return entity
}

func (we *WasmEngine) SetWasmBinary(wasmBinary []byte) {
	cVec, cLen := byteArrayToRawPointer(wasmBinary)
	C.set_wasm_binary(C.int(we.id), cVec, cLen)
}

func (we *WasmEngine) ComputeTrace() (traceJson []byte, err error) {
	res := C.compute_trace(C.int(we.id))
	traceJson = C.GoBytes(unsafe.Pointer(res.ptr), C.int(res.len))
	if len(traceJson) < 15 {
		traceJsonStr := string(traceJson)
		if strings.HasPrefix(traceJsonStr, "error:") {
			errorCodeStr := strings.TrimPrefix(traceJsonStr, "error:")
			errorCode, err := strconv.Atoi(errorCodeStr)
			if err != nil {
				return nil, err
			}
			return nil, ComputeTraceErrorFromInt32(int32(errorCode))
		}
	}
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

func (we *WasmEngine) register(name string, callback Callback) {
	we.execContextsMutex.Lock()
	defer we.execContextsMutex.Unlock()

	if _, ok := we.execContexts[name]; ok {
		log.Panicf("name '%s' already occupied\n", name)
	}
	we.execContexts[name] = callback
}

func (we *WasmEngine) getRegistered(name string) Callback {
	we.execContextsMutex.Lock()
	defer we.execContextsMutex.Unlock()

	found, ok := we.execContexts[name]
	if !ok {
		log.Panicf("no exec context registered for '%s'\n", name)
	}
	return found
}

func (we *WasmEngine) RegisterCallbackOnAfterItemAddedToLogs(callback Callback) {
	we.onAfterItemAddedToLogsCallback = callback
	C.register_cb_on_after_item_added_to_logs(C.int(we.id), (C.callback_fn_json_trace)(C.callbackHandle_cgo_on_item_added_to_logs))
}

func (we *WasmEngine) UnRegisterOnAfterItemAddedToLogsCallback() {
	we.onAfterItemAddedToLogsCallback = nil
}

func (we *WasmEngine) RegisterHostFnI32(fnName string, paramsCount int, callback Callback) bool {
	we.register(fnName, callback)
	funcNameCStr := C.CString(fnName)
	defer C.free(unsafe.Pointer(funcNameCStr))
	result := false
	res := C.register_host_fn_i32(C.int(we.id), (*C.int8_t)(funcNameCStr), (C.callback_fn_i32_t)(C.callbackHandle_cgo_i32), C.int32_t(paramsCount))
	result = bool(res)
	return result
}

func (we *WasmEngine) RegisterHostFnI64(fnName string, paramsCount int, callback Callback) bool {
	we.register(fnName, callback)
	funcNameCStr := C.CString(fnName)
	defer C.free(unsafe.Pointer(funcNameCStr))
	result := false
	res := C.register_host_fn_i64(C.int(we.id), (*C.int8_t)(funcNameCStr), (C.callback_fn_i64_t)(C.callbackHandle_cgo_i64), C.int32_t(paramsCount))
	result = bool(res)
	return result
}

func cArrayToSliceI32(array *C.int32_t, len C.int) []int32 {
	var list []int32
	sliceHeader := (*reflect.SliceHeader)(unsafe.Pointer(&list))
	sliceHeader.Cap = int(len)
	sliceHeader.Len = int(len)
	sliceHeader.Data = uintptr(unsafe.Pointer(array))
	return list
}

func cArrayToSliceI64(array *C.int64_t, len C.int) []int64 {
	var list []int64
	sliceHeader := (*reflect.SliceHeader)(unsafe.Pointer(&list))
	sliceHeader.Cap = int(len)
	sliceHeader.Len = int(len)
	sliceHeader.Data = uintptr(unsafe.Pointer(array))
	return list
}

func cArrayToString(charPtr *C.char, len C.int) string {
	var list []byte
	sliceHeader := (*reflect.SliceHeader)(unsafe.Pointer(&list))
	sliceHeader.Cap = int(len)
	sliceHeader.Len = int(len)
	sliceHeader.Data = uintptr(unsafe.Pointer(charPtr))
	return string(list)
}

func cCharPtrToString(charPtr *C.char, len C.int32_t) string {
	s := C.GoStringN(charPtr, C.int(len))
	C.free(unsafe.Pointer(charPtr))
	return s
}

//export callbackHandle_cgo_on_item_added_to_logs
func callbackHandle_cgo_on_item_added_to_logs(engine_id C.int32_t, json_trace *C.char, json_trace_len C.int32_t) {
	engineId := int32(engine_id)
	jsonTrace := cArrayToString(json_trace, json_trace_len)
	wasmEngine := wasmEnginesPool.Get(engineId)
	if wasmEngine == nil {
		log.Panicf("wasm engine id %d doesn't exist", engineId)
	}
	if wasmEngine.onAfterItemAddedToLogsCallback != nil {
		if cb, ok := wasmEngine.onAfterItemAddedToLogsCallback.(func(jsonTrace string)); ok {
			cb(jsonTrace)
		} else {
			log.Panicf("registered callback (engine id %d) has invalid type", engineId)
		}
	}
}

//export callbackHandle_cgo_i32
func callbackHandle_cgo_i32(engine_id C.int32_t, fn_name *C.char, fn_name_len C.int32_t, data *C.int32_t, data_len C.int32_t) C.int32_t {
	engineId := int32(engine_id)
	fnName := cCharPtrToString(fn_name, fn_name_len)
	args := cArrayToSliceI32(data, data_len)
	wasmEngine := wasmEnginesPool.Get(engineId)
	if wasmEngine == nil {
		log.Panicf("wasm engine id %d doesn't exist", engineId)
	}
	execContext := wasmEngine.getRegistered(fnName)
	if cb, ok := execContext.(func(params []int32) int32); ok {
		res := cb(args[1:]) // start from index 1 to get rid of engine id value
		return C.int32_t(res)
	} else {
		log.Panicf("failed to cast fn '%s' (func(params []int32) int32)\n", fnName)
	}
	return C.int32_t(0)
}

//export callbackHandle_cgo_i64
func callbackHandle_cgo_i64(engine_id C.int32_t, fn_name *C.char, fn_name_len C.int32_t, data *C.int64_t, data_len C.int32_t) C.int32_t {
	engineId := int32(engine_id)
	fnName := cArrayToString(fn_name, fn_name_len)
	args := cArrayToSliceI64(data, data_len)
	wasmEngine := wasmEnginesPool.Get(engineId)
	if wasmEngine == nil {
		log.Panicf("wasm engine id %d doesn't exist", engineId)
	}
	execContext := wasmEngine.getRegistered(fnName)
	if cb, ok := execContext.(func(params []int64) int32); ok {
		res := cb(args[1:])
		return C.int32_t(res)
	} else {
		log.Panicf("failed to cast fn '%s' to (func(params []int64) int32)\n", fnName)
	}
	return C.int32_t(0)
}
