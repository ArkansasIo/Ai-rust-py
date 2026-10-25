# Synthetic Brain Architecture — Hardware Draft Schematic

## 1. System block diagram

```text
                         SYNTHETIC BRAIN ARCHITECTURE

       +-------------------- CUSTOM SKULL / SENSOR SYSTEM --------------------+
       |                                                                      |
       |   LEFT EYE                 RIGHT EYE                  AUDIO           |
       | RGB + IR + DEPTH        RGB + IR + DEPTH        L/R MICROPHONE ARRAYS|
       |       |                        |                         |             |
       +-------+------------------------+-------------------------+-------------+
               | synchronized sensor links / preprocessing
               v
       +---------------------------------------------------------------------+
       |                         NEURO-LINK BACKPLANE                         |
       |  clock | reset | I2C management | SPI | diagnostics | neural data  |
       |                         |                    |                       |
       |      +------------------+--------------------+------------------+    |
       |      |                  |                    |                  |    |
       |      v                  v                    v                  v    |
       |  [BRAINSTEM]       [THALAMUS]          [CEREBELLUM]       [HIPPOCAM.]|
       |  [AMYGDALA ]       [BASAL GAN.]         [FRONTAL   ]       [PARIETAL] |
       |  [TEMPORAL ]       [OCCIPITAL]          [MOTOR     ]       [SOMATO. ] |
       |      |                  |                    |                  |    |
       |      +------------------+--------------------+------------------+    |
       |                             |                                        |
       |                      +------+-------+                                |
       |                      | MASTER BOARD |                                |
       |                      | ARM64        |                                |
       |                      | GPU/NPU      |                                |
       |                      | 64 GB RAM    |                                |
       |                      | NVMe 2 TB    |                                |
       |                      +------+-------+                                |
       +-----------------------------|----------------------------------------+
                                     |
                            service / management
                                     |
                         USB-C | Ethernet | UART | JTAG
                                     |
                               SERVICE BOARD

POWER PATH

48 V DC INPUT
     |
     v
+----------------+
| POWER CONTROLLER|
| OVP/OCP/REVERSE |
+--------+-------+
         |
         +----> MASTER POLL
         +----> 12 x SLOT FUSES + CURRENT MONITORS
         +----> 5 V / 3.3 V / LOW-NOISE AUDIO
         +----> POWER-GOOD / EMERGENCY CUTOFF

THERMAL PATH

CPU/GPU/NPU -> copper spreader or vapor chamber -> aluminum fin stack
                                              |
                                              v
                                      PWM centrifugal blower
                                              |
                                    ducted skull airflow
                                              |
                                      monitored exhaust
```

## 2. Standard regional cartridge

Every regional cartridge follows the same external contract:

```text
120 mm x 80 mm x 25 mm provisional envelope

+------------------------------------------------+
| USB-C SERVICE                                  |
|                                                |
|  CPU / SoC     GPU/NPU      RAM                |
|                                                |
|  microSD/eMMC  NVMe/SSD     management MCU    |
|                                                |
|  thermal sensors / watchdog / power telemetry  |
|                                                |
|  Neuro-Link high-speed differential interface  |
+------------------------------------------------+
       |       |       |       |       |
      PWR     I2C     SPI    UART    GPIO
       |
  standardized backplane connector
```

## 3. Backplane contract

The 12 slots share:

- 48 V prototype input distribution
- per-slot fuse and current measurement
- power-good and slot-enable
- distributed low-skew clock
- distributed reset
- I2C management
- SPI local peripherals
- UART diagnostics
- high-speed differential Neuro-Link data

GPIO is reserved for low-level control and status. High-bandwidth inter-region communication is not routed through ordinary GPIO.

## 4. Master board draft

The master board is a provisional 120 × 80 mm, 10-layer board with:

- ARM64 compute platform to be selected after platform evaluation
- integrated or attached GPU/NPU
- 64 GB target RAM
- microSD or eMMC boot
- 2 TB NVMe target storage
- independent management MCU
- USB-C, UART, JTAG
- Ethernet management
- high-speed Neuro-Link interface
- thermal monitoring and watchdog

## 5. Sensor boards

### Vision

Stereo RGB + IR + depth sensing is treated as one synchronized eye-camera subsystem. The design includes hardware synchronization, preprocessing, replaceable optical windows, and a high-speed link into the Neuro-Link fabric. Primary software destination is the occipital region, with routing to thalamic, parietal, temporal, amygdala, frontal, and motor functions.

### Audio

Left/right synchronized microphone arrays feed an audio front end with ADC/DSP processing, directional localization, noise filtering, and a speaker output path. Primary software destination is temporal processing with thalamic/global routing.

## 6. Thermal draft

Initial engineering heat budget is approximately 12 regional modules at 25–60 W each, plus master and auxiliary electronics. The final cooling system must be sized from measured silicon power and thermal resistance, not the provisional envelope alone.

Target control points:

- warning: 70 °C
- throttle: 78 °C
- system shutdown: 85 °C provisional safety limit

Actual device-specific limits must follow the selected processors, memory, PMICs, and regulators.

## 7. Safety and service

The architecture includes:

- independent management MCU
- hardware watchdog
- secure boot/provisioning path
- per-slot power isolation
- thermal sensors
- emergency power cutoff
- USB-C service/programming
- UART recovery console
- JTAG/SWD development access
- module replacement and recovery workflow

## 8. Draft status

This document is a system-level draft schematic and documentation baseline. It deliberately does not claim exact component placement, impedance-controlled routing, validated DDR topology, power-integrity margins, thermal resistance, EMC compliance, or manufacturing readiness.

The next electrical-design stage is to select actual compute silicon/SoMs, PMICs, memory, connectors, transceivers, clock devices, sensors, and protection components, then generate the KiCad schematic, PCB, stackup, net classes, BOM, and manufacturing outputs.
