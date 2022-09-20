[profile.release]
strip = true # strip Symbols from Binary. Automatically strip symbols from the binary
lto = true # Enable Link Time Optimization (LTO). Instructs the linker to optimize at the link stage
codegen-units = 1 # Reduce Parallel Code Generation Units to Increase Optimization
panic = "abort" # Abort on Panic

RUSTFLAGS="-C target-cpu=native" cargo build --release
