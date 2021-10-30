use ed25519_compact::PublicKey as Ed25519PublicKey;
use jwt_compact::{
    alg::{
        Ed25519, Es256k, Hs256, Hs256Key, Hs384, Hs384Key, Hs512, Hs512Key, Rsa, RsaPublicKey,
        SecretBytes,
    },
    jwk::{JsonWebKey, JwkError},
    AlgorithmExt, Claims, Token, UntrustedToken, ValidationError,
};
use k256::ecdsa::VerifyingKey as K256PublicKey;
use sha2::Sha256;

use std::convert::TryFrom;

pub type GenericToken = Token<serde_json::Value>;
pub type GenericClaims = Claims<serde_json::Value>;

#[derive(Debug)]
pub enum KeyInstance {
    Symmetric(SecretBytes<'static>),
    Rsa(RsaPublicKey),
    Ed25519(Ed25519PublicKey),
    K256(K256PublicKey),
}

impl KeyInstance {
    pub fn new(jwk: &JsonWebKey<'_>) -> Result<Self, JwkError> {
        match jwk {
            JsonWebKey::Symmetric { secret } => {
                let secret = SecretBytes::owned(secret.to_vec());
                Ok(Self::Symmetric(secret))
            }
            JsonWebKey::Rsa { .. } => RsaPublicKey::try_from(jwk).map(Self::Rsa),
            JsonWebKey::KeyPair { .. } => Ed25519PublicKey::try_from(jwk).map(Self::Ed25519),
            JsonWebKey::EllipticCurve { .. } => K256PublicKey::try_from(jwk).map(Self::K256),
            _ => unreachable!(),
        }
    }

    pub fn verify_token(
        &self,
        token: &UntrustedToken<'_>,
    ) -> Result<GenericToken, ValidationError> {
        let alg = token.algorithm();
        match self {
            Self::Symmetric(secret) => match alg {
                "HS256" => Hs256.validate_integrity(token, &Hs256Key::new(secret)),
                "HS384" => Hs384.validate_integrity(token, &Hs384Key::new(secret)),
                "HS512" => Hs512.validate_integrity(token, &Hs512Key::new(secret)),
                _ => Err(ValidationError::AlgorithmMismatch {
                    expected: "HS256, HS384 or HS512".to_owned(),
                    actual: alg.to_owned(),
                }),
            },

            // FIXME: check alg
            Self::Rsa(key) => Rsa::with_name(alg).validate_integrity(token, key),

            Self::Ed25519(key) => Ed25519.validate_integrity(token, key),

            Self::K256(key) => Es256k::<Sha256>::default().validate_integrity(token, key),
        }
    }
}
