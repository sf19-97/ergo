# Execution Model — v0

This document defines the minimal execution semantics required to implement a correct executor
for the v0 primitive ontology. It is not an ontology document and does not introduce new concepts.

The execution model is intentionally simple: single-pass, deterministic, acyclic evaluation.

---

## 1. Evaluation Pass

An **evaluation pass** is a single, discrete execution of a graph over a snapshot of inputs.

- A pass has a clear start and end.
- All nodes are evaluated at most once per pass.
- No node may observe effects produced by actions within the same pass.

Evaluation cadence (when passes occur) is defined by the orchestration layer and is out of scope
for this document.

Trigger semantics assume a discrete evaluation model; evaluation cadence is defined by the
execution environment.

---

## 2. Graph Structure

Graphs are directed acyclic graphs (DAGs).

- Cycles are forbidden.
- All dependencies must be statically resolvable before execution.
- Topological ordering defines evaluation order.

Graphs may contain nodes of different primitive kinds but must respect wiring rules defined
in ontology.md.

---

## 3. Node Evaluation Order

Nodes are evaluated in topological order, respecting declared dependencies and wiring rules.

Primitive kinds do not impose global execution phases; ordering is dependency-driven.

Source, Compute, Trigger, and Action represent causal roles, not execution phases. Compute and
Trigger may interleave as long as dependencies are satisfied.

---

## 4. Values and Events

- Events are values with restricted wiring rules.
- Events propagate through the graph like other outputs.
- No event queue or multi-pass propagation exists in v0.

Trigger nodes consume values and/or events and emit events.
Actions consume events and do not emit graph-propagated values.

The specialness of events is in the type system and wiring rules, not the execution model.

---

## 5. Trigger Execution Semantics

- Trigger nodes may hold internal state.
- Trigger state is scoped to the executor lifecycle.
- Trigger state resets only at orchestrator-defined lifecycle boundaries
  (e.g., new backtest run, new live session, new graph instantiation).

Trigger chaining (Trigger → Trigger) is allowed.
Trigger cycles are forbidden.

Triggers operate purely on inputs available within the evaluation pass and stored internal state.

Trigger state must be replay-reconstructible from inputs and lifecycle resets.
Live introspection of trigger state is out of scope for v0.

---

## 6. Action Execution Semantics

- Actions are terminal nodes in the graph.
- Actions are executed at most once per evaluation pass.
- Actions must be stateless.
- Actions emit an acknowledgment record to the orchestrator (non-causal, for logging/audit).

Action effects are external and must not influence other nodes during the same pass.

Action acknowledgment records do not participate in graph causality. They are metadata for
accountability, not events for propagation.

---

## 7. Action Execution Order

All nodes must pass validation before any action executes.

Actions execute sequentially in topological order inherited from their trigger dependencies.

If multiple actions are topologically independent (no ordering relationship between their
upstream triggers), their relative execution order is undefined in v0.

If an action fails during execution:
- Subsequent actions in the same evaluation pass are aborted.
- Prior external effects are not reversible.
- No retry or compensation occurs within the same pass.
- Error handling and retries are orchestrator concerns.

---

## 8. Determinism

Within an evaluation pass:

- Node evaluation must be deterministic.
- Given identical inputs and identical internal state, outputs must be identical.
- No internal randomness is permitted.
- No hidden mutable state is permitted.

External nondeterminism is handled at the adapter boundary.

For Triggers: determinism means identical behavior given identical inputs and lifecycle state.
For Actions: determinism means identical command emission given identical trigger events and parameters.

---

## 9. Out of Scope

This document does not specify:

- Evaluation cadence
- Multi-pass execution
- Feedback loops
- Multi-graph coordination
- Action outcome feedback into the graph
- Trigger state introspection APIs

These are explicitly deferred beyond v0.

Multi-phase orchestration (e.g., "wait for fill then place second leg") is handled via
environment-state observation through Sources in subsequent evaluation passes, not via
direct cyclic wiring.
