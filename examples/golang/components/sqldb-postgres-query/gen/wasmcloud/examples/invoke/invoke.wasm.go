// Code generated by wit-bindgen-go. DO NOT EDIT.

package invoke

// This file contains wasmimport and wasmexport declarations for "wasmcloud:examples".

//go:wasmexport wasmcloud:examples/invoke#call
//export wasmcloud:examples/invoke#call
func wasmexport_Call() (result *string) {
	result_ := Exports.Call()
	result = &result_
	return
}
