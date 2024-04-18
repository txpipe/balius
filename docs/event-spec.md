# Attributes as events

## HTTP calls

- `extrinsic`

This attribute creates associates a function to an endpoint of the name of the function associated to attribute. 

This examples creates an endpoint at the `<root>/resolve` path of the server and will validate the body of the request to match the type of the `ResolveOrderBody` struct.
```rust
#[extrinsic]
fn resolve(request: Request<ResolveOrderBody>) {}
```
 

- `on_http(method=<METHOD>, path=<PATH>)`

This attribute is associates a function to an HTTP request specified by METHOD and PATH. The body of the request if there is one can be specified by the type of the parameter associated to the body.
PATH can be *dynamic* as shown in the **Order Book** example.

```rust
#[on_http(method="POST", path=("/order"/ String))]
fn cancel(tx_out_ref: String, cancel_data: CancelOrderBody) {}
```

- The decorated function executes on a POST operation to a path like `/order/001117d7817713204579ba11f8f9584dd41ccc5deee05b79e867a18a876a3e09#0` and it extracts the dynamic path such that: `tx_out_ref` is `001117d7817713204579ba11f8f9584dd41ccc5deee05b79e867a18a876a3e09#0` and the body of the request is an instance of `CancelOrderBody` (similar to warp)
- First parameter of associated function is the query param and the second parameter is the validated body of the request. If a dynamic parameter doesn't exist, then its first parameter is the body of the request. If the body of the request is not important, then there is no body parameter.


## Chain event

- `match....(...)` // TODO: how much more could this be extended?

This attribute is associated to a function that executes based on the blockchain event is listening to. Let's explore `match_tx_variant`. 


a) `match_tx_variant(type="address_eq", value="addr_...", details=true)`: It executes if there is a new transaction in the blockchain such that a UTxO is consumed from or created into the address passed in to the second parameter. With details as true, then the full transaction is sent as a parameter to the associated function.
b) `match_tx_variant(type="policy_eq", value="ae12...", details=false)`: It executes if there is a new transaction in the blockchain such that a UTxO is consumed or created and it involves a token with the policy id that matches the one specified by value. The associated function receives the base information of a transaction as a parameter given that details is fase

In the orderbook example:

```rust
#[match_tx_variant(type="address_eq",address_equal=ORDER_BOOK, details=true)]
fn on_order_book_change(tx: Transaction) {}
```
The fuction `on_order_book_change` executes when a transaction involving the address `ORDER_BOOK` is added to the blockchain. This transaction is received as the first parameter of the function and similarly to `Oura`, it will include the details of the transaction (input, output, etc.).


## Timer event

- `[on_timer(cron=<CRON>)]`

A function with this attribute will be executed at a time specified by cron. The format of `CRON` is [here](https://en.wikipedia.org/wiki/Cron). The associated function must have a parameter that accepts a date. This function will be called with the date it was called on.