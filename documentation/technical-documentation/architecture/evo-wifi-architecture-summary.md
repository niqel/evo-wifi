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

The repository is now a Cargo workspace:

- `crates/evo-wifi-core` is the reusable library. It contains `borrowed`, `contracts`, `resolvers`, `agents`, and `commands`.
- `crates/evo-wifi-cli` is the binary runtime. It parses CLI arguments, creates concrete providers, and calls core commands.
- `crates/evo-wifi-nu-plugin` is the Nushell plugin runtime. It exposes Nushell commands and converts provider values to `nu_protocol::Value`.
- `crates/evo-wifi-provider-linux-wpa` implements Linux WPA system providers.
- `crates/evo-wifi-provider-nushell` implements Nushell-facing providers for typed inputs and structured outputs.
- `crates/evo-wifi-provider-terminal` implements terminal input and output providers.

## Runtime Shape

At runtime, the project works like this:

```text
evo-wifi-cli main.rs -> providers + evo-wifi-core commands -> agents -> resolvers -> contracts -> providers -> external world
evo-wifi-nu-plugin -> providers + evo-wifi-core commands -> agents -> resolvers -> contracts -> providers -> Nushell Value
```

These are the actual implementation routes used by the CLI and by the first Nushell plugin commands.

### What Each Piece Does

- `evo-wifi-cli/main.rs` selects the command from CLI arguments.
- `evo-wifi-cli/main.rs` assembles the concrete providers needed by each command.
- `evo-wifi-nu-plugin` exposes `evo-nu-wifi status` and `evo-nu-wifi networks`.
- `evo-wifi-nu-plugin` converts `evo-wifi-provider-nushell` values to `nu_protocol::Value`.
- `commands` act as runtime-agnostic command objects in `evo-wifi-core`.
- `agents` coordinate the use case by chaining resolvers.
- `resolvers` decide whether borrowed data can become operable.
- `contracts` define the expected behavior by role: `inputs`, `outputs`, and `actions`.
- provider crates implement one contract responsibility through `provide`.
- the external world is terminal input/output, Nushell values, WiFi, and the operating system.

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
- provider crates contain concrete implementations such as terminal, Nushell, or Linux WPA.

In evo-wifi, WiFi is the specialty of the workspace and its concrete provider crates, not a separate contracts folder.
Nushell is a presentation/runtime specialty. The provider owns its received strings and structured output state, then lends borrowed values to the core through `provide`. The plugin owns the final `nu_protocol` boundary.

Examples:

```text
wifi_interface_resolver.resolve
LinuxWpaWifiInterfaceProvider.provide
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
- replace individual provider crates without rewriting the core flow

## What the Architecture Diagram Must Show

The project architecture diagram should make visible:

- CLI entry point
- command objects
- agent coordination
- resolver chain
- contract boundary
- external provider crates with single responsibilities
- borrowed domain forms
- external world

That is the architecture the project is trying to prove.
