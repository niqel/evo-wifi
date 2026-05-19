# Sprint 001: UC-009 View Current Network Status

## Goal

Build the first complete vertical slice of `evo-wifi`: show the current WiFi network status through the architecture.

This sprint validates:

- borrowed data models
- contracts / traits
- providers
- resolvers
- agent subject
- composition root
- terminal execution path

## Source Documents

- Functional story: `documentation/functional-documentation/user-stories/US-009-view-current-network-status.md`
- Technical use case: `documentation/technical-documentation/use-cases/UC-009-view-current-network-status.md`
- Borrowed data model: `documentation/technical-documentation/borrowed-data-models/wifi-borrowed-data-model.dot`
- Component diagram: `documentation/technical-documentation/component-diagrams/wifi-component-diagram-uml.d2`
- Sequence diagram: `documentation/technical-documentation/sequence-diagrams/UC-009-view-current-network-status-sequence.d2`

## Architectural Flow

```text
agent_subjects::wifi_connection_status_shower::show
  -> wifi_interface_resolver::resolve
  -> wifi_connection_status_resolver::resolve
  -> wifi_connection_status_output_resolver::resolve
```

The composition root creates the concrete providers and passes them into the use case flow through contracts.

The agent subject must not instantiate providers.

## Tasks

### CT-UC-009-001: Create Rust Project Structure

**Type:** setup

**Purpose:** Create the initial Rust crate layout for the architecture.

**Work:**

- Create `Cargo.toml`.
- Create `src/main.rs`.
- Create module folders for:
  - `agent_subjects`
  - `resolvers`
  - `contracts`
  - `providers`
  - `borrowed_data`
  - `composition`

**Done when:**

- `cargo check` runs.
- The module tree compiles without implementation logic.

### CT-UC-009-002: Define Borrowed Data Models

**Type:** model

**Purpose:** Represent borrowed views used by UC-009 without creating unnecessary packages.

**Work:**

- Define `WifiInterfaceView<'a>`.
- Define `WifiConnectionStatusView<'a>`.
- Keep all fields as borrowed string slices.

**Expected structures:**

```rust
pub struct WifiInterfaceView<'a> {
    pub name: &'a str,
}

pub struct WifiConnectionStatusView<'a> {
    pub ssid: &'a str,
    pub status: &'a str,
}
```

**Done when:**

- The borrowed data models compile.
- No enum is introduced for status.
- No owned `String` is introduced in these views.

### CT-UC-009-003: Define System WiFi Contracts

**Type:** contract

**Purpose:** Define the system-side contracts required by the resolvers.

**Work:**

- Define `WifiInterfaceSystemWifiContract`.
- Define `WifiStatusSystemWifiContract`.
- Expose behavior needed to resolve:
  - WiFi interface
  - current WiFi status from an interface
- Return borrowed views or a small error/result type.

**Done when:**

- Resolvers can depend on traits instead of a concrete provider.
- The contracts do not mention Void-specific commands in their public APIs.

### CT-UC-009-004: Define Presentation Render Contract

**Type:** contract

**Purpose:** Define the presentation-side output contract for rendering the current WiFi status.

**Work:**

- Define `WifiStatusRenderPresentationContract`.
- Add behavior to render:
  - connected status
  - disconnected status
  - missing interface message
  - unresolved status message

**Done when:**

- The output resolver can depend on the trait instead of a concrete terminal implementation.
- The contract receives borrowed data and does not own provider data unnecessarily.

### CT-UC-009-005: Implement Void System WiFi Provider

**Type:** provider

**Purpose:** Implement system WiFi access for Void Linux using existing base system tools.

**Work:**

- Create `VoidSystemWifiProvider`.
- Implement `WifiStatusSystemWifiContract`.
- Use `wpa_cli interface_list` to resolve the WiFi interface.
- Use `wpa_cli -i <iface> status` to resolve current status.
- Avoid adding third-party crates at this stage.

**Done when:**

- Provider compiles behind the contract.
- Provider does not leak command parsing details into resolvers.
- Provider returns borrowed views backed by data it owns for the call lifetime, or exposes a safe temporary snapshot strategy if borrowing directly is not possible.

### CT-UC-009-006: Implement Terminal Presentation Provider

**Type:** provider

**Purpose:** Render UC-009 output in the terminal.

**Work:**

- Create `TerminalPresentationProvider`.
- Implement `WifiStatusRenderPresentationContract`.
- Print current status clearly for:
  - connected with SSID
  - disconnected
  - no WiFi interface
  - unresolved status

**Done when:**

- The output path works through the presentation contract.
- Rendering logic is not placed inside the agent subject.

### CT-UC-009-007: Implement WiFi Interface Resolver

**Type:** resolver

**Purpose:** Resolve the WiFi interface needed by the rest of the pipeline.

**Work:**

- Create `resolvers::wifi_interface_resolver::resolve`.
- Accept the system WiFi contract.
- Return `WifiInterfaceView<'a>` or an unresolved result.

**Done when:**

- The resolver does not instantiate `VoidSystemWifiProvider`.
- The resolver has a single public operation: `resolve`.

### CT-UC-009-008: Implement WiFi Connection Status Resolver

**Type:** resolver

**Purpose:** Resolve the current WiFi connection status from a resolved interface.

**Work:**

- Create `resolvers::wifi_connection_status_resolver::resolve`.
- Accept the system WiFi contract and `WifiInterfaceView<'a>`.
- Return `WifiConnectionStatusView<'a>` or an unresolved result.

**Done when:**

- The resolver only depends on the contract and input borrowed view.
- The resolver has a single public operation: `resolve`.

### CT-UC-009-009: Implement WiFi Connection Status Output Resolver

**Type:** resolver

**Purpose:** Resolve the output path through the presentation contract.

**Work:**

- Create `resolvers::wifi_connection_status_output_resolver::resolve`.
- Accept the presentation contract.
- Accept either resolved status or the unresolved path information.
- Send the final output through `WifiStatusRenderPresentationContract`.

**Done when:**

- The agent subject does not render directly.
- Presentation output remains behind the contract.
- The resolver has a single public operation: `resolve`.

### CT-UC-009-010: Implement Agent Subject

**Type:** agent_subject

**Purpose:** Coordinate the UC-009 resolver pipeline.

**Work:**

- Create `agent_subjects::wifi_connection_status_shower::show`.
- Execute the resolver pipeline:
  - `wifi_interface_resolver::resolve`
  - `wifi_connection_status_resolver::resolve`
  - `wifi_connection_status_output_resolver::resolve`
- Pass only contracts and borrowed views between steps.

**Done when:**

- The agent subject does not call providers directly.
- The agent subject does not render directly.
- The agent subject does not parse command output.

### CT-UC-009-011: Implement Composition Root

**Type:** composition

**Purpose:** Wire concrete providers to the use case flow.

**Work:**

- Create `EvoWifiCompositionRoot`.
- Instantiate `VoidSystemWifiProvider`.
- Instantiate `TerminalPresentationProvider`.
- Invoke `agent_subjects::wifi_connection_status_shower::show`.

**Done when:**

- Concrete provider creation is isolated in the composition root.
- Resolvers and agent subjects receive contracts, not concrete provider construction logic.

### CT-UC-009-012: Add Terminal Command Entry Point

**Type:** command

**Purpose:** Make the UC-009 flow executable from the terminal.

**Work:**

- Update `src/main.rs`.
- Execute the composition root.
- For the first slice, default execution can show current network status directly.

**Done when:**

- Running the binary triggers UC-009.
- No command parser dependency is added.

### CT-UC-009-013: Add Tests for Resolver and Agent Behavior

**Type:** test

**Purpose:** Verify the architecture behavior without depending on real WiFi services.

**Work:**

- Add fake system WiFi contract implementation for tests.
- Add fake presentation contract implementation for tests.
- Test:
  - connected status
  - disconnected status
  - unresolved interface
  - unresolved status

**Done when:**

- Tests validate resolver pipeline behavior.
- Tests do not call `wpa_cli`.
- Tests do not require a real WiFi interface.

### CT-UC-009-014: Manual Validation on Void Linux

**Type:** validation

**Purpose:** Confirm the vertical slice works against the actual system provider.

**Work:**

- Run `cargo run`.
- Verify output when connected.
- Verify output when disconnected.
- Verify behavior if no interface can be resolved.

**Done when:**

- Manual behavior matches UC-009 acceptance notes.
- Any system-specific limitations are documented.

## Out of Scope

- Scanning available networks.
- Connecting to a selected network.
- Asking for passwords.
- Showing saved passwords.
- Forgetting saved networks.
- TUI navigation.
- Web or desktop presentation providers.
- Third-party crates unless the implementation proves they are necessary.

## Definition of Done

- `cargo check` passes.
- Tests for the UC-009 resolver pipeline pass.
- UC-009 works manually on Void Linux.
- The implementation follows the documented dependency direction:

```text
composition root -> agent subject -> resolvers -> contracts -> providers -> external system
```

- No resolver or agent subject instantiates concrete providers.
- No presentation logic exists inside the agent subject.
- No unnecessary package/DTO is introduced for borrowed data that can stay borrowed.
