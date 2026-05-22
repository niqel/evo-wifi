# US-003: Handle already connected network

## User Story

As a Linux user using wpa_supplicant,
I want the application to detect when I select the network I am already connected to,
so that it does not reconnect unnecessarily.

## Acceptance Criteria

- Given I am already connected to a WiFi network, when I select the same network, then the application detects the existing connection.
- Given the selected network is already connected, when I try to connect, then the application shows a message indicating that I already have a connection to that network.
- Given the selected network is already connected, when the message is shown, then no reconnection attempt is made.
