# US-007: Show current connection password

## User Story

As a Linux user using wpa_supplicant,
I want to view the saved password for the current WiFi connection,
so that I can consult it when I need it.

## Acceptance Criteria

- Given I am connected to a WiFi network with a saved password, when I request the current connection password, then the application shows the saved password.
- Given I am not connected to a WiFi network, when I request the current connection password, then the application reports that there is no current WiFi connection.
- Given the current connection has no saved password, when I request the password, then the application reports that no saved password exists.
