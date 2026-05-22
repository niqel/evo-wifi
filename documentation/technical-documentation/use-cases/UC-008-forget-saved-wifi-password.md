# UC-008: Forget Saved WiFi Password

## Related User Story

US-008: Forget saved WiFi password

## Purpose

Forget a saved WiFi password for a selected network so the system no longer keeps that saved credential for future connections.

## Primary Actor

Void Linux user

## Trigger

The user requests to forget a saved WiFi password for a specific WiFi network.

## Preconditions

- The application can access the input provider.
- The application can access the WiFi provider.
- The WiFi provider can provide a usable WiFi interface through the interface contract.
- The selected WiFi network has a saved network entry when the user expects removal to succeed.

## Main Flow

1. The actor requests to forget a saved WiFi password for a selected network.
2. The agent subject starts the forget flow.
3. The WiFi interface resolver resolves a usable WiFi interface.
4. The saved network resolver resolves the saved network for the selected SSID.
5. The forget resolver asks the WiFi provider to remove the saved network.
6. The output resolver sends a success message through the output contract.
7. The output provider provides confirmation that the saved WiFi password was removed.

## Alternative Flows

### No WiFi Interface Can Be Resolved

1. The WiFi interface resolver cannot resolve a WiFi interface.
2. The output resolver sends a message through the output contract.
3. The output provider provides output indicating that no usable WiFi interface was found.

### No Saved Network Can Be Resolved

1. The WiFi interface is resolved.
2. The saved network resolver cannot resolve a saved network for the selected SSID.
3. The output resolver sends a message through the output contract.
4. The output provider provides output indicating that there is no saved WiFi password to remove.

### Saved Network Cannot Be Removed

1. The WiFi interface is resolved.
2. The saved network is resolved.
3. The forget resolver cannot remove the saved network.
4. The output resolver sends a message through the output contract.
5. The output provider provides output indicating that the saved WiFi password could not be removed.

## Borrowed Data

- `WifiInterfaceBorrowed<'a>`
  - `name: &'a str`
- `WifiNetworkSelectionInputBorrowed<'a>`
  - `raw: &'a str`
- `WifiSavedNetworkBorrowed<'a>`
  - `ssid: &'a str`
  - `network_id: &'a str`

## Technical Mapping

### Agent Subject

- `agents::wifi_saved_network_forgetter::forget`

### Resolver Chain

```text
agents::wifi_saved_network_forgetter::forget
  -> wifi_network_selection_input_resolver::resolve
  -> wifi_interface_resolver::resolve
  -> wifi_saved_network_resolver::resolve
  -> wifi_forget_resolver::resolve
```

### Resolvers

- `wifi_network_selection_input_resolver::resolve`
- `wifi_interface_resolver::resolve`
- `wifi_saved_network_resolver::resolve`
- `wifi_forget_resolver::resolve`

### WiFi Contracts

- `WifiNetworkSelectionInputContract`
- `WifiInterfaceContract`
- `WifiSavedNetworkContract`
- `WifiForgetContract`

### WiFi Providers

- `VoidWifiInterfaceProvider`
- `VoidWifiSavedNetworkProvider`
- `VoidWifiForgetProvider`

### Output Contract

- `WifiMessageOutputContract`

### Output Provider

- `TerminalMessageOutputProvider`

### Contract Function Rule

- Every contract exposes a single operation named `provide`.
- Providers provide.
- Resolvers resolve.
- Agent subjects coordinate their resolver chain.

## Development Task Candidates

- Define `WifiNetworkSelectionInputBorrowed<'a>`.
- Define `WifiInterfaceBorrowed<'a>`.
- Define `WifiSavedNetworkBorrowed<'a>`.
- Define `WifiNetworkSelectionInputContract`.
- Define `WifiInterfaceContract`.
- Define `WifiSavedNetworkContract`.
- Define `WifiForgetContract`.
- Implement `VoidWifiForgetProvider` forget behavior.
- Implement `TerminalMessageOutputProvider` forget messages.
- Implement `wifi_network_selection_input_resolver::resolve`.
- Implement `wifi_interface_resolver::resolve`.
- Implement `wifi_saved_network_resolver::resolve`.
- Implement `wifi_forget_resolver::resolve`.
- Implement `agents::wifi_saved_network_forgetter::forget`.
- Add tests for resolved, missing saved network, unresolved interface, and unresolved removal paths.

## Acceptance Notes

- The agent subject must not call providers directly.
- The agent subject must work through the resolver chain.
- Output must go through the output contract.
- Forgetting a saved network is a standalone command flow.
- The selected WiFi network is provided as an input parameter.
