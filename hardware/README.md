# hardware/

Physical build notes for the monitor node and scale platform. The BOM is in
[`BOM.md`](BOM.md).

## Scale platform (per hive)

Two options; decide after the Flow Hive 2+ arrives and we can measure the
leg footprint (see open question in `docs/research/monitoring-sensors.md`).

**A. Whole-hive platform (preferred).** 3/4" exterior plywood top,
600 × 500 mm, on four 50 kg half-bridge cells in corner mounts, on a second
plywood base that sits on the stand. Flow legs retracted or removed; the
hive base sits flat on the top plate. Pros: standard, well-documented
design; hive is very stable. Cons: lose the built-in levelling; must level
the stand instead.

**B. Four leg cups.** Each Flow leg stands in a cup on its own load cell.
Pros: keeps the legs. Cons: cells see point loads and moments; harder to
keep four cells co-planar; wobbly.

Load cell wiring: the four half-bridges form a full Wheatstone bridge
(standard "bathroom scale" wiring: E+, E-, A+, A-) into one HX711. Keep the
HX711 within 20 cm of the cells, shielded cable, in its own small IP65
box. A DS18B20 taped to the HX711 board feeds temperature compensation.

## Enclosures

- Main node box: IP66 ABS ~150 × 100 × 70 mm, Gore/PTFE vent plug, PG7
  cable glands (one per cable), mounted on the stand *under* the hive
  overhang, not on the hive (vibration during inspections, and you'll want
  to lift the hive without unplugging).
- Sensor cable into the hive: through the inner-cover feed hole with a
  split grommet; DS18B20 probes on 1 m stainless leads.
- Mic: mount to the underside of the inner cover in a small foam-lined
  pocket, screened so bees don't propolise the port. Expect to clean it
  each autumn.

## Power

Decision pending (see open questions). If mains: 12 V 2 A outdoor PSU on
its own GFCI → node box. Inside: fuse, 12→5 V buck (for the ESP32 + HX711
excitation) and, if the heater is ever built, the 12 V heater branch with
its own fuse, hardware thermostat, and MOSFET.

If battery: 18650 ×2 in a holder with a BMS, a 6 V 3 W panel angled south
at ~50°, TP4056-based charger with load sharing — and expect to swap the
cells indoors December–February.

## Safety

- Only low voltage (≤ 12 V DC) past the PSU. No mains at the hive.
- Every 12 V branch fused. Heater branch has a hardware thermal cutoff in
  series regardless of what the firmware does.
- Lithium cells: use protected cells or a BMS; never charge below 0 °C
  (the charger needs a temperature cutoff, or disable charging in winter).
