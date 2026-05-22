# US-010: Disconnect from a WiFi network

## User Story

As a Linux user using wpa_supplicant,
I want to disconnect from the current WiFi network,
so that I can stop the active wireless connection without changing networks.

## Acceptance Criteria

- Given I am connected to a WiFi network, when I choose to disconnect, then the application stops the current WiFi connection.
- Given the disconnect succeeds, when the process finishes, then the system reports that the WiFi connection is no longer active.
- Given no WiFi connection is active, when I choose to disconnect, then the application reports that there is no active WiFi connection to disconnect.
