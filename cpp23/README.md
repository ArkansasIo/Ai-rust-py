# C++23 Native Systems Layer

The C++23 layer hosts native libraries and performance-sensitive components that benefit from the C++ ecosystem. Rust remains the primary systems boundary and accesses selected C++ components through explicit FFI interfaces.

Planned areas:

- robotics and device SDK adapters
- high-performance simulation
- computer vision / DSP integrations
- native inference backends
- GPU and accelerator bindings
- filesystem and networking adapters
- C++23 library interoperability

Do not duplicate Rust functionality without a measurable interoperability or performance reason.
