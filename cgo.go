package zkwasm_wasmi

// #cgo CFLAGS: -I${SRCDIR}/packaged/include
// #cgo LDFLAGS: -lwasmi_c_api
//
// //#cgo linux,amd64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/linux-amd64 -L${SRCDIR}/packaged/lib/linux-amd64
// //#cgo linux,arm64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/linux-aarch64 -L${SRCDIR}/packaged/lib/linux-aarch64
// #cgo darwin,amd64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/darwin-amd64 -L${SRCDIR}/packaged/lib/darwin-amd64
// #cgo darwin,arm64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/darwin-aarch64 -L${SRCDIR}/packaged/lib/darwin-aarch64
//
// #include "packaged/include/wasmi.h"
import "C"

import (
    "unsafe"

    _ "embed"

    _ "github.com/wasm0/zkwasm-wasmi/packaged/include"
    _ "github.com/wasm0/zkwasm-wasmi/packaged/lib"
)

func ExecuteWasmBinaryToJson(wasmBinary []byte) (traceJson []byte, err error) {
    cVec, cLen := byteArrayToRawPointer(wasmBinary)
    res := C.execute_wasm_binary_to_json(cVec, cLen)
    traceJson = C.GoBytes(unsafe.Pointer(res.ptr), C.int(res.len))
    return traceJson, nil
}

func byteArrayToRawPointer(input []byte) (*C.uchar, C.size_t) {
    var argv = make([]C.uchar, len(input))
    for i, item := range input {
        argv[i] = C.uchar(item)
    }
    return (*C.uchar)(unsafe.Pointer(&argv[0])), C.size_t(len(input))
}
