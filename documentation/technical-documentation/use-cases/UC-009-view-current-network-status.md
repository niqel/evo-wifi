# UC-009: View Current Network Status

## Related User Story

US-009: View current network status

## Purpose

Show whether the system is disconnected or connected to a WiFi network, including the current SSID and status when available.

This use case is the first technical implementation candidate because it validates the architecture with a small complete flow.

## Primary Actor

Void Linux user

## Trigger

The user requests the current network status from the application.

## Preconditions

- The application can access the output provider.
- The application can access the WiFi provider.
- The WiFi provider can provide a WiFi interface through the interface contract.

## Main Flow

1. The actor requests the current network status.
2. The agent subject starts the status showing flow.
3. The WiFi interface resolver resolves a usable WiFi interface.
4. The WiFi connection status resolver resolves the current connection status for that interface.
5. The WiFi connection status output resolver sends the borrowed status data to the output contract.
6. The output provider provides the current network status output.

## Alternative Flows

### No WiFi Interface Can Be Resolved

1. The WiFi interface resolver cannot resolve a WiFi interface.
2. The output resolver sends a status message through the output contract.
3. The output provider provides output indicating that no usable WiFi interface was found.

### Current Status Cannot Be Resolved

1. The WiFi interface is resolved.
2. The WiFi connection status resolver cannot resolve the current status.
3. The output resolver sends a status message through the output contract.
4. The output provider provides output indicating that the current network status could not be determined.

### No Active WiFi Connection

1. The WiFi interface is resolved.
2. The WiFi connection status resolver resolves a disconnected status.
3. The output resolver sends the borrowed status data to the output contract.
4. The output provider provides output indicating that there is no active WiFi connection.

## Borrowed Data

- `WifiInterfaceBorrowed<'a>`
  - `name: &'a str`
- `WifiConnectionStatusBorrowed<'a>`
  - `ssid: &'a str`
  - `status: &'a str`

## Technical Mapping

### Agent Subject

- `agents::wifi_connection_status_shower::show`

### Resolver Pipeline

```text
agents::wifi_connection_status_shower::show
  -> wifi_interface_resolver::resolve
  -> wifi_connection_status_resolver::resolve
  -> wifi_connection_status_output_resolver::resolve
```

### Resolvers

- `wifi_interface_resolver::resolve`
- `wifi_connection_status_resolver::resolve`
- `wifi_connection_status_output_resolver::resolve`

### WiFi Contracts

- `WifiInterfaceContract`
- `WifiStatusContract`

### WiFi Provider

- `VoidWifiProvider`

### Output Contract

- `WifiStatusOutputContract`

### Output Provider

- `TerminalOutputProvider`

### Contract Function Rule

- Every contract exposes a single operation named `provide`.
- Providers provide.
- Resolvers resolve.
- Agent subjects coordinate their resolver pipeline.

## Development Task Candidates

- Define `WifiInterfaceBorrowed<'a>`.
- Define `WifiConnectionStatusBorrowed<'a>`.
- Define `WifiInterfaceContract`.
- Define `WifiStatusContract`.
- Define `WifiStatusOutputContract`.
- Implement `VoidWifiProvider` status behavior.
- Implement `TerminalOutputProvider` status output provisioning.
- Implement `wifi_interface_resolver::resolve`.
- Implement `wifi_connection_status_resolver::resolve`.
- Implement `wifi_connection_status_output_resolver::resolve`.
- Implement `agents::wifi_connection_status_shower::show`.
- Add tests for resolved, disconnected, unresolved interface, and unresolved status paths.

## Acceptance Notes

- The agent subject must not call providers directly.
- The agent subject must work through the resolver pipeline.
- Output must go through the output contract.
- WiFi access must go through the WiFi contracts.
- Status values remain borrowed strings and are not converted to enums at this stage.
