use chrono::Duration;
use ed25519_compact::PublicKey as Ed25519PublicKey;
use getrandom::getrandom;
use jwt_compact::{
    alg::{
        Ed25519, Es256k, Hs256, Hs256Key, Hs384, Hs384Key, Hs512, Hs512Key, Rsa, RsaPublicKey,
        SecretBytes,
    },
    jwk::{JsonWebKey, JwkError},
    AlgorithmExt, Claims, Header, TimeOptions, Token, UntrustedToken, ValidationError,
};
use k256::ecdsa::VerifyingKey as K256PublicKey;
use sha2::Sha256;
use uuid::Uuid;
use wasm_bindgen::UnwrapThrowExt;

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
    /// # Errors
    ///
    /// Returns an error if `jwk` is incorrect or not supported.
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

    pub fn random_key() -> Hs256Key {
        let mut bytes = [0_u8; 64];
        getrandom(&mut bytes).expect_throw("cannot access CSPRNG");
        Hs256Key::new(bytes)
    }

    /// # Errors
    ///
    /// Returns an error if the token is not valid. This includes cases when the token has
    /// an algorithm incompatible with this key.
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

            Self::Rsa(key) => alg
                .parse::<Rsa>()
                .map_err(|_| ValidationError::AlgorithmMismatch {
                    expected: "RS* or PS* algorithm".to_owned(),
                    actual: alg.to_owned(),
                })?
                .validate_integrity(token, key),

            Self::Ed25519(key) => Ed25519.validate_integrity(token, key),

            Self::K256(key) => Es256k::<Sha256>::default().validate_integrity(token, key),
        }
    }

    pub fn random_token(key: &Hs256Key) -> String {
        let header = Header::default()
            .with_token_type("JWT")
            .with_key_id(Self::random_uuid().to_string());

        let claims = serde_json::json!({
            "iss": "https://justwebtoken.io/",
            "sub": Self::random_uuid().to_string(),
            "jti": Self::random_uuid().to_string(),
            "aud": ["https://justwebtoken.io/", "https://example.com"],
        });
        let claims = GenericClaims::new(claims)
            .set_duration_and_issuance(&TimeOptions::default(), Duration::hours(1));

        Hs256
            .token(header, &claims, key)
            .expect_throw("cannot create token")
    }

    // Copied verbatim from the `uuid` crate. Using `Uuid::new_v4()` from the crate requires
    // enabling the `wasm-bindgen/js` feature, which we don't want to do
    // (see the `rng` module` as to why).
    fn random_uuid() -> Uuid {
        let mut bytes = [0_u8; 16];
        getrandom::getrandom(&mut bytes).expect_throw("cannot access CSPRNG");

        uuid::Builder::from_bytes(bytes)
            .set_variant(uuid::Variant::RFC4122)
            .set_version(uuid::Version::Random)
            .build()
    }
}
