# Boot, Loading, and Executable Design

The project executable uses a DOS-inspired text interface while remaining a modern application. The sequence is:

```text
firmware/boot → loader → kernel boundary → runtime → shell → application
```

A host-mode build starts directly at the loader/runtime boundary. A bare-metal build can later provide a real bootloader and kernel image for a supported architecture.

The loading screen is diagnostic: each stage reports pending, running, success, or failure. It never reports hardware initialization that did not occur.

## Executables

- `ai-shell` — interactive terminal/build frontend
- `ai-runtime` — synthetic-intelligence runtime
- `ai-kernel-sim` — host-mode kernel/system simulator
- future target images — architecture-specific bootable artifacts
