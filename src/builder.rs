//! Build and write a GVDB database
//!
//! ## Examples
//!
//! ```rust,no_run
//! extern crate glib;
//! extern crate gvdb;
//!
//! fn main() {
//!     let db =
//!         gvdb::Builder::new()
//!                       .item("foo", glib::Variant::from("hello"))
//!                       .item("bar", glib::Variant::from("world"))
//!                       .item("baz", glib::Variant::from(42))
//!                       .write_to_buffer();
//! }
//! ```

#![deny(missing_docs)]

use std::path::{PathBuf, Path};
use std::ptr;

use glib::{
    Bytes,
    Error,
    Variant,
};

use glib::translate::*;
use glib_sys;
use gvdb_sys;

/// Builds a glib::Variant database and saves it to a buffer or file
#[derive(Clone, Debug)]
pub struct Builder {
    table: Vec<(String, Variant)>,
    filename: Option<PathBuf>,
    byteswap: bool,
}

impl Builder {
    /// Creates a new database builder instance.
    ///
    /// New keys are added to the database using the [`item`] method.
    ///
    /// The builder is finished with the [`write_to_buffer`] or the [`write_to_file`]
    /// methods.
    ///
    /// [`item`]: struct.Builder.html#method.item
    /// [`write_to_buffer`]: struct.Builder.html#method.write_to_buffer
    /// [`write_to_file`]: struct.Builder.html#method.write_to_file
    pub fn new() -> Builder {
        Builder {
            table: Vec::new(),
            filename: None,
            byteswap: false,
        }
    }

    /// Sets whether the contents of the database should be byteswapped
    /// when writing.
    ///
    /// Typically, you want to use the default value inside Builder.
    ///
    /// A case for overriding the default behavior is if you wish to
    /// serialize the database contents using the system's endianness.
    pub fn byteswap(&mut self, byteswap: bool) -> &mut Builder {
        self.byteswap = byteswap;
        self
    }

    /// Adds a new value, using the given key, to the database builder.
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// extern crate glib;
    /// extern crate gvdb;
    ///
    /// let db = gvdb::Builder::new()
    ///                        .item("hello", glib::Variant::from("world"))
    ///                        .item("answer", glib::Variant::from(42))
    ///                        .write_to_buffer();
    /// ```
    pub fn item(&mut self, key: &str, value: Variant) -> &mut Builder {
        self.table.push((key.to_string(), value.clone()));
        self
    }

    /// Writes the content of the database to a glib::Bytes buffer.
    pub fn write_to_buffer(&mut self) -> Bytes{
        unsafe {
            let ht = gvdb_sys::gvdb_hash_table_new(ptr::null_mut(), ptr::null_mut());

            for iter in &self.table {
                let key = &iter.0;
                let value = &iter.1;
                let item = gvdb_sys::gvdb_hash_table_insert(ht, key.to_glib_none().0);

                gvdb_sys::gvdb_item_set_value(item, value.to_glib_none().0);
            }

            let bytes = from_glib_full(gvdb_sys::gvdb_table_write_bytes(ht, self.byteswap.to_glib()));

            glib_sys::g_hash_table_unref(ht);

            bytes
        }
    }

    /// Writes the content of the database to a given file
    ///
    /// ## Example
    ///
    /// ```rust,no_run
    /// extern crate glib;
    /// extern crate gvdb;
    ///
    /// use std::path::Path;
    ///
    /// let path = Path::new("/path/to/file.db");
    ///
    /// gvdb::Builder::new().item("hello", glib::Variant::from("world")).write_to_file(path);
    /// ```
    pub fn write_to_file<P: AsRef<Path>>(&mut self, filename: P) -> Result<(), Error> {
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
    use std::fs;
    use std::path::Path;
    use super::Builder;
    use glib;

    #[test]
    fn build_empty() {
        Builder::new().write_to_buffer();
    }

    #[test]
    fn build_byteswap() {
        Builder::new().byteswap(true).write_to_buffer();
    }

    #[test]
    fn build_items() {
        Builder::new()
                .item("foo", glib::Variant::from("hello, world"))
                .item("bar", glib::Variant::from(42))
                .write_to_buffer();
    }

    #[test]
    fn build_file() {
        let db_path = Path::new("test-builder.db");
        Builder::new()
                .item("hello", glib::Variant::from("world"))
                .write_to_file(db_path);
        assert_eq!(db_path.exists(), true);
        fs::remove_file(db_path);
    }
}
