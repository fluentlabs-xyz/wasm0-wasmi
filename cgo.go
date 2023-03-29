package zkwasm_wasmi

// #cgo CFLAGS: -I${SRCDIR}/packaged/include
// #cgo LDFLAGS: -lwasmi_c_api
//
// //#cgo linux,amd64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/linux-amd64 -L${SRCDIR}/packaged/lib/linux-amd64
// //#cgo linux,arm64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/linux-aarch64 -L${SRCDIR}/packaged/lib/linux-aarch64
// #cgo darwin,amd64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/darwin-amd64 -L${SRCDIR}/packaged/lib/darwin-amd64
// #cgo darwin,arm64 LDFLAGS: -Wl,-rpath,${SRCDIR}/packaged/lib/darwin-aarch64 -L${SRCDIR}/packaged/lib/darwin-aarch64
//
// const char* execute_wasm_binary_to_json(const char* wasm_binary);
import "C"

import (
	"unsafe"

	"github.com/pkg/errors"

	_ "github.com/wasm0/zkwasm-wasmi/packaged/include"
	_ "github.com/wasm0/zkwasm-wasmi/packaged/lib"
)

func Inject(wasmBinary []byte) (traceJson []byte, err error) {
	if wasmBinary == nil {
		return nil, errors.New("parameter [watStrOrBinaryAsm] must be set")
	}
	var argv = make([]C.uchar, len(wasmBinary))
	for i, item := range wasmBinary {
		argv[i] = C.uchar(item)
	}
	cResultStruct := C.inject_into_utf8_wat_or_binary_wasm_external(&argv[0])
	if cResultStruct.exit_code != 0 {
		return nil, errors.New("execution failed")
	}
	var sliceRes = unsafe.Slice(cResultStruct.data, int(cResultStruct.len))
	traceJson = make([]byte, len(sliceRes))
	for i, v := range sliceRes {
		traceJson[i] = byte(v)
	}
	return
}
