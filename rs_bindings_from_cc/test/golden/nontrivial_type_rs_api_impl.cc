// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/nontrivial_type.h"

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wthread-safety-analysis"
extern "C" void __rust_thunk___ZN16NontrivialInlineC1Ei(
    class NontrivialInline* __this, int field) {
  rs_api_impl_support::construct_at(std::forward<decltype(__this)>(__this),
                                    std::forward<decltype(field)>(field));
}
extern "C" void __rust_thunk___ZN16NontrivialInlineC1EOS_(
    class NontrivialInline* __this, class NontrivialInline&& __param_0) {
  rs_api_impl_support::construct_at(
      std::forward<decltype(__this)>(__this),
      std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN16NontrivialInlineD1Ev(
    class NontrivialInline* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}
extern "C" void __rust_thunk___ZN16NontrivialInline14MemberFunctionEv(
    class NontrivialInline* __this) {
  __this->MemberFunction();
}
extern "C" void __rust_thunk___ZN17NontrivialMembersC1EOS_(
    class NontrivialMembers* __this, class NontrivialMembers&& __param_0) {
  rs_api_impl_support::construct_at(
      std::forward<decltype(__this)>(__this),
      std::forward<decltype(__param_0)>(__param_0));
}
extern "C" void __rust_thunk___ZN17NontrivialMembersD1Ev(
    class NontrivialMembers* __this) {
  std::destroy_at(std::forward<decltype(__this)>(__this));
}

static_assert(sizeof(class Nontrivial) == 4);
static_assert(alignof(class Nontrivial) == 4);
static_assert(offsetof(class Nontrivial, field) * 8 == 0);

static_assert(sizeof(class NontrivialInline) == 4);
static_assert(alignof(class NontrivialInline) == 4);
static_assert(offsetof(class NontrivialInline, field) * 8 == 0);

static_assert(sizeof(class NontrivialMembers) == 4);
static_assert(alignof(class NontrivialMembers) == 4);
static_assert(offsetof(class NontrivialMembers, nontrivial_member) * 8 == 0);

#pragma clang diagnostic pop
