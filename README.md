# hip-sys
Rust bindings for HIP

# Dependencies

  - A ROCm platform ie a compatible AMD GPU
    * Will compile without an AMD GPU, but device functions will fail
    * Currently CUDA platform support is not available (see cuda-sys https://github.com/rust-cuda/cuda-sys)
  - Install ROCm (see https://rocmdocs.amd.com/en/latest/Installation_Guide/Installation-Guide.html#deploying-rocm)
    * Works with the hcc backend `sudo apt install hip-hcc` (from ROCm package registry) 
  - You may need to install rocsolver as well `sudo apt install rocsolver` 
  
If you have any problems please post an issue.
  
# Tests

Run the tests with:
```
cargo test 
```

# Docs

Open the documentation with:
```
cargo doc --open
```


