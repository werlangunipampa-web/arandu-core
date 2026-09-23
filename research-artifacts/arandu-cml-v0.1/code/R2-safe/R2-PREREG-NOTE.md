# AURORA ACA M1-B R2 — preregistration package

Experiment: `ACA-M1B-R2-BLIND-GENERALIZATION-0001`

Purpose: test blind generalization of the hybrid local reader architecture on a new synthetic corpus after the v0.3.4 engineering success.

## Important methodological note

The R2 package does **not** reuse the original nonce vocabulary. The public corpus contains a new vocabulary and a mix of previously supported surface frames and paraphrases. The private gold is not loaded until after the learner snapshot is frozen.

Before R2, one preregistered decontamination refactor was made: the retriever's context-token heuristic no longer hard-codes `K1/K2/K3`; it recognizes generic uppercase letter+digit context identifiers. The learning families, proposer semantics, MAS/ECT pathway, reader prompt/schema, literal-rescue patterns, thresholds, and no-assimilation policy otherwise remain inherited from the v0.3.4 line.

This means R2 is a test of the **preregistered v0.3.5-R2 architecture**, not a confirmatory rerun of the exact v0.3.4 binary.

## Frozen thresholds

Reader: RVR >= .90, GCP >= .95, SemanticCoverage >= .85, PolarityViolationRate = 0, ContextCollapseRate = 0.

Core: EP >= .90, ER >= .85, TFA >= .80, SOA >= .85, FAR = 0, MAR <= .20.

F13-like future task and superordinate ablation are mandatory.

## Run sequence

1. `python scripts/verify_r2.py`
2. `export AURORA_M1B_MODEL='qwen3:4b'`
3. `python scripts/check_ollama.py`
4. `python scripts/run_r2.py --freeze-provider`
5. `python scripts/run_r2.py --check`
6. `python scripts/run_r2.py --execute`
7. `python scripts/run_r2.py --check`

Do not edit corpus, gold, reader prompt/schema, rescue, proposer, validator, retriever, or thresholds after provider freeze.
