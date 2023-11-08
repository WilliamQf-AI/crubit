// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_DEFINITION2_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_DEFINITION2_H_

#include "rs_bindings_from_cc/test/forward_declaration/forward_declaration.h"
#include "rs_bindings_from_cc/test/forward_declaration/forward_declaration2.h"

class A final {};

namespace my_namespace {
class B final {};
}  // namespace my_namespace

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_FORWARD_DECLARATION_DEFINITION2_H_
