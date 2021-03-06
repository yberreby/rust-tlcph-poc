// Test that dlopen of dynamic runtime is prohibited.
//
// RUN: %clangxx %s -DRT=\"%shared_libasan\" -o %t -ldl
// RUN: not %run %t 2>&1 | FileCheck %s
// REQUIRES: asan-dynamic-runtime

#include <dlfcn.h>

int main(int argc, char **argv) {
  dlopen(RT, RTLD_LAZY);
  return 0;
}

// CHECK: ASan runtime does not come first in initial library list
