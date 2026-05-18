# US-006: Use saved password when available

## User Story

As a Void Linux user,
I want the application to use a saved password when one exists for the selected network,
so that I can connect without entering the password again.

## Acceptance Criteria

- Given I select a protected WiFi network with a saved password, when the application prepares the connection, then it resolves the saved password.
- Given a saved password is resolved, when I confirm the connection, then the application attempts to connect without asking me for the password again.
- Given the saved password is no longer valid, when the connection fails, then the application reports the failure.
