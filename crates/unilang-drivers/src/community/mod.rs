// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

//! Community-contributed drivers.
//!
//! Drop a `<name>.rs` file in this directory that implements [`crate::UniLangDriver`].
//! The build system will auto-discover it and register it with the VM — no other
//! changes needed.
//!
//! See `CONTRIBUTING_DRIVERS.md` in the repo root for a step-by-step guide
//! and a copy-paste template.

// Auto-discovered community driver modules are declared here at compile time.
// The build.rs script generates `pub mod <name>;` lines into community_mods.rs
// which is included below.  This file only holds the boilerplate include.
include!(concat!(env!("OUT_DIR"), "/community_mods.rs"));
