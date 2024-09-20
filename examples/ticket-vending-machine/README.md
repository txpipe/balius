# Ticket Vending Machine

This headless dApp is about selling tickets for a particular event (like for a show or sports event).

The external interface provides a way to:

- request a purchase (replies with an un-signed transaction)
- see the number of available tickets
- see a history of ticket purchases

The on-chain transactions provides a way to:

- charge ADA for each ticket
- send the ticket (NFT) to the buyer address

The state of the dApp is:

- a list of txs that represent each purchase

The dApp is configured by the following parameters:

- an identifier for the particular event
- the max amount of tickets for the event
- the price in ADA for each ticket
- an max slot after with no more ticket can be purchased