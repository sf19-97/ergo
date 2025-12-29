---
Authority: CANONICAL
Version: v0
Last Updated: 2025-12-22
Owner: Claude (Structural Auditor)
---

# Terminology — v0

This document defines canonical terminology for the Primitive Library.

---

## 1. Ontological Primitives (The Four Roles)

The term **"primitive"** refers exclusively to the four ontological roles:

| Primitive | Causal Role | Definition |
|-----------|-------------|------------|
| **Source** | Origin | Introduces data into the graph |
| **Compute** | Truth | Transforms values deterministically |
| **Trigger** | Causality | Converts continuous values into discrete events |
| **Action** | Agency | Attempts to affect the external world |

These are frozen. No additional primitives may be introduced in v0.

**Usage:** "Source is a primitive." "The four primitives are..."

**Do not use "primitive" to mean:** a concrete implementation, a node, a behavior, or an operator.

---

## 2. Implementations (Concrete Nodes)

The term **"implementation"** refers to a concrete, executable unit within an ontological role.

| Role | Example Implementations |
|------|------------------------|
| Source | `price_series`, `account_equity`, `timestamp` |
| Compute | `add`, `multiply`, `sma`, `ema` |
| Trigger | `gt`, `crossover`, `once`, `debounce` |
| Action | `submit_order`, `cancel_order` |

**Usage:** "The `add` implementation." "Register a new Compute implementation."

**Code symbol:** `ImplementationInstance` (not `PrimitiveInstance`)

**Directory:** `src/compute/implementations/` (not `src/compute/primitives/`)

---

## 3. Macro-Primitives (Authoring Conveniences)

The term **"macro-primitive"** refers to authoring-layer constructs that:
- Are composed from ontological primitives
- Exist for ergonomics and intent expression
- Compile away before execution
- Add no new runtime semantics

**Examples:**
- Risk policies
- Constraints and guards
- Named compositions
- Reusable templates

**Usage:** "Risk is a macro-primitive, not an ontological primitive."

Macro-primitives are expressed as **clusters** in the authoring layer.

---

## 4. Clusters (Composed Structures)

The term **"cluster"** refers to a named, bounded subgraph that:
- Contains implementations and/or other clusters
- Exposes boundary ports
- Compiles away before execution

**Usage:** "Define a cluster for the entry logic." "Clusters are expanded during compilation."

Clusters are the primary abstraction for modularity and reuse in the authoring layer.

---

## 5. Operators (Alternative Term)

The term **"operator"** may be used interchangeably with **"implementation"** when emphasizing the algebraic or functional nature of a node.

**Usage:** "The `add` operator." "Compute operators are pure functions."

This term is optional. "Implementation" is preferred in technical documentation.

---

## 6. Disambiguation Table

| Term | Means | Does NOT Mean |
|------|-------|---------------|
| Primitive | One of the four ontological roles | A concrete implementation |
| Implementation | A concrete executable node | An ontological role |
| Macro-primitive | An authoring convenience that compiles away | A runtime node |
| Cluster | A composed structure in the authoring layer | A runtime concept |
| Operator | Same as implementation (alternative term) | — |

---

## 7. Code Symbol Mapping

| Old Symbol | New Symbol | Reason |
|------------|------------|--------|
| `PrimitiveInstance` | `ImplementationInstance` | Clarity |
| `NodeKind::Primitive` | `NodeKind::Impl` | Clarity |
| `primitive_id` | `impl_id` | Clarity |
| `src/*/primitives/` | `src/*/implementations/` | Clarity |

Migration of code symbols is tracked in PHASE_INVARIANTS.md.

---

## 8. Usage Examples

**Correct:**
- "Source, Compute, Trigger, and Action are the four primitives."
- "The `add` implementation performs numeric addition."
- "Risk is a macro-primitive expressed as a cluster."
- "Clusters compile away before execution."

**Incorrect:**
- "The `add` primitive..." (should be "implementation")
- "Register a new primitive..." (should be "implementation" unless referring to a new ontological role, which is forbidden)
- "The primitive folder contains..." (should be "implementations folder")

---

## Authority

This document is canonical for terminology.

It is subordinate to frozen specs (ontology.md, execution_model.md, V0_FREEZE.md, adapter_contract.md).

Changes to this document require review but not v1.
