# Compute and Connector Selection — Regional Cartridge Rev A0

## Regional compute

**Prototype selection: NVIDIA Jetson Orin NX 16GB.**

Reason: the module is listed at 70 x 45 mm and 10–25 W, so it fits the 120 x 80 mm standardized cartridge envelope while leaving room for carrier routing, management electronics, NVMe, thermal hardware, and the standardized connector zone. NVIDIA lists the Orin NX family at up to 157 TOPS. 

The production architecture remains modular: the compute device is a replaceable module on a custom ArkansasIo carrier PCB. This avoids committing the first revision to a chip-down BGA/LPDDR/PMIC implementation before the software and electrical interfaces are proven.

## Master compute

The master board remains a separate class. Jetson AGX Thor is a candidate for the master because NVIDIA lists up to 2070 FP4 TFLOPS, 128 GB unified memory, and 40–130 W configurable power. The Thor module envelope is approximately 100 x 87 mm, so it is **not** forced into the 120 x 80 mm regional cartridge. 

## High-speed connector

**Prototype selection: Samtec SEARAY family, open-pin-field high-density board-to-board connector.**

The family is appropriate for a standardized cartridge/backplane because it supports dense differential routing, flexible grounding, and high-speed protocols. Exact series, row count, contact count, mating height, guide/keying option, and plating are still to be selected from the final mechanical stack-up and SI simulation.

## Frozen interface intent

- 48 VDC input
- independent management MCU
- I2C/SPI/UART/GPIO management/local I/O
- USB-C service/programming
- M.2 NVMe local persistent storage
- dedicated differential Neuro-Link lanes
- distributed reference clock/reset
- thermal telemetry and fan PWM/tach

## Selection gate before PCB fabrication

1. Download and lock the exact NVIDIA module interface design guide and pinmux data.
2. Select exact SEARAY mating pair and stack height.
3. Select the PCB fabricator stack-up.
4. Generate controlled-impedance calculator results.
5. Import manufacturer land patterns.
6. Build the complete KiCad schematic.
7. Run ERC, DRC, SI/PI review and thermal review.
8. Generate fabrication outputs only after those gates pass.
