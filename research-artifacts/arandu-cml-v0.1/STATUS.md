# Experimental status registry

| Stage | Evidence class | Preserved status | Public artifact status |
|---|---|---|---|
| M0 | controlled development | favorable engineering/synthetic-cognition evidence | historical hash/manifest pending archive audit |
| M1-A | controlled validation with deterministic/rule-assisted Reader | downstream gates, restart, future-learning and ablation passed; Reader limitation preserved | historical hash/manifest pending archive audit |
| M1-B Local Run 0001 | valid local-LLM historical run | **NOT SUPPORTED** | negative report scheduled for public artifact |
| v0.3.2–v0.3.4 | post-hoc development | engineering only | lineage referenced; not confirmatory |
| R2 / v0.3.5 | preregistered blind confirmatory run | **NOT SUPPORTED** | safe subset audited; private_eval excluded |
| v0.3.6 | post-hoc development | POST-HOC ENGINEERING SUCCESS; does not reverse R2 | lineage referenced |
| R3 | attempted blind confirmation | **TECHNICALLY INCONCLUSIVE**; corpus consumed | lineage referenced |
| R4 | attempted blind confirmation | **TECHNICALLY INCONCLUSIVE / TECHNICAL_ABORT_PARTIAL_RUN**; corpus consumed | lineage referenced |
| v0.3.12 | fresh Reader instrumentation | raw frame coverage 0.4167; FAIL | lineage referenced |
| v0.3.13 | model comparison | 4b 0.5625; 8b 0.6875; 14b 0.8125; gate not met | lineage referenced |
| v0.3.14 | structured decoding | Reader frame coverage 0.8333; gate FAIL | lineage referenced |
| v0.3.15 | semantic-frame router | router/model/Reader coverage 0.95; one Reader reject; gate FAIL | safe development patch/evidence targeted for public artifact |
| v0.3.16 | ambiguity/confidence separation | **ENGINEERING GATE PASS**; eligible for Reader Freeze Review | safe development patch/evidence targeted for public artifact |
| R5 | future blind confirmation | **NOT PART OF THIS ARTIFACT'S RESULTS** | no future blind assets public |

## Interpretation rule

`DEVELOPMENT PASS ≠ BLIND CONFIRMATORY PASS`.

A scientifically exposed corpus is treated as consumed. A later correction may establish engineering progress but cannot retroactively convert a historical FAIL into a PASS.
