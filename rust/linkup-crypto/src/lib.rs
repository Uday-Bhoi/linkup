//! LinkUp Cryptography Crate (`linkup-crypto`)
//!
//! Provides identity key management (Ed25519), Diffie-Hellman key exchange (X25519),
//! HKDF key derivation, and AEAD payload encryption/decryption (ChaCha20-Poly1305).

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use hkdf::Hkdf;
use rand::rngs::OsRng;
use sha2::Sha256;
use thiserror::Error;
use x25519_dalek::{Ephemeralsecret, PublicKey as X25519PublicKey, StaticSecret};

#[derive(Error, Debug, PartialEq)]
pub enum CryptoError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Signature verification failed")]
    InvalidSignature,

    #[error("Key derivation error: {0}")]
    KeyDerivationFailed(String),

    #[error("Invalid public key byte length")]
    InvalidKeyLength,

    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

/// Device Cryptographic Identity (`Ed25519`)
pub struct NodeIdentity {
    signing_key: SigningKey,
}

impl NodeIdentity {
    /// Generate a fresh random device identity keypair using OS entropy
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        Self { signing_key }
    }

    /// Restore an identity from 32-byte secret key seed
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(bytes);
        Self { signing_key }
    }

    /// Export the 32-byte secret key seed
    pub fn to_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    /// Export the 32-byte Ed25519 Public Key
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.signing_key.verifying_key().to_bytes()
    }

    /// Sign data bytes returning 64-byte Ed25519 signature
    pub fn sign(&self, data: &[u8]) -> [u8; 64] {
        let signature: Signature = self.signing_key.sign(data);
        signature.to_bytes()
    }
}

/// Verify an Ed25519 digital signature against public key bytes
pub fn verify_signature(
    public_key_bytes: &[u8; 32],
    data: &[u8],
    signature_bytes: &[u8; 64],
) -> Result<(), CryptoError> {
    let verifying_key = VerifyingKey::from_bytes(public_key_bytes)
        .map_err(|_| CryptoError::InvalidKeyLength)?;
    let signature = Signature::from_bytes(signature_bytes);

    verifying_key
        .verify(data, &signature)
        .map_err(|_| CryptoError::InvalidSignature)
}

/// Perform X25519 Diffie-Hellman Key Exchange and derive 256-bit symmetric key via HKDF-SHA256
pub fn derive_shared_secret(
    my_static_secret: &[u8; 32],
    peer_public_key_bytes: &[u8; 32],
    info_context: &[u8],
) -> Result<[u8; 32], CryptoError> {
    let secret = StaticSecret::from(*my_static_secret);
    let peer_public = X25519PublicKey::from(*peer_public_key_bytes);

    let dh_shared = secret.diffie_hellman(&peer_public);

    let hk = Hkdf::<Sha256>::new(None, dh_shared.as_bytes());
    let mut okm = [0u8; 32];
    hk.expand(info_context, &mut okm)
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;

    Ok(okm)
}

/// Encrypt plaintext using ChaCha20-Poly1305 AEAD with HKDF symmetric key
pub fn encrypt_payload(
    symmetric_key: &[u8; 32],
    nonce_bytes: &[u8; 12],
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let cipher = ChaCha20Poly1305::new(symmetric_key.into());
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| CryptoError::EncryptionFailed(e.to_string()))
}

/// Decrypt ciphertext using ChaCha20-Poly1305 AEAD with HKDF symmetric key
pub fn decrypt_payload(
    symmetric_key: &[u8; 32],
    nonce_bytes: &[u8; 12],
    ciphertext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let cipher = ChaCha20Poly1305::new(symmetric_key.into());
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| CryptoError::DecryptionFailed(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_signature_verification() {
        let identity = NodeIdentity::generate();
        let pub_key = identity.public_key_bytes();
        let message = b"Mesh Packet Signed Data Payload";

        let signature = identity.sign(message);
        assert!(verify_signature(&pub_key, message, &signature).is_ok());

        let tampered_message = b"Tampered Packet Signed Data Payload";
        assert_eq!(
            verify_signature(&pub_key, tampered_message, &signature),
            Err(CryptoError::InvalidSignature)
        );
    }

    #[test]
    fn test_ecdh_shared_secret_derivation() {
        let alice_secret_bytes = [0x11; 32];
        let bob_secret_bytes = [0x22; 32];

        let alice_secret = StaticSecret::from(alice_secret_bytes);
        let alice_public = X25519PublicKey::from(&alice_secret);

        let bob_secret = StaticSecret::from(bob_secret_bytes);
        let bob_public = X25519PublicKey::from(&bob_secret);

        let context = b"LinkUp-Session-v1";

        let alice_derived = derive_shared_secret(&alice_secret_bytes, bob_public.as_bytes(), context).unwrap();
        let bob_derived = derive_shared_secret(&bob_secret_bytes, alice_public.as_bytes(), context).unwrap();

        assert_eq!(alice_derived, bob_derived);
    }

    #[test]
    fn test_chacha20_poly1305_encryption_decryption() {
        let key = [0x42; 32];
        let nonce = [0x07; 12];
        let plaintext = b"Top Secret Mesh Message Payload";

        let ciphertext = encrypt_payload(&key, &nonce, plaintext).expect("Encryption failed");
        assert_ne!(plaintext.to_vec(), ciphertext);

        let decrypted = decrypt_payload(&key, &nonce, &ciphertext).expect("Decryption failed");
        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn test_tampered_ciphertext_rejection() {
        let key = [0x99; 32];
        let nonce = [0x01; 12];
        let plaintext = b"E2EE Mesh Payload";

        let mut ciphertext = encrypt_payload(&key, &nonce, plaintext).unwrap();
        // Tamper with last byte (auth tag byte)
        let last_idx = ciphertext.len() - 1;
        ciphertext[last_idx] ^= 0xFF;

        let result = decrypt_payload(&key, &nonce, &ciphertext);
        assert!(result.is_err());
    }
}
