// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:polymorphic_cc
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

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(8))]
pub struct PolymorphicBase {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 8],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("PolymorphicBase"),
    crate::PolymorphicBase
);

impl ::ctor::CtorNew<()> for PolymorphicBase {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN15PolymorphicBaseC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for PolymorphicBase {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN15PolymorphicBaseC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for PolymorphicBase {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for PolymorphicBase {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN15PolymorphicBaseaSERKS_(self, __param_0);
        }
    }
}

impl ::ctor::PinnedDrop for PolymorphicBase {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN15PolymorphicBaseD1Ev(self)
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(8))]
pub struct PolymorphicBase2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 8],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("PolymorphicBase2"),
    crate::PolymorphicBase2
);

impl ::ctor::CtorNew<()> for PolymorphicBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN16PolymorphicBase2C1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for PolymorphicBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN16PolymorphicBase2C1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for PolymorphicBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for PolymorphicBase2 {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN16PolymorphicBase2aSERKS_(self, __param_0);
        }
    }
}

impl PolymorphicBase2 {
    #[inline(always)]
    pub fn Foo<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN16PolymorphicBase23FooEv(self) }
    }
}

impl ::ctor::PinnedDrop for PolymorphicBase2 {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN16PolymorphicBase2D1Ev(self)
    }
}

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(8))]
pub struct PolymorphicDerived {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 16],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("PolymorphicDerived"),
    crate::PolymorphicDerived
);

impl ::ctor::CtorNew<()> for PolymorphicDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN18PolymorphicDerivedC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for PolymorphicDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN18PolymorphicDerivedC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for PolymorphicDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for PolymorphicDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN18PolymorphicDerivedC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for PolymorphicDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for PolymorphicDerived {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN18PolymorphicDerivedD1Ev(self)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for PolymorphicDerived {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN18PolymorphicDerivedaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for PolymorphicDerived {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN18PolymorphicDerivedaSEOS_(self, __param_0);
        }
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_POLYMORPHIC_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN15PolymorphicBaseC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::PolymorphicBase>,
        );
        pub(crate) fn __rust_thunk___ZN15PolymorphicBaseC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::PolymorphicBase>,
            __param_0: &'b crate::PolymorphicBase,
        );
        pub(crate) fn __rust_thunk___ZN15PolymorphicBaseaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::PolymorphicBase>,
            __param_0: &'b crate::PolymorphicBase,
        ) -> ::core::pin::Pin<&'a mut crate::PolymorphicBase>;
        pub(crate) fn __rust_thunk___ZN15PolymorphicBaseD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::PolymorphicBase>,
        );
        pub(crate) fn __rust_thunk___ZN16PolymorphicBase2C1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::PolymorphicBase2>,
        );
        pub(crate) fn __rust_thunk___ZN16PolymorphicBase2C1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::PolymorphicBase2>,
            __param_0: &'b crate::PolymorphicBase2,
        );
        pub(crate) fn __rust_thunk___ZN16PolymorphicBase2aSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::PolymorphicBase2>,
            __param_0: &'b crate::PolymorphicBase2,
        ) -> ::core::pin::Pin<&'a mut crate::PolymorphicBase2>;
        pub(crate) fn __rust_thunk___ZN16PolymorphicBase23FooEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::PolymorphicBase2>,
        );
        pub(crate) fn __rust_thunk___ZN16PolymorphicBase2D1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::PolymorphicBase2>,
        );
        pub(crate) fn __rust_thunk___ZN18PolymorphicDerivedC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::PolymorphicDerived>,
        );
        pub(crate) fn __rust_thunk___ZN18PolymorphicDerivedC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::PolymorphicDerived>,
            __param_0: &'b crate::PolymorphicDerived,
        );
        pub(crate) fn __rust_thunk___ZN18PolymorphicDerivedC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::PolymorphicDerived>,
            __param_0: ::ctor::RvalueReference<'b, crate::PolymorphicDerived>,
        );
        pub(crate) fn __rust_thunk___ZN18PolymorphicDerivedD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::PolymorphicDerived>,
        );
        pub(crate) fn __rust_thunk___ZN18PolymorphicDerivedaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::PolymorphicDerived>,
            __param_0: &'b crate::PolymorphicDerived,
        ) -> ::core::pin::Pin<&'a mut crate::PolymorphicDerived>;
        pub(crate) fn __rust_thunk___ZN18PolymorphicDerivedaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::PolymorphicDerived>,
            __param_0: ::ctor::RvalueReference<'b, crate::PolymorphicDerived>,
        ) -> ::core::pin::Pin<&'a mut crate::PolymorphicDerived>;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::PolymorphicBase>() == 8);
const _: () = assert!(::core::mem::align_of::<crate::PolymorphicBase>() == 8);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::PolymorphicBase: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::PolymorphicBase: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::PolymorphicBase2>() == 8);
const _: () = assert!(::core::mem::align_of::<crate::PolymorphicBase2>() == 8);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::PolymorphicBase2: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::PolymorphicBase2: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::PolymorphicDerived>() == 16);
const _: () = assert!(::core::mem::align_of::<crate::PolymorphicDerived>() == 8);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::PolymorphicDerived: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::PolymorphicDerived: Drop);
};
