# evo-wifi Architecture Summary

## Purpose

This document summarizes the architecture that evo-wifi demonstrates in practice.

The project exists to prove a borrowed-first architecture built on clear responsibilities and minimal ownership.

## Main Idea

The project is organized around these concepts:

- `agents`
- `resolvers`
- `contracts`
- `providers`

And around this domain form:

- `borrowed`

## Runtime Shape

At runtime, the project works like this:

```text
main.rs -> commands -> agents -> resolvers -> contracts -> providers -> external world
```

That is the actual implementation route used by the CLI.

### What Each Piece Does

- `main.rs` selects the command from CLI arguments.
- `commands` act as the CLI façade.
- `commands` assemble the concrete providers needed by each command.
- `agents` coordinate the use case by chaining resolvers.
- `resolvers` decide whether borrowed data can become operable.
- `contracts` define the expected behavior by role: `inputs`, `outputs`, and `actions`.
- `providers` implement one contract responsibility through `provide`.
- the external world is terminal input/output, WiFi, and the operating system.

## Semantic Operation Rule

The architecture uses a strict naming and operation rule:

```text
provider.provide
resolver.resolve
agent_subject.primary_action
command.execute
```

Providers provide exactly one contract responsibility.
Resolvers resolve whether the flow can continue.
Agent subjects perform the action named by their module.
Commands execute the selected CLI entry point.

Contracts are not specialized by technology or concrete use case. The specialty lives in the provider:

- `contracts/inputs` declares input or selection contracts.
- `contracts/outputs` declares output contracts.
- `contracts/actions` declares contracts that execute an action against the external world.
- `providers/inputs`, `providers/outputs`, and `providers/actions` contain concrete implementations such as terminal or void.

In evo-wifi, WiFi is the specialty of the whole crate and its concrete providers, not a separate contracts folder.

Examples:

```text
wifi_interface_resolver.resolve
VoidWifiInterfaceProvider.provide
wifi_connection_status_shower.show
wifi_network_switcher.switch
```

This follows the subject-agent rule:

```text
blender.blend
washer.wash
network_switcher.switch
connection_status_shower.show
```

If a name cannot defend its primary action, the piece should be renamed or split.

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

It is represented by:

- `borrowed/` for non-owning data views

This keeps the model close to the actual data flow and prevents artificial materialization.

## Architecture Meaning

This project is not proving a component diagram alone.

The component diagram shows modules and dependencies.
The architecture summary shows how the system is organized semantically and operationally.

The key architectural rules are:

- the agent coordinates
- the resolver decides
- the contract states the expected behavior
- the provider provides one contract responsibility through `provide`
- borrowed forms are preferred when they are enough
- contracts are organized by operational role, not by technology
- providers provide the concrete specialty

## Practical Result

This architecture allows evo-wifi to:

- keep the terminal as a working interface
- preserve clear responsibilities
- avoid unnecessary intermediate packages
- test the domain without real providers
- replace individual providers without rewriting the core flow

## What the Architecture Diagram Must Show

The project architecture diagram should make visible:

- CLI entry point
- command façade
- agent coordination
- resolver chain
- contract boundary
- provider implementations with single responsibilities
- borrowed domain forms
- external world

That is the architecture the project is trying to prove.
