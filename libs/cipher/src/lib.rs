//! Cryptography utilities for AgilePlus
//!
//! Provides secure encryption, key derivation, and hashing for:
//! - Securing credentials (database passwords, API keys)
//! - Token generation and validation
//! - Data encryption at rest

pub mod cipher;
pub mod key_derivation;
pub mod hash;
pub mod error;

pub use cipher::{AesGcmCipher, Cipher};
pub use key_derivation::{derive_key, derive_key_with_salt, Argon2Config};
pub use hash::{Hash, sha256_hash, Sha256Hash};
pub use error::{CipherError, CipherResult};
