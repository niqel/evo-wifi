# UC-004: Switch to Another WiFi Network

## Related User Story

US-004: Switch to another WiFi network

## Purpose

Switch from the current WiFi network to a different selected WiFi network by coordinating the existing status, saved-network, password, and connection flows.

## Primary Actor

Void Linux user

## Trigger

The user requests to switch to another WiFi network.

## Preconditions

- The application can access the input provider.
- The application can access the WiFi provider.
- The WiFi provider can provide a usable WiFi interface through the interface contract.

## Main Flow

1. The actor requests to switch to a different WiFi network.
2. The agent subject starts the switch flow.
3. The WiFi interface resolver resolves a usable WiFi interface.
4. The WiFi connection status resolver resolves the current connection status for that interface.
5. The switch resolver determines whether the selected network is already connected.
6. If the selected network is already connected, the output resolver sends a message through the output contract.
7. If the selected network is not already connected and a saved network exists, the saved-network resolver resolves it and the connect resolver attempts the connection.
8. If the selected network is not already connected and no saved network exists, the password resolver resolves a password and the new-connect resolver attempts the connection.
9. The output provider provides the resulting status or message.

## Alternative Flows

### No WiFi Interface Can Be Resolved

1. The WiFi interface resolver cannot resolve a WiFi interface.
2. The output resolver sends a message through the output contract.
3. The output provider provides output indicating that no usable WiFi interface was found.

### Selected Network Is Already Connected

1. The WiFi interface is resolved.
2. The WiFi connection status resolver resolves a status whose SSID matches the selected network.
3. The switch resolver routes to the already-connected message path.
4. The output provider provides a message indicating that the selected network is already connected.

### Selected Network Has a Saved Entry

1. The WiFi interface is resolved.
2. The selected network is not already connected.
3. The saved-network resolver resolves a saved network entry.
4. The connect resolver attempts the network switch using the saved network entry.
5. The output provider provides the resulting connection status.

### Selected Network Has No Saved Entry

1. The WiFi interface is resolved.
2. The selected network is not already connected.
3. The saved-network resolver cannot resolve a saved network entry.
4. The password resolver resolves a password from input.
5. The new-connect resolver attempts the network switch using the supplied password.
6. The output provider provides the resulting connection status.

### No Password Is Provided For A New Connection

1. The WiFi interface is resolved.
2. The selected network is not already connected.
3. The saved-network resolver cannot resolve a saved network entry.
4. The password resolver cannot resolve a password input.
5. The output resolver sends a message through the output contract.
6. The output provider provides output indicating that a password is required to connect to the selected network.

## Borrowed Data

- `WifiInterfaceBorrowed<'a>`
  - `name: &'a str`
- `WifiConnectionStatusBorrowed<'a>`
  - `ssid: &'a str`
  - `status: &'a str`
- `WifiNetworkSelectionInputBorrowed<'a>`
  - `raw: &'a str`
- `WifiPasswordInputBorrowed<'a>`
  - `raw: &'a str`
- `WifiSavedNetworkBorrowed<'a>`
  - `ssid: &'a str`
  - `network_id: &'a str`

## Technical Mapping

### Agent Subject

- `agents::wifi_network_switcher::switch`

### Resolver Pipeline

```text
agents::wifi_network_switcher::switch
  -> wifi_network_selection_input_resolver::resolve
  -> wifi_interface_resolver::resolve
  -> wifi_connection_status_resolver::resolve
  -> wifi_network_switch_resolver::resolve
```

### Resolvers

- `wifi_network_selection_input_resolver::resolve`
- `wifi_interface_resolver::resolve`
- `wifi_connection_status_resolver::resolve`
- `wifi_network_switch_resolver::resolve`
- `wifi_already_connected_network_resolver::resolve`
- `wifi_saved_network_resolver::resolve`
- `wifi_connect_resolver::resolve`
- `wifi_password_input_resolver::resolve`
- `wifi_new_network_connect_resolver::resolve`
- `wifi_connection_status_output_resolver::resolve`
- `wifi_message_output_resolver::resolve`

### WiFi Contracts

- `WifiNetworkSelectionInputContract`
- `WifiInterfaceContract`
- `WifiStatusContract`
- `WifiSavedNetworkContract`
- `WifiConnectContract`
- `WifiPasswordInputContract`
- `WifiNewNetworkConnectContract`

### WiFi Provider

- `VoidWifiProvider`

### Output Contracts

- `WifiStatusOutputContract`
- `WifiMessageOutputContract`

### Output Providers

- `TerminalStatusOutputProvider`
- `TerminalMessageOutputProvider`

### Contract Function Rule

- Every contract exposes a single operation named `provide`.
- Providers provide.
- Resolvers resolve.
- Agent subjects coordinate their resolver pipeline.

## Development Task Candidates

- Define `WifiNetworkSelectionInputBorrowed<'a>`.
- Define `WifiInterfaceBorrowed<'a>`.
- Define `WifiConnectionStatusBorrowed<'a>`.
- Define `WifiPasswordInputBorrowed<'a>`.
- Define `WifiSavedNetworkBorrowed<'a>`.
- Define `WifiNetworkSelectionInputContract`.
- Define `WifiInterfaceContract`.
- Define `WifiStatusContract`.
- Define `WifiSavedNetworkContract`.
- Define `WifiConnectContract`.
- Define `WifiPasswordInputContract`.
- Define `WifiNewNetworkConnectContract`.
- Define `WifiStatusOutputContract`.
- Define `WifiMessageOutputContract`.
- Implement `wifi_network_switch_resolver::resolve`.
- Implement `agents::wifi_network_switcher::switch`.
- Add tests for already-connected, saved-network, password-fallback, and missing-password paths.

## Acceptance Notes

- The agent subject must not decide the route itself.
- The switch decision is made inside the resolver.
- The agent subject must only connect resolver pipes.
- The implementation must reuse the existing connect and new-connect pipes.
