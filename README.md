# gvdb.rs

### Rust binding for GVDB

GVDB is a simple database file format that stores a mapping from strings to
[GVariant][gvariant-api] values in a way that is extremely efficient for
lookups.

The database is written once and can not be modified.

The `gvdb` crate provides a Rust API around the low level C implementation
of the GVDB builder and reader API.

[gvariant-api]: https://developer.gnome.org/glib/stable/glib-GVariant.html
