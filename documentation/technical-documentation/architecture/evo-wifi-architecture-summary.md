# evo-wifi Architecture Summary

## Purpose

This document summarizes the architecture that evo-wifi demonstrates in practice.

The project exists to prove a borrowed-first architecture built on clear responsibilities and minimal ownership.

## Main Idea

The project is organized around these concepts:

- `agents`
- `pipelines` as the way an agent works
- `resolvers`
- `tools`
- `contracts`
- `providers`

And around these domain forms:

- `specifications`
- `borrowed`
- `entities`

The important refinement for evo-wifi is this:

- `pipeline` is not a peer layer competing with `agent`
- `pipeline` is the working form of the `agent`
- in the current codebase, that pipeline is expressed through the agent call chain, not through a dedicated `pipelines/` module

## Runtime Shape

At runtime, the project works like this:

```text
main.rs -> commands -> agents -> pipeline -> resolvers -> contracts -> providers -> external world
```

That is the actual implementation route used by the CLI.

### What Each Piece Does

- `main.rs` selects the command from CLI arguments.
- `commands` act as the CLI façade.
- `agents` coordinate the use case.
- the `pipeline` is the way the agent works.
- `resolvers` decide whether borrowed data can become operable.
- `contracts` define the expected behavior.
- `providers` implement that behavior against the real world.
- the external world is terminal input/output, WiFi, and the operating system.

## Borrowed-First Data Flow

Input and output are treated as external resources first.

The system prefers to work on borrowed forms such as:

- `WifiInterfaceBorrowed`
- `WifiConnectionStatusBorrowed`
- `WifiNetworkBorrowed`
- `WifiSavedNetworkBorrowed`
- `WifiPasswordInputBorrowed`

This keeps ownership where it naturally belongs and avoids unnecessary wrappers or duplicated payloads.

## Domain Model

The domain is intentionally lightweight.

It is divided into:

- `specifications/` for minimal acceptance or operability rules
- `borrowed/` for non-owning data views
- `entities/` only when real ownership is required

This keeps the model close to the actual data flow and prevents artificial materialization.

## Architecture Meaning

This project is not proving a component diagram alone.

The component diagram shows modules and dependencies.
The architecture summary shows how the system is organized semantically and operationally.

The key architectural rules are:

- the agent coordinates
- the pipeline is how the agent works
- the resolver decides
- the tool transforms
- the contract states the expected behavior
- the provider implements the contract
- borrowed forms are preferred when they are enough

## Practical Result

This architecture allows evo-wifi to:

- keep the terminal as a working interface
- preserve clear responsibilities
- avoid unnecessary intermediate packages
- test the domain without real providers
- replace providers without rewriting the core flow

## What the Architecture Diagram Must Show

The project architecture diagram should make visible:

- CLI entry point
- command façade
- agent coordination
- agent working form (`pipeline`)
- resolver chain
- contract boundary
- provider implementation
- borrowed domain forms
- external world

That is the architecture the project is trying to prove.
