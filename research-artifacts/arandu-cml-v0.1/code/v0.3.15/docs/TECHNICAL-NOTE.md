# AURORA ACA v0.3.15-dev — Semantic Frame Router

Status: development engineering only. No scientific claim.

## Purpose

v0.3.14 eliminated schema rejects and context loss in the fresh S01–S18 probe but missed the frozen reader coverage gate (0.8333 < 0.85). The remaining errors were semantic-frame discrimination errors rather than JSON/schema failures.

v0.3.15 introduces a two-stage Reader boundary:

1. **semantic frame router** — classifies only the explicit kind(s) of linguistic content in the current unit using a closed ontology;
2. **structured extractor** — extracts literal arguments into the already existing v0.3.14 structured output schema, using router frames only as routing hints.

The router does not access ECA state, gold, prior units, future units, literal rescue, or external knowledge. It does not choose assimilation or cognitive transformation families.

## Preserved boundaries

Unchanged: MAS, ECA, ECT, proposer, literal rescue, candidate builder, reader validator, historical R2/R3/R4 results, v0.3.13 snapshot, v0.3.14 snapshot, and the old engineering gate.

## Gates

The previous engineering gate is preserved exactly:

- reader_frame_coverage >= 0.85
- context_qualifier_misses = 0
- reader_rejects = 0

v0.3.15 adds a prospective diagnostic router gate:

- semantic_router_coverage >= 0.85

`eligible_for_reader_freeze_review` requires both gates. This does not retroactively alter any earlier result.

## Exposure rule

S01–S18 remain DEVELOPMENT-EXPOSED. The v0.3.15 runtime is frozen before the T01–T20 fresh probe. T01–T20 become DEVELOPMENT-EXPOSED after first execution and are never an R5 blind corpus.
