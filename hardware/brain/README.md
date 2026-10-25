# Layered Synthetic Brain Hardware

This directory defines a Raspberry-Pi-class distributed synthetic-brain computer.

The design uses multiple compute controllers rather than one monolithic MCU. Each controller owns a logical brain region and exposes a stable Neuro-Link Protocol (NLP) interface.

## Region controllers

- `controllers/brainstem_controller.rs`
- `controllers/thalamus_controller.rs`
- `controllers/cerebellum_controller.rs`
- `controllers/hippocampus_controller.rs`
- `controllers/amygdala_controller.rs`
- `controllers/basal_ganglia_controller.rs`
- `controllers/frontal_lobe_controller.rs`
- `controllers/parietal_lobe_controller.rs`
- `controllers/temporal_lobe_controller.rs`
- `controllers/occipital_lobe_controller.rs`
- `controllers/motor_cortex_controller.rs`
- `controllers/somatosensory_controller.rs`

These are architectural mappings, not claims that a controller reproduces a biological lobe one-for-one.

## Physical stack

1. Power and safety board
2. System management controller
3. Neuro-Link backplane
4. Region compute controllers
5. Shared high-speed memory
6. Sensor/actuator I/O
7. Debug and service interface

The first implementation target is conventional Linux-capable boards and RP2040/STM32/ESP32-class controllers. FPGA/NPU/neuromorphic implementations can replace individual tiles later.
