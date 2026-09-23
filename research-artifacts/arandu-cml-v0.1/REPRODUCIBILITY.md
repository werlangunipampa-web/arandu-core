# Reproducibility and audit guide

## Audit chain

The intended traceability chain is:

`claim → manuscript statement → experiment/version → frozen code/configuration → corpus status → raw/terminal evidence → cryptographic hash`.

## Reader / learner separation

The local LLM is treated as a linguistic Reader only. It is not the ECA, MAS, ECT, or the decision mechanism for assimilation.

The v0.3.16 frozen Reader metadata records explicitly state that no gold access or ECA-state access was added, while provider, schema, literal rescue, and the prospective engineering gate remained unchanged.

## v0.3.16 engineering gate

The preserved fresh V01–V20 development probe reported:

- semantic router coverage = 0.95;
- Reader frame coverage = 0.95;
- context qualifier misses = 0;
- Reader rejects = 0;
- ambiguous Reader rejects = 0;
- engineering gate = true.

This is a component engineering result only. It is not R5.

## R2 public subset

The historical frozen R2 archive contains `experiments/m1b_r2/private_eval/`, including gold data and private future units. Those files are intentionally excluded from the public artifact. The public corpus/protocol, preregistration metadata, source, tests, and relevant documentation can be released without changing R2's already-frozen scientific status.

## Blindness rule

Re-running an exposed historical corpus can verify software behavior but cannot restore confirmatory blindness.

## Future R5 boundary

R5 must use a newly frozen architecture and a previously unexposed corpus. No R5 private gold, future units, blind corpus, or result material belongs in this v0.1 public artifact.
