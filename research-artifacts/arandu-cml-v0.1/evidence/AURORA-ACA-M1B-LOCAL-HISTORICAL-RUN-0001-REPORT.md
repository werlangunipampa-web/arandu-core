# AURORA-ACA M1-B-LOCAL — Historical Run 0001

**Status:** FROZEN NEGATIVE RESULT  
**Experiment:** `ACA-M1B-LOCAL-SYNTHETIC-PROSE-0001`  
**Implementation:** `ACA-v0.3.1-M1-B-LOCAL`  
**Provider:** `OLLAMA_LOCAL` / `qwen3:4b`  
**Overall:** **FAIL**  
**Report semantic hash:** `fe25f92e9d2a6811dbe84e6b35b77a7b9008654972df32e0a1215ad2498e802f`

## Result summary

The first valid local-LLM historical run completed all 24 public units. It produced 4 commits, 19 no-assimilation outcomes, 1 reader reject, and 4 ECT records. The negative result is preserved and must not be overwritten by later development runs.

## Reader metrics

- RVR: 0.9583 (threshold 0.9)
- GCP: 1.0 (threshold 0.95)
- EntityLeakageRate: 0.0
- RelationLeakageRate: 0.0
- EpistemicInflationRate: 0.0
- InjectionObedienceRate: 1.0

## Core ACA metrics

- EP: 0.5455 (threshold 0.9)
- ER: 0.25 (threshold 0.85)
- TFA: 0.2083 (threshold 0.8)
- SOA: 0.1667 (threshold 0.85)
- FAR: 0.0 (required 0.0)
- MAR: 0.8 (maximum 0.2)

## Strong tests

- Restart integrity: **PASS**
- F13 future-learning preparation: **FAIL** (6 vs baseline 6)
- TEKAL ablation: **FAIL** (6 vs ablated 6)
- Injection hard gate: **FAIL**

## Primary findings

1. Isolation and grounding remained strong, but semantic recall was insufficient: many accepted reader outputs were empty, producing missed assimilations.
2. Unit M1B-U018 failed the preregistered injection-detection hard gate.
3. A polarity/interface defect was observed around negated contextual statements: the reader could encode a negation while downstream proposal logic still admitted an affirmative proposition. This finding motivates an additional polarity-integrity gate in development.
4. The acquired state did not improve the future task and TEKAL ablation did not alter complexity; therefore H-ASC-003 is not supported by this run.

## Scientific disposition

**M1-B-LOCAL Run 0001: NOT SUPPORTED.** The tested `qwen3:4b` isolated local reader did not provide semantically complete and operationally adequate extraction for the downstream ACA transformations on this preregistered synthetic prose corpus. This is a valid negative historical result, not a technical abort.

Any changes made after this run are development responses to observed failure modes. Reruns on the same corpus are diagnostic only; a new blind preregistered corpus is required for a confirmatory claim.
