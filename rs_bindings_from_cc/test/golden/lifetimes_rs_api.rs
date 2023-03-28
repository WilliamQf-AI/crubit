// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:lifetimes_cc
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

#[inline(always)]
pub fn AddHook(__param_0: Option<extern "C" fn()>) {
    unsafe { crate::detail::__rust_thunk___Z7AddHookPFvvE(__param_0) }
}

pub type FunctionPointer = Option<extern "C" fn()>;

#[inline(always)]
pub fn AddHookWithTypedef(hook: Option<extern "C" fn()>) {
    unsafe { crate::detail::__rust_thunk___Z18AddHookWithTypedefPFvvE(hook) }
}

#[inline(always)]
pub fn AddAnotherHook(__param_0: extern "C" fn()) {
    unsafe { crate::detail::__rust_thunk___Z14AddAnotherHookRFvvE(__param_0) }
}

pub type FunctionReference = extern "C" fn();

#[inline(always)]
pub fn AddAnotherHookWithTypedef(hook: extern "C" fn()) {
    unsafe { crate::detail::__rust_thunk___Z25AddAnotherHookWithTypedefRFvvE(hook) }
}

#[inline(always)]
pub unsafe fn ConsumeArray(pair: *mut i32) {
    crate::detail::__rust_thunk___Z12ConsumeArrayPi(pair)
}

// Error while generating bindings for item 'Arr':
// Unsupported type 'int[2]': Unsupported clang::Type class 'ConstantArray'

#[inline(always)]
pub unsafe fn ConsumeArrayWithTypedef(__param_0: *mut i32) {
    crate::detail::__rust_thunk___Z23ConsumeArrayWithTypedefPi(__param_0)
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_LIFETIMES_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_Z7AddHookPFvvE"]
        pub(crate) fn __rust_thunk___Z7AddHookPFvvE(__param_0: Option<extern "C" fn()>);
        #[link_name = "_Z18AddHookWithTypedefPFvvE"]
        pub(crate) fn __rust_thunk___Z18AddHookWithTypedefPFvvE(hook: Option<extern "C" fn()>);
        #[link_name = "_Z14AddAnotherHookRFvvE"]
        pub(crate) fn __rust_thunk___Z14AddAnotherHookRFvvE(__param_0: extern "C" fn());
        #[link_name = "_Z25AddAnotherHookWithTypedefRFvvE"]
        pub(crate) fn __rust_thunk___Z25AddAnotherHookWithTypedefRFvvE(hook: extern "C" fn());
        #[link_name = "_Z12ConsumeArrayPi"]
        pub(crate) fn __rust_thunk___Z12ConsumeArrayPi(pair: *mut i32);
        #[link_name = "_Z23ConsumeArrayWithTypedefPi"]
        pub(crate) fn __rust_thunk___Z23ConsumeArrayWithTypedefPi(__param_0: *mut i32);
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());
