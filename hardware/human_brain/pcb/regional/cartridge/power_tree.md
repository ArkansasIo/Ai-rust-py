# Regional Cartridge Power Tree

Prototype input is 48 V DC from the Neuro-Link backplane.

```text
48V BACKPLANE
    |
    +-- slot fuse / eFuse
    |
    +-- reverse-polarity + surge protection
    |
    +-- 48V current monitor
    |
    +--+-------------------------------+
       |                               |
       v                               v
 compute point-of-load              5V regulator
       |                               |
       v                               +--> USB-C/service
 CPU / GPU / NPU rails                 +--> audio/peripherals as required
                                       |
                                       v
                                    3V3 regulator
                                       |
                                       +--> MCU / I2C / SPI / GPIO

All rails require power-good reporting to the management MCU.

## Safety sequence

1. Backplane asserts slot-enable.
2. eFuse validates input and current limit.
3. Logic rail starts and management MCU boots.
4. MCU validates power-good and thermal sensors.
5. Compute rails are enabled.
6. Bootloader performs secure-boot verification.
7. Neuro-Link is enabled only after local health checks pass.
8. On over-temperature, over-current, or watchdog failure, compute rails are disabled and the slot reports FAULT.

Exact regulator topology and component values must be selected after compute silicon and peak-current requirements are frozen.
