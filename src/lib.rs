//! Direct3D 9 to Direct3D 11 / DXGI converter.

#![feature(rust_2018_preview)]
#![feature(const_fn)]
#![feature(try_trait)]
#![feature(integer_atomics)]
#![cfg_attr(feature = "cargo-clippy", warn(clippy))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(new_without_default, not_unsafe_ptr_arg_deref)
)]

#[macro_use]
extern crate log;

mod error;
pub use self::error::{Error, Result};

pub mod core;

mod entry;
pub use self::entry::*;