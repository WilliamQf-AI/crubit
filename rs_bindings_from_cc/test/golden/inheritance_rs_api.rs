// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:inheritance_cc
// Features: experimental, extern_c, supported

#![rustfmt::skip]
#![feature(
    custom_inner_attributes,
    impl_trait_in_assoc_type,
    negative_impls,
    register_tool,
    type_alias_impl_trait
)]
#![allow(stable_features)]
#![no_std]
#![register_tool(__crubit)]
#![allow(improper_ctypes)]
#![allow(nonstandard_style)]
#![deny(warnings)]

/// Using classes to force these to be non-POD.
/// In the Itanium ABI, the tail padding of POD types cannot be reused by other
/// objects, even if the POD type is potentially-overlapping.
#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cc_type = "Base0")]
pub struct Base0 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for Base0 {}
impl !Sync for Base0 {}
forward_declare::unsafe_define!(forward_declare::symbol!("Base0"), crate::Base0);

impl Default for Base0 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5Base0C1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for Base0 {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5Base0C1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for Base0 {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN5Base0aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for Base0 {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN5Base0aSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(8))]
#[__crubit::annotate(cc_type = "Base1")]
pub struct Base1 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b1_1_: [::core::mem::MaybeUninit<u8>; 8],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b1_2_: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for Base1 {}
impl !Sync for Base1 {}
forward_declare::unsafe_define!(forward_declare::symbol!("Base1"), crate::Base1);

impl Default for Base1 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5Base1C1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for Base1 {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5Base1C1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for Base1 {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN5Base1aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for Base1 {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN5Base1aSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(2))]
#[__crubit::annotate(cc_type = "Base2")]
pub struct Base2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) b2_1_: [::core::mem::MaybeUninit<u8>; 2],
}
impl !Send for Base2 {}
impl !Sync for Base2 {}
forward_declare::unsafe_define!(forward_declare::symbol!("Base2"), crate::Base2);

impl Default for Base2 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5Base2C1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for Base2 {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN5Base2C1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for Base2 {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN5Base2aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for Base2 {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN5Base2aSEOS_(self, __param_0);
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C, align(8))]
#[__crubit::annotate(cc_type = "Derived")]
pub struct Derived {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 12],
    pub derived_1: ::core::ffi::c_char,
}
impl !Send for Derived {}
impl !Sync for Derived {}
forward_declare::unsafe_define!(forward_declare::symbol!("Derived"), crate::Derived);

impl Default for Derived {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7DerivedC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for Derived {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN7DerivedC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for Derived {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN7DerivedaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for Derived {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN7DerivedaSEOS_(self, __param_0);
        }
    }
}

unsafe impl oops::Inherits<crate::Base0> for crate::Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base0 {
        (derived as *const _ as *const u8).offset(0) as *const crate::Base0
    }
}
unsafe impl oops::Inherits<crate::Base1> for crate::Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base1 {
        (derived as *const _ as *const u8).offset(0) as *const crate::Base1
    }
}
unsafe impl oops::Inherits<crate::Base2> for crate::Derived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base2 {
        (derived as *const _ as *const u8).offset(10) as *const crate::Base2
    }
}

#[::ctor::recursively_pinned]
#[repr(C, align(8))]
#[__crubit::annotate(cc_type = "VirtualBase1")]
pub struct VirtualBase1 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 24],
}
impl !Send for VirtualBase1 {}
impl !Sync for VirtualBase1 {}
forward_declare::unsafe_define!(forward_declare::symbol!("VirtualBase1"), crate::VirtualBase1);

impl ::ctor::CtorNew<()> for VirtualBase1 {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN12VirtualBase1C1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for VirtualBase1 {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN12VirtualBase1C1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for VirtualBase1 {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for VirtualBase1 {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN12VirtualBase1C1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for VirtualBase1 {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for VirtualBase1 {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN12VirtualBase1aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for VirtualBase1 {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN12VirtualBase1aSEOS_(self, __param_0);
        }
    }
}

unsafe impl oops::Inherits<crate::Base1> for crate::VirtualBase1 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base1 {
        crate::detail::__crubit_dynamic_upcast__12VirtualBase1__to__5Base1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(derived)
    }
}

#[::ctor::recursively_pinned]
#[repr(C, align(8))]
#[__crubit::annotate(cc_type = "VirtualBase2")]
pub struct VirtualBase2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 24],
}
impl !Send for VirtualBase2 {}
impl !Sync for VirtualBase2 {}
forward_declare::unsafe_define!(forward_declare::symbol!("VirtualBase2"), crate::VirtualBase2);

impl ::ctor::CtorNew<()> for VirtualBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN12VirtualBase2C1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for VirtualBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN12VirtualBase2C1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for VirtualBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for VirtualBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN12VirtualBase2C1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for VirtualBase2 {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for VirtualBase2 {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN12VirtualBase2aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for VirtualBase2 {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN12VirtualBase2aSEOS_(self, __param_0);
        }
    }
}

unsafe impl oops::Inherits<crate::Base1> for crate::VirtualBase2 {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base1 {
        crate::detail::__crubit_dynamic_upcast__12VirtualBase2__to__5Base1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(derived)
    }
}

#[::ctor::recursively_pinned]
#[repr(C, align(8))]
#[__crubit::annotate(cc_type = "VirtualDerived")]
pub struct VirtualDerived {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 32],
}
impl !Send for VirtualDerived {}
impl !Sync for VirtualDerived {}
forward_declare::unsafe_define!(forward_declare::symbol!("VirtualDerived"), crate::VirtualDerived);

impl ::ctor::CtorNew<()> for VirtualDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN14VirtualDerivedC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for VirtualDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN14VirtualDerivedC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for VirtualDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for VirtualDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN14VirtualDerivedC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for VirtualDerived {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for VirtualDerived {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN14VirtualDerivedaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for VirtualDerived {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN14VirtualDerivedaSEOS_(self, __param_0);
        }
    }
}

unsafe impl oops::Inherits<crate::VirtualBase1> for crate::VirtualDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::VirtualBase1 {
        crate::detail::__crubit_dynamic_upcast__14VirtualDerived__to__12VirtualBase1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(derived)
    }
}
unsafe impl oops::Inherits<crate::Base1> for crate::VirtualDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::Base1 {
        crate::detail::__crubit_dynamic_upcast__14VirtualDerived__to__5Base1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(derived)
    }
}
unsafe impl oops::Inherits<crate::VirtualBase2> for crate::VirtualDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::VirtualBase2 {
        crate::detail::__crubit_dynamic_upcast__14VirtualDerived__to__12VirtualBase2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(derived)
    }
}

#[::ctor::recursively_pinned]
#[repr(C, align(8))]
#[__crubit::annotate(cc_type = "MyAbstractClass")]
pub struct MyAbstractClass {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 8],
}
impl !Send for MyAbstractClass {}
impl !Sync for MyAbstractClass {}
forward_declare::unsafe_define!(
    forward_declare::symbol!("MyAbstractClass"),
    crate::MyAbstractClass
);

// Error while generating bindings for item 'MyAbstractClass::MyAbstractClass':
// Can't directly construct values of type `MyAbstractClass` as it has a non-public or deleted destructor

// Error while generating bindings for item 'MyAbstractClass::MyAbstractClass':
// Can't directly construct values of type `MyAbstractClass` as it has a non-public or deleted destructor

impl<'b> ::ctor::Assign<&'b Self> for MyAbstractClass {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN15MyAbstractClassaSERKS_(self, __param_0);
        }
    }
}

/// Method inheritance
#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cc_type = "MethodBase1")]
pub struct MethodBase1 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for MethodBase1 {}
impl !Sync for MethodBase1 {}
forward_declare::unsafe_define!(forward_declare::symbol!("MethodBase1"), crate::MethodBase1);

impl Default for MethodBase1 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11MethodBase1C1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for MethodBase1 {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11MethodBase1C1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for MethodBase1 {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN11MethodBase1aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for MethodBase1 {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN11MethodBase1aSEOS_(self, __param_0);
        }
    }
}

impl MethodBase1 {
    #[inline(always)]
    pub fn Public<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase16PublicEv(self) }
    }
}

impl MethodBase1 {
    #[inline(always)]
    pub fn Colliding1<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase110Colliding1Ev(self) }
    }
}

impl MethodBase1 {
    #[inline(always)]
    pub fn Colliding2<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase110Colliding2Ev(self) }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cc_type = "MethodBase2")]
pub struct MethodBase2 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for MethodBase2 {}
impl !Sync for MethodBase2 {}
forward_declare::unsafe_define!(forward_declare::symbol!("MethodBase2"), crate::MethodBase2);

impl Default for MethodBase2 {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11MethodBase2C1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for MethodBase2 {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11MethodBase2C1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for MethodBase2 {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN11MethodBase2aSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for MethodBase2 {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN11MethodBase2aSEOS_(self, __param_0);
        }
    }
}

impl MethodBase2 {
    #[inline(always)]
    pub fn Colliding1<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase210Colliding1Ev(self) }
    }
}

impl MethodBase2 {
    #[inline(always)]
    pub fn Colliding2<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN11MethodBase210Colliding2Ev(self) }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
#[__crubit::annotate(cc_type = "MethodDerived")]
pub struct MethodDerived {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
impl !Send for MethodDerived {}
impl !Sync for MethodDerived {}
forward_declare::unsafe_define!(forward_declare::symbol!("MethodDerived"), crate::MethodDerived);

impl Default for MethodDerived {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13MethodDerivedC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for MethodDerived {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13MethodDerivedC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for MethodDerived {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13MethodDerivedaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for MethodDerived {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN13MethodDerivedaSEOS_(self, __param_0);
        }
    }
}

unsafe impl oops::Inherits<crate::MethodBase1> for crate::MethodDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::MethodBase1 {
        (derived as *const _ as *const u8).offset(0) as *const crate::MethodBase1
    }
}
unsafe impl oops::Inherits<crate::MethodBase2> for crate::MethodDerived {
    unsafe fn upcast_ptr(derived: *const Self) -> *const crate::MethodBase2 {
        (derived as *const _ as *const u8).offset(0) as *const crate::MethodBase2
    }
}

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN5Base0C1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Base0>,
        );
        pub(crate) fn __rust_thunk___ZN5Base0C1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Base0>,
            __param_0: ::ctor::RvalueReference<'b, crate::Base0>,
        );
        pub(crate) fn __rust_thunk___ZN5Base0aSERKS_<'a, 'b>(
            __this: &'a mut crate::Base0,
            __param_0: &'b crate::Base0,
        ) -> &'a mut crate::Base0;
        pub(crate) fn __rust_thunk___ZN5Base0aSEOS_<'a, 'b>(
            __this: &'a mut crate::Base0,
            __param_0: ::ctor::RvalueReference<'b, crate::Base0>,
        ) -> &'a mut crate::Base0;
        pub(crate) fn __rust_thunk___ZN5Base1C1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Base1>,
        );
        pub(crate) fn __rust_thunk___ZN5Base1C1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Base1>,
            __param_0: ::ctor::RvalueReference<'b, crate::Base1>,
        );
        pub(crate) fn __rust_thunk___ZN5Base1aSERKS_<'a, 'b>(
            __this: &'a mut crate::Base1,
            __param_0: &'b crate::Base1,
        ) -> &'a mut crate::Base1;
        pub(crate) fn __rust_thunk___ZN5Base1aSEOS_<'a, 'b>(
            __this: &'a mut crate::Base1,
            __param_0: ::ctor::RvalueReference<'b, crate::Base1>,
        ) -> &'a mut crate::Base1;
        pub(crate) fn __rust_thunk___ZN5Base2C1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Base2>,
        );
        pub(crate) fn __rust_thunk___ZN5Base2C1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Base2>,
            __param_0: ::ctor::RvalueReference<'b, crate::Base2>,
        );
        pub(crate) fn __rust_thunk___ZN5Base2aSERKS_<'a, 'b>(
            __this: &'a mut crate::Base2,
            __param_0: &'b crate::Base2,
        ) -> &'a mut crate::Base2;
        pub(crate) fn __rust_thunk___ZN5Base2aSEOS_<'a, 'b>(
            __this: &'a mut crate::Base2,
            __param_0: ::ctor::RvalueReference<'b, crate::Base2>,
        ) -> &'a mut crate::Base2;
        pub(crate) fn __rust_thunk___ZN7DerivedC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Derived>,
        );
        pub(crate) fn __rust_thunk___ZN7DerivedC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Derived>,
            __param_0: ::ctor::RvalueReference<'b, crate::Derived>,
        );
        pub(crate) fn __rust_thunk___ZN7DerivedaSERKS_<'a, 'b>(
            __this: &'a mut crate::Derived,
            __param_0: &'b crate::Derived,
        ) -> &'a mut crate::Derived;
        pub(crate) fn __rust_thunk___ZN7DerivedaSEOS_<'a, 'b>(
            __this: &'a mut crate::Derived,
            __param_0: ::ctor::RvalueReference<'b, crate::Derived>,
        ) -> &'a mut crate::Derived;
        pub(crate) fn __rust_thunk___ZN12VirtualBase1C1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::VirtualBase1>,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase1C1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::VirtualBase1>,
            __param_0: &'b crate::VirtualBase1,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase1C1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::VirtualBase1>,
            __param_0: ::ctor::RvalueReference<'b, crate::VirtualBase1>,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase1aSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::VirtualBase1>,
            __param_0: &'b crate::VirtualBase1,
        ) -> ::core::pin::Pin<&'a mut crate::VirtualBase1>;
        pub(crate) fn __rust_thunk___ZN12VirtualBase1aSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::VirtualBase1>,
            __param_0: ::ctor::RvalueReference<'b, crate::VirtualBase1>,
        ) -> ::core::pin::Pin<&'a mut crate::VirtualBase1>;
        pub fn __crubit_dynamic_upcast__12VirtualBase1__to__5Base1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(
            from: *const crate::VirtualBase1,
        ) -> *const crate::Base1;
        pub(crate) fn __rust_thunk___ZN12VirtualBase2C1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::VirtualBase2>,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase2C1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::VirtualBase2>,
            __param_0: &'b crate::VirtualBase2,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase2C1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::VirtualBase2>,
            __param_0: ::ctor::RvalueReference<'b, crate::VirtualBase2>,
        );
        pub(crate) fn __rust_thunk___ZN12VirtualBase2aSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::VirtualBase2>,
            __param_0: &'b crate::VirtualBase2,
        ) -> ::core::pin::Pin<&'a mut crate::VirtualBase2>;
        pub(crate) fn __rust_thunk___ZN12VirtualBase2aSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::VirtualBase2>,
            __param_0: ::ctor::RvalueReference<'b, crate::VirtualBase2>,
        ) -> ::core::pin::Pin<&'a mut crate::VirtualBase2>;
        pub fn __crubit_dynamic_upcast__12VirtualBase2__to__5Base1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(
            from: *const crate::VirtualBase2,
        ) -> *const crate::Base1;
        pub(crate) fn __rust_thunk___ZN14VirtualDerivedC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::VirtualDerived>,
        );
        pub(crate) fn __rust_thunk___ZN14VirtualDerivedC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::VirtualDerived>,
            __param_0: &'b crate::VirtualDerived,
        );
        pub(crate) fn __rust_thunk___ZN14VirtualDerivedC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::VirtualDerived>,
            __param_0: ::ctor::RvalueReference<'b, crate::VirtualDerived>,
        );
        pub(crate) fn __rust_thunk___ZN14VirtualDerivedaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::VirtualDerived>,
            __param_0: &'b crate::VirtualDerived,
        ) -> ::core::pin::Pin<&'a mut crate::VirtualDerived>;
        pub(crate) fn __rust_thunk___ZN14VirtualDerivedaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::VirtualDerived>,
            __param_0: ::ctor::RvalueReference<'b, crate::VirtualDerived>,
        ) -> ::core::pin::Pin<&'a mut crate::VirtualDerived>;
        pub fn __crubit_dynamic_upcast__14VirtualDerived__to__12VirtualBase1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(
            from: *const crate::VirtualDerived,
        ) -> *const crate::VirtualBase1;
        pub fn __crubit_dynamic_upcast__14VirtualDerived__to__5Base1___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(
            from: *const crate::VirtualDerived,
        ) -> *const crate::Base1;
        pub fn __crubit_dynamic_upcast__14VirtualDerived__to__12VirtualBase2___2f_2fthird_5fparty_2fcrubit_2frs_5fbindings_5ffrom_5fcc_2ftest_2fgolden_3ainheritance_5fcc(
            from: *const crate::VirtualDerived,
        ) -> *const crate::VirtualBase2;
        pub(crate) fn __rust_thunk___ZN15MyAbstractClassaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::MyAbstractClass>,
            __param_0: &'b crate::MyAbstractClass,
        ) -> ::core::pin::Pin<&'a mut crate::MyAbstractClass>;
        pub(crate) fn __rust_thunk___ZN11MethodBase1C1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::MethodBase1>,
        );
        pub(crate) fn __rust_thunk___ZN11MethodBase1C1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::MethodBase1>,
            __param_0: ::ctor::RvalueReference<'b, crate::MethodBase1>,
        );
        pub(crate) fn __rust_thunk___ZN11MethodBase1aSERKS_<'a, 'b>(
            __this: &'a mut crate::MethodBase1,
            __param_0: &'b crate::MethodBase1,
        ) -> &'a mut crate::MethodBase1;
        pub(crate) fn __rust_thunk___ZN11MethodBase1aSEOS_<'a, 'b>(
            __this: &'a mut crate::MethodBase1,
            __param_0: ::ctor::RvalueReference<'b, crate::MethodBase1>,
        ) -> &'a mut crate::MethodBase1;
        #[link_name = "_ZN11MethodBase16PublicEv"]
        pub(crate) fn __rust_thunk___ZN11MethodBase16PublicEv<'a>(
            __this: &'a mut crate::MethodBase1,
        );
        #[link_name = "_ZN11MethodBase110Colliding1Ev"]
        pub(crate) fn __rust_thunk___ZN11MethodBase110Colliding1Ev<'a>(
            __this: &'a mut crate::MethodBase1,
        );
        #[link_name = "_ZN11MethodBase110Colliding2Ev"]
        pub(crate) fn __rust_thunk___ZN11MethodBase110Colliding2Ev<'a>(
            __this: &'a mut crate::MethodBase1,
        );
        pub(crate) fn __rust_thunk___ZN11MethodBase2C1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::MethodBase2>,
        );
        pub(crate) fn __rust_thunk___ZN11MethodBase2C1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::MethodBase2>,
            __param_0: ::ctor::RvalueReference<'b, crate::MethodBase2>,
        );
        pub(crate) fn __rust_thunk___ZN11MethodBase2aSERKS_<'a, 'b>(
            __this: &'a mut crate::MethodBase2,
            __param_0: &'b crate::MethodBase2,
        ) -> &'a mut crate::MethodBase2;
        pub(crate) fn __rust_thunk___ZN11MethodBase2aSEOS_<'a, 'b>(
            __this: &'a mut crate::MethodBase2,
            __param_0: ::ctor::RvalueReference<'b, crate::MethodBase2>,
        ) -> &'a mut crate::MethodBase2;
        #[link_name = "_ZN11MethodBase210Colliding1Ev"]
        pub(crate) fn __rust_thunk___ZN11MethodBase210Colliding1Ev<'a>(
            __this: &'a mut crate::MethodBase2,
        );
        #[link_name = "_ZN11MethodBase210Colliding2Ev"]
        pub(crate) fn __rust_thunk___ZN11MethodBase210Colliding2Ev<'a>(
            __this: &'a mut crate::MethodBase2,
        );
        pub(crate) fn __rust_thunk___ZN13MethodDerivedC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::MethodDerived>,
        );
        pub(crate) fn __rust_thunk___ZN13MethodDerivedC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::MethodDerived>,
            __param_0: ::ctor::RvalueReference<'b, crate::MethodDerived>,
        );
        pub(crate) fn __rust_thunk___ZN13MethodDerivedaSERKS_<'a, 'b>(
            __this: &'a mut crate::MethodDerived,
            __param_0: &'b crate::MethodDerived,
        ) -> &'a mut crate::MethodDerived;
        pub(crate) fn __rust_thunk___ZN13MethodDerivedaSEOS_<'a, 'b>(
            __this: &'a mut crate::MethodDerived,
            __param_0: ::ctor::RvalueReference<'b, crate::MethodDerived>,
        ) -> &'a mut crate::MethodDerived;
    }
}

const _: () = {
    assert!(::core::mem::size_of::<crate::Base0>() == 1);
    assert!(::core::mem::align_of::<crate::Base0>() == 1);
    static_assertions::assert_impl_all!(crate::Base0:Clone);
    static_assertions::assert_impl_all!(crate::Base0:Copy);
    static_assertions::assert_not_impl_any!(crate::Base0:Drop);

    assert!(::core::mem::size_of::<crate::Base1>() == 16);
    assert!(::core::mem::align_of::<crate::Base1>() == 8);
    static_assertions::assert_impl_all!(crate::Base1:Clone);
    static_assertions::assert_impl_all!(crate::Base1:Copy);
    static_assertions::assert_not_impl_any!(crate::Base1:Drop);
    assert!(memoffset::offset_of!(crate::Base1, b1_1_) == 0);
    assert!(memoffset::offset_of!(crate::Base1, b1_2_) == 8);

    assert!(::core::mem::size_of::<crate::Base2>() == 2);
    assert!(::core::mem::align_of::<crate::Base2>() == 2);
    static_assertions::assert_impl_all!(crate::Base2:Clone);
    static_assertions::assert_impl_all!(crate::Base2:Copy);
    static_assertions::assert_not_impl_any!(crate::Base2:Drop);
    assert!(memoffset::offset_of!(crate::Base2, b2_1_) == 0);

    assert!(::core::mem::size_of::<crate::Derived>() == 16);
    assert!(::core::mem::align_of::<crate::Derived>() == 8);
    static_assertions::assert_impl_all!(crate::Derived:Clone);
    static_assertions::assert_impl_all!(crate::Derived:Copy);
    static_assertions::assert_not_impl_any!(crate::Derived:Drop);
    assert!(memoffset::offset_of!(crate::Derived, derived_1) == 12);

    assert!(::core::mem::size_of::<crate::VirtualBase1>() == 24);
    assert!(::core::mem::align_of::<crate::VirtualBase1>() == 8);
    static_assertions::assert_not_impl_any!(crate::VirtualBase1:Copy);
    static_assertions::assert_not_impl_any!(crate::VirtualBase1:Drop);

    assert!(::core::mem::size_of::<crate::VirtualBase2>() == 24);
    assert!(::core::mem::align_of::<crate::VirtualBase2>() == 8);
    static_assertions::assert_not_impl_any!(crate::VirtualBase2:Copy);
    static_assertions::assert_not_impl_any!(crate::VirtualBase2:Drop);

    assert!(::core::mem::size_of::<crate::VirtualDerived>() == 32);
    assert!(::core::mem::align_of::<crate::VirtualDerived>() == 8);
    static_assertions::assert_not_impl_any!(crate::VirtualDerived:Copy);
    static_assertions::assert_not_impl_any!(crate::VirtualDerived:Drop);

    assert!(::core::mem::size_of::<crate::MyAbstractClass>() == 8);
    assert!(::core::mem::align_of::<crate::MyAbstractClass>() == 8);
    static_assertions::assert_not_impl_any!(crate::MyAbstractClass:Copy);
    static_assertions::assert_not_impl_any!(crate::MyAbstractClass:Drop);

    assert!(::core::mem::size_of::<crate::MethodBase1>() == 1);
    assert!(::core::mem::align_of::<crate::MethodBase1>() == 1);
    static_assertions::assert_impl_all!(crate::MethodBase1:Clone);
    static_assertions::assert_impl_all!(crate::MethodBase1:Copy);
    static_assertions::assert_not_impl_any!(crate::MethodBase1:Drop);

    assert!(::core::mem::size_of::<crate::MethodBase2>() == 1);
    assert!(::core::mem::align_of::<crate::MethodBase2>() == 1);
    static_assertions::assert_impl_all!(crate::MethodBase2:Clone);
    static_assertions::assert_impl_all!(crate::MethodBase2:Copy);
    static_assertions::assert_not_impl_any!(crate::MethodBase2:Drop);

    assert!(::core::mem::size_of::<crate::MethodDerived>() == 1);
    assert!(::core::mem::align_of::<crate::MethodDerived>() == 1);
    static_assertions::assert_impl_all!(crate::MethodDerived:Clone);
    static_assertions::assert_impl_all!(crate::MethodDerived:Copy);
    static_assertions::assert_not_impl_any!(crate::MethodDerived:Drop);
};
