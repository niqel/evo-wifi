# US-005: Request password when needed

## User Story

As a Linux user using wpa_supplicant,
I want the application to request a password when the selected network has no saved password,
so that I can connect to protected WiFi networks.

## Acceptance Criteria

- Given I select a protected WiFi network without a saved password, when the application prepares the connection, then it asks me for the network password.
- Given I enter a password, when I confirm the connection, then the application uses that password for the connection attempt.
- Given I cancel the password prompt, when the process finishes, then the application does not attempt to connect.
