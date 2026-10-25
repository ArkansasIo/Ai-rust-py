# Regional Cartridge Power Tree

```text
48V_BACKPLANE
    |
    +-- fuse / surge protection / eFuse / hot-swap
    |
    +-- 12V_INTERMEDIATE
    |      |
    |      +-- compute-module input / module power sequencing
    |      +-- blower / fan rail
    |      +-- auxiliary converters
    |
    +-- 5V_AUX
    |      +-- USB-C service
    |      +-- local peripherals
    |
    +-- 3V3_AUX
           +-- management MCU
           +-- sensors / GPIO / control
```

## Rules

- Never connect 48 V directly to the compute module.
- The 48 V input is protected and current-monitored at the cartridge boundary.
- The compute module uses its specified carrier input and sequencing requirements.
- The management MCU remains alive long enough to report faults and command safe shutdown when compute power is disabled.
- Every major rail has power-good monitoring.
- Thermal shutdown is independent of application software.
- Final converter part numbers and inductors are selected from measured load budgets and the chosen PCB thermal stack-up.

## Initial budget

Design the Rev A carrier around a 25 W compute-module operating envelope plus storage, management, networking, and fan overhead. Provide transient margin above the nominal load. Higher-power master compute is not part of this regional rail budget.
