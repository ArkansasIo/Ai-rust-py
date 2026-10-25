# Standard Regional Neural Cartridge

This is the common electrical, mechanical, thermal, and service template for all twelve synthetic-brain regional computers.

## Fixed contract

- 120 x 80 x 25 mm provisional envelope
- 10-layer prototype PCB target
- 48 V backplane input
- independent management MCU
- ARM64 compute architecture target
- GPU/NPU accelerator interface
- 32 GB baseline RAM target
- microSD/eMMC boot target
- NVMe local storage target
- I2C management
- SPI local peripherals
- UART diagnostics
- USB-C service/programming
- GPIO for low-speed control/status
- high-speed differential Neuro-Link for inter-region neural traffic
- per-board thermal monitoring and fan control
- secure boot and watchdog

## Regional specialization

The physical cartridge remains standardized. Region-specific behavior is selected through the region map, firmware profile, accelerator configuration, memory/storage sizing, and software runtime.

The current region profiles are defined in `../region_map.toml` and include brainstem, thalamus, cerebellum, hippocampus, amygdala, basal ganglia, frontal, parietal, temporal, occipital, motor, and somatosensory systems.

## Manufacturing gate

Do not release Gerbers or order a production board until the processor/SoM, memory, PMIC, connectors, high-speed transceivers, and exact thermal envelope are selected. The current files are architecture-level source, not fabrication-ready electronics.
