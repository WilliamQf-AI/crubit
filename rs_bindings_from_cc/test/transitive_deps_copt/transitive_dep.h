// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TRANSITIVE_DEPS_COPT_TRANSITIVE_DEP_H_
#define THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TRANSITIVE_DEPS_COPT_TRANSITIVE_DEP_H_

#if USE_FUNCA
inline void FuncA() {}
#else
inline void FuncB() {}
#endif  // USE_FUNC_A

#endif  // THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_TRANSITIVE_DEPS_COPT_TRANSITIVE_DEP_H_
