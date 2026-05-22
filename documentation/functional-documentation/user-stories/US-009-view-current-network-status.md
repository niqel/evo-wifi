# US-009: View current network status

## User Story

As a Linux user using wpa_supplicant,
I want to view the current network status,
so that I can know whether the system is disconnected or connected to a WiFi network.

## Acceptance Criteria

- Given the system is not connected to a WiFi network, when I request the current network status, then the application reports that there is no active WiFi connection.
- Given the system is connected to a WiFi network, when I request the current network status, then the application shows the connected network SSID.
- Given the system is connected to a WiFi network, when I request the current network status, then the application shows the connection status.
- Given the current status cannot be resolved, when I request the current network status, then the application reports that the status could not be determined.
