# Build Chains

## Chain A — native system

```text
assembly → C/C++23 → Rust → linker → executable
```

Assembly is used only where architecture-specific control is required. C++23 provides native libraries and hardware-facing components. Rust owns the memory-safe system/runtime boundary.

## Chain B — AI

```text
Python ML → model artifact/service → Rust runtime → application
```

Python is not required to compile the native executable. The runtime can communicate with Python through a local process/API boundary.

## Chain C — full system image

```text
bootloader → kernel image → native runtime → AI runtime → shell
```

The initial implementation may execute the kernel layer as a simulator/host service. A real bare-metal target is a separate engineering phase.

## Toolchain discovery

The build service should detect and report:

- Rust/Cargo/rustc
- Python
- C/C++ compiler with C++23 support
- assembler/linker
- CMake/Ninja when configured
- target architecture
- optional cross-compilers

No command should silently claim a compiler is installed when it is not.
