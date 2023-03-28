// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:types_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Error while generating bindings for item 'PtrDiff':
// Unsupported type 'decltype(static_cast<int *>(nullptr) - static_cast<int *>(nullptr))': Unsupported clang::Type class 'Decltype'

// Error while generating bindings for item 'Size':
// Unsupported type 'decltype(sizeof (0))': Unsupported clang::Type class 'Decltype'

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SomeStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeStruct"), crate::SomeStruct);

impl Default for SomeStruct {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for SomeStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for SomeStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SomeStruct {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructaSEOS_(self, __param_0);
        }
    }
}

forward_declare::forward_declare!(pub ForwardDeclaredStruct = forward_declare::symbol!("ForwardDeclaredStruct"));

#[derive(Clone, Copy)]
#[repr(C, align(8))]
pub struct FieldTypeTestStruct {
    pub bool_field: bool,
    pub char_field: u8,
    pub unsigned_char_field: u8,
    pub signed_char_field: i8,
    pub char16_t_field: u16,
    pub char32_t_field: u32,
    pub wchar_t_field: i32,
    pub short_field: i16,
    pub int_field: i32,
    pub long_field: i64,
    pub long_long_field: i64,
    pub unsigned_short_field: u16,
    pub unsigned_int_field: u32,
    pub unsigned_long_field: u64,
    pub unsigned_long_long_field: u64,
    pub signed_short_field: i16,
    pub signed_int_field: i32,
    pub signed_long_field: i64,
    pub signed_long_long_field: i64,
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'PtrDiff': No generated bindings found for 'PtrDiff'
    pub(crate) ptrdiff_t_field: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Unsupported type 'Size': No generated bindings found for 'Size'
    pub(crate) size_t_field: [::core::mem::MaybeUninit<u8>; 8],
    pub float_field: f32,
    pub double_field: f64,
    pub ptr_field: *mut i32,
    pub void_ptr_field: *mut ::core::ffi::c_void,
    pub const_void_ptr_field: *const ::core::ffi::c_void,
    pub void_double_ptr_field: *mut *mut ::core::ffi::c_void,
    pub struct_field: crate::SomeStruct,
    pub struct_ptr_field: *mut crate::SomeStruct,
    pub const_struct_ptr_field: *const crate::SomeStruct,
    pub struct_ref_field: *mut crate::SomeStruct,
    pub const_struct_ref_field: *const crate::SomeStruct,
    /// TODO(b/226580208): Uncomment when these don't cause struct import to fail.
    /// SomeStruct&& struct_rvalue_ref_field;
    /// const SomeStruct&& const_struct_rvalue_ref_field;
    pub forward_declared_ptr_field: *mut crate::ForwardDeclaredStruct,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("FieldTypeTestStruct"),
    crate::FieldTypeTestStruct
);

impl<'b> From<::ctor::RvalueReference<'b, Self>> for FieldTypeTestStruct {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN19FieldTypeTestStructC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

#[inline(always)]
pub fn VoidReturningFunction() {
    unsafe { crate::detail::__rust_thunk___Z21VoidReturningFunctionv() }
}

/// Note especially the use of references. If we convert those to pointers,
/// this becomes un-compilable. The syntax here is awful, but this is a function
/// returning a function. In ML-like syntax:
/// FunctionPointerReturningFunction : () -> (const int&, int*) -> int&
#[inline(always)]
pub fn FunctionPointerReturningFunction() -> Option<extern "C" fn(*const i32, *mut i32) -> *mut i32>
{
    unsafe { crate::detail::__rust_thunk___Z32FunctionPointerReturningFunctionv() }
}

#[inline(always)]
pub unsafe fn FunctionWithVoidPointers(
    __param_0: *mut ::core::ffi::c_void,
    __param_1: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    crate::detail::__rust_thunk___Z24FunctionWithVoidPointersPvPKv(__param_0, __param_1)
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPES_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN10SomeStructC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeStruct>,
        );
        pub(crate) fn __rust_thunk___ZN10SomeStructC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeStruct>,
        );
        pub(crate) fn __rust_thunk___ZN10SomeStructaSERKS_<'a, 'b>(
            __this: &'a mut crate::SomeStruct,
            __param_0: &'b crate::SomeStruct,
        ) -> &'a mut crate::SomeStruct;
        pub(crate) fn __rust_thunk___ZN10SomeStructaSEOS_<'a, 'b>(
            __this: &'a mut crate::SomeStruct,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeStruct>,
        ) -> &'a mut crate::SomeStruct;
        pub(crate) fn __rust_thunk___ZN19FieldTypeTestStructC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::FieldTypeTestStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::FieldTypeTestStruct>,
        );
        pub(crate) fn __rust_thunk___Z21VoidReturningFunctionv();
        pub(crate) fn __rust_thunk___Z32FunctionPointerReturningFunctionv()
        -> Option<extern "C" fn(*const i32, *mut i32) -> *mut i32>;
        pub(crate) fn __rust_thunk___Z24FunctionWithVoidPointersPvPKv(
            __param_0: *mut ::core::ffi::c_void,
            __param_1: *const ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::SomeStruct>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::SomeStruct>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeStruct: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::FieldTypeTestStruct>() == 200);
const _: () = assert!(::core::mem::align_of::<crate::FieldTypeTestStruct>() == 8);
const _: () = {
    static_assertions::assert_impl_all!(crate::FieldTypeTestStruct: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::FieldTypeTestStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::FieldTypeTestStruct: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, bool_field) == 0);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, char_field) == 1);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, unsigned_char_field) == 2);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, signed_char_field) == 3);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, char16_t_field) == 4);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, char32_t_field) == 8);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, wchar_t_field) == 12);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, short_field) == 16);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, int_field) == 20);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, long_field) == 24);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, long_long_field) == 32);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, unsigned_short_field) == 40);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, unsigned_int_field) == 44);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, unsigned_long_field) == 48);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, unsigned_long_long_field) == 56);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, signed_short_field) == 64);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, signed_int_field) == 68);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, signed_long_field) == 72);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, signed_long_long_field) == 80);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, ptrdiff_t_field) == 88);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, size_t_field) == 96);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, float_field) == 104);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, double_field) == 112);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, ptr_field) == 120);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, void_ptr_field) == 128);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, const_void_ptr_field) == 136);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, void_double_ptr_field) == 144);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, struct_field) == 152);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, struct_ptr_field) == 160);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, const_struct_ptr_field) == 168);
const _: () = assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, struct_ref_field) == 176);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, const_struct_ref_field) == 184);
const _: () =
    assert!(memoffset::offset_of!(crate::FieldTypeTestStruct, forward_declared_ptr_field) == 192);
