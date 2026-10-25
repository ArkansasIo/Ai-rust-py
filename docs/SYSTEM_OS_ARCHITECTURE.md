# AI-Rust-Py System OS Architecture

This document defines the project's DOS-inspired development shell and bootable system architecture.

## Layers

1. **Boot/firmware** — architecture-specific assembly, boot entry, startup, interrupts, and hardware initialization.
2. **System kernel** — a project-specific Linux-inspired kernel boundary. It is not a Linux fork; it provides scheduler, memory, IPC, device, and process abstractions that can evolve into a real kernel target.
3. **C++23 native layer** — high-performance native libraries, robotics, simulation, graphics, device adapters, and optional C/C++ FFI.
4. **Rust systems layer** — memory-safe runtime, APIs, orchestration, cognitive runtime, security, and hardware abstraction.
5. **Synthetic-brain layer** — perception, neural simulation, memory, cognition, emotion, personality, self-model, planning, and learning.
6. **Python layer** — ML training, inference, experiments, RAG, evaluation, and scientific tooling.
7. **DOS-style shell/build environment** — executable terminal, commands, settings, compiler/build chains, diagnostics, and project management.

## DOS-style executable

The target executable is a native terminal application with a boot/loading sequence, command interpreter, settings, project compiler, diagnostics, and an extensible API. It is inspired by the usability of MS-DOS, but it is a new implementation and does not contain Microsoft DOS code.

## Kernel boundary

The initial kernel is an abstraction and research target. It can run as a user-space system simulator while the low-level assembly/firmware path is developed. This avoids pretending that a Rust application is already a complete operating-system kernel.

## Build chains

The project supports two primary compilation chains:

- **Native chain:** Assembly → C/C++23 → Rust → final executable/system image.
- **AI chain:** Rust runtime → Python ML/AI → model artifacts → Rust runtime.

Cross-compilation targets should be added independently for x86_64, aarch64, and riscv64.
