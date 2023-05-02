#![allow(rustdoc::bare_urls)]
#![doc = include_str!("../readme.md")]

pub mod bindings;
pub mod macros;
pub mod special_consts;
pub mod value_type;

mod hidden;
mod ruby_abi_version;
mod utils;

pub use bindings::*;
pub use macros::*;
pub use ruby_abi_version::*;
pub use special_consts::*;
pub use value_type::*;

pub type Value = VALUE;
pub type RubyValue = VALUE;

#[cfg(use_global_allocator)]
ruby_global_allocator!();

#[cfg(use_ruby_abi_version)]
ruby_abi_version!();
