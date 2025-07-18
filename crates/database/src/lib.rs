#![expect(clippy::missing_errors_doc, reason = "WIP")]
#![expect(clippy::unused_async, reason = "WIP")]

#[expect(clippy::pub_use, reason = "macros")]
pub mod schema;

pub mod gardyns;
pub mod models;
pub mod plants;
pub mod slots;

pub fn database() {}
