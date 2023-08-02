//! Stable ABI functions which provide access to Ruby internals that
//! is compatible across Ruby versions, and are guaranteed to be not break due
//! to Ruby binary changes.
//!
//! ### Goals
//!
//! 1. To provide access to Ruby internals that are not exposed by the libruby
//!    (i.e. C macros and inline functions).
//! 2. Provide support for Ruby development versions, which can make breaking
//!    changes without semantic versioning. We want to support these versions
//!    to ensure Rust extensions don't prevent the Ruby core team from testing
//!    changes in production.

use crate::VALUE;
use std::os::raw::{c_char, c_long};

pub trait StableApiDefinition {
    /// Get the length of a Ruby string (akin to `RSTRING_LEN`).
    ///
    /// # Safety
    /// This function is unsafe because it dereferences a raw pointer to get
    /// access to underlying Ruby data. The caller must ensure that the pointer
    /// is valid.
    unsafe fn rstring_len(&self, obj: VALUE) -> c_long;

    /// Get a pointer to the bytes of a Ruby string (akin to `RSTRING_PTR`).
    ///
    /// # Safety
    /// This function is unsafe because it dereferences a raw pointer to get
    /// access to underlying Ruby data. The caller must ensure that the pointer
    /// is valid.
    unsafe fn rstring_ptr(&self, obj: VALUE) -> *const c_char;

    /// Get the length of a Ruby array (akin to `RARRAY_LEN`).
    ///
    /// # Safety
    /// This function is unsafe because it dereferences a raw pointer to get
    /// access to underlying Ruby data. The caller must ensure that the pointer
    /// is valid.
    unsafe fn rarray_len(&self, obj: VALUE) -> c_long;

    /// Get a pointer to the elements of a Ruby array (akin to `RARRAY_CONST_PTR`).
    ///
    /// # Safety
    /// This function is unsafe because it dereferences a raw pointer to get
    /// access to underlying Ruby data. The caller must ensure that the pointer
    /// is valid.
    unsafe fn rarray_const_ptr(&self, obj: VALUE) -> *const VALUE;

    /// Tests if the given value is a special constant.
    fn special_const_p(&self, value: VALUE) -> bool;

    /// Queries the type of the object.
    ///
    /// # Note
    /// The input `obj` must not be a special constant.
    ///
    /// # Safety
    /// This function is unsafe because it could dereference a raw pointer when
    /// attemping to access the underlying [`RBasic`] struct.
    unsafe fn builtin_type(&self, obj: VALUE) -> crate::ruby_value_type;

    /// Tests if the object's type is the given type.
    ///
    /// # Safety
    /// This function is unsafe because it could dereference a raw pointer when
    /// attemping to access the underlying [`RBasic`] struct.
    unsafe fn type_p(&self, obj: VALUE, ty: crate::ruby_value_type) -> bool;

    /// Checks if the given object is nil.
    fn nil_p(&self, obj: VALUE) -> bool;

    /// Checks if the given object is a so-called Fixnum.
    fn fixnum_p(&self, obj: VALUE) -> bool;

    /// Checks if the given object is a dynamic symbol.
    ///
    /// # Safety
    /// This function is unsafe because it could dereference a raw pointer when
    /// attemping to access the underlying [`RBasic`] struct.
    unsafe fn dynamic_sym_p(&self, obj: VALUE) -> bool;

    /// Checks if the given object is a static symbol.
    fn static_sym_p(&self, obj: VALUE) -> bool;

    /// Checks if the given object is a symbol.
    ///
    /// # Safety
    /// This function is unsafe because it could dereference a raw pointer when
    /// attemping to access the underlying [`RBasic`] struct.
    unsafe fn symbol_p(&self, obj: VALUE) -> bool;

    /// Checks if the given object is a so-called Flonum.
    ///
    /// # Safety
    /// This function is unsafe because it could dereference a raw pointer when
    /// attemping to access the underlying [`RBasic`] struct.
    unsafe fn float_type_p(&self, obj: VALUE) -> bool;

    /// Checks if the given object is an integer type
    ///
    /// # Safety
    /// This function is unsafe because it could dereference a raw pointer when
    /// attemping to access the underlying [`RBasic`] struct.
    unsafe fn integer_type_p(&self, obj: VALUE) -> bool;

    /// Checks if the given object is a so-called Flonum.
    fn flonum_p(&self, obj: VALUE) -> bool;

    /// Checks if the given  object is  an immediate  i.e. an  object which  has
    /// no corresponding storage inside of the object space.
    fn immediate_p(&self, obj: VALUE) -> bool;

    /// Emulates Ruby's "if" statement by testing if the given `obj` is neither `Qnil` or `Qfalse`.
    ///
    /// # Safety
    /// This function is unsafe because it could dereference a raw pointer when
    /// attemping to access the underlying [`RBasic`] struct.
    fn rb_test(&self, ob: VALUE) -> bool;

    /// Queries the type of the object. Identical to `StableApi.builtin_type`,
    /// except it can also accept special constants.
    ///
    /// # Safety
    /// This function is unsafe because it could dereference a raw pointer when
    /// attemping to access the underlying [`RBasic`] struct.
    unsafe fn rb_type(&self, obj: VALUE) -> crate::ruby_value_type;
}

#[cfg(stable_api_enable_compiled_mod)]
mod compiled;
#[cfg(stable_api_export_compiled_as_api)]
use compiled as api;

#[cfg(stable_api_include_rust_impl)]
#[cfg_attr(ruby_eq_2_6, path = "stable_api/ruby_2_6.rs")]
#[cfg_attr(ruby_eq_2_7, path = "stable_api/ruby_2_7.rs")]
#[cfg_attr(ruby_eq_3_0, path = "stable_api/ruby_3_0.rs")]
#[cfg_attr(ruby_eq_3_1, path = "stable_api/ruby_3_1.rs")]
#[cfg_attr(ruby_eq_3_2, path = "stable_api/ruby_3_2.rs")]
mod rust;
#[cfg(not(stable_api_export_compiled_as_api))]
use rust as api;

/// Get the default stable API definition for the current Ruby version.
pub const fn get_default() -> &'static api::Definition {
    const API: api::Definition = api::Definition {};
    &API
}

/// Get the fallback stable API definition for the current Ruby version, which
/// is compiled C code that is linked into to this crate.
#[cfg(all(stable_api_enable_compiled_mod))]
pub const fn get_compiled() -> &'static compiled::Definition {
    const COMPILED_API: compiled::Definition = compiled::Definition {};
    &COMPILED_API
}
