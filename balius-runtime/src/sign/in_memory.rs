use std::collections::HashMap;

use pallas::crypto::key::ed25519;

use crate::wit::balius::app::sign as wit;

#[derive(Default, Clone)]
pub struct Signer {
    keys: HashMap<String, SignerKey>,
}

impl Signer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_key(&mut self, name: &str, key: impl Into<SignerKey>) {
        self.keys.insert(name.to_string(), key.into());
    }

    pub fn sign_payload(
        &self,
        key_name: &str,
        algorithm: &str,
        payload: wit::Payload,
    ) -> Result<wit::Signature, wit::SignError> {
        let Some(key) = self.keys.get(key_name) else {
            return Err(wit::SignError::KeyNotFound(key_name.to_string()));
        };
        key.sign_payload(algorithm, payload)
    }

    pub fn get_public_key(
        &self,
        key_name: &str,
        algorithm: &str,
    ) -> Result<wit::PublicKey, wit::SignError> {
        let Some(key) = self.keys.get(key_name) else {
            return Err(wit::SignError::KeyNotFound(key_name.to_string()));
        };
        key.get_public_key(algorithm)
    }
}

#[derive(Clone)]
pub enum SignerKey {
    Ed25519(Ed25519Key),
}

impl From<ed25519::SecretKey> for SignerKey {
    fn from(value: ed25519::SecretKey) -> Self {
        Self::Ed25519(Ed25519Key::SecretKey(value))
    }
}

impl From<ed25519::SecretKeyExtended> for SignerKey {
    fn from(value: ed25519::SecretKeyExtended) -> Self {
        Self::Ed25519(Ed25519Key::SecretKeyExtended(value))
    }
}

impl SignerKey {
    fn sign_payload(
        &self,
        algorithm: &str,
        payload: wit::Payload,
    ) -> Result<wit::Signature, wit::SignError> {
        match (algorithm, self) {
            ("ed25519", Self::Ed25519(key)) => Ok(key.sign_payload(payload)),
            (_, _) => Err(wit::SignError::UnsupportedAlgorithm(algorithm.to_string())),
        }
    }

    fn get_public_key(&self, algorithm: &str) -> Result<wit::PublicKey, wit::SignError> {
        match (algorithm, self) {
            ("ed25519", Self::Ed25519(key)) => Ok(key.get_public_key()),
            (_, _) => Err(wit::SignError::UnsupportedAlgorithm(algorithm.to_string())),
        }
    }
}

#[derive(Clone)]
pub enum Ed25519Key {
    SecretKey(ed25519::SecretKey),
    SecretKeyExtended(ed25519::SecretKeyExtended),
}
impl Ed25519Key {
    fn sign_payload(&self, payload: wit::Payload) -> wit::Signature {
        let signature = match self {
            Self::SecretKey(key) => key.sign(payload),
            Self::SecretKeyExtended(key) => key.sign(payload),
        };
        signature.as_ref().to_vec()
    }
    fn get_public_key(&self) -> wit::PublicKey {
        let public_key = match self {
            Self::SecretKey(key) => key.public_key(),
            Self::SecretKeyExtended(key) => key.public_key(),
        };
        public_key.as_ref().to_vec()
    }
}
