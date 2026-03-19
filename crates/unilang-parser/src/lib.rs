// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

//! # UniLang Parser
//!
//! Parses a token stream into a unified AST that represents both Python
//! and Java syntax. Uses a context-stack approach for disambiguation
//! and a Pratt parser for expressions.
//!
//! **Status:** Placeholder — implementation in progress.

pub mod ast;

pub use ast::Module;
