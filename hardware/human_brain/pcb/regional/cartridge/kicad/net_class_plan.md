# Regional Cartridge Net-Class Plan

| Class | Nominal impedance | Rule |
|---|---:|---|
| NEURO_DIFF | 100 ohm differential | Length-match each pair; reference continuous ground; no plane splits |
| CLOCK_DIFF | 100 ohm differential | Tight skew control; isolated from switching nodes |
| USB_DIFF | 90 ohm differential | Route as USB-C high-speed pair per selected PHY/module guide |
| PCIe_DIFF | 85 ohm differential | Use only where the selected compute module exposes PCIe; length/skew per platform guide |
| MGMT_I2C | N/A | Short, low-speed, pulled up at one defined point |
| MGMT_SPI | N/A | Short local bus; controlled source termination if required |
| DEBUG_UART | N/A | Keep away from high-speed lanes and switching inductors |
| GPIO | N/A | Low-speed only; series protection where leaving the enclosure |
| POWER_48V | N/A | Wide copper; protected input; current/temperature monitored |
| POWER_LOCAL | N/A | Width and copper determined from load and fabricator stack-up |

## High-speed rules

1. No high-speed pair crosses a split reference plane.
2. Differential pair geometry must be calculated from the actual PCB dielectric stack-up.
3. Connector launch geometry must use the connector manufacturer's recommended footprint and breakout.
4. Avoid stubs and unnecessary vias; back-drill where required by the final data rate.
5. Validate insertion loss, return loss, crosstalk, and eye margin before production release.

These are design classes, not fabrication guarantees. Final trace widths and spacing are calculated from the selected fabricator's stack-up.
