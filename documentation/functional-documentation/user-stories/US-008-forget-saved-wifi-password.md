# US-008: Forget saved WiFi password

## User Story

As a Void Linux user,
I want to forget a saved WiFi password,
so that the system no longer stores it for future connections.

## Acceptance Criteria

- Given a WiFi network has a saved password, when I choose to forget that password, then the application removes the saved password.
- Given the saved password is removed, when I try to connect to that network again, then the application asks for the password if the network is protected.
- Given no saved password exists for the selected network, when I choose to forget it, then the application reports that there is no saved password to remove.
