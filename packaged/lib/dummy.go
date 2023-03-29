// See https://github.com/golang/go/issues/26366
package lib

import (
	_ "github.com/wasm0/zkwasm-gas-injector/packaged/lib/darwin-aarch64"
	_ "github.com/wasm0/zkwasm-gas-injector/packaged/lib/darwin-amd64"
	_ "github.com/wasm0/zkwasm-gas-injector/packaged/lib/linux-amd64"
)
