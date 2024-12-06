// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "rs_bindings_from_cc/test/function/unsafe_attributes/unsafe_attributes.h"

void SafeSignatureWithoutAnnotation() {}
void SafeSignatureButAnnotatedUnsafe() {}
void SafeSignatureButAnnotatedSafe() {}
void UnsafeSignatureWithoutAnnotation(void*) {}
void UnsafeSignatureButAnnotatedUnsafe(void*) {}
void UnsafeSignatureButAnnotatedSafe(void*) {}
