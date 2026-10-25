# Assembly Hardware Layer

Assembly is reserved for architecture-specific, timing-sensitive, boot, interrupt, firmware, and direct hardware operations.

Target families:

- `x86_64/`
- `aarch64/`
- `riscv64/`

Higher layers must access assembly through stable C ABI or Rust FFI boundaries. Architecture-specific instructions must not leak into the cognitive model.
