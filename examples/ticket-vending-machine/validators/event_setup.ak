use aiken/transaction.{ScriptContext}

type SetupEvent {
  event: ByteArray,
  total: Int,
}

type EventState {
  event: ByteArray,
  available: Int,
}

validator {
  fn mint(redeemer: SetupEvent, context: ScriptContext) -> Bool {
    True
  }
}
