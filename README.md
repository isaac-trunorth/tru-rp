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

- figure out auth
- get managers employees
- figure out error messages

# Tips

- `podman machine start && podman start f3e4ae0f7156`
- `sea-orm-cli generate entity -o .\entity\src\entities --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")'`

# Final Q's
- User struct - first name, last name, username?
- Projects - include customer? other misc?
- Approval - include approver ID? if so, use separate table?
- Status Paid - allow mark of payout for accounting purposes
- Access levels - 1=users, 2=business admin, 3=manager, 99=owner?

# Future Work
- Add user roles
    - could restrict available WorkCodes based upon role