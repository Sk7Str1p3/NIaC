# NIaC-bootstrap
Bootstrap script for my NixOS configuration

## Why
Script is supposed to automate installation of __my__
NixOS configuration, doing things usually done manually.
If you have approximately same configuration, you can also
use this script

## Stages
- Decrypt master keys
- Partition disks (with `disko`)
- Setup `SecureBoot` (if keys exist)
- Install NixOS
- Run some post-install operations

Please note that script is not yet ready for use, because
of how keys are stored. Currently, **they're encrypted with
__password__ which negates the reliability of asymmetric `SOPS` keys.**
In future, FIDO key will be used
