// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:bitfields_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, type_alias_impl_trait)]
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

#[::ctor::recursively_pinned]
#[repr(C, align(4))]
pub struct WithBitfields {
    // f1 : 2 bits
    __bitfields0: [::core::mem::MaybeUninit<u8>; 1],
    pub f2: i32,
    // f3 : 4 bits
    // f4 : 8 bits
    //  : 45 bits
    __bitfields2: [::core::mem::MaybeUninit<u8>; 10],
    pub f5: i32,
    // f6 : 23 bits
    __bitfields4: [::core::mem::MaybeUninit<u8>; 3],
    pub(crate) f7: [::core::mem::MaybeUninit<u8>; 1],
    // f8 : 2 bits
    __bitfields6: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("WithBitfields"), crate::WithBitfields);
impl WithBitfields {
    pub fn f7(&self) -> &u8 {
        unsafe { &*(&self.f7 as *const _ as *const u8) }
    }
}

impl ::ctor::CtorNew<()> for WithBitfields {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN13WithBitfieldsC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for WithBitfields {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN13WithBitfieldsC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for WithBitfields {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for WithBitfields {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN13WithBitfieldsC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for WithBitfields {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for WithBitfields {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13WithBitfieldsaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for WithBitfields {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN13WithBitfieldsaSEOS_(self, __param_0);
        }
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_BITFIELDS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN13WithBitfieldsC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::WithBitfields>,
        );
        pub(crate) fn __rust_thunk___ZN13WithBitfieldsC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::WithBitfields>,
            __param_0: &'b crate::WithBitfields,
        );
        pub(crate) fn __rust_thunk___ZN13WithBitfieldsC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::WithBitfields>,
            __param_0: ::ctor::RvalueReference<'b, crate::WithBitfields>,
        );
        pub(crate) fn __rust_thunk___ZN13WithBitfieldsaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::WithBitfields>,
            __param_0: &'b crate::WithBitfields,
        ) -> ::core::pin::Pin<&'a mut crate::WithBitfields>;
        pub(crate) fn __rust_thunk___ZN13WithBitfieldsaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::WithBitfields>,
            __param_0: ::ctor::RvalueReference<'b, crate::WithBitfields>,
        ) -> ::core::pin::Pin<&'a mut crate::WithBitfields>;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::WithBitfields>() == 32);
const _: () = assert!(::core::mem::align_of::<crate::WithBitfields>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::WithBitfields: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::WithBitfields: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::WithBitfields, f2) == 4);
const _: () = assert!(memoffset::offset_of!(crate::WithBitfields, f5) == 20);
const _: () = assert!(memoffset::offset_of!(crate::WithBitfields, f7) == 27);
