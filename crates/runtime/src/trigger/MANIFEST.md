Trigger Primitive Manifest — v0

1. Definition

A Trigger Primitive is a deterministic event extractor that converts
typed inputs (usually series or booleans) into discrete events.

Triggers:
	•	do not perform actions
	•	do not manage external state
	•	do not execute side effects
	•	exist solely to detect when something happens

A trigger answers one question:

“Did an event occur at this evaluation point?”

⸻

2. Required Manifest Fields

Every trigger primitive must declare all of the following.

⸻

2.1 Identity

id: string
version: string
kind: trigger

Rules:
	•	id is unique and stable
	•	version is semver or monotonic
	•	kind must be trigger

⸻

2.2 Inputs

inputs:
  - name: string
    type: series | number | bool
    required: true
    cardinality: single | multiple

Rules:
	•	Inputs are explicit, named, and typed
	•	No implicit inputs (time, identifier, state must be upstream)
	•	Triggers do not read execution context
	•	Inputs may be continuous values or booleans

⸻

2.3 Outputs

outputs:
  - name: event
    type: event

Rules:
	•	Triggers always emit events
	•	Output type is always event
	•	Events are discrete (occur or not)
	•	No additional outputs allowed in v0

⸻

2.4 Parameters

parameters:
  - name: string
    type: int | number | bool | string | enum
    default: any
    bounds: optional

Rules:
	•	Parameters are static presets
	•	Parameters must be serializable
	•	Parameters must be fully declared
	•	No runtime mutation allowed

⸻

2.5 Execution Semantics

execution:
  cadence: continuous | event
  deterministic: true

Rules:
	•	continuous = evaluated every bar / tick
	•	event = evaluated only when upstream event occurs
	•	Determinism is required

⸻

2.6 State (Prohibited)

```yaml
state:
  allowed: false  # REQUIRED — triggers are stateless
  description: optional
```

**Triggers are stateless.** The `state.allowed` field must be `false` for all trigger
implementations. The registry will reject any trigger manifest with `allowed: true`.

#### Execution-Local Bookkeeping

Trigger implementations may use ephemeral, execution-local bookkeeping during evaluation
(temporary variables, scratch registers). This is permitted because it:

- Is not observable by the runtime
- Is not serialized or captured
- Is not preserved across evaluations
- Does not participate in causality

Such bookkeeping does not constitute "state" in the system's ontological sense.

#### Temporal Patterns

Behaviors requiring cross-evaluation memory are **not triggers**. They are compositional
patterns (clusters) that must be built from:

| Primitive | Role in Pattern |
|-----------|-----------------|
| Source | Read persisted state from environment |
| Compute | Evaluate policy / transform state |
| Trigger | Emit event based on computed boolean |
| Action | Write updated state to environment |

Examples of temporal patterns (implemented as clusters, not triggers):
- `OnceCluster` — emit only on first occurrence
- `CountCluster` — count events
- `LatchCluster` — set/reset state machine
- `DebounceCluster` — cooldown gating

#### Amendment Record

> **Amended 2025-12-28** by Sebastian (Freeze Authority)
>
> Prior language allowing `state.allowed: true` was a semantic error. Triggers are
> ontologically stateless. This field must always be `false`.

⸻

2.7 Side Effects

side_effects: false

Hard rule:
	•	Triggers may not:
	•	perform I/O
	•	access network
	•	access external state
	•	emit actions
	•	log as behavior

If it touches the world, it is not a trigger.

⸻

3. Event Semantics (Critical)

An event is:
	•	a discrete occurrence
	•	tied to a specific evaluation index
	•	either emitted or not emitted

Rules:
	•	Events do not persist
	•	Events do not carry payloads in v0
	•	Events are consumed downstream by:
	•	actions
	•	other triggers (via event cadence)

⸻

4. Prohibited Behavior

A trigger primitive may not:
	•	Emit multiple event streams
	•	Emit continuous values
	•	Access execution mode
	•	Access external state
	•	Perform actions
	•	Mutate global state
	•	Accept undeclared inputs or parameters
	•	Emit undeclared outputs

Violation invalidates the primitive.

⸻

5. Composition Rule

Triggers sit between compute and action.

	•	Compute → Trigger → Action
	•	Triggers may consume:
	•	compute outputs
	•	boolean series
	•	upstream events
	•	Triggers may not:
	•	execute side effects
	•	modify state outside themselves

Composition occurs only via the graph.

⸻

6. Orchestrator Contract

The orchestrator guarantees:
	•	Inputs match declared types
	•	Cadence is respected
	•	State is reset deterministically
	•	Events are emitted exactly once per evaluation

The orchestrator does not:
	•	interpret meaning
	•	debounce implicitly
	•	infer intent
	•	tolerate non-determinism

⸻

7. Canonical Examples (Mental Model)

> **Note:** Examples marked with state requirements beyond `state: false` must be
> implemented as clusters, not primitive triggers. See §2.6 for temporal pattern guidance.

gt (greater-than)
	•	inputs: a:number, b:number
	•	outputs: event
	•	state: false
	•	cadence: continuous
	•	emits event when a > b at evaluation point

crossover
	•	inputs: fast:series, slow:series
	•	outputs: event
	•	state: false
	•	cadence: continuous
	•	emits event on crossing boundary
	•	*(Stateful crossover detection requires cluster implementation; see §2.6 Temporal Patterns)*

once
	•	inputs: event
	•	outputs: event
	•	state: false
	•	cadence: event
	•	emits only first occurrence
	•	*(Once-only emission requires cluster implementation; see §2.6 Temporal Patterns)*

⸻

8. Scope

This document defines Trigger Primitive Manifest v0.
	•	Payload-carrying events are out of scope
	•	Multi-output triggers are out of scope
	•	Cross-identifier triggers are out of scope

Those belong to later versions.

⸻

9. Freeze Point

This contract is intentionally minimal.

Do not expand it yet.
Do not generalize it yet.
Do not weaken enforcement.

⸻

Bottom line

Compute gave you truth.
Triggers give you causality.

With this locked:
	•	actions become trivial
	•	strategies become closures over events
	•	the system becomes fully temporal and deterministic
