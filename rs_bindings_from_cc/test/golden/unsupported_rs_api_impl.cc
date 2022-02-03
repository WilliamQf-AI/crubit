// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>

#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/unsupported.h"

extern "C" void __rust_thunk___ZN20NontrivialCustomTypeD1Ev(
    class NontrivialCustomType* __this) {
  std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN16ContainingStructC1Ev(
    class ContainingStruct* __this) {
  rs_api_impl_support ::construct_at(__this);
}
extern "C" void __rust_thunk___ZN16ContainingStructC1ERKS_(
    class ContainingStruct* __this, const class ContainingStruct& __param_0) {
  rs_api_impl_support ::construct_at(__this, __param_0);
}
extern "C" void __rust_thunk___ZN16ContainingStructD1Ev(
    class ContainingStruct* __this) {
  std ::destroy_at(__this);
}

static_assert(sizeof(class NontrivialCustomType) == 4);
static_assert(alignof(class NontrivialCustomType) == 4);
static_assert(offsetof(class NontrivialCustomType, i) * 8 == 0);

static_assert(sizeof(class ContainingStruct) == 1);
static_assert(alignof(class ContainingStruct) == 1);
