# Neuro-Link Neural Backplane

## Logical buses

- NLP control channel: topology, discovery, health
- NLP signal channel: neural events and activations
- NLP learning channel: reward, plasticity, adaptation
- Memory channel: hippocampal/long-term storage traffic
- Sensor channel: perception input
- Motor channel: actuator commands
- Management channel: firmware, telemetry, watchdogs

## Board interfaces

Recommended physical interfaces are SPI for local controllers, I2C for low-speed management, UART for diagnostics, Ethernet/USB for development, and a future FPGA/SerDes fabric for high-throughput deployments.

All physical transports terminate in a Neuro-Link adapter. Brain software therefore does not depend directly on a particular bus.
