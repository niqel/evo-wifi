# US-004: Switch to another WiFi network

## User Story

As a Void Linux user,
I want to connect to a different available WiFi network,
so that I can change my current connection.

## Acceptance Criteria

- Given I am connected to a WiFi network, when I select a different available network, then the application prepares a network switch.
- Given the selected network can be resolved for connection, when I confirm the switch, then the application attempts to connect to the selected network.
- Given the switch succeeds, when the process finishes, then the system is connected to the newly selected WiFi network.
- Given the switch fails, when the process finishes, then the application reports the failure.
