## Apisix Local Dev

The script starts two Docker containers, apisix-quickstart and etcd. APISIX uses etcd to save and synchronize configurations.
Both the etcd and the APISIX use host Docker network mode. That is, the APISIX can be accessed from local.

The script can be executed directly from the online source.
A copy of the script is available in the `scripts` folder.

```shell
curl -sL https://run.api7.ai/apisix/quickstart | sh
```

## Build

```zsh
cargo build
```

## Tests

```zsh
cargo test
```