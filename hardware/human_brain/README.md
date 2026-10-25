# human.brain Hardware

This directory defines the custom modular hardware architecture for the human.brain synthetic-compute system.

## Design status

This is an engineering prototype specification, not a production medical device or a validated human-anatomy replica. PCB dimensions, thermal limits, connectors, and component selections remain provisional until the compute silicon and mechanical envelope are frozen.

## PCB stack

- `pcb/master/` — master compute and orchestration board
- `pcb/backplane/` — 12-cartridge Neuro-Link and power backplane
- `pcb/regional/` — standardized regional compute cartridges
- `pcb/vision/` — stereo RGB/IR/depth camera interface
- `pcb/audio/` — synchronized microphone and speaker interface
- `pcb/power/` — protected power entry and distribution
- `pcb/thermal/` — fan, sensor, and thermal-control board
- `pcb/service/` — USB-C/JTAG/UART recovery and diagnostics

## Provisional cartridge envelope

120 mm x 80 mm x 25 mm. This is a packaging target only. Final dimensions must follow the selected compute module, connector, storage, and thermal hardware.

## Electrical philosophy

High-bandwidth neural traffic uses differential high-speed links through the backplane. I2C, SPI, UART, and GPIO are management/local-peripheral interfaces. GPIO is not the primary inter-region neural transport.

## Manufacturing outputs

Final PCB work will produce KiCad schematics/layouts, fabrication drawings, Gerbers or ODB++, drill files, pick-and-place data, BOMs, assembly notes, and mechanical DXF/STEP exports.
