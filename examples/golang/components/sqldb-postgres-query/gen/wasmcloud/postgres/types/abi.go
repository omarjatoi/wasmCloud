// Code generated by wit-bindgen-go. DO NOT EDIT.

package types

import (
	"github.com/bytecodealliance/wasm-tools-go/cm"
	"unsafe"
)

// QueryErrorShape is used for storage in variant or result types.
type QueryErrorShape struct {
	_     cm.HostLayout
	shape [unsafe.Sizeof(QueryError{})]byte
}

// TupleLowerLeftPointUpperRightPointShape is used for storage in variant or result types.
type TupleLowerLeftPointUpperRightPointShape struct {
	_     cm.HostLayout
	shape [unsafe.Sizeof(cm.Tuple[LowerLeftPoint, UpperRightPoint]{})]byte
}
