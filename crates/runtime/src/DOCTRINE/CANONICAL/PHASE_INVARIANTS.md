# Phase Invariants — v0

This document defines the invariants that must hold at each phase boundary in the system. It is the authoritative reference for what is true, where that truth is enforced, and what happens if it is violated.

**An invariant without an enforcement locus is not an invariant. It is a wish.**

---

## Preamble

### Purpose

This document serves as:
- The constitution of the system's correctness guarantees
- An audit baseline for code review
- A gap-detection tool for implementation work
- A portable reference for future contributors

### Enforcement Locus Definitions

| Locus | Meaning | Strength |
|-------|---------|----------|
| **Spec** | Documented in frozen/stable specification | Declarative only — requires other loci for enforcement |
| **Type** | Impossible to violate due to Rust type system | Strongest — compile-time guarantee |
| **Assertion** | Enforced via `assert!` / `debug_assert!` / panic | Strong — fails loudly at runtime |
| **Validation** | Enforced by validation logic returning `Result::Err` | Strong — recoverable, explicit |
| **Test** | Enforced by test coverage | Weakest — detects regression, does not prevent |

**Rule:** Every invariant must have at least one enforcement locus beyond **Spec**. Spec alone is insufficient.

### Source Documents

This checklist draws from:
- `ontology.md` (frozen)
- `execution_model.md` (frozen)
- `V0_FREEZE.md` (frozen)
- `adapter_contract.md` (frozen)
- `AUTHORING_LAYER.md` (stable)
- `CLUSTER_SPEC.md` (stable)

---

## Core v0.1 Freeze Declaration

**Effective:** 2025-12-22

Core is frozen at this point. The following constraints are now in force:

1. **No new core implementations** without a vertical proof demonstrating necessity
2. **Any core change** must introduce a new invariant with explicit enforcement locus
3. **Action implementations in core = zero** by design; capability atoms live in verticals

This freeze applies to:
- `src/source/`
- `src/compute/`
- `src/trigger/`
- `src/action/`
- `src/cluster.rs`
- `src/runtime/`

Doctrine documents (FROZEN/, STABLE/, CANONICAL/) retain their existing authority levels.

**To unfreeze:** Requires joint escalation to Sebastian with justification referencing a specific vertical that cannot function without the change.

---

## 0. Cross-Phase Invariants

These invariants hold across all phases. Violation at any point is a system-level failure.

| ID | Invariant | Spec | Type | Assertion | Validation | Test |
|----|-----------|:----:|:----:|:---------:|:----------:|:----:|
| X.1 | Exactly four ontological primitives exist: Source, Compute, Trigger, Action | ontology.md §2 | `PrimitiveKind` enum | — | — | — |
| X.2 | Wiring matrix is never violated (see ontology.md §3) | ontology.md §3 | — | — | ✓ | ✓ |
| X.3 | All graphs are directed acyclic graphs (DAGs) | execution_model.md §2 | — | — | ✓ | ✓ |
| X.4 | Determinism: identical inputs + identical state → identical outputs | execution_model.md §8 | — | — | — | ✓ |
| X.5 | Actions are terminal; Action → * is forbidden | ontology.md §3 | — | — | ✓ | ✓ |
| X.6 | Sources have no inputs | ontology.md §2.1 | — | — | ✓ | ✓ |
| X.7 | Compute primitives have ≥1 input | ontology.md §2.2 | — | — | ✓ | ✓ |
| X.8 | Triggers emit events | ontology.md §2.3 | — | — | ✓ | ✓ |
| X.9 | Authoring constructs compile away before execution | V0_FREEZE.md §7 | — | ✓ | — | ✓ |

### Notes

- **X.1:** Enforced by type system. `PrimitiveKind` enum has exactly four variants.
- **X.4:** Determinism is tested but not structurally enforced. Acceptable for v0.
- **X.7:** ✅ **CLOSED.** Enforced in `PrimitiveRegistry::validate_manifest()` at `compute/registry.rs`. Test: `compute_with_zero_inputs_rejected`.
- **X.9:** Requires assertion at execution entry that no `ClusterDefinition` or `NodeKind::Cluster` survives.

---

## 1. Definition Phase

**Scope:** When a cluster is authored and saved.

**Entry invariants:** None (this is the origin point for authoring).

### Exit Invariants

| ID | Invariant | Spec | Type | Assertion | Validation | Test |
|----|-----------|:----:|:----:|:---------:|:----------:|:----:|
| D.1 | Cluster contains ≥1 node | CLUSTER_SPEC.md §6.1 | — | — | ✓ | ✓ |
| D.2 | All edges reference existing nodes and ports | CLUSTER_SPEC.md §6.1 | — | — | ✓ | ✓ |
| D.3 | All edges satisfy wiring matrix | CLUSTER_SPEC.md §6.1 | — | — | ✓ | ✓ |
| D.4 | Every output port references a valid internal node output | CLUSTER_SPEC.md §6.1 | — | — | ✓ | ✓ |
| D.5 | Every input port has a unique name | CLUSTER_SPEC.md §6.1 | — | — | ✓ | ✓ |
| D.6 | Every output port has a unique name | CLUSTER_SPEC.md §6.1 | — | — | ✓ | ✓ |
| D.7 | All parameters have valid types | CLUSTER_SPEC.md §6.1 | — | — | ✓ | ✓ |
| D.8 | Parameter defaults are type-compatible | CLUSTER_SPEC.md §6.1 | — | — | ✓ | ✓ |
| D.9 | No duplicate parameter names | CLUSTER_SPEC.md §6.1 | — | — | ✓ | ✓ |
| D.10 | If declared signature exists, it is compatible with inferred | CLUSTER_SPEC.md §4 | — | — | ✓ | ✓ |
| D.11 | Declared wireability cannot exceed inferred wireability | CLUSTER_SPEC.md §4, §6 | — | — | ✓ | ✓ |

### Notes

- **D.11:** Recently added as editorial hardening. Requires explicit test coverage.

---

## 2. Instantiation Phase

**Scope:** When a cluster is placed in a parent context.

**Entry invariants:**
- Parent context exists and is valid
- Cluster definition passes Definition phase validation

### Exit Invariants

| ID | Invariant | Spec | Type | Assertion | Validation | Test |
|----|-----------|:----:|:----:|:---------:|:----------:|:----:|
| I.1 | Wiring from parent edge source to cluster boundary kind is legal | CLUSTER_SPEC.md §6.2 | — | — | ✓ | ✓ |
| I.2 | Port types match at connection points | CLUSTER_SPEC.md §6.2 | — | — | ✓ | ✓ |
| I.3 | All required parameters are either bound or exposed | CLUSTER_SPEC.md §6.2 | — | — | ✓ | ✓ |
| I.4 | Bound parameter values are type-compatible | CLUSTER_SPEC.md §6.2 | — | — | ✓ | ✓ |
| I.5 | Exposed parameters reference parameters that exist in parent context | CLUSTER_SPEC.md §6.2 | — | — | ✓ | ✓ |
| I.6 | Version constraints are satisfied | CLUSTER_SPEC.md §6.2 | — | — | ✓ | — |

### Notes

- **I.6:** Version constraint validation exists but lacks dedicated test coverage.

---

## 3. Expansion Phase

**Scope:** Recursive flattening of clusters to primitives.

**Entry invariants:**
- All referenced clusters are loadable
- All parameters are concretely bound (no unresolved `Exposed` bindings at root)

### Exit Invariants

| ID | Invariant | Spec | Type | Assertion | Validation | Test |
|----|-----------|:----:|:----:|:---------:|:----------:|:----:|
| E.1 | Output contains only primitives (no `NodeKind::Cluster` survives) | CLUSTER_SPEC.md §7 | — | ✓ | — | ✓ |
| E.2 | All placeholder edges are rewritten to node-to-node edges | CLUSTER_SPEC.md §7 | — | ✓ | — | ✓ |
| E.3 | `ExternalInput` does not appear as edge target (sink) | (inferred) | — | ✓ | — | — |
| E.4 | Authoring path is preserved for each expanded node | CLUSTER_SPEC.md §7.2 | — | — | — | ✓ |
| E.5 | Empty clusters are rejected | CLUSTER_SPEC.md §6.1 | — | — | ✓ | ✓ |
| E.6 | Original cluster definitions are not mutated | (inferred) | — | — | — | — |
| E.7 | `ExpandedGraph` carries boundary ports for inference only | (inferred) | — | — | — | — |

### Notes

- **E.3:** Requires assertion. Silent assumption is unacceptable.
- **E.6:** True by clone semantics but not explicitly enforced.
- **E.7:** Requires doc comment on `ExpandedGraph` to make contract explicit:

```rust
/// Expansion output. Contains only topology, primitive identity, and authoring trace.
/// `boundary_inputs` and `boundary_outputs` are retained for signature inference only
/// and must not influence runtime execution.
```

---

## 4. Inference Phase

**Scope:** Deriving signature from expanded graph.

**Entry invariants:**
- Expanded graph is complete (E.1–E.5 hold)
- `PrimitiveCatalog` is canonical and version-consistent

### Exit Invariants

| ID | Invariant | Spec | Type | Assertion | Validation | Test |
|----|-----------|:----:|:----:|:---------:|:----------:|:----:|
| F.1 | Input ports are never wireable | CLUSTER_SPEC.md §3.2 | — | ✓ | — | ✓ |
| F.2 | Output wireability is determined by source node kind (Action → non-wireable) | CLUSTER_SPEC.md §3.2 | — | — | — | ✓ |
| F.3 | `BoundaryKind` inference follows precedence: ActionLike → SourceLike → TriggerLike → ComputeLike | CLUSTER_SPEC.md §3.4 | — | — | — | ✓ |
| F.4 | `has_side_effects` is true iff any expanded node is Action | CLUSTER_SPEC.md §3.3 | — | — | — | ✓ |
| F.5 | `is_origin` is true iff no inputs AND all roots are Sources | CLUSTER_SPEC.md §3.3 | — | — | — | ✓ |
| F.6 | Signature inference depends only on expanded graph + catalog (no other state) | (inferred) | — | — | — | — |

### Notes

- **F.1:** ✅ **CLOSED.** Fixed in cluster.rs. Enforcement:
  - Assertion: `debug_assert!` at cluster.rs:258
  - Test: `input_ports_are_never_wireable` at cluster.rs:1106
  - Merged.

- **F.6:** True by construction. Document on `infer_signature`:

```rust
/// Signature inference assumes a canonical, version-consistent PrimitiveCatalog.
/// Providing a catalog with different or incomplete primitive metadata will produce
/// undefined or incorrect signatures.
```

---

## 5. Validation Phase

**Scope:** Validating the unified DAG before execution.

**Entry invariants:**
- Graph is fully expanded (no clusters remain)
- Signature inference is complete

### Exit Invariants

| ID | Invariant | Spec | Type | Assertion | Validation | Test |
|----|-----------|:----:|:----:|:---------:|:----------:|:----:|
| V.1 | No cycles exist in the graph | execution_model.md §2 | — | — | ✓ | ✓ |
| V.2 | All edges satisfy wiring matrix | ontology.md §3 | — | — | ✓ | ✓ |
| V.3 | All required inputs are connected | execution_model.md §2 | — | — | ✓ | ✓ |
| V.4 | All type constraints are satisfied at edges | CLUSTER_SPEC.md §6.3 | — | — | ✓ | ✓ |
| V.5 | All action nodes are gated by trigger events | ontology.md §3 | — | — | ✓ | ✓ |
| V.6 | All nodes pass validation before any action executes | execution_model.md §7 | — | — | ✓ | ✓ |

### Notes

- Validation phase is well-covered by existing executor tests.

---

## 6. Execution Phase

**Scope:** Running the validated graph.

**Entry invariants:**
- All V.* invariants hold
- State is initialized per lifecycle rules

### Exit Invariants

| ID | Invariant | Spec | Type | Assertion | Validation | Test |
|----|-----------|:----:|:----:|:---------:|:----------:|:----:|
| R.1 | Each node executes at most once per pass | execution_model.md §1 | — | — | — | ✓ |
| R.2 | Nodes execute in topological order | execution_model.md §3 | — | — | — | ✓ |
| R.3 | No node observes effects from actions in same pass | execution_model.md §1 | — | — | ✓ | ✓ |
| R.4 | Action failure aborts subsequent actions in same pass | execution_model.md §7 | — | — | — | ✓ |
| R.5 | Trigger state resets only at lifecycle boundaries | execution_model.md §5 | — | — | — | — |
| R.6 | Outputs are deterministic given inputs + state | execution_model.md §8 | — | — | — | ✓ |

### Notes

- **R.3:** ✅ **CLOSED.** Compositionally enforced by existing invariants:
  - F.2: Action outputs are non-wireable (`cluster.rs:324: wireable = meta.kind != PrimitiveKind::Action`)
  - X.5: "Actions are terminal; Action → * is forbidden" (validated at D.3, V.2)
  - Since no edge can originate from an Action, no node can observe action effects.
  - No separate test needed — enforcement is structural via wiring matrix validation.
- **R.4:** ✅ **CLOSED (by design).** `Result::Err` propagation via `?` is sufficient. `ActionOutcome::Failed` is data, not control flow — structural halt must be expressed via Trigger gating/wiring, not implicit runtime payload semantics.
- **R.5:** Lifecycle boundaries are orchestrator-defined. Enforcement is outside current scope.

---

# Appendix A: Gap Summary

| ID | Invariant | Issue | Priority | Status |
|----|-----------|-------|----------|--------|
| ~~F.1~~ | ~~Input ports never wireable~~ | ~~Code violation~~ | ~~BLOCKER~~ | ✅ CLOSED |
| ~~E.3~~ | ~~ExternalInput not as sink~~ | ~~No assertion~~ | ~~HIGH~~ | ✅ CLOSED |
| ~~E.7~~ | ~~Boundary ports for inference only~~ | ~~No doc comment~~ | ~~MEDIUM~~ | ✅ CLOSED |
| ~~D.11~~ | ~~Declared wireability ≤ inferred~~ | ~~Validation missing~~ | ~~MEDIUM~~ | ✅ CLOSED |
| ~~X.9~~ | ~~Authoring compiles away~~ | ~~Structurally enforced — type system~~ | ~~MEDIUM~~ | ✅ CLOSED |
| ~~F.6~~ | ~~Inference depends only on graph + catalog~~ | ~~Documented~~ | ~~LOW~~ | ✅ CLOSED |
| ~~R.3~~ | ~~No same-pass action observation~~ | ~~Compositionally enforced via F.2, X.5~~ | ~~LOW~~ | ✅ CLOSED |
| ~~X.7~~ | ~~Compute inputs ≥1~~ | ~~Validation missing~~ | ~~HIGH~~ | ✅ CLOSED |
| ~~R.4~~ | ~~Action failure aborts subsequent actions~~ | ~~Closed by design — Result::Err propagation~~ | ~~LOW~~ | ✅ CLOSED |

---

## Appendix B: Code Review Protocol

When reviewing any PR, ask:

1. **Which invariants does this code touch?**
2. **For each touched invariant, is enforcement preserved or strengthened?**
3. **Does this PR introduce any new implicit assumptions?**
4. **If an invariant is weakened, is the weakening explicitly documented and justified?**

A PR that cannot answer these questions is incomplete.

---

## Authority

This document is canonical for v0.

It joins the frozen doctrine set:
- `ontology.md`
- `execution_model.md`
- `V0_FREEZE.md`
- `adapter_contract.md`

And the stable specification set:
- `AUTHORING_LAYER.md`
- `CLUSTER_SPEC.md`

Changes to this document require the same review bar as changes to frozen specs.

---

## Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| v0.1 | 2025-01-XX | Claude (Structural Auditor) | Initial draft |
| v0.2 | 2025-01-XX | Claude Prime | F.1 closed — merged to cluster.rs |
| v0.3 | 2025-12-21 | Claude Code | E.3, E.7, D.11 closed; D.11 validation added; gap summary corrected |
| v0.4 | 2025-12-21 | Claude Code | X.9 closed — structurally enforced by type system |
| v0.5 | 2025-12-21 | Claude Code | F.6 closed — documented on infer_signature |
| v0.6 | 2025-12-21 | Claude Code | R.3 closed — compositionally enforced via F.2, X.5 |
| v0.7 | 2025-12-22 | Claude Code | X.7 closed — validation added to compute/registry.rs; R.4 closed by design |
| v0.8 | 2025-12-22 | Claude Prime | Core v0.1 freeze declared |