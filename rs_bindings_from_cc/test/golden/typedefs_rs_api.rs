// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:typedefs_cc
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
#[repr(C)]
pub struct SomeStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeStruct"), crate::SomeStruct);

impl ::ctor::CtorNew<()> for SomeStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10SomeStructC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for SomeStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10SomeStructC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for SomeStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for SomeStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10SomeStructC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for SomeStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for SomeStruct {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for SomeStruct {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN10SomeStructaSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for item 'nested_type':
// Typedefs nested in classes are not supported yet

// Error while generating bindings for item 'SomeStruct':
// Typedef only used to introduce a name in C. Not importing.

#[::ctor::recursively_pinned]
#[repr(C)]
pub struct SomeOtherStruct {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("SomeOtherStruct"),
    crate::SomeOtherStruct
);

impl ::ctor::CtorNew<()> for SomeOtherStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN15SomeOtherStructC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for SomeOtherStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN15SomeOtherStructC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for SomeOtherStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for SomeOtherStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN15SomeOtherStructC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for SomeOtherStruct {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for SomeOtherStruct {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN15SomeOtherStructaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for SomeOtherStruct {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN15SomeOtherStructaSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union SomeUnion {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeUnion"), crate::SomeUnion);

impl Default for SomeUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for SomeUnion {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for SomeUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeUnionaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SomeUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN9SomeUnionaSEOS_(self, __param_0);
        }
    }
}

// Error while generating bindings for item 'SomeUnion':
// Typedef only used to introduce a name in C. Not importing.

#[derive(Clone, Copy)]
#[repr(C)]
pub union SomeOtherUnion {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeOtherUnion"), crate::SomeOtherUnion);

impl Default for SomeOtherUnion {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14SomeOtherUnionC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for SomeOtherUnion {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN14SomeOtherUnionC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for SomeOtherUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN14SomeOtherUnionaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for SomeOtherUnion {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN14SomeOtherUnionaSEOS_(self, __param_0);
        }
    }
}

#[inline(always)]
pub fn FunctionUsingNestedType() -> i32 {
    unsafe { crate::detail::__rust_thunk___Z23FunctionUsingNestedTypev() }
}

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TYPEDEFS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN10SomeStructC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeStruct>,
        );
        pub(crate) fn __rust_thunk___ZN10SomeStructC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeStruct>,
            __param_0: &'b crate::SomeStruct,
        );
        pub(crate) fn __rust_thunk___ZN10SomeStructC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeStruct>,
        );
        pub(crate) fn __rust_thunk___ZN10SomeStructaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::SomeStruct>,
            __param_0: &'b crate::SomeStruct,
        ) -> ::core::pin::Pin<&'a mut crate::SomeStruct>;
        pub(crate) fn __rust_thunk___ZN10SomeStructaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::SomeStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeStruct>,
        ) -> ::core::pin::Pin<&'a mut crate::SomeStruct>;
        pub(crate) fn __rust_thunk___ZN15SomeOtherStructC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeOtherStruct>,
        );
        pub(crate) fn __rust_thunk___ZN15SomeOtherStructC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeOtherStruct>,
            __param_0: &'b crate::SomeOtherStruct,
        );
        pub(crate) fn __rust_thunk___ZN15SomeOtherStructC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeOtherStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeOtherStruct>,
        );
        pub(crate) fn __rust_thunk___ZN15SomeOtherStructaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::SomeOtherStruct>,
            __param_0: &'b crate::SomeOtherStruct,
        ) -> ::core::pin::Pin<&'a mut crate::SomeOtherStruct>;
        pub(crate) fn __rust_thunk___ZN15SomeOtherStructaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::SomeOtherStruct>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeOtherStruct>,
        ) -> ::core::pin::Pin<&'a mut crate::SomeOtherStruct>;
        pub(crate) fn __rust_thunk___ZN9SomeUnionC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeUnion>,
        );
        pub(crate) fn __rust_thunk___ZN9SomeUnionC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeUnion>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeUnion>,
        );
        pub(crate) fn __rust_thunk___ZN9SomeUnionaSERKS_<'a, 'b>(
            __this: &'a mut crate::SomeUnion,
            __param_0: &'b crate::SomeUnion,
        ) -> &'a mut crate::SomeUnion;
        pub(crate) fn __rust_thunk___ZN9SomeUnionaSEOS_<'a, 'b>(
            __this: &'a mut crate::SomeUnion,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeUnion>,
        ) -> &'a mut crate::SomeUnion;
        pub(crate) fn __rust_thunk___ZN14SomeOtherUnionC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeOtherUnion>,
        );
        pub(crate) fn __rust_thunk___ZN14SomeOtherUnionC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::SomeOtherUnion>,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeOtherUnion>,
        );
        pub(crate) fn __rust_thunk___ZN14SomeOtherUnionaSERKS_<'a, 'b>(
            __this: &'a mut crate::SomeOtherUnion,
            __param_0: &'b crate::SomeOtherUnion,
        ) -> &'a mut crate::SomeOtherUnion;
        pub(crate) fn __rust_thunk___ZN14SomeOtherUnionaSEOS_<'a, 'b>(
            __this: &'a mut crate::SomeOtherUnion,
            __param_0: ::ctor::RvalueReference<'b, crate::SomeOtherUnion>,
        ) -> &'a mut crate::SomeOtherUnion;
        #[link_name = "_Z23FunctionUsingNestedTypev"]
        pub(crate) fn __rust_thunk___Z23FunctionUsingNestedTypev() -> i32;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::SomeStruct>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::SomeStruct>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::SomeOtherStruct>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::SomeOtherStruct>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeOtherStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeOtherStruct: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::SomeUnion>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::SomeUnion>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeUnion: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::SomeOtherUnion>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::SomeOtherUnion>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeOtherUnion: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::SomeOtherUnion: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeOtherUnion: Drop);
};
