# Regional Cartridge KiCad Project

This directory is the source-of-truth entry point for the standardized regional cartridge PCB.

## Frozen prototype architecture

- Mechanical envelope: 120 x 80 x 25 mm
- Regional compute: NVIDIA Jetson Orin NX 16GB module on a custom carrier
- Management MCU: independent safety/watchdog controller
- Persistent storage: M.2 NVMe on the carrier
- Boot/system storage: microSD/eMMC interface as permitted by the selected compute module
- High-speed regional fabric: dedicated differential Neuro-Link lanes through a high-density board-to-board/backplane connector
- Management: I2C, SPI, UART, GPIO
- Service: USB-C
- Input power: 48 VDC to protected intermediate rail, then local regulated rails
- Thermal: copper heat spreader + fin stack + PWM blower interface

The Orin NX 16GB is used for the first physical prototype because its 70 x 45 mm module envelope fits inside the 120 x 80 mm cartridge carrier. Current NVIDIA documentation lists the Orin NX family at up to 157 TOPS and 10–25 W module power.

The high-speed connector family is frozen at the **Samtec SEARAY family** level for Rev A. Exact mating part numbers and stack height remain a BOM-level selection after the mechanical stack-up is finalized. The family provides high-density differential routing and high-speed board-to-board/backplane options.

## Important design boundary

This is a prototype architecture, not a validated production PCB. Exact BGA escape, impedance geometry, power-integrity values, connector part number, and manufacturer land patterns must be validated against the selected module design guide and the PCB fabricator stack-up before fabrication.

## Source files

- `cartridge.kicad_pro` - KiCad project metadata
- `net_contract.toml` - logical electrical contract
- `pinout.csv` - connector and subsystem signal allocation
- `net_class_plan.md` - routing/impedance classes
- `schematic_manifest.md` - schematic sheet hierarchy and required blocks
- `power_tree.md` - power architecture
- `mechanical_keepouts.md` - mechanical/thermal constraints
