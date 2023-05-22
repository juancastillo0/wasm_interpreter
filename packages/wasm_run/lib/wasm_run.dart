/// A Web Assembly executor for the Dart programming language.
///
/// Currently it uses the [`wasmtime 8.0`](https://github.com/bytecodealliance/wasmtime)
/// or [`wasmi 0.29`](https://github.com/paritytech/wasmi) Rust crates
/// for parsing and executing WASM modules. Bindings are created using
/// [`package:flutter_rust_bridge`](https://github.com/fzyzcjy/flutter_rust_bridge).
///
/// For more information on usage and documentation, please visit the
/// main repository: https://github.com/juancastillo0/wasm_interpreter.
library wasm_run;

export 'src/ffi.dart' show setDynamicLibrary;
export 'src/wasm_bindings/wasm.dart';