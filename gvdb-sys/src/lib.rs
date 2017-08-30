#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;
extern crate glib_sys;

use libc::{c_char, c_void};

use glib_sys::{gboolean, GBytes, GError, GHashTable, GVariant};

#[repr(C)]
pub struct GvdbItem(c_void);

#[repr(C)]
pub struct GvdbTable(c_void);

extern "C" {
    pub fn gvdb_hash_table_insert(table: *mut GHashTable, key: *const c_char) -> *mut GvdbItem;
    pub fn gvdb_hash_table_insert_string(table: *mut GHashTable, key: *const c_char, value: *const c_char);
    pub fn gvdb_hash_table_new(parent: *mut GHashTable, key: *const c_char) -> *mut GHashTable;

    pub fn gvdb_item_set_hash_table(item: *mut GvdbItem, table: *mut GHashTable);
    pub fn gvdb_item_set_parent(item: *mut GvdbItem, parent: *mut GvdbItem);
    pub fn gvdb_item_set_value(item: *mut GvdbItem, value: *mut GVariant);

    pub fn gvdb_table_free(table: *mut GvdbTable);
    pub fn gvdb_table_get_names(table: *mut GvdbTable) -> *mut *mut c_char;
    pub fn gvdb_table_get_raw_value(table: *mut GvdbTable, key: *const c_char) -> *mut GVariant;
    pub fn gvdb_table_get_table(table: *mut GvdbTable) -> *mut GvdbTable;
    pub fn gvdb_table_get_value(table: *mut GvdbTable, key: *const c_char) -> *mut GVariant;
    pub fn gvdb_table_has_value(table: *mut GvdbTable, key: *const c_char) -> gboolean;
    pub fn gvdb_table_is_valid(table: *mut GvdbTable) -> gboolean;
    pub fn gvdb_table_list(table: *mut GvdbTable) -> *mut *mut c_char;
    pub fn gvdb_table_new(filename: *const c_char, trusted: gboolean, error: *mut *mut GError) -> *mut GvdbTable;
    pub fn gvdb_table_new_from_bytes(bytes: *mut GBytes, trusted: gboolean, error: *mut *mut GError) -> *mut GvdbTable;
    pub fn gvdb_table_write_bytes(table: *mut GHashTable, byteswap: gboolean) -> *mut GBytes;
    pub fn gvdb_table_write_contents(table: *mut GHashTable, filename: *const c_char, byteswap: gboolean, error: *mut *mut GError) -> gboolean;
}
