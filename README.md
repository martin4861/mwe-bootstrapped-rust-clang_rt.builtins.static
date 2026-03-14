# MWE

## Failure on Mac

Toolchain `llvm` with `rules_rust` (or `rules_rs`) and crate `aws-lc-rs` fails during `bazel build //...` with:

```bash
clang: error: no such file or directory:
'bazel-out/darwin_arm64-fastbuild-ST-bdec89fd5d65/bin/external/llvm++llvm_source+compiler-rt/clang_rt.builtins.static_/libclang_rt.builtins.static.a'
```

## Failure on Linux

Toolchain `llvm` with `rules_rust` (or `rules_rs`) and crate `aws-lc-rs` fails during `bazel build //...` with:

```bash
/home/user/.cache/bazel/_bazel_user/0fcc946ce1d5a982e2a4a27b4c99af53/sandbox/linux-sandbox/959/execroot/_main/external/llvm++glibc+glibc_headers_x86_64-linux-gnu.2.28/include/stdio.h:33:10: fatal error: 'stddef.h' file not found
   33 | #include <stddef.h>
      |          ^~~~~~~~~~
```

Analysis of the rust action:
 - env variable `CXXFLAGS` contains: `-isystem ${pwd}/bazel-out/k8-fastbuild/bin/external/llvm++llvm_source+libcxx/libcxx_headers_include_search_directory`
 - the underlying compiler call does not contain an `-isystem` argument with this path
 - interestingly the linker flag is set: `-Lbazel-out/k8-fastbuild/bin/external/llvm+/runtimes/libcxx/libcxx_library_search_directory`
