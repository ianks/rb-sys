#include "ruby.h"

long
impl_rstring_len(VALUE obj) {
  return RSTRING_LEN(obj);
}

char *
impl_rstring_ptr(VALUE obj) {
  return RSTRING_PTR(obj);
}

long
impl_rarray_len(VALUE obj) {
  return RARRAY_LEN(obj);
}

const VALUE *
impl_rarray_const_ptr(VALUE obj) {
  return RARRAY_CONST_PTR(obj);
}

int
impl_special_const_p(VALUE obj) {
  return SPECIAL_CONST_P(obj);
}

enum ruby_value_type
impl_builtin_type(VALUE obj) {
  return RB_BUILTIN_TYPE(obj);
}

int
impl_nil_p(VALUE obj) {
  return NIL_P(obj);
}

int
impl_fixnum_p(VALUE obj) {
  return FIXNUM_P(obj);
}

int
impl_static_sym_p(VALUE obj) {
  return STATIC_SYM_P(obj);
}

int
impl_flonum_p(VALUE obj) {
  return FLONUM_P(obj);
}

int
impl_immediate_p(VALUE obj) {
  return IMMEDIATE_P(obj);
}

int
impl_rb_test(VALUE obj) {
  return RB_TEST(obj);
}

int
impl_type_p(VALUE obj, enum ruby_value_type type) {
  return RB_TYPE_P(obj, type);
}

int
impl_dynamic_sym_p(VALUE obj) {
  return RB_DYNAMIC_SYM_P(obj);
}

int impl_symbol_p(VALUE obj) {
  return RB_SYMBOL_P(obj);
}

int impl_float_type_p(VALUE obj) {
  return RB_FLOAT_TYPE_P(obj);
}

enum ruby_value_type
impl_rb_type(VALUE obj) {
  return rb_type(obj);
}

int
impl_integer_type_p(VALUE obj) {
  return RB_INTEGER_TYPE_P(obj);
}
