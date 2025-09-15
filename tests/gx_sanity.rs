//! GX Package Sanity Tests
//!
//! Wrapper file following RSB test conventions for gx module testing.

#[path = "gx/sanity/string.rs"]
mod gx_string_sanity;

#[path = "gx/sanity/id.rs"]
mod gx_id_sanity;

#[path = "gx/sanity/collection.rs"]
mod gx_collection_sanity;