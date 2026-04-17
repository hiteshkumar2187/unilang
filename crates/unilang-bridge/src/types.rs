// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

//! Type marshaling between UniLang RuntimeValue and cross-VM bridge values.
//!
//! This module defines [`BridgeValue`], which mirrors [`unilang_runtime::value::RuntimeValue`]
//! but includes variants for opaque JVM and CPython object handles. The conversion functions
//! [`runtime_to_bridge`] and [`bridge_to_runtime`] are stubs reserved for v2.0 implementation.

use unilang_runtime::value::RuntimeValue;

/// A value that can be passed across VM boundaries (JVM or CPython).
///
/// This mirrors `RuntimeValue` but adds opaque handle variants for foreign objects
/// that live in a JVM or CPython heap and cannot be represented as native Rust values.
#[derive(Debug, Clone)]
pub enum BridgeValue {
    /// The null/None value.
    Null,
    /// A boolean value.
    Bool(bool),
    /// A 64-bit signed integer.
    Int(i64),
    /// A 64-bit floating-point number.
    Float(f64),
    /// A UTF-8 string.
    String(String),
    /// An ordered list of bridge values.
    List(Vec<BridgeValue>),
    /// An ordered map of string keys to bridge values.
    Dict(Vec<(String, BridgeValue)>),
    /// An opaque reference to a live JVM object.
    JavaObject {
        /// Fully-qualified class name (e.g. `"java.util.ArrayList"`).
        class: String,
        /// Opaque handle into the JVM object table managed by the JNI layer.
        handle: u64,
    },
    /// An opaque reference to a live CPython object.
    PythonObject {
        /// The module the object originates from (e.g. `"collections"`).
        module: String,
        /// The type/class name of the object (e.g. `"OrderedDict"`).
        name: String,
        /// Opaque handle into the CPython object table managed by the bridge layer.
        handle: u64,
    },
}

/// Convert a [`RuntimeValue`] into a [`BridgeValue`].
///
/// # Panics
///
/// Always panics — this is a v2.0 stub. Full implementation requires the `jvm` or
/// `cpython` feature and the corresponding native bridge runtime to be active.
pub fn runtime_to_bridge(_v: &RuntimeValue) -> BridgeValue {
    panic!("not yet implemented: JVM/CPython bridge is a v2.0 feature")
}

/// Convert a [`BridgeValue`] back into a [`RuntimeValue`].
///
/// # Panics
///
/// Always panics — this is a v2.0 stub. Full implementation requires the `jvm` or
/// `cpython` feature and the corresponding native bridge runtime to be active.
pub fn bridge_to_runtime(_v: BridgeValue) -> RuntimeValue {
    panic!("not yet implemented: JVM/CPython bridge is a v2.0 feature")
}
