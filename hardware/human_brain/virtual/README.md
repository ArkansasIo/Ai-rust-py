# Virtual Human-Brain Hardware

The project now has a virtual hardware target that emulates the interfaces expected by the synthetic-brain runtime.

## Targets

| Target | Purpose |
|---|---|
| `virtual` | Full deterministic hardware simulation |
| `raspberry-pi3` | Low-cost physical development controller |
| `custom-cartridge` | Future standardized regional PCB |

The virtual target is deliberately independent of the eventual processor, GPU, NPU, connector, and PCB implementation.

## Virtualized subsystems

- CPU/compute scheduler
- sensor channels
- actuator channels
- thermal state
- power-state transitions
- watchdog behavior
- Neuro-Link transport
- GPIO
- I2C/SPI/UART abstractions
- storage devices
- virtual cameras and microphones
- synthetic proprioception/touch channels
- fault injection and recovery testing

## Development model

The brain logic communicates only through the HAL. A virtual device can therefore be run on a Raspberry Pi 3 without requiring the custom cartridge to exist.

This is a simulation/development platform. It does not claim to reproduce biological neural tissue or real human brain physiology.
