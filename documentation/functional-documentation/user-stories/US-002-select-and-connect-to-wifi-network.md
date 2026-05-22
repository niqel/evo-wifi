# US-002: Select and connect to a WiFi network

## User Story

As a Linux user using wpa_supplicant,
I want to select an available WiFi network and connect to it,
so that I can access the internet through the selected network.

## Acceptance Criteria

- Given available WiFi networks are shown, when I select one network, then the application prepares the connection flow for that network.
- Given the selected network can be resolved for connection, when I confirm the connection, then the application attempts to connect to it.
- Given the connection succeeds, when the process finishes, then the system is connected to the selected WiFi network.
- Given the connection fails, when the process finishes, then the application reports the failure.
