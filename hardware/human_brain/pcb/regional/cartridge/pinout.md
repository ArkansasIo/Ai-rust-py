# Standard Regional Cartridge Pinout

All twelve regional cartridges use the same external electrical contract. Regional specialization is implemented by compute, firmware, accelerator, and storage configuration rather than by changing the external connector geometry.

## Connector groups

| Group | Function | Direction |
|---|---|---|
| PWR | 48 V input, return, power-good, slot-enable | power/control |
| CTRL | I2C management, reset, interrupt, watchdog | bidirectional |
| LOCAL | SPI peripheral bus | bidirectional |
| DBG | UART diagnostics | bidirectional |
| NL-A | high-speed differential Neuro-Link pair A | bidirectional |
| NL-B | high-speed differential Neuro-Link pair B | bidirectional |
| CLK | low-skew distributed reference clock | input |
| GPIO | low-speed regional control/status | bidirectional |
| SERVICE | USB-C service/programming | bidirectional |

## Design rules

1. High-bandwidth inter-region traffic must use Neuro-Link differential lanes.
2. GPIO is reserved for low-speed control and status.
3. I2C is management-only and must not carry bulk neural payloads.
4. Local SPI devices remain electrically local to the cartridge.
5. The management MCU controls slot enable, watchdog, thermal alerts, and safe shutdown.
6. Every cartridge exposes the same mechanical and connector contract.

Exact connector part numbers and lane pin assignments remain provisional until the selected compute platform and signal-integrity budget are frozen.
