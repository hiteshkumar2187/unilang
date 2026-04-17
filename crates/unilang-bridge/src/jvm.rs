// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

//! JVM bridge stubs for UniLang v2.0 interop.
//!
//! This module provides a [`JvmBridge`] type that will eventually expose UniLang
//! values to the JVM via JNI. All methods are stubs that return
//! [`BridgeError::JvmNotAvailable`] until the `jvm` feature is enabled and a
//! working JVM installation is present.

use crate::error::BridgeError;
use crate::types::BridgeValue;

/// A handle to an active JVM session.
///
/// Construction and all operations return [`BridgeError::JvmNotAvailable`] until
/// the `jvm` feature is enabled and the JNI layer is implemented (v2.0 roadmap).
pub struct JvmBridge {
    _priv: (),
}

impl JvmBridge {
    /// Attempt to initialise a JVM session.
    ///
    /// # Errors
    ///
    /// Always returns [`BridgeError::JvmNotAvailable`] — the `jvm` feature and a
    /// JVM installation are required; this is a v2.0 feature.
    pub fn new() -> Result<Self, BridgeError> {
        Err(BridgeError::JvmNotAvailable(
            "JVM bridge requires the 'jvm' feature and a JVM installation; this is a v2.0 feature"
                .to_string(),
        ))
    }

    /// Call a static method on a JVM class.
    ///
    /// # Arguments
    ///
    /// * `class`  — fully-qualified class name (e.g. `"java.lang.Math"`)
    /// * `method` — method name (e.g. `"abs"`)
    /// * `args`   — arguments to pass, already marshaled to [`BridgeValue`]
    ///
    /// # Errors
    ///
    /// Always returns [`BridgeError::JvmNotAvailable`] (stub).
    pub fn call_static(
        &self,
        _class: &str,
        _method: &str,
        _args: &[BridgeValue],
    ) -> Result<BridgeValue, BridgeError> {
        Err(BridgeError::JvmNotAvailable(
            "JVM bridge requires the 'jvm' feature and a JVM installation; this is a v2.0 feature"
                .to_string(),
        ))
    }

    /// Call an instance method on a JVM object identified by its handle.
    ///
    /// # Arguments
    ///
    /// * `handle` — opaque object handle obtained from [`Self::import_class`] or a previous call
    /// * `method` — method name to invoke
    /// * `args`   — arguments to pass, already marshaled to [`BridgeValue`]
    ///
    /// # Errors
    ///
    /// Always returns [`BridgeError::JvmNotAvailable`] (stub).
    pub fn call_instance(
        &self,
        _handle: u64,
        _method: &str,
        _args: &[BridgeValue],
    ) -> Result<BridgeValue, BridgeError> {
        Err(BridgeError::JvmNotAvailable(
            "JVM bridge requires the 'jvm' feature and a JVM installation; this is a v2.0 feature"
                .to_string(),
        ))
    }

    /// Look up a JVM class by name and return an opaque class handle.
    ///
    /// # Arguments
    ///
    /// * `class_name` — fully-qualified class name (e.g. `"java.util.ArrayList"`)
    ///
    /// # Errors
    ///
    /// Always returns [`BridgeError::JvmNotAvailable`] (stub).
    pub fn import_class(&self, _class_name: &str) -> Result<u64, BridgeError> {
        Err(BridgeError::JvmNotAvailable(
            "JVM bridge requires the 'jvm' feature and a JVM installation; this is a v2.0 feature"
                .to_string(),
        ))
    }

    /// Add a JAR file to the JVM class path.
    ///
    /// # Arguments
    ///
    /// * `path` — filesystem path to the `.jar` file
    ///
    /// # Errors
    ///
    /// Always returns [`BridgeError::JvmNotAvailable`] (stub).
    pub fn load_jar(&self, _path: &str) -> Result<(), BridgeError> {
        Err(BridgeError::JvmNotAvailable(
            "JVM bridge requires the 'jvm' feature and a JVM installation; this is a v2.0 feature"
                .to_string(),
        ))
    }
}
