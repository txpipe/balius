use std::marker::PhantomData;

use crate::_internal::Handler;
use crate::wit;

#[derive(Debug)]
pub enum Error {
    Internal(String),
    BadConfig,
    BadParams,
    Ledger(wit::balius::app::ledger::LedgerError),
}

impl From<Error> for wit::HandleError {
    fn from(error: Error) -> Self {
        match error {
            Error::Internal(x) => wit::HandleError {
                code: 0,
                message: x,
            },
            Error::BadConfig => wit::HandleError {
                code: 1,
                message: "bad config".to_owned(),
            },
            Error::BadParams => wit::HandleError {
                code: 2,
                message: "bad params".to_owned(),
            },
            Error::Ledger(e) => wit::HandleError {
                code: e,
                message: "ledger error".to_string(),
            },
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Internal(error.to_string())
    }
}

impl From<wit::balius::app::ledger::LedgerError> for Error {
    fn from(error: wit::balius::app::ledger::LedgerError) -> Self {
        Error::Ledger(error)
    }
}

impl From<crate::txbuilder::BuildError> for Error {
    fn from(error: crate::txbuilder::BuildError) -> Self {
        Error::Internal(error.to_string())
    }
}

pub type WorkerResult<T> = std::result::Result<T, Error>;

pub struct FnHandler<F, C, E, R>
where
    F: Fn(C, E) -> WorkerResult<R> + 'static,
    C: TryFrom<wit::Config>,
    E: TryFrom<wit::Event>,
    R: TryInto<wit::Response>,
{
    func: F,
    phantom: PhantomData<(C, E)>,
}

impl<F, C, E, R> Handler for FnHandler<F, C, E, R>
where
    C: TryFrom<wit::Config, Error = Error> + Send + Sync + 'static,
    E: TryFrom<wit::Event, Error = Error> + Send + Sync + 'static,
    R: TryInto<wit::Response, Error = Error> + Send + Sync + 'static,
    F: Fn(C, E) -> WorkerResult<R> + Send + Sync + 'static,
{
    fn handle(
        &self,
        config: wit::Config,
        event: wit::Event,
    ) -> Result<wit::Response, wit::HandleError> {
        let config: C = config.try_into()?;
        let event: E = event.try_into()?;
        let response = (self.func)(config, event)?;
        Ok(response.try_into()?)
    }
}

impl<F, C, E, R> From<F> for FnHandler<F, C, E, R>
where
    C: TryFrom<wit::Config, Error = Error> + Send + Sync + 'static,
    E: TryFrom<wit::Event, Error = Error> + Send + Sync + 'static,
    R: TryInto<wit::Response, Error = Error> + Send + Sync + 'static,
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

impl<T> TryFrom<wit::Config> for Config<T>
where
    T: serde::de::DeserializeOwned,
{
    type Error = Error;

    fn try_from(config: wit::Config) -> Result<Self, Self::Error> {
        let t = serde_json::from_slice(config.as_slice()).map_err(|_| Error::BadConfig)?;
        Ok(Config(t))
    }
}

impl<T> std::ops::Deref for Config<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Params<T>(pub T);

impl<T> TryFrom<wit::Event> for Params<T>
where
    T: serde::de::DeserializeOwned,
{
    type Error = Error;

    fn try_from(value: wit::Event) -> Result<Self, Self::Error> {
        let bytes = match value {
            wit::Event::Request(x) => x,
            _ => todo!(),
        };

        let t = serde_json::from_slice(bytes.as_slice()).map_err(|_| Error::BadParams)?;
        Ok(Params(t))
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

impl<T> TryFrom<Json<T>> for wit::Response
where
    T: serde::Serialize,
{
    type Error = Error;

    fn try_from(value: Json<T>) -> Result<Self, Self::Error> {
        let bytes = serde_json::to_vec(&value.0)?;
        Ok(wit::Response::Json(bytes))
    }
}

impl<T> std::ops::Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct NewTx(pub Box<dyn crate::txbuilder::TxExpr>);

impl TryInto<wit::Response> for NewTx {
    type Error = Error;

    fn try_into(self) -> Result<wit::Response, Self::Error> {
        let ledger = crate::txbuilder::ExtLedgerFacade;
        let tx = crate::txbuilder::build(self.0, ledger)?;
        let cbor = pallas_codec::minicbor::to_vec(&tx).unwrap();
        Ok(wit::Response::PartialTx(cbor))
    }
}

impl crate::_internal::Worker {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn init(&mut self, config: wit::Config) {
        self.config = Some(config);
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
