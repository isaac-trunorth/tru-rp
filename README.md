# TruRP System

A resource management tool for TruNorth.

# Developer Setup

- Install [wsl](https://learn.microsoft.com/en-us/windows/wsl/install)
- Install [podman](https://podman.io/docs/installation)
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [PGAdmin4](https://www.pgadmin.org/download/pgadmin-4-windows/)

# Starting

- `podman machine start && podman start f3e4ae0f7156`
- `podman start --all` to get the id, then `podman start f3e4ae0f7156` where f3... is the id

# Next

- get timelogs: (Add tests)
    - get by week end date
    - get by employee id
    - get by manager id
- update timelogs:
    - Manager to approval (exclusive endpoint?)
    - User to update their own
- figure out error messages

# Tips

- `podman machine start && podman start f3e4ae0f7156`
- `sea-orm-cli generate entity -o .\entity\src\entities --with-serde both --serde-skip-deserializing-primary-key --model-extra-attributes 'ts(export)'`
