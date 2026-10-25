# Synthetic Skull Hardware Architecture

The synthetic brain is packaged as a modular skull-shaped computer. The skull is a mechanical enclosure and thermal-management system for twelve independent neural-compute region modules plus system management and power electronics.

## Design principles

- Custom PCB modules are the primary hardware target.
- Every neural region is an independent compute node with CPU/SoC, accelerator, RAM, microSD boot media, SSD/NVMe persistent storage, GPIO, power management, watchdog, and Neuro-Link interface.
- The microSD card contains bootloader, firmware, recovery, and base system software.
- The SSD/NVMe device is the region's persistent memory bank for models, learned state, databases, logs, and long-term data.
- GPIO handles hardware control signals such as reset, boot mode, interrupt, power-good, ready, and fault.
- The Neuro-Link backplane carries inter-region communication; raw GPIO is not used as the high-bandwidth neural data path.
- USB-C provides the external service/programming connection through the system-management controller.

## Physical modules

1. Brainstem
2. Thalamus
3. Cerebellum
4. Hippocampus
5. Amygdala
6. Basal Ganglia
7. Frontal Lobe
8. Parietal Lobe
9. Temporal Lobe
10. Occipital Lobe
11. Motor Cortex
12. Somatosensory Cortex

## Mechanical layers

```text
Outer shell
  -> impact/protection structure
  -> cooling/airflow or cold-plate layer
  -> regional PCB mounting layer
  -> Neuro-Link backplane
  -> power and system-management layer
  -> service access
```

The skull geometry must be finalized from the dimensions of the custom PCBs, connectors, storage devices, cooling hardware, and power system. The repository therefore defines parametric mechanical constraints rather than pretending a final physical envelope is already validated.

## Service access

A service hatch provides access to the USB-C management connector, diagnostic UART, removable microSD cards, and replaceable regional compute modules. The design supports replacing one region without disassembling the entire brain.

## Safety

The enclosure must provide electrical isolation where required, thermal monitoring, airflow/cooling paths, emergency power cutoff, current limiting, watchdog reset, and protected service access. The mechanical design is a prototype engineering specification and must be validated before fabrication or operation.
