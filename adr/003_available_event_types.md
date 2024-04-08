# ADR 003: Available event types

> ::info::
> status = proposed

## Context

Bods need to receive information about external events. We need a strict definition of the type and scope of these events in order to define a clear interface between bod & crane.

## Decision

We define the following events as available to a [bod](001_nomenclature.md):

### Request event

Represent an external message sent directly to the _bod_ that is expecting a response. The mechanism used to relay the message is out-of-scope of the framework and it's a responsibility of the [crane](001_nomenclature.md), but it's usually through an HTTP call.

A request handler will receive a parameter containing the specifics of the request (aka: body) and the output will be relayed as the response of the request.

The following snippet show a non-spec approach of how we could potentially connect message events onto handler functions:

```rust
#[on_request]
fn new_order(payload: OrderSpec) -> Result<Order>  {}
```

### PubSub event

Represent an external message sent directly to the _bod_ that is NOT expecting a synchronous response. The mechanism used to relay the message is out-of-scope of the framework and it's a responsibility of the [crane](001_nomenclature.md), but it's usually through a message queue.

A pubsub message is composed of two elements:

- `topic`: a arbitrary key that is used by the bod to discriminate between handlers of the event.
- `payload`: arbitrary, structured data with relevant information for the handler of the event.

The following snippet show a non-spec approach of how we could potentially connect message events onto handler functions:

```rust
#[on_message(topic="xyz")]
fn new_order(payload: OrderSpec)  {}
```

### Chain event

Represent an event that occurred on-chain. The mechanism used to monitor the chain is out of scope and it's a responsability of the [crane](001_nomenclature.md). Chain events are one-way, they don't expect any response.

Since on-chain data is quite complex, we need to define stereotypes for particular payloads relevant to dApps. We call these payloads _projections_.

We identify the following projections:

- Txs: a view of a self-contained Tx.
- UTxO: a view of a self-contained UTxO.
- Cert: a view of a self-container Certificate.

From these projections we can identify events that map to on-chain operations:

- A Tx is seen on-chain
- A Tx has been rolled-back
- An UTxO is produces by a Tx
- An UTxO is consumed by a Tx

From the above events we identify optional filters that can be used to narrow down specific operations:

- A Tx that consumes UTxOs from a specific address
- A Tx that produces UTxOs to a specific address
- A Tx that mints a specific token
- A Tx that burns a specific token
- A tx that consumes an UTxO holding a specific token 
- A tx that produces an UTxO holding a specific token
- An UTxO that is being locked in a specific address
- An UTxO that is being unlocked from a specific address
- An UTxO that is being locked holding a specific token
- An UTxO that is being unlocked holding a specific token
- A certificate of a specific type

The following snippet show a non-spec approach of how we could potentially connect message events onto handler functions:

```rust
#[on_chain(address="addr1")]
fn my_handler(utxo: Utxo);

#[on_chain(address="stake1")]
fn my_handler(utxo: Utxo);

#[on_chain(address="addr1")]
fn my_handler(stxi: Stxi);

#[on_chain(holds="policy1xxx")]
fn my_handler(utxo: Utxo);

#[on_chain(holds="asset1xxx")]
fn my_handler(utxo: Utxo);

#[on_chain(mints="asset1xxx")]
fn my_handler(tx: Tx);

#[on_chain(burns="asset1xxx")]
fn my_handler(tx: Tx);

#[on_chain(certifies="Delegation")]
fn my_handler(cert: Cert);

#[on_chain(certifies="PoolRegistration")]
fn my_handler(cert: Cert);
```

### Timer event

A timer event is one that gets triggered automatically at specific intervals. The intervals are defined by the bod using cron syntax (eg: `0 15 * * *`).

```rust
#[on_timer(cron="0 15 * * *")]
fn my_handler(now: Instant);
```