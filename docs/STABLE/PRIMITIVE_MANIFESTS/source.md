---
Authority: STABLE
Version: v0
Last Updated: 2025-12-22
---

# Source Primitive Manifest — v0

A source defines what data exists at an evaluation point.
It introduces originating values into the graph without transformation, inference, or side effects.

This is the authoritative contract.

---

## 1. Definition

A Source Primitive is a deterministic data origin that introduces
external or contextual data into the graph as typed values or series.

Sources:
- do not transform data
- do not infer signals
- do not emit events
- do not cause side effects
- are pure providers

A source answers one question:

*"What data is available at this evaluation point?"*

---

## 2. Required Manifest Fields

Every source primitive must declare all of the following.

---

### 2.1 Identity

```yaml
id: string
version: string
kind: source
```

Rules:
- `id` is unique and stable
- `version` is semver or monotonic
- `kind` must be `source`

---

### 2.2 Outputs (Primary Contract)

```yaml
outputs:
  - name: string
    type: series | number | bool | string | context
```

Rules:
- Sources do not take inputs
- All outputs are named and typed
- Outputs may be:
  - continuous (series)
  - point-in-time (number, bool, string)
- Outputs must always be produced when evaluated

---

### 2.3 Parameters (Configuration Only)

```yaml
parameters:
  - name: string
    type: int | number | bool | string | enum
    default: any
    bounds: optional
```

Rules:
- Parameters configure what data is exposed
- Parameters are static presets
- Parameters must be serializable
- Parameters do not change at runtime

Examples:
- identifier
- interval
- lookback window
- field selection

---

### 2.4 Execution Semantics

```yaml
execution:
  cadence: continuous
  deterministic: true
```

Rules:
- Sources are evaluated on every engine tick
- Cadence is always continuous in v0
- Determinism is required

---

### 2.5 State

```yaml
state:
  allowed: false
```

Rules:
- Sources may not hold internal state
- Caching, buffering, or accumulation is forbidden
- Any temporal behavior must be modeled downstream

---

### 2.6 Side Effects

```yaml
side_effects: false
```

Rules:
- Sources may not:
  - write files
  - mutate global state
  - emit events
  - perform actions
- External reads are permitted only through orchestrator-managed adapters

The source primitive itself is declarative, not imperative.

---

## 3. Input Prohibition (Critical)

Source primitives take no inputs.

Hard rule:
- No `inputs` section allowed in v0
- No graph wiring into sources
- All dependencies must be parameters or orchestrator context

This prevents feedback loops and preserves causality.

---

## 4. Orchestrator Contract

The orchestrator guarantees:
- Source outputs are available before compute evaluation
- Values are correctly typed
- Data is aligned to the evaluation clock
- Source execution is deterministic per tick

The orchestrator does not:
- infer missing data
- backfill implicitly
- mutate source outputs

---

## 5. Prohibited Behavior

A source primitive may not:
- Accept inputs
- Emit events
- Perform computation
- Hold state
- Branch on execution mode
- Access external state directly (must use adapter)
- Mutate external systems

Violation invalidates the primitive.

---

## 6. Canonical Source Examples (v0)

**data_series**
- outputs: `value:series`
- parameters: `identifier`, `interval`

**state_value**
- outputs: `value:number`
- parameters: none

**timestamp**
- outputs: `now:number`
- parameters: none

**context_string**
- outputs: `value:string`
- parameters: none

---

## 7. Composition Rule

Sources start the graph.

- Source → Compute
- Source → Trigger (via compute)
- Source → Action (via compute + trigger)

Sources may not consume anything downstream.

---

## 8. Scope

This document defines Source Primitive Manifest v0.

Out of scope:
- event-emitting sources
- multi-identifier fan-out
- streaming adapters
- stateful ingestion
- user-defined IO

Those belong to later versions.

---

## 9. Contract Stability

This contract is STABLE.

Breaking changes require a manifest version bump.

---

## Bottom Line

With Source v0, the ontology is complete:
- Source → origin
- Compute → truth
- Trigger → causality
- Action → agency

Everything else is composition.
