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

- The application can access the presentation provider.
- The application can access the system WiFi provider.
- The system WiFi provider can attempt to resolve a WiFi interface.

## Main Flow

1. The actor requests the current network status.
2. The agent subject starts the status showing flow.
3. The WiFi interface resolver resolves a usable WiFi interface.
4. The WiFi connection status resolver resolves the current connection status for that interface.
5. The WiFi connection status output resolver sends the borrowed status view to the presentation contract.
6. The presentation provider renders the current network status.

## Alternative Flows

### No WiFi Interface Can Be Resolved

1. The WiFi interface resolver cannot resolve a WiFi interface.
2. The output resolver sends a status message through the presentation contract.
3. The presentation provider renders that no usable WiFi interface was found.

### Current Status Cannot Be Resolved

1. The WiFi interface is resolved.
2. The WiFi connection status resolver cannot resolve the current status.
3. The output resolver sends a status message through the presentation contract.
4. The presentation provider renders that the current network status could not be determined.

### No Active WiFi Connection

1. The WiFi interface is resolved.
2. The WiFi connection status resolver resolves a disconnected status.
3. The output resolver sends the borrowed status view to the presentation contract.
4. The presentation provider renders that there is no active WiFi connection.

## Borrowed Data

- `WifiInterfaceView<'a>`
  - `name: &'a str`
- `WifiConnectionStatusView<'a>`
  - `ssid: &'a str`
  - `status: &'a str`

## Technical Mapping

### Agent Subject

- `agent_subjects::wifi_connection_status_shower::show`

### Resolver Pipeline

```text
agent_subjects::wifi_connection_status_shower::show
  -> wifi_interface_resolver::resolve
  -> wifi_connection_status_resolver::resolve
  -> wifi_connection_status_output_resolver::resolve
```

### Resolvers

- `wifi_interface_resolver::resolve`
- `wifi_connection_status_resolver::resolve`
- `wifi_connection_status_output_resolver::resolve`

### System WiFi Contracts

- `WifiInterfaceSystemWifiContract`
- `WifiStatusSystemWifiContract`

### System WiFi Provider

- `VoidSystemWifiProvider`

### Presentation Contract

- `WifiStatusRenderPresentationContract`

### Presentation Provider

- `TerminalPresentationProvider`

## Development Task Candidates

- Define `WifiInterfaceView<'a>`.
- Define `WifiConnectionStatusView<'a>`.
- Define `WifiInterfaceSystemWifiContract`.
- Define `WifiStatusSystemWifiContract`.
- Define `WifiStatusRenderPresentationContract`.
- Implement `VoidSystemWifiProvider` status behavior.
- Implement `TerminalPresentationProvider` status rendering.
- Implement `wifi_interface_resolver::resolve`.
- Implement `wifi_connection_status_resolver::resolve`.
- Implement `wifi_connection_status_output_resolver::resolve`.
- Implement `agent_subjects::wifi_connection_status_shower::show`.
- Add tests for resolved, disconnected, unresolved interface, and unresolved status paths.

## Acceptance Notes

- The agent subject must not call providers directly.
- The agent subject must work through the resolver pipeline.
- Presentation output must go through the presentation contract.
- System WiFi access must go through the system WiFi contracts.
- Status values remain borrowed strings and are not converted to enums at this stage.
