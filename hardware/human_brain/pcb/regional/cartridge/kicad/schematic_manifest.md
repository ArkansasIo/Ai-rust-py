# Cartridge Schematic Manifest

## Sheet hierarchy

```text
cartridge.kicad_sch
├── 01_power
│   ├── 48V input protection
│   ├── intermediate 12V rail
│   ├── 5V / 3V3 auxiliary rails
│   └── module power-enable / power-good
├── 02_management
│   ├── safety MCU
│   ├── watchdog
│   ├── temperature ADC / digital sensors
│   ├── fan PWM + tach
│   ├── I2C / SPI / UART
│   └── USB-C service
├── 03_compute
│   ├── Jetson Orin NX module interface
│   ├── boot storage
│   ├── M.2 NVMe
│   └── module reset / recovery / power sequencing
├── 04_neuro_link
│   ├── high-speed differential lanes
│   ├── reference clock
│   ├── reset / link-enable
│   └── link status / telemetry
├── 05_gpio
│   ├── low-speed GPIO bank
│   ├── interrupts
│   └── expansion/service signals
└── 06_thermal
    ├── CPU/GPU/board sensors
    ├── fan driver
    ├── over-temperature shutdown
    └── thermal telemetry
```

## Design rule

High-bandwidth neural traffic does not use GPIO, I2C, SPI, or UART. Those buses are management/local-peripheral paths. Neuro-Link uses dedicated controlled-impedance differential pairs routed from the compute subsystem to the standardized backplane connector.

## KiCad implementation gate

The actual `.kicad_sch` is generated only after the exact Orin NX carrier interface pins, connector mating part, stack height, and manufacturer footprint data are frozen. This prevents a cosmetically complete but electrically invalid schematic.
