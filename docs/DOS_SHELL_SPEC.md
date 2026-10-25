# DOS-Style AI Shell Specification

## Purpose

Provide a keyboard-first terminal environment for building, running, inspecting, configuring, and debugging Ai-rust-py without requiring an IDE.

## Command families

```text
HELP        command reference
BUILD       compile the current project
RUN         launch the AI/runtime
TEST        execute tests
CLEAN       remove build artifacts
CONFIG      read/change settings
STATUS      show runtime and toolchain state
MODEL       inspect/select AI models
BRAIN       inspect synthetic-brain subsystems
MEMORY      inspect memory stores
DEVICE      inspect hardware devices
KERNEL      inspect kernel/runtime services
LOG         inspect logs
DIAG        run diagnostics
PACKAGE     create distributable artifacts
CHAIN       select a build chain
EXIT        terminate the shell
```

## UI states

```text
BOOT → HARDWARE CHECK → KERNEL INIT → RUNTIME INIT → SHELL READY
```

The loading screen reports actual subsystem status rather than simulating successful initialization.

## Executable behavior

`ai-shell` is the command-line frontend. It invokes the compiler/build service, Rust runtime, Python environment, and optional C++23/assembly toolchains through explicit process boundaries.
