# evo-wifi

Borrowed-first WiFi CLI and provider workspace.

## Workspace

- `crates/evo-wifi-core`: library with borrowed models, contracts, resolvers, agents, and commands.
- `crates/evo-wifi-cli`: binary that parses CLI arguments and wires providers into core commands.
- `crates/evo-wifi-nu-plugin`: Nushell plugin binary exposing simple structured commands.
- `crates/evo-wifi-provider-linux-wpa`: Linux WPA provider implementations backed by `wpa_cli`, `iw`, and related system tools.
- `crates/evo-wifi-provider-nushell`: Nushell-facing input and output providers that own typed data and expose structured values.
- `crates/evo-wifi-provider-terminal`: terminal input and output providers.

## Commands

```bash
cargo run -- status
cargo run -- networks
cargo run -- scan
cargo run -- connect <ssid>
cargo run -- connect <ssid> <password>
cargo run -- switch <ssid>
cargo run -- switch <ssid> <password>
cargo run -- disconnect
cargo run -- forget <ssid>
cargo run -- password
cargo run -- secret
```

## Nushell Plugin

```bash
cargo build -p evo-wifi-nu-plugin
plugin add target/debug/nu_plugin_evo_wifi
plugin use evo_wifi
evo-nu-wifi status
evo-nu-wifi networks
```
