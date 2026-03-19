// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

/// Indicates which syntax style a construct was written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SyntaxOrigin {
    /// Python-style syntax (e.g., `def`, indentation blocks).
    Python,
    /// Java-style syntax (e.g., `public`, brace-delimited blocks).
    Java,
    /// Could not be definitively attributed to either language.
    Ambiguous,
    /// UniLang-specific constructs (e.g., `val`, `bridge`).
    UniLang,
}
