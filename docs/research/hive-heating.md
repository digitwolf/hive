# Keeping the colony warm: insulation first, heating maybe

**Status:** researching
**Last reviewed:** 2026-09-27
**Decision (provisional):** Winter 2027–28 = insulation + moisture management only, instrumented. A heater is a 2028–29 experiment on *one* hive, thermostatically limited, only if the 2027–28 data shows the cluster struggling. Never heat the cluster to brood temperature.

## Question

The PNW winter problem is not cold, it is **damp + cold + long**. Can we
reduce winter losses and spring build-up lag by keeping the hive warmer, and
is active heating ever a good idea?

## Facts (with sources)

- Bees don't heat the hive; they heat the **cluster**. Cluster core stays
  ~20–35 °C, and the bees regulate it by metabolizing honey. Heat loss →
  more honey consumed → more metabolic water → more condensation in a cold
  box. (Seeley; Southwick 1985 on cluster metabolism vs ambient.)
- Colony energy use rises sharply below about 10 °C ambient and again
  below ~0 °C. A well-insulated hive (R-8 to R-10 walls and top) can halve
  winter stores consumption versus a bare 3/4" pine box. (Mitchell 2016,
  "Ratios of colony mass to thermal conductance of tree and man-made nest
  enclosures"; Ontario/Alberta wrapped-hive trials.)
- Top insulation matters more than side insulation: warm humid air rises,
  hits a cold roof, condenses, drips on the cluster. A 2" polyiso board
  above the inner cover, or a moisture quilt (wood shavings in a screened
  box), keeps the roof underside above dew point. Wet bees die; cold dry
  bees mostly don't.
- Commercial "hive heaters" exist (12–24 V resistive pads, 10–40 W, placed
  on/under the bottom board or as a heated frame). Reported uses:
  small/weak colonies, nucs, queen banks, and *thermal varroa treatment*
  (raising brood to ~42 °C for a few hours — a separate, specialist use).
  Evidence for heating strong, well-insulated colonies is thin and mixed.
- Overheating risks: bees break cluster in winter, fly out on cold days
  and die; queen starts laying too early, colony eats stores raising
  brood it can't keep warm when the heater fails; humidity goes up.
  A heater that fails **on** is worse than no heater. A heater that fails
  **off** mid-January on a colony that has adapted to it is also bad.
- The bottom-board position is the least risky: it warms the air at the
  base, doesn't touch comb, and keeps the bottom board dry. Under-the-lid
  heaters interact badly with the condensation logic above.

## Local interpretation

- Woodinville winter: 0–8 °C, 85–95 % RH outside for weeks. The enemy is
  condensation and slow starvation, not −20 °C. **Insulation and a
  moisture quilt solve most of it** and cost ~$30/hive.
- If, after a monitored winter, we see (a) cluster temp falling below
  ~15 °C repeatedly, (b) stores consumption > ~1 kg/week in Dec–Jan, or
  (c) a small cluster after a late split, that's the case for a heater.
- Because the hives will be instrumented, a heater experiment can be
  honest: hive A heated, hive B not, same insulation, log everything.
- Mains power at the stand is a prerequisite. Don't run a heater from a
  battery.

## Options considered

| Option | Cost | Complexity | Risk | Verdict |
|--------|------|------------|------|---------|
| 2" polyiso on top + foam/reflectix wrap on sides + moisture quilt + reduced entrance + tilt forward | ~$30 | Low | Low | **Do in 2027** |
| Insulated hive cover (Bee Cozy style wrap) | ~$25 | Low | Low | Alternative to DIY wrap |
| Resistive pad under/on bottom board, 12 V, 10–20 W, thermostatic (target: keep bottom-board air ≥ 5 °C, never above ~10 °C), hard over-temp cutoff, watchdog | ~$40 + PSU | Medium | Medium | 2028 experiment, one hive |
| Heated frame in the brood box | ~$60 | High | High (comb melt if runaway, blocks comb) | No |
| Thermosolar hive / lid | $$$ | Med | Med | Not for Flow Hive |
| Thermal varroa treatment (42 °C) | Special kit | High | High | Out of scope; note only |

## Heater controller design constraints (if we build one)

1. **Two independent limits**: firmware setpoint via DS18B20 at the pad,
   *and* a mechanical thermal cutoff (e.g. KSD9700-type normally-closed
   thermostat at 15 °C, or a self-regulating PTC element) in series. The
   firmware cannot make the pad hotter than the hardware allows.
2. **Watchdog**: if the ESP32 stops toggling the relay/MOSFET, the heater
   turns off (hardware watchdog or a timed relay; no "stuck on").
3. Power budget ≤ 20 W per hive at 12 V; fuse at 3 A; outdoor-rated PSU
   with its own GFCI; low-voltage only past the enclosure.
4. Duty-cycle logged to MQTT so the heater's own effect shows in the data.
5. Manual off switch at the hive.

This lives in `firmware/hive-monitor` behind a build flag
(`HEATER_ENABLED`, default off) so the same node does both jobs.

## Open questions

- [ ] What is the actual cluster temperature profile across a Woodinville
      winter in an insulated Flow Hive 2+? (The 2027–28 data answers this.)
- [ ] Does the Flow Hive 2+ roof/inner cover assembly leave room for 2"
      polyiso, or does the roof need a shim ring?
- [ ] Moisture quilt vs vented polyiso lid on a Flow Hive — which fits?
- [ ] Any published PNW-specific trial of bottom-board heaters? (Search WSU,
      OSU, UBC/BC Ministry of Agriculture.)

## Sources

- Seeley, T. *The Lives of Bees* (2019), ch. on thermoregulation.
- Southwick, E.E. (1985) "Allometric relations, metabolism and heat
  conductance in clusters of honey bees at cool temperatures." *J Comp
  Physiol B*.
- Mitchell, D. (2016) "Ratios of colony mass to thermal conductance of tree
  and man-made nest enclosures of *Apis mellifera*." *Int J Biometeorol*.
- Mitchell, D. (2017) "Honey bee engineering: Top ventilation and top
  entrances." *American Bee Journal* — argues for top insulation and
  against top venting.
- WSU Extension winter management guidance for western Washington.
