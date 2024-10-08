<div align="center">
    <h1><code>Apisix admin Rust client</code></h1>
    <div><img src="./assets/apisix.jpeg" width="200"/></div>
    <a href="https://docs.rs/smart_id_rust_client/">
        <img src="https://docs.rs/teloxide/badge.svg">  
    </a>
    <a href="https://crates.io/crates/smart_id_rust_client">
        <img src="https://img.shields.io/crates/v/smart_id_rust_client.svg">
    </a>
    <h3><code>⚡ Maintained by ⚡</code></h3>
    <div><img src="./assets/t1t.png" width="250"/></div>


<div>A straightforward Apisix admin client library.</div>

</div>

# Introduction
This library implements the Apisix Admin interface [Reference API](https://apisix.apache.org/docs/apisix/admin-api).

Apisix API Version: **3.10**

## Status
Under development!

## Documentation

[Apisix Getting Started](https://apisix.apache.org/docs/apisix/getting-started/README/)


## Apisix Local Dev

The script starts two Docker containers, apisix-quickstart and etcd. APISIX uses etcd to save and synchronize configurations.
Both the etcd and the APISIX use host Docker network mode. That is, the APISIX can be accessed from local.

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

## Documentation

```zsh
cargo doc --no-deps --open
```
 
## Example Client

See the examples folder for a simple example client.
To create an SmartID account, download the app:

The example application goes through the following use cases:
- Verify Certificate Existence
- SmartID Authentication
- SmartID Digital Signature

The example is using a MOCK ID to simulate the SmartID user.

```shell
cargo run --example smart_id_client
```

### Configuration
Default values are provided and targets the TEST environment of Smart ID. 
The required environment variables used:

| ENV_VAR                                   | DESCRIPTION                                                                    | REQUIRED       |
|-------------------------------------------|--------------------------------------------------------------------------------|----------------|
| HOST_URL                                  | The host address of the Smart ID Service                                       | Y              |
| RELYING_PARTY_UUID                        | The UUID assigned to the Relying-party - provided by Smart ID                  | Y              |
| RELYING_PARTY_NAME                        | The unique name assigned to the Relying-Party - provided by Smart ID           | Y              |
| CLIENT_REQ_NETWORK_TIMEOUT_MILLIS         | The timeout for the REST client when requesting Smart ID Services              | N - default () |
| CLIENT_REQ_MAX_ATTEMPTS                   | The maximum attempts for the REST client retry mechanism                       |                |
| CLIENT_REQ_DELAY_SECONDS_BETWEEN_ATTEMPTS | The wait time between consecutive REST client requests                         |                |
| ENABLE_POLLING_BY_LIB                     | Enable automatic polling (when `false` - polling MUST be implemented by callee |                |


## Plugins implemented
- [X] consumer-restriction
- [X] apikey
- [X] limit-count
- [X] proxy-rewrite
- [X] authz-keycloak
- [X] redirect
- [X] openid-connect
- [X] cors
- [X] serverless
- [X] forward-auth

## TODO

This library started with basic CRUD operations for Apisix resources mostly used in projects. 
Is does not (yet) implement the full API, but feel free to contribute.

- [ ] Implement the control API endpoints
- [ ] Response object mapping to Rust Structs (mostly returned as JSON value)
- [ ] Upstream: Update Upstream builder: add pass_host, upstream_host, health_check, hash_on key, labels, tls, keepalive
- [ ] Upstream: patch use cases
- [ ] Service: patch use cases
- [ ] Route: patch use cases
- [ ] Missing Plugins (only adding plugins used, but feel free to extend))
- [ ] Long list of plugins to implement

## Remarks

Read the conventions upon ID Syntax:

[ID Syntax](https://apisix.apache.org/docs/apisix/admin-api/#quick-note-on-id-syntax)

This library assumes that all Apisix resources are created with a unique ID. The ID is a string that is unique within the scope of the resource type. The ID is used to identify the resource in the Apisix Admin API. 
The ID must be a string literal, integer literals are not supported yet (both are possible in Apisix).

## Examples

Run the examples:
In case you need debug logs:
```shell
RUST_LOG=info,apisix_admin=debug,apisix_plugins=debug cargo run --example apisix-admin
```

### Admin Resources examples:

```shell
cargo run --example apisix-admin
```

### Plugin examples:

```shell
cargo run --example apisix-plugins
```

