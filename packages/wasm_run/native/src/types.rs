use std::fmt::Display;

use anyhow::Result;
use flutter_rust_bridge::RustOpaque;

use crate::external::*;
#[cfg(not(feature = "wasmtime"))]
pub use wasmi::{core::Pages, Func, Global, GlobalType, Memory, Mutability, Table};
#[cfg(not(feature = "wasmtime"))]
use wasmi::{core::ValueType, *};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum WasmVal {
    /// Value of 32-bit signed or unsigned integer.
    i32(i32),
    /// Value of 64-bit signed or unsigned integer.
    i64(i64),
    /// Value of 32-bit IEEE 754-2008 floating point number.
    f32(f32),
    /// Value of 64-bit IEEE 754-2008 floating point number.
    f64(f64),
    /// A 128 bit number.
    v128([u8; 16]),
    /// A nullable function.
    funcRef(Option<RustOpaque<WFunc>>),
    /// A nullable external object reference.
    externRef(Option<u32>), // NonZeroU32
}

impl WasmVal {
    #[cfg(not(feature = "wasmtime"))]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_value(self, ctx: impl AsContextMut) -> Value {
        match self {
            WasmVal::i32(i) => Value::I32(i),
            WasmVal::i64(i) => Value::I64(i),
            WasmVal::f32(i) => Value::F32(i.to_bits().into()),
            WasmVal::f64(i) => Value::F64(i.to_bits().into()),
            WasmVal::v128(_i) => panic!("v128 is not supported in wasmi"),
            WasmVal::funcRef(i) => {
                let inner = i.map(|f| Func::clone(&f.func_wasmi));
                Value::FuncRef(FuncRef::new(inner))
            }
            WasmVal::externRef(i) => Value::ExternRef(ExternRef::new::<u32>(ctx, i)),
        }
    }

    #[cfg(not(feature = "wasmtime"))]
    pub fn from_value<'a, T: 'a>(value: &Value, ctx: impl Into<StoreContext<'a, T>>) -> Self {
        match value {
            Value::I32(i) => WasmVal::i32(*i),
            Value::I64(i) => WasmVal::i64(*i),
            Value::F32(i) => WasmVal::f32(i.to_float()),
            Value::F64(i) => WasmVal::f64(i.to_float()),
            Value::FuncRef(i) => WasmVal::funcRef(i.func().map(|f| RustOpaque::new((*f).into()))), // NonZeroU32::new(1).unwrap()),
            Value::ExternRef(i) => {
                WasmVal::externRef(i.data(ctx).map(|i| *i.downcast_ref::<u32>().unwrap()))
            } // NonZeroU32::new(1).unwrap()),
        }
    }

    #[cfg(feature = "wasmtime")]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_val(self) -> wasmtime::Val {
        match self {
            WasmVal::i32(i) => wasmtime::Val::I32(i),
            WasmVal::i64(i) => wasmtime::Val::I64(i),
            WasmVal::f32(i) => wasmtime::Val::F32(i.to_bits()),
            WasmVal::f64(i) => wasmtime::Val::F64(i.to_bits()),
            WasmVal::v128(i) => wasmtime::Val::V128(u128::from_ne_bytes(i)),
            WasmVal::funcRef(i) => wasmtime::Val::FuncRef(i.map(|f| f.func_wasmtime)),
            WasmVal::externRef(i) => wasmtime::Val::ExternRef(i.map(wasmtime::ExternRef::new)),
        }
    }

    #[cfg(feature = "wasmtime")]
    pub fn from_val(val: wasmtime::Val) -> Self {
        match val {
            wasmtime::Val::I32(i) => WasmVal::i32(i),
            wasmtime::Val::I64(i) => WasmVal::i64(i),
            wasmtime::Val::V128(i) => WasmVal::v128(i.to_ne_bytes()),
            wasmtime::Val::F32(i) => WasmVal::f32(f32::from_bits(i)),
            wasmtime::Val::F64(i) => WasmVal::f64(f64::from_bits(i)),
            wasmtime::Val::FuncRef(i) => WasmVal::funcRef(i.map(|f| RustOpaque::new(f.into()))),
            wasmtime::Val::ExternRef(i) => {
                WasmVal::externRef(i.map(|i| *i.data().downcast_ref::<u32>().unwrap()))
            }
        }
    }
}

#[derive(Debug)]
pub struct GlobalTy {
    /// The value type of the global variable.
    pub value: ValueTy,
    /// The mutability of the global variable.
    pub mutable: bool,
}

#[cfg(not(feature = "wasmtime"))]
impl From<&GlobalType> for GlobalTy {
    fn from(value: &GlobalType) -> Self {
        GlobalTy {
            value: (&value.content()).into(),
            mutable: value.mutability() == Mutability::Var,
        }
    }
}

#[cfg(feature = "wasmtime")]
impl From<&wasmtime::GlobalType> for GlobalTy {
    fn from(value: &wasmtime::GlobalType) -> Self {
        GlobalTy {
            value: (*value.content()).into(),
            mutable: value.mutability() == wasmtime::Mutability::Var,
        }
    }
}

#[derive(Debug)]
pub struct TableTy {
    /// The type of values stored in the [WasmTable].
    pub element: ValueTy,
    /// The minimum number of elements the [WasmTable] must have.
    pub minimum: u32,
    /// The optional maximum number of elements the [WasmTable] can have.
    ///
    /// If this is `None` then the [WasmTable] is not limited in size.
    pub maximum: Option<u32>,
}

#[cfg(not(feature = "wasmtime"))]
impl From<&TableType> for TableTy {
    fn from(value: &TableType) -> Self {
        TableTy {
            element: (&value.element()).into(),
            minimum: value.minimum(),
            maximum: value.maximum(),
        }
    }
}

#[cfg(feature = "wasmtime")]
impl From<&wasmtime::TableType> for TableTy {
    fn from(value: &wasmtime::TableType) -> Self {
        TableTy {
            element: value.element().into(),
            minimum: value.minimum(),
            maximum: value.maximum(),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum ValueTy {
    /// 32-bit signed or unsigned integer.
    i32,
    /// 64-bit signed or unsigned integer.
    i64,
    /// 32-bit IEEE 754-2008 floating point number.
    f32,
    /// 64-bit IEEE 754-2008 floating point number.
    f64,
    /// A 128 bit number.
    v128,
    /// A nullable function reference.
    funcRef,
    /// A nullable external reference.
    externRef,
}

#[cfg(not(feature = "wasmtime"))]
impl From<&ValueType> for ValueTy {
    fn from(value: &ValueType) -> Self {
        match value {
            ValueType::I32 => ValueTy::i32,
            ValueType::I64 => ValueTy::i64,
            ValueType::F32 => ValueTy::f32,
            ValueType::F64 => ValueTy::f64,
            ValueType::FuncRef => ValueTy::funcRef,
            ValueType::ExternRef => ValueTy::externRef,
        }
    }
}

#[cfg(feature = "wasmtime")]
impl From<wasmtime::ValType> for ValueTy {
    fn from(value: wasmtime::ValType) -> Self {
        match value {
            wasmtime::ValType::I32 => ValueTy::i32,
            wasmtime::ValType::I64 => ValueTy::i64,
            wasmtime::ValType::F32 => ValueTy::f32,
            wasmtime::ValType::F64 => ValueTy::f64,
            wasmtime::ValType::V128 => ValueTy::v128,
            wasmtime::ValType::FuncRef => ValueTy::funcRef,
            wasmtime::ValType::ExternRef => ValueTy::externRef,
        }
    }
}

#[cfg(not(feature = "wasmtime"))]
impl From<ValueTy> for ValueType {
    fn from(value: ValueTy) -> Self {
        match value {
            ValueTy::i32 => ValueType::I32,
            ValueTy::i64 => ValueType::I64,
            ValueTy::f32 => ValueType::F32,
            ValueTy::f64 => ValueType::F64,
            ValueTy::v128 => panic!("V128 not supported for wasmi"),
            ValueTy::funcRef => ValueType::FuncRef,
            ValueTy::externRef => ValueType::ExternRef,
        }
    }
}

#[cfg(feature = "wasmtime")]
impl From<ValueTy> for wasmtime::ValType {
    fn from(value: ValueTy) -> Self {
        match value {
            ValueTy::i32 => wasmtime::ValType::I32,
            ValueTy::i64 => wasmtime::ValType::I64,
            ValueTy::f32 => wasmtime::ValType::F32,
            ValueTy::f64 => wasmtime::ValType::F64,
            ValueTy::v128 => wasmtime::ValType::V128,
            ValueTy::funcRef => wasmtime::ValType::FuncRef,
            ValueTy::externRef => wasmtime::ValType::ExternRef,
        }
    }
}

#[derive(Debug)]
pub struct ModuleImport {
    pub module: String,
    pub name: String,
    pub value: ExternalValue,
}

/// The type of an external (imported or exported) WASM value.
#[derive(Debug)]
pub enum ExternalType {
    /// A [FuncTy].
    Func(FuncTy),
    /// A [GlobalTy].
    Global(GlobalTy),
    /// A [TableTy].
    Table(TableTy),
    /// A [MemoryTy].
    Memory(MemoryTy),
}

#[cfg(not(feature = "wasmtime"))]
impl From<&ExternType> for ExternalType {
    fn from(import: &ExternType) -> Self {
        match import {
            ExternType::Func(f) => ExternalType::Func(f.into()),
            ExternType::Global(f) => ExternalType::Global(f.into()),
            ExternType::Table(f) => ExternalType::Table(f.into()),
            ExternType::Memory(f) => ExternalType::Memory(f.into()),
        }
    }
}

#[cfg(feature = "wasmtime")]
impl From<&wasmtime::ExternType> for ExternalType {
    fn from(import: &wasmtime::ExternType) -> Self {
        match import {
            wasmtime::ExternType::Func(f) => ExternalType::Func(f.into()),
            wasmtime::ExternType::Global(f) => ExternalType::Global(f.into()),
            wasmtime::ExternType::Table(f) => ExternalType::Table(f.into()),
            wasmtime::ExternType::Memory(f) => ExternalType::Memory(f.into()),
        }
    }
}

#[derive(Debug)]
pub struct ModuleImportDesc {
    pub module: String,
    pub name: String,
    pub ty: ExternalType,
}

#[cfg(not(feature = "wasmtime"))]
impl From<&ImportType<'_>> for ModuleImportDesc {
    fn from(import: &ImportType) -> Self {
        ModuleImportDesc {
            module: import.module().to_string(),
            name: import.name().to_string(),
            ty: import.ty().into(),
        }
    }
}

#[cfg(feature = "wasmtime")]
impl From<&wasmtime::ImportType<'_>> for ModuleImportDesc {
    fn from(import: &wasmtime::ImportType) -> Self {
        ModuleImportDesc {
            module: import.module().to_string(),
            name: import.name().to_string(),
            ty: (&import.ty()).into(),
        }
    }
}

#[derive(Debug)]
pub struct FuncTy {
    /// The number of function parameters.
    pub parameters: Vec<ValueTy>,
    /// The ordered and merged parameter and result types of the function type.]
    pub results: Vec<ValueTy>,
}

#[cfg(not(feature = "wasmtime"))]
impl From<&FuncType> for FuncTy {
    fn from(func: &FuncType) -> Self {
        FuncTy {
            parameters: func.params().iter().map(ValueTy::from).collect(),
            results: func.results().iter().map(ValueTy::from).collect(),
        }
    }
}

#[cfg(feature = "wasmtime")]
impl From<&wasmtime::FuncType> for FuncTy {
    fn from(func: &wasmtime::FuncType) -> Self {
        FuncTy {
            parameters: func.params().map(ValueTy::from).collect(),
            results: func.results().map(ValueTy::from).collect(),
        }
    }
}

#[allow(dead_code)]
pub enum ParallelExec {
    Ok(Vec<WasmVal>),
    Err(String),
    Call(FunctionCall),
}

pub struct FunctionCall {
    pub args: Vec<WasmVal>,
    pub function_id: u32,
    pub function_pointer: usize,
    pub num_results: usize,
    pub worker_index: usize,
}

#[derive(Debug)]
pub struct ModuleExportDesc {
    pub name: String,
    pub ty: ExternalType,
}

#[cfg(not(feature = "wasmtime"))]
impl From<&ExportType<'_>> for ModuleExportDesc {
    fn from(export: &ExportType) -> Self {
        ModuleExportDesc {
            name: export.name().to_string(),
            ty: export.ty().into(),
        }
    }
}

#[cfg(feature = "wasmtime")]
impl From<&wasmtime::ExportType<'_>> for ModuleExportDesc {
    fn from(export: &wasmtime::ExportType) -> Self {
        ModuleExportDesc {
            name: export.name().to_string(),
            ty: (&export.ty()).into(),
        }
    }
}

#[derive(Debug)]
pub struct ModuleExportValue {
    pub desc: ModuleExportDesc,
    pub value: ExternalValue,
}

impl ModuleExportValue {
    #[cfg(not(feature = "wasmtime"))]
    pub fn from_export<T>(export: Export, store: &Store<T>) -> Self {
        ModuleExportValue {
            desc: ModuleExportDesc {
                name: export.name().to_string(),
                ty: (&export.ty(store)).into(),
            },
            value: export.into_extern().into(),
        }
    }

    #[cfg(feature = "wasmtime")]
    pub fn from_export(
        export: (String, wasmtime::Extern),
        store: impl wasmtime::AsContext,
    ) -> Self {
        ModuleExportValue {
            desc: ModuleExportDesc {
                name: export.0,
                ty: (&export.1.ty(store)).into(),
            },
            value: export.1.into(),
        }
    }
}

#[cfg(feature = "wasmtime")]
#[derive(Debug)]
pub enum ExternalValue {
    Func(RustOpaque<WFunc>),
    Global(RustOpaque<wasmtime::Global>),
    Table(RustOpaque<wasmtime::Table>),
    Memory(RustOpaque<wasmtime::Memory>),
    SharedMemory(crate::api::WasmRunSharedMemory),
}

#[cfg(not(feature = "wasmtime"))]
#[derive(Debug)]
pub enum ExternalValue {
    Func(RustOpaque<WFunc>),
    Global(RustOpaque<Global>),
    Table(RustOpaque<Table>),
    Memory(RustOpaque<Memory>),
    SharedMemory(crate::api::WasmRunSharedMemory),
}

#[cfg(not(feature = "wasmtime"))]
impl From<Extern> for ExternalValue {
    fn from(extern_: Extern) -> Self {
        match extern_ {
            Extern::Func(f) => ExternalValue::Func(RustOpaque::new(f.into())),
            Extern::Global(g) => ExternalValue::Global(RustOpaque::new(g)),
            Extern::Table(t) => ExternalValue::Table(RustOpaque::new(t)),
            Extern::Memory(m) => ExternalValue::Memory(RustOpaque::new(m)),
        }
    }
}

#[cfg(feature = "wasmtime")]
impl From<wasmtime::Extern> for ExternalValue {
    fn from(extern_: wasmtime::Extern) -> Self {
        match extern_ {
            wasmtime::Extern::Func(f) => ExternalValue::Func(RustOpaque::new(f.into())),
            wasmtime::Extern::Global(g) => ExternalValue::Global(RustOpaque::new(g)),
            wasmtime::Extern::Table(t) => ExternalValue::Table(RustOpaque::new(t)),
            wasmtime::Extern::Memory(m) => ExternalValue::Memory(RustOpaque::new(m)),
            wasmtime::Extern::SharedMemory(m) => ExternalValue::SharedMemory(m.into()),
        }
    }
}

#[cfg(not(feature = "wasmtime"))]
impl From<&ExternalValue> for Extern {
    fn from(e: &ExternalValue) -> Extern {
        match e {
            ExternalValue::Func(f) => Extern::Func(f.func_wasmi),
            ExternalValue::Global(g) => Extern::Global(**g),
            ExternalValue::Table(t) => Extern::Table(**t),
            ExternalValue::Memory(m) => Extern::Memory(**m),
            ExternalValue::SharedMemory(_) => unreachable!(),
        }
    }
    // fn to_extern<T>(&self, store: &mut Store<T>) -> Result<Extern> {
    //     match self {
    //         ExternalValue::Global { value, mutability } => {
    //             let mapped = value.to_value(store);
    //             let global = Global::new(store, mapped, *mutability);
    //             Ok(Extern::Global(global))
    //         }
    //         ExternalValue::Table { value, ty } => {
    //             let mapped_value = value.to_value(store);
    //             let table = Table::new(
    //                 store,
    //                 TableType::new(mapped_value.ty(), ty.min, ty.max),
    //                 mapped_value,
    //             )
    //             .map_err(to_anyhow)?;
    //             Ok(Extern::Table(table))
    //         }
    //         ExternalValue::Memory { ty } => {
    //             let memory = Memory::new(store, ty.to_memory_type()?).map_err(to_anyhow)?;
    //             Ok(Extern::Memory(memory))
    //         }
    //         ExternalValue::Func { pointer } => {
    //             let f: wasm_func = unsafe { std::mem::transmute(*pointer) };
    //             // TODO: let func = Func::wrap(store, f);
    //             let func = Func::wrap(store, || {});
    //             Ok(Extern::Func(func))
    //         }
    //     }
    // }
}

#[cfg(feature = "wasmtime")]
impl From<&ExternalValue> for wasmtime::Extern {
    fn from(e: &ExternalValue) -> wasmtime::Extern {
        match e {
            ExternalValue::Func(f) => wasmtime::Extern::Func(f.func_wasmtime),
            ExternalValue::Global(g) => wasmtime::Extern::Global(**g),
            ExternalValue::Table(t) => wasmtime::Extern::Table(**t),
            ExternalValue::Memory(m) => wasmtime::Extern::Memory(**m),
            ExternalValue::SharedMemory(m) => {
                wasmtime::Extern::SharedMemory(m.0.read().unwrap().clone())
            }
        }
    }
}

#[derive(Debug)]
pub struct TableArgs {
    /// The minimum number of elements the [`Table`] must have.
    pub minimum: u32,
    /// The optional maximum number of elements the [`Table`] can have.
    ///
    /// If this is `None` then the [`Table`] is not limited in size.
    pub maximum: Option<u32>,
}

#[derive(Debug)]
pub struct MemoryTy {
    /// Whether or not this memory could be shared between multiple processes.
    pub shared: bool,
    /// The number of initial pages associated with the memory.
    pub minimum: u32,
    /// The maximum number of pages this memory can have.
    pub maximum: Option<u32>,
}

impl MemoryTy {
    #[cfg(not(feature = "wasmtime"))]
    pub fn to_memory_type(&self) -> Result<MemoryType> {
        MemoryType::new(self.minimum, self.maximum).map_err(to_anyhow)
    }

    #[cfg(feature = "wasmtime")]
    pub fn to_memory_type(&self) -> Result<wasmtime::MemoryType> {
        if self.shared {
            return Ok(wasmtime::MemoryType::shared(
                self.minimum,
                self.maximum.ok_or(anyhow::anyhow!(
                    "maximum_pages is required for shared memories"
                ))?,
            ));
        }
        Ok(wasmtime::MemoryType::new(self.minimum, self.maximum))
    }
}

#[cfg(not(feature = "wasmtime"))]
impl From<&MemoryType> for MemoryTy {
    fn from(memory_type: &MemoryType) -> Self {
        MemoryTy {
            minimum: memory_type.initial_pages().into(),
            maximum: memory_type.maximum_pages().map(|v| v.into()),
            shared: false,
        }
    }
}

#[cfg(feature = "wasmtime")]
impl From<&wasmtime::MemoryType> for MemoryTy {
    fn from(memory_type: &wasmtime::MemoryType) -> Self {
        MemoryTy {
            minimum: memory_type.minimum().try_into().unwrap(),
            maximum: memory_type.maximum().map(|v| v.try_into().unwrap()),
            shared: memory_type.is_shared(),
        }
    }
}

pub struct PointerAndLength {
    pub pointer: usize,
    pub length: usize,
}

pub fn to_anyhow<T: Display>(value: T) -> anyhow::Error {
    anyhow::Error::msg(value.to_string())
}
