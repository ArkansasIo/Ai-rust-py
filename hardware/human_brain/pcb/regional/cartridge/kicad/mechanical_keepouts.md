# Regional Cartridge Mechanical Keepouts

## Envelope

- Board: 120 x 80 mm
- Cartridge maximum thickness: 25 mm
- Connector and guide hardware must remain inside the standardized outer mechanical envelope.
- Keep a defined service side for USB-C, status LEDs and debug access.

## Compute keepout

Reserve the Jetson Orin NX module footprint at 70 x 45 mm plus the manufacturer's required keepout and carrier connector region. Do not place tall components beneath the module.

## Thermal keepout

- Copper heat spreader directly over the compute heat source.
- No tall components in the heat-spreader contact region.
- Fan interface aligned with the cartridge airflow duct.
- Air inlet and outlet must not short-circuit internally.
- Thermal sensors must contact the intended heat zones rather than merely measuring ambient PCB temperature.

## Backplane keepout

Reserve the selected Samtec high-density connector and its guide/keying hardware before placing any other edge components. Final connector dimensions are controlled by the selected manufacturer part number.

## Service keepout

USB-C service access, status indicators and emergency disable controls must remain reachable without removing the cartridge from the skull/backplane assembly.
