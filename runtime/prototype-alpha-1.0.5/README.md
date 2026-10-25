# Synthetic Brain Architecture — Prototype Alpha 1.0.5

**Developer:** ArkansasIo  
**Build ID:** `SBA-PA1-0105`  
**Status:** Prototype Alpha

## Runtime targets

1. Windows x86-64 virtual hardware
2. Raspberry Pi 3 ARMv7 hybrid virtual hardware
3. Future custom regional cartridge hardware

## What this executable does

The first runtime is intentionally a validation executable. It boots the twelve regional runtime slots, initializes the virtual backplane and hardware services, runs a deterministic scheduler loop, and reports health state.

It is **not** a human-brain simulation and does not reproduce human cognition. The architecture is a software/hardware research framework inspired by functional organization of the brain.

## Build

From this directory:

```text
cargo run
```

Release build:

```text
cargo build --release
```

The next runtime increment will replace the validation loop with the real HAL-backed perception, memory, reasoning, decision, action, learning, and inter-region message pipelines.
