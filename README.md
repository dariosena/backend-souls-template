# BackendSouls Template

This is an example template that uses hook scripts to let the user select a license file.

The selected license file will be renamed into `LICENSE`, and besides this `README.md` be the only files left after expansion.

## Expansion

```sh
cargo generate --name my-backend-api gh:dariosena/backend-souls-template
```

or to select the license directly from command line:

```sh
cargo generate --name my-backend-api gh:dariosena/backend-souls-template -d license=mit
```
