// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:static_methods_cc
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

#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct SomeClass {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::core::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeClass"), crate::SomeClass);

impl Default for SomeClass {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for SomeClass {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for SomeClass {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SomeClass {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClassaSEOS_(self, __param_0);
        }
    }
}

impl SomeClass {
    /// Example of a factory method.
    #[inline(always)]
    pub fn static_factory_method(initial_value_of_field: i32) -> crate::SomeClass {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<Self>::uninit();
            crate::detail::__rust_thunk___ZN9SomeClass21static_factory_methodEi(
                &mut __return,
                initial_value_of_field,
            );
            __return.assume_init()
        }
    }
}

impl SomeClass {
    /// Static method working on primitive types (and unrelated to the struct).
    #[inline(always)]
    pub fn static_method_that_multiplies_its_args(x: i32, y: i32) -> i32 {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeClass38static_method_that_multiplies_its_argsEii(
                x, y,
            )
        }
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_STATIC_METHODS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN9SomeClassC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeClass>,
        );
        pub(crate) fn __rust_thunk___ZN9SomeClassC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeClass>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeClass>,
        );
        pub(crate) fn __rust_thunk___ZN9SomeClassaSERKS_<'a, 'b>(
            __this: &'a mut crate::SomeClass,
            __param_0: &'b crate::SomeClass,
        ) -> &'a mut crate::SomeClass;
        pub(crate) fn __rust_thunk___ZN9SomeClassaSEOS_<'a, 'b>(
            __this: &'a mut crate::SomeClass,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeClass>,
        ) -> &'a mut crate::SomeClass;
        pub(crate) fn __rust_thunk___ZN9SomeClass21static_factory_methodEi(
            __return: &mut ::core::mem::MaybeUninit<crate::SomeClass>,
            initial_value_of_field: i32,
        );
        #[link_name = "_ZN9SomeClass38static_method_that_multiplies_its_argsEii"]
        pub(crate) fn __rust_thunk___ZN9SomeClass38static_method_that_multiplies_its_argsEii(
            x: i32,
            y: i32,
        ) -> i32;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::SomeClass>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::SomeClass>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeClass: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeClass: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeClass: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::SomeClass, field_) == 0);
