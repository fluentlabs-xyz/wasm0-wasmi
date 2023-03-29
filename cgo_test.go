package zkwasm_wasmi

import (
	_ "embed"
	"fmt"
	"testing"
)

//go:embed testdata/fixtures/gas/simple.wat
var gasSimpleWatData string

//go:embed testdata/expectations/gas/simple_host_fn.wat
var gasSimpleWatDataExpected string

func TestGasSimple(t *testing.T) {
	moduleBytes := []byte(gasSimpleWatData)
	bytesRes, err := Inject(
		moduleBytes,
		InjectTypeGas,
		InjectGasTypeHost,
		1,
		0,
		0,
		1024,
		ReturnFormatWat,
	)
	if err != nil {
		fmt.Printf("failed to Inject, reason '%s'", err)
	}
	stringRes := string(bytesRes)
	if stringRes != gasSimpleWatDataExpected {
		t.Errorf(
			"result!=gasSimpleWatDataExpected. result='%s' gasSimpleWatDataExpected='%s'",
			stringRes,
			gasSimpleWatDataExpected,
		)
	}
}

//go:embed testdata/fixtures/gas/hello.wat
var gasHelloWatData string

//go:embed testdata/expectations/gas/hello_host_fn.wat
var gasHelloWatDataExpected string

func TestGasHello(t *testing.T) {
	moduleBytes := []byte(gasHelloWatData)
	bytesRes, err := Inject(
		moduleBytes,
		InjectTypeBoth,
		InjectGasTypeHost,
		1,
		10000,
		0,
		1024,
		ReturnFormatWat,
	)
	if err != nil {
		fmt.Printf("failed to Inject, reason '%s'", err)
	}
	stringRes := string(bytesRes)
	if stringRes != gasHelloWatDataExpected {
		t.Errorf(
			"result!=gasSimpleWatDataExpected. result='%s' gasSimpleWatDataExpected='%s'",
			stringRes,
			gasHelloWatDataExpected,
		)
	}
}

//go:embed testdata/fixtures/stack-height/simple.wat
var stackHeightSimpleWatData string

//go:embed testdata/expectations/stack-height/simple.wat
var stackHeightSimpleWatDataExpected string

func TestStackHeightSimple(t *testing.T) {
	moduleBytes := []byte(stackHeightSimpleWatData)
	bytesRes, err := Inject(
		moduleBytes,
		InjectTypeStackHeight,
		InjectGasTypeHost,
		1,
		0,
		0,
		1024,
		ReturnFormatWat,
	)
	if err != nil {
		fmt.Printf("failed to Inject, reason '%s'", err)
	}
	stringRes := string(bytesRes)
	if stringRes != stackHeightSimpleWatDataExpected {
		t.Errorf(
			"result!=gasSimpleWatDataExpected. result='%s' gasSimpleWatDataExpected='%s'",
			stringRes,
			gasSimpleWatDataExpected,
		)
	}
}
