// Copyright 2017  Emmanuele Bassi
// Released under the terms of the GNU Lesser General Public License, either
// version 2.1 or, at your option, any later version.
// See the LICENSE file for the licensing terms.

//! # **gvdb** bindings for Rust
//!
//! GVDB is a simple database file format that stores a mapping between
//! strings and values expressed as [GVariant](https://developer.gnome.org/glib/stable/glib-GVariant.html).
//!
//! The database is written once, and cannot be modified.

#![cfg_attr(feature = "cargo-clippy", allow(doc_markdown))]

extern crate libc;
extern crate glib;
extern crate glib_sys;
extern crate gvdb_sys;

pub mod builder;
pub use builder::Builder;
