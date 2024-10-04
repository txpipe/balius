use std::marker::PhantomData;

use crate::_internal::Handler;
use crate::wit;

#[derive(Debug)]
pub enum Error {
    Internal(String),
    Ledger(wit::balius::app::ledger::LedgerError),
}

impl From<Error> for wit::HandleError {
    fn from(error: Error) -> Self {
        match error {
            Error::Internal(_) => 0,
            Error::Ledger(e) => e.into(),
        }
    }
}

impl From<wit::balius::app::ledger::LedgerError> for Error {
    fn from(error: wit::balius::app::ledger::LedgerError) -> Self {
        Error::Ledger(error)
    }
}

pub type WorkerResult<T> = std::result::Result<T, Error>;

pub struct FnHandler<F, C, E, R>
where
    F: Fn(C, E) -> WorkerResult<R> + 'static,
    C: From<wit::Env>,
    E: From<wit::Event>,
    R: Into<wit::Response>,
{
    func: F,
    phantom: PhantomData<(C, E)>,
}

impl<F, C, E, R> Handler for FnHandler<F, C, E, R>
where
    C: From<wit::Env> + Send + Sync + 'static,
    E: From<wit::Event> + Send + Sync + 'static,
    R: Into<wit::Response> + Send + Sync + 'static,
    F: Fn(C, E) -> WorkerResult<R> + Send + Sync + 'static,
{
    fn handle(
        &self,
        config: wit::Env,
        event: wit::Event,
    ) -> Result<wit::Response, wit::HandleError> {
        let config: C = config.into();
        let event: E = event.into();
        let response = (self.func)(config, event)?;
        Ok(response.into())
    }
}

impl<F, C, E, R> From<F> for FnHandler<F, C, E, R>
where
    C: From<wit::Env> + Send + Sync + 'static,
    E: From<wit::Event> + Send + Sync + 'static,
    R: Into<wit::Response> + Send + Sync + 'static,
    F: Fn(C, E) -> WorkerResult<R> + Send + Sync + 'static,
{
    fn from(func: F) -> Self {
        FnHandler {
            func,
            phantom: PhantomData,
        }
    }
}

pub struct Config<T>(pub T);

impl<T> From<wit::Env> for Config<T>
where
    T: serde::de::DeserializeOwned,
{
    fn from(env: wit::Env) -> Self {
        let t = serde_json::from_slice(env.as_slice()).unwrap();
        Config(t)
    }
}

impl<T> std::ops::Deref for Config<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Params<T>(pub T);

impl<T> From<wit::Event> for Params<T>
where
    T: serde::de::DeserializeOwned,
{
    fn from(value: wit::Event) -> Self {
        let bytes = match value {
            wit::Event::Request(x) => x,
            _ => todo!(),
        };

        let t = serde_json::from_slice(bytes.as_slice()).unwrap();
        Params(t)
    }
}

impl<T> From<Params<T>> for wit::Response
where
    T: serde::Serialize,
{
    fn from(value: Params<T>) -> Self {
        Self::Json(serde_json::to_vec(&value.0).unwrap())
    }
}

impl<T> std::ops::Deref for Params<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Json<T>(pub T);

impl<T> From<Json<T>> for wit::Response
where
    T: serde::Serialize,
{
    fn from(value: Json<T>) -> Self {
        Self::Json(serde_json::to_vec(&value.0).unwrap())
    }
}

impl<T> std::ops::Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct NewTx(pub Box<dyn crate::txbuilder::TxExpr>);

impl crate::txbuilder::TxExpr for NewTx {
    fn eval_body(
        &self,
        ctx: &crate::txbuilder::BuildContext,
    ) -> Result<crate::txbuilder::primitives::TransactionBody, crate::txbuilder::BuildError> {
        self.0.eval_body(ctx)
    }

    fn eval_witness_set(
        &self,
        ctx: &crate::txbuilder::BuildContext,
    ) -> Result<crate::txbuilder::primitives::WitnessSet, crate::txbuilder::BuildError> {
        self.0.eval_witness_set(ctx)
    }
}

impl Into<wit::Response> for NewTx {
    fn into(self) -> wit::Response {
        // TODO: create a build context from the Balius world interface and evaluate the
        // tx.

        todo!()
    }
}

impl crate::_internal::Worker {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn init(&mut self, env: wit::Env) {
        self.env = Some(env);
    }

    pub fn with_request_handler(
        mut self,
        method: &str,
        handler: impl Handler + Send + Sync + 'static,
    ) -> Self {
        self.channels.insert(
            self.channels.len() as u32,
            crate::_internal::Channel {
                handler: Box::new(handler),
                pattern: wit::balius::app::driver::EventPattern::Request(method.to_owned()),
            },
        );

        self
    }
}
