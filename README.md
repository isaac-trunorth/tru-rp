# TruRP System

A resource management tool for TruNorth.

# Developer Setup

- Install [wsl](https://learn.microsoft.com/en-us/windows/wsl/install)
- Install [podman](https://podman.io/docs/installation)
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [PGAdmin4](https://www.pgadmin.org/download/pgadmin-4-windows/)
- Install npm in the wsl environment

# Starting

- `podman machine start && podman start f3e4ae0f7156`
- `podman start --all` to get the id, then `podman start f3e4ae0f7156` where f3... is the id

# Next

- update timelogs
- delete timelogs
- figure out error messages

# Tips

- `podman machine start && podman start f3e4ae0f7156`
- `sea-orm-cli generate entity -o .\entity\src\entities --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")'`
