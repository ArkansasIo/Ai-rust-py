# Regional Compute Cartridge PCB

All 12 brain-region cartridges share the same mechanical envelope, connector map, power interface, service interface, thermal interface, and firmware contract. Internal compute can be specialized by region.

## Regions

1. brainstem
2. thalamus
3. cerebellum
4. hippocampus
5. amygdala
6. basal_ganglia
7. frontal
8. parietal
9. temporal
10. occipital
11. motor
12. somatosensory

## Standard interface

- 48 V nominal backplane input, locally converted
- I2C management
- SPI local peripherals
- UART recovery/diagnostics
- high-speed differential Neuro-Link
- reset and power-good
- fan/tach/thermal telemetry
- USB-C service access
- microSD/eMMC-class boot/recovery
- NVMe local persistent storage
- GPIO for local sensors and control

The connector pinout is defined separately so a region cartridge can be replaced without changing the backplane.
