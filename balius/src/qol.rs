use std::ops::Deref;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UtxoSource;

#[derive(Serialize, Deserialize)]
pub struct UnsignedTx(pub Vec<u8>);

impl Into<crate::bindings::Response> for UnsignedTx {
    fn into(self) -> crate::bindings::Response {
        crate::bindings::Response::PartialTx(self.0)
    }
}

#[derive(Debug)]
pub enum Error {
    Internal(String),
}

impl From<Error> for crate::bindings::balius::odk::driver::HandleError {
    fn from(error: Error) -> Self {
        match error {
            Error::Internal(_) => 0,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Env<T>(pub T);

impl<T> Deref for Env<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<crate::bindings::Env> for Env<T>
where
    T: DeserializeOwned,
{
    fn from(value: crate::bindings::Env) -> Self {
        Env(serde_json::from_slice(&value).unwrap())
    }
}
