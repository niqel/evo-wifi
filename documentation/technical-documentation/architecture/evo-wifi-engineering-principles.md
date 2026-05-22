# Engineering Principles

## Purpose

This document defines reusable design principles for all projects.

It does not describe a specific technology or framework. It describes a way of thinking about, naming, and structuring software.

## Base Principle

Every project must be built on SOLID principles.

Of all of them, the Single Responsibility Principle is indispensable:

- each module must have one dominant responsibility
- each function must express one clear action
- each type must represent a comprehensible domain entity
- if a piece tries to do too many things, it should be split

## Single Responsibility

A software piece has single responsibility when its primary reason for change is only one.

This implies:

- a module must not mix domain logic with UI, persistence, networking, or external coordination if that is not its main function
- a function must not hide several independent transformations
- a type must not exist as a generic container of unrelated behaviors

Single responsibility is not only an architecture rule. It is also a language rule.

If the name of a piece does not make clear what it does, its responsibility is probably not well defined.

## Subject Agent

The concept of subject agent is adopted as a design criterion.

In linguistics, an agent noun can be derived from a verb to name the subject that performs an action.

Examples:

- to blend -> blender
- to run -> runner
- to dry -> dryer
- to profess -> professor
- to build -> builder
- to print -> printer
- to generate -> generator

In software, this concept is used to name modules, types, commands, and components as conceptual subjects defined by the dominant action they perform.

Rule:

- if a piece receives a subject-agent name, it must have one clear and defensible primary action

Examples:

- `Generator` generates
- `Printer` prints
- `Heatsink` dissipates
- `VolumeController` controls volume

If a piece cannot justify its name through a dominant action, its design should be revisited.

The subject-agent idea does not force the creation of a `struct`, a class, or a stateful object.

It can be expressed as a module, type, command, script, or component, depending on the real boundary the system needs.

## Relationship Between Object, Action, and Subject Agent

The following semantic relationship is considered useful:

```text
object + action -> subject agent
```

Conceptual examples:

- clothes + wash -> washer
- audio + route -> audio router
- stream + move -> stream mover
- sink + select -> sink selector

This relationship helps design components whose name reveals:

- what it receives
- what it transforms
- what its main function is

In this document, the object in that relationship is first understood as a semantic piece of the domain, not necessarily as an entity owned by the system.

In some projects, that object will be its own entity. In others, it may be a borrowed resource, a view, or a minimal capability defined by a contract or specification.

## Linguistic Inspiration

Part of the inspiration for this approach comes from the use of functional compounds in Persian, where the relation between object and action can form a clear semantic identity for the subject agent.

In this document, that idea is adopted as a conceptual reference to improve clarity in names, modules, and responsibilities in software, not as a strict linguistic rule.

## Principle of Semantic Transparency

System names should move closer to the real domain and not only to technical mechanics.

Preferred:

- names that express function
- small semantically closed modules
- clear verbs for operations
- types that represent domain concepts

Avoid:

- generic names such as `Manager`, `Helper`, `Utils`, `Thing`, `DataProcessor`
- modules that only group code without identity
- structures created only by habit when a module would suffice

## Module as Subject Agent

Not every responsibility needs to be represented as a `struct`.

When there is no own state, a responsibility can live correctly in a module.

The module can act as the semantic subject agent of the domain.

Example:

```rust
mod washer {
    pub struct Garment {
        pub clean: bool,
        pub material: String,
    }

    pub fn wash(garment: &mut Garment) {
        garment.clean = true;
    }
}
```

Semantic reading:

- `Garment` is the entity receiving the transformation
- `wash` is the action
- `washer` is the functional context that gives identity to the act

The existence of the module already establishes a responsibility boundary.

According to the architecture of the project, that object can be represented as its own entity or as a borrowed resource under a minimal form sufficient for the operation.

A `struct Washer` should not be created if there is no state, coordination, or persistent behavior to justify it.

## Design Rule

Before creating a new piece, answer these questions:

1. What is the dominant action?
2. What entity receives the action?
3. Does the subject agent need state or only a semantic context?
4. Does the name clearly reflect its responsibility?
5. Does the piece have only one primary reason to change?

If these questions do not have clear answers, the design is not ready yet.

## Recommended Conventions

- use names with real semantic weight
- prefer small modules over generic containers
- create types only when they add real identity or invariants
- use functions with precise verbs
- avoid empty abstractions
- review whether each name can be explained from the verb that gives it origin

## Review Criterion

A code piece should be reviewed if any of the following happen:

- its name does not explain what it does
- it does more than one main thing
- it mixes responsibilities from different levels
- it uses a `struct` where a module would be enough
- it uses a module where explicit state is actually needed
- the domain cannot be reconstructed by reading names and signatures

## General Application

These principles apply to:

- scripts
- crates
- CLIs
- internal utilities
- automation
- system utilities

## Synthesis

Architecture is not only organized with types and dependencies. It is also organized with grammar.

A good design can be read as a clear relation between:

- entity
- action
- subject agent

When the name, responsibility, and behavior match, the system gains clarity, maintainability, and coherence.
