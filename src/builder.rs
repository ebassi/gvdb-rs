//! Build and write a GVDB database
//!
//! ## Examples
//!
//! extern crate glib;
//! extern crate gvdb;
//!
//! fn main() {
//!     gvdb::Builder::new()
//!                   .item("foo", glib::Variant::from("hello"))
//!                   .item("bar", glib::Variant::from("world"))
//!                   .item("baz", glib::Variant::from(42))
//!                   .filename("entries.db")
//!                   .write();
//! }

use std::path::{PathBuf, Path};
use std::ptr;

use glib::translate::*;
use glib_sys;
use gvdb_sys;
use Error;
use Variant;

#[derive(Clone, Debug)]
pub struct Builder {
    table: Vec<(String, Variant)>,
    filename: Option<PathBuf>,
    byteswap: bool,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            table: Vec::new(),
            filename: None,
            byteswap: false,
        }
    }

    pub fn byteswap(&mut self, byteswap: bool) -> &mut Builder {
        self.byteswap = byteswap;
        self
    }

    pub fn item(&mut self, key: &str, value: Variant) -> &mut Builder {
        self.table.push((key.to_string(), value.clone()));
        self
    }

    pub fn write<P: AsRef<Path>>(&mut self, filename: P) -> Result<(), Error> {
        unsafe {
            let ht = gvdb_sys::gvdb_hash_table_new(ptr::null_mut(), ptr::null_mut());

            for iter in &self.table {
                let key = &iter.0;
                let value = &iter.1;
                let item = gvdb_sys::gvdb_hash_table_insert(ht, key.to_glib_none().0);

                gvdb_sys::gvdb_item_set_value(item, value.to_glib_none().0);
            }

            let mut error = ptr::null_mut();
            gvdb_sys::gvdb_table_write_contents(ht,
                                                filename.as_ref().to_glib_none().0,
                                                self.byteswap.to_glib(),
                                                &mut error);

            glib_sys::g_hash_table_unref(ht);

            if error.is_null() {
                Ok(())
            }
            else {
                Err(from_glib_full(error))
            }
        }
    }
}

#[cfg(test)]
#[cfg_attr(test, allow(dead_code))]
mod tests {
    use super::Builder;
    use glib;

    #[test]
    fn build_empty() {
        let _ = Builder::new().write("test-builder.db");
    }

    #[test]
    fn build_byteswap() {
        let _ = Builder::new().byteswap(true).write("test-builder.db");
    }

    #[test]
    fn build_items() {
        let _ =
            Builder::new()
                    .item("foo", glib::Variant::from("hello, world"))
                    .item("bar", glib::Variant::from(42))
                    .write("test-builder.db");
    }
}
