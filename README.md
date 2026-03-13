# MWE

Toolchain `llvm` with `rules_rust` (or `rules_rs`) and crate `aws-lc-rs` fails during `bazel build //...` with:

```bash
clang: error: no such file or directory:
'bazel-out/darwin_arm64-fastbuild-ST-bdec89fd5d65/bin/external/llvm++llvm_source+compiler-rt/clang_rt.builtins.static_/libclang_rt.builtins.static.a'
```
