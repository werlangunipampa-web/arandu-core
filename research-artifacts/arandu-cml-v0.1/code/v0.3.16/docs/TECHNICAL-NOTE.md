# AURORA ACA v0.3.16-dev — Epistemic Ambiguity / Extraction Confidence Separation

Status: DEVELOPMENT ENGINEERING ONLY.

The v0.3.15 fresh router probe remains a formal FAIL. This patch does not
retroactively change that result. The audit isolated the sole Reader rejection
to `AMBIGUITY_CONFIDENCE_CAP`, which conflated source-level epistemic ambiguity
with confidence that the Reader extracted that ambiguity correctly.

## Prospective correction

- `ambiguous` continues to describe ambiguity/uncertainty expressed by the source.
- item `confidence` continues to be range-validated in [0,1], but now means
  extraction confidence, not confidence that the source proposition is true.
- high extraction confidence is permitted for explicitly ambiguous content.
- no gate is weakened.
- provider, schema, MAS/ECA/ECT, learner, proposer, literal rescue, engine and
  candidate builder are unchanged.

Reader boundary hash: `755959f4d81a74894713606812d247b07ab402a27eedce600c421e5b38917fb5`
Validator hash: `e87604b20701b43c0b7d39f2ef97fb7102ef30fcca3638d5d326b6cfe46c8b4b`
Previous frozen v0.3.15 snapshot root: `b478ea0dde06c782f203f14569945c6214b4e23e6f00cc97313191feb2cc1ff0`.
