// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

//! `unilang-bridge` — JVM/CPython interop scaffolding for UniLang v2.0.
//!
//! This crate provides the type marshaling layer and stub bridge implementations
//! that will enable UniLang programs to call into Java (via JNI) and CPython
//! (via the C API / pyo3) in a future release.
//!
//! # Feature flags
//!
//! | Flag       | Description                                                          |
//! |------------|----------------------------------------------------------------------|
//! | `jvm`      | Enable the JNI bridge (requires a JVM installation and the `jni` crate) |
//! | `cpython`  | Enable the CPython bridge (requires Python headers and the `pyo3` crate) |
//!
//! Neither flag is on by default — all public API currently returns an appropriate
//! `BridgeError` explaining that the feature is not yet implemented.

/// Cross-VM error types.
pub mod error;

/// Type marshaling between [`unilang_runtime::value::RuntimeValue`] and [`types::BridgeValue`].
pub mod types;

/// JVM interop stubs (v2.0).
pub mod jvm;

/// CPython interop stubs (v2.0).
pub mod cpython;
