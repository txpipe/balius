use aiken/transaction.{ScriptContext}

type SetupEvent {
  event: ByteArray,
  total: Int,
}

type EventState {
  event: ByteArray,
  available: Int,
}

type PurchaseTicket {
  event: ByteArray,
  quantity: Int,
}

validator {
  fn redeem(
    datum: EventState,
    redeemer: PurchaseTicket,
    context: ScriptContext,
  ) -> Bool {
    True
  }
}
