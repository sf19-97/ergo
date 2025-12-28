# Primitive Ontology & Execution — v0 Freeze

This document defines what is frozen in v0, what may be patched, and where each constraint is specified.

Anything listed as frozen requires a v1 to change.

---

## 1. Ontological Primitives (Frozen)

The v0 system has exactly four ontological primitives:

- **Source** — origin of data
- **Compute** — derivation of truth from inputs
- **Trigger** — causality ("when something happens")
- **Action** — agency (external intent)

No additional ontological primitives may be introduced in v0.

📍 Defined in: `ontology.md`

---

## 2. Load-Bearing Invariants (Frozen)

The following invariants are foundational and frozen in v0.

### 2.1 Role Separation

- Sources have no inputs.
- Compute primitives must declare ≥1 input.
- Triggers emit events.
- Actions are terminal and stateless.
- Trigger and Compute share execution semantics; they differ in declared causal role and wiring permissions.

📍 Defined in: `ontology.md`, Compute/Trigger/Action manifest specs

### 2.2 Wiring Rules (v0)

The following wiring rules are authoritative:

```
Source → Compute     : allowed
Source → Trigger     : forbidden (v0)
Compute → Compute    : allowed
Compute → Trigger    : allowed
Compute → Action     : forbidden (must be mediated by Trigger)
Trigger → Trigger    : allowed
Trigger → Action     : allowed
Action → *           : forbidden (terminal)
* → Source           : forbidden
```

Graphs violating these rules are invalid.

📍 Defined in: `ontology.md`

### 2.3 Graph Structure

- Graphs are directed acyclic graphs (DAGs).
- Cycles are forbidden.
- Trigger → Trigger chaining is allowed.
- Trigger cycles are forbidden.

📍 Defined in: `ontology.md`, `execution_model.md`

### 2.4 Execution Model

- Execution occurs in single evaluation passes.
- Each node executes at most once per pass.
- Nodes are evaluated in topological order.
- Primitive kinds do not impose global execution phases.

📍 Defined in: `execution_model.md`

### 2.5 Action Semantics

- All nodes must pass validation before any action executes.
- Actions execute sequentially in topological order inherited from their trigger dependencies.
- If multiple actions are topologically independent, their relative execution order is undefined in v0.
- If an action fails during execution:
  - Subsequent actions in the same pass are aborted.
  - Prior external effects are not reversible.

📍 Defined in: `execution_model.md`

### 2.6 Trigger State

- Triggers may hold internal state.
- Trigger state is deterministic and replayable.
- Trigger state resets only at orchestrator-defined lifecycle boundaries
  (e.g., new backtest run, new live session, new graph instantiation).

📍 Defined in: `execution_model.md`

### 2.7 Determinism

- Given identical inputs and identical internal state, node outputs must be identical.
- External nondeterminism is confined to the adapter boundary.

📍 Defined in: `execution_model.md`, `adapter_contract.md`

### 2.8 Trigger vs Risk Distinction

- Trigger parameters may encode temporal structure only; they govern *when* events propagate.
- Risk parameters govern *whether* actions execute (acceptability of outcomes).
- Trigger operators are blind to downstream action content and consequences.
- Risk operators are not blind to action content.

📍 Defined in: `ontology.md`

---

## 3. Adapter Contract (Load-Bearing)

Adapters form a trust boundary between the external world and the graph.

Adapters must satisfy:

1. **Replay determinism** — Identical captured inputs must produce identical outputs.
2. **Declared semantic shaping** — Any transformation that changes semantic meaning (units, currency, aggregation, interpolation, timezone) must be declared.
3. **Capture support** — Adapters must support input capture sufficient for replay.

Enforcement is trust-based in v0. Violations invalidate Source guarantees.

📍 Defined in: `adapter_contract.md`

---

## 4. Explicitly Out of Scope for v0 (Non-Frozen)

The following are intentionally excluded from v0 and must not be solved by introducing new primitives:

- Multi-pass or iterative execution
- Action outcome feedback into the graph
- Presence-based triggers (Source → Trigger)
- Multi-graph coordination or portfolio-level logic
- Trigger state introspection APIs
- Adapter format standardization (v1 concern)

Any future addition must preserve all frozen invariants above.

---

## 5. What May Change in v0.x (Patchable)

The following may be modified without breaking v0:

- Executor implementation details
- Validation bugs
- Error messages and diagnostics
- Performance optimizations
- Non-normative documentation

No v0.x change may:

- Add or remove primitives
- Change wiring rules
- Alter execution semantics
- Weaken determinism guarantees

---

## 6. Versioning Rule

If a change requires violating any frozen item in this document, it is a v1 change.

---
## 7. Authoring Layer (Not Frozen)

The authoring layer (clusters, macros, fractal composition) is explicitly outside the v0 freeze.
It may evolve without triggering a v1.
7.1 What the Authoring Layer Includes

Cluster definitions and boundaries
Fractal composition (arbitrary nesting)
Parameter binding and exposure
Cluster versioning and reuse
Signature inference algorithms

These are specified in AUTHORING_LAYER.md and CLUSTER_SPEC.md.
7.2 The Frozen Invariant
The authoring layer must satisfy one invariant:

All authoring constructs compile away before execution.
The runtime sees only the four primitives and their wiring rules.

This invariant is frozen. The authoring layer mechanics are not.
7.3 What This Means

Cluster format may change without breaking v0
Signature inference may be refined without breaking v0
New authoring features may be added without breaking v0

As long as the expanded graph:

Contains only Source, Compute, Trigger, Action
Obeys the frozen wiring rules
Executes per the frozen execution model

📍 Defined in: AUTHORING_LAYER.md, CLUSTER_SPEC.md

## Status

**v0 frozen**

## Authority

This document, together with ontology.md, execution_model.md, adapter_contract.md,
AUTHORING_LAYER.md, and CLUSTER_SPEC.md, is the canonical reference for system behavior.