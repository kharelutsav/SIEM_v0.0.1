
# Rust Performance Guide

**Release profile for small and performant rust build** [[Reference]](https://nnethercote.github.io/perf-book/heap-allocations.html)
```
[profile.release]
strip = true # strip Symbols from Binary. Automatically strip symbols from the binary
lto = true # Enable Link Time Optimization (LTO). Instructs the linker to optimize at the link stage
codegen-units = 1 # Reduce Parallel Code Generation Units to Increase Optimization
panic = "abort" # Abort on Panic
```

---

**Create rust build native to the build platform**

>`RUSTFLAGS="-C target-cpu=native" cargo build --release`

**Create library on mac os on intel platform**

>`cargo build --release --target x86_64-apple-darwin`

**Waiting for file lock on cache file**

>`rm ~/.cargo/.package-cache`

---
