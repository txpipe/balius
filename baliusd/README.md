# Balius Daemon (`baliusd`)

A standalone Balius runtime that can be used as a daemon. The Balius Daemon
executes WebAssembly dApps built with `balius-sdk`, manages chain
synchronization, JSON-RPC APIs, metrics, logging, and persistent state stores.

## Installation

Install via Cargo:

```bash
cargo install baliusd
```

## Usage

Run the daemon (it automatically loads configuration files and environment
variable overrides). You can also enable debug mode with the `--debug` flag,
which loads both the chain cursor store and key-value backend into ephemeral
(in-memory) mode so that all runtime state is lost when the process exits:

```bash
baliusd --debug
```

## Configuration

baliusd is configured using a TOML file (`baliusd.toml`). Configuration files are loaded in this order:

1. `/etc/baliusd/daemon.toml` (optional system-wide config)
2. `baliusd.toml` in the current working directory (optional project config)
3. Environment variables (`BALIUSD_*`) override any setting from files

### Environment Variables

Any configuration field can be overridden by an environment variable prefixed
with `BALIUSD_`, using underscores to separate nested keys. For example:

```bash
export BALIUSD_RPC_LISTEN_ADDRESS=0.0.0.0:4000
```

### Example (`baliusd.toml`)

This is an example toml file, it loads up the `comprehensive` example worker,
which is a worker that does a walkthrough of most of the features. Some
prerequisites to run this are:

* Compile the worker: Run `cargo balius build` on the `examples/comprehensive`
  directory. This creates the `../examples/comprehensive/comprehensive-c.wasm`
  WASM file refferenced on the config.

* Have a local U5C instance running on port `50051`. The easiest way to have
  this is to install `dolos` locally and run a testnet network (for more
  information, see https://docs.txpipe.io/dolos).

```toml
# Persist information for intersecting with the chain. If undefined, the daemon
# will always intersect with the tip
[store]
path = "cursors"

[rpc]
listen_address = "0.0.0.0:3001"

# Configuration for Prometheus server. If undefined, metrics will not be exposed.
[metrics]
listen_address = "0.0.0.0:8080"

# Logging configuration for the daemon.
[logging]
max_level = "info"
include_tokio = true

# U5C endpoint to resolve ledger queries.
[ledger]
endpoint_url = "http://localhost:50051"

# U5C endpoint to perform chainsync.
[chainsync]
endpoint_url = "http://localhost:50051"

# Key value backend.
[kv]
type = "memory"

# ReDB for having persistence across restarts.
# type = "redb"
# path = "kv"

# Worker logs backend.
[logger]
# Ignore logs
# type = "silent"

# Write each worker logs into a different file inside the `logs` folder
type = "file"
folder = "logs"

# Subscribe worker logs into tracing output.
# type = "tracing"

# Backend for providing signing keys to workers.
[signing]
type = "memory"

# You can hardcode the keys for the workers. If a key is undefined it will be
# created randomly, not consistent across restarts.
[[signing.keys]]
worker = "comprehensive"
name = "alice"
algorithm = "ed25519"
private_key = "06cc7c4372cd1036719cd79b8349de5ca6b54ccf5487578834d32064b4b1ec53"

# List of workers to be loaded by the runtime.
[[workers]]
name = "comprehensive"
module = "../examples/comprehensive/comprehensive-c.wasm"
config = "comprehensive.json"
```

## Configuration Options

### [store] (optional)

- **path** (string): Path to a local file to persist the chain sync cursor and
  internal daemon state. If omitted, state is kept in memory and chain sync will
  start from the beginning on each restart.

### [rpc] (required)

- **listen_address** (string): Address and port for the JSON-RPC server.
  Example: `"0.0.0.0:3001"`.

### [metrics] (optional)

- **listen_address** (string): Address and port for the Prometheus metrics
  endpoint (`/metrics`). If omitted, no metrics server is started.

### [logging] (required)

- **max_level** (`"error"`, `"warn"`, `"info"`, `"debug"`, `"trace"`): Sets the
  maximum log level for the daemon's console output.
- **include_tokio** (boolean, default: `false`): Include logs from the Tokio
  runtime (e.g. async task management).

### [ledger] (required)

- **endpoint_url** (string): URI of the UTxORPC query endpoint.
- **headers** (table<string, string>, optional): HTTP headers to send with each
  request (e.g. API keys).

### [chainsync] (required)

- **endpoint_url** (string): URI of the UTxORPC chain-sync endpoint.
- **headers** (table<string, string>, optional): HTTP headers to send with each
  request.

### [kv] (optional)

- **type** (`"memory"` or `"redb"`): Type of key-value store.
- **path** (string, required for `"redb"`): Filesystem path for the Redb store.
- **cache_size** (integer, optional): Cache size in bytes for Redb.

> **Default**: If the `[kv]` section is omitted, an in-memory mock KV store is
> used (no persistence).

### [logger] (optional)

- **type** (`"silent"`, `"tracing"`, or `"file"`): Runtime logging mode.
- **folder** (string, optional for `"file"`): Directory to write log files.
  Defaults to the current directory.

> **Default**: Silent (no runtime logs).

### [signing] (optional)

- **type** (`"memory"`): Only in-memory signing is supported.
- **keys** (array of tables): List of in-memory signing keys.

Each `[[signing.keys]]` entry:

- **worker** (string): Name of the worker to register the key for.
- **name** (string): Key identifier within the worker.
- **algorithm** (string, only `"ed25519"` supported).
- **private_key** (string): Hex-encoded private key.

> **Default**: No signing keys registered, they are generated randomly on each
> restart.

### [http] (optional)

If undefined, the default HTTP client will be created with a timeout of 10
seconds.

- **type** (`"reqwest"`): HTTP client backend.
- **timeout** (integer, seconds, default: `10`): Request timeout.

### [[workers]] (required, multiple)

- **name** (string): Unique name of the worker.
- **module** (string): Path to the worker's WebAssembly module (`.wasm`).
- **config** (string, optional): Path to a JSON file passed to the worker on
  startup.

## Examples

See the `example-mainnet/` and `example-preview/` directories for
production-ready examples targeting Cardano mainnet and preview networks.
