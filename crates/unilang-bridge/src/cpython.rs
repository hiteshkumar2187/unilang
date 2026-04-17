// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

//! CPython bridge stubs for UniLang v2.0 interop.
//!
//! This module provides a [`CpythonBridge`] type that will eventually expose UniLang
//! values to a CPython interpreter via the C API (likely through the `pyo3` crate).
//! All methods are stubs that return [`BridgeError::CpythonNotAvailable`] until the
//! `cpython` feature is enabled and Python headers are available.

use crate::error::BridgeError;
use crate::types::BridgeValue;

/// A handle to an active CPython interpreter session.
///
/// Construction and all operations return [`BridgeError::CpythonNotAvailable`] until
/// the `cpython` feature is enabled and the Python C API layer is implemented
/// (v2.0 roadmap).
pub struct CpythonBridge {
    _priv: (),
}

impl CpythonBridge {
    /// Attempt to initialise a CPython interpreter session.
    ///
    /// # Errors
    ///
    /// Always returns [`BridgeError::CpythonNotAvailable`] — the `cpython` feature
    /// and Python headers are required; this is a v2.0 feature.
    pub fn new() -> Result<Self, BridgeError> {
        Err(BridgeError::CpythonNotAvailable(
            "CPython bridge requires the 'cpython' feature and Python headers; this is a v2.0 feature"
                .to_string(),
        ))
    }

    /// Import a Python module and return an opaque module handle.
    ///
    /// # Arguments
    ///
    /// * `module` — dotted module name (e.g. `"os.path"`)
    ///
    /// # Errors
    ///
    /// Always returns [`BridgeError::CpythonNotAvailable`] (stub).
    pub fn import_module(&self, _module: &str) -> Result<u64, BridgeError> {
        Err(BridgeError::CpythonNotAvailable(
            "CPython bridge requires the 'cpython' feature and Python headers; this is a v2.0 feature"
                .to_string(),
        ))
    }

    /// Call a top-level function in an already-imported module.
    ///
    /// # Arguments
    ///
    /// * `module_handle` — handle returned by [`Self::import_module`]
    /// * `func`          — function name within the module
    /// * `args`          — positional arguments, already marshaled to [`BridgeValue`]
    ///
    /// # Errors
    ///
    /// Always returns [`BridgeError::CpythonNotAvailable`] (stub).
    pub fn call_function(
        &self,
        _module_handle: u64,
        _func: &str,
        _args: &[BridgeValue],
    ) -> Result<BridgeValue, BridgeError> {
        Err(BridgeError::CpythonNotAvailable(
            "CPython bridge requires the 'cpython' feature and Python headers; this is a v2.0 feature"
                .to_string(),
        ))
    }

    /// Call a method on a live Python object identified by its handle.
    ///
    /// # Arguments
    ///
    /// * `obj_handle` — opaque object handle from a previous bridge call
    /// * `method`     — method name to invoke
    /// * `args`       — positional arguments, already marshaled to [`BridgeValue`]
    ///
    /// # Errors
    ///
    /// Always returns [`BridgeError::CpythonNotAvailable`] (stub).
    pub fn call_method(
        &self,
        _obj_handle: u64,
        _method: &str,
        _args: &[BridgeValue],
    ) -> Result<BridgeValue, BridgeError> {
        Err(BridgeError::CpythonNotAvailable(
            "CPython bridge requires the 'cpython' feature and Python headers; this is a v2.0 feature"
                .to_string(),
        ))
    }

    /// Read an attribute from a live Python object.
    ///
    /// # Arguments
    ///
    /// * `obj_handle` — opaque object handle from a previous bridge call
    /// * `attr`       — attribute name (e.g. `"__name__"`)
    ///
    /// # Errors
    ///
    /// Always returns [`BridgeError::CpythonNotAvailable`] (stub).
    pub fn get_attribute(&self, _obj_handle: u64, _attr: &str) -> Result<BridgeValue, BridgeError> {
        Err(BridgeError::CpythonNotAvailable(
            "CPython bridge requires the 'cpython' feature and Python headers; this is a v2.0 feature"
                .to_string(),
        ))
    }
}
