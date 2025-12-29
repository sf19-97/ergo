# Ergo Documentation

This directory is the authoritative home for specifications and doctrine. Authority order:
1. `FROZEN/` — cannot change without explicit escalation.
2. `STABLE/` — stable contracts; additive changes only.
3. `CANONICAL/` — derived checklists and terminology.
4. `CONTRACTS/` — external interfaces and integration contracts.

## Index
- `FROZEN/`
  - `ontology.md`
  - `execution_model.md`
  - `V0_FREEZE.md`
  - `adapter_contract.md`
  - `SUPERVISOR.md`
- `STABLE/`
  - `AUTHORING_LAYER.md`
  - `CLUSTER_SPEC.md`
  - `PRIMITIVE_MANIFESTS/`
    - `source.md`
    - `compute.md`
    - `trigger.md`
    - `action.md`
- `CANONICAL/`
  - `PHASE_INVARIANTS.md`
  - `TERMINOLOGY.md`
- `CONTRACTS/`
  - `UI_RUNTIME_CONTRACT.md`

Crate-local READMEs under `crates/` remain informational for their respective packages.
