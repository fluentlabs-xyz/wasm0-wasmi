package zkwasm_wasmi

import (
	_ "embed"
	"fmt"
	"testing"
)

//go:embed testdata/simple.wasm
var simpleWasmBinary []byte

func TestJsonTrace(t *testing.T) {
	traceJson, err := ExecuteWasmBinaryToJson(simpleWasmBinary)
	if err != nil {
		fmt.Printf("failed to Inject, reason '%s'", err)
	}
	stringRes := string(traceJson)
	println(stringRes)
}
