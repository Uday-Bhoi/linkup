//! LinkUp UniFFI Foreign Function Interface (`linkup-ffi`)
//!
//! Exposes thread-safe FFI wrappers bridging the shared Rust Mesh Core Engine
//! to native Android Kotlin `core-domain` repositories using UniFFI proc macros.

use linkup_crypto::NodeIdentity;
use linkup_protocol::{MeshPacket, PacketFlags, PacketHeader, PacketType};
use linkup_routing::{RoutingDecision, RoutingEngine};
use std::sync::Mutex;
use thiserror::Error;

uniffi::setup_scaffolding!();

#[derive(Error, Debug, uniffi::Error)]
pub enum FfiError {
    #[error("Engine lock poisoned")]
    LockPoisoned,

    #[error("Serialization failure: {0}")]
    SerializationFailed(String),

    #[error("Routing error: {0}")]
    RoutingFailed(String),

    #[error("Invalid byte length: {0}")]
    InvalidLength(String),
}

/// FFI High-Level Engine Manager
#[derive(uniffi::Object)]
pub struct LinkUpCoreEngineFFI {
    identity: NodeIdentity,
    routing_engine: Mutex<RoutingEngine>,
}

#[uniffi::export]
impl LinkUpCoreEngineFFI {
    /// Initialize a new Mesh Core Engine with a fresh identity keypair
    #[uniffi::constructor]
    pub fn new() -> Self {
        let identity = NodeIdentity::generate();
        let pub_key = identity.public_key_bytes();
        let routing_engine = RoutingEngine::new(pub_key, 1000, 50);

        Self {
            identity,
            routing_engine: Mutex::new(routing_engine),
        }
    }

    /// Restore engine from existing 32-byte secret key seed
    #[uniffi::constructor]
    pub fn from_secret_seed(seed: Vec<u8>) -> Result<Self, FfiError> {
        let seed_bytes: [u8; 32] = seed
            .try_into()
            .map_err(|_| FfiError::InvalidLength("Seed must be exactly 32 bytes".to_string()))?;
        let identity = NodeIdentity::from_bytes(&seed_bytes);
        let pub_key = identity.public_key_bytes();
        let routing_engine = RoutingEngine::new(pub_key, 1000, 50);

        Ok(Self {
            identity,
            routing_engine: Mutex::new(routing_engine),
        })
    }

    /// Return local device 32-byte Ed25519 Public Key
    pub fn get_public_key(&self) -> Vec<u8> {
        self.identity.public_key_bytes().to_vec()
    }

    /// Sign payload bytes returning 64-byte signature
    pub fn sign_payload(&self, payload: Vec<u8>) -> Vec<u8> {
        self.identity.sign(&payload).to_vec()
    }

    /// Register a newly connected peer node ID over radio transport
    pub fn on_peer_connected(&self, peer_id: Vec<u8>) -> Result<Vec<Vec<u8>>, FfiError> {
        let peer_array: [u8; 32] = peer_id
            .try_into()
            .map_err(|_| FfiError::InvalidLength("Peer ID must be exactly 32 bytes".to_string()))?;

        let mut engine = self.routing_engine.lock().map_err(|_| FfiError::LockPoisoned)?;
        let flushed_packets = engine.on_peer_connected(peer_array);

        let mut serialized_list = Vec::new();
        for packet in flushed_packets {
            let bytes = packet.serialize().map_err(|e| FfiError::SerializationFailed(e.to_string()))?;
            serialized_list.push(bytes);
        }
        Ok(serialized_list)
    }

    /// Process incoming binary packet bytes from radio transport
    pub fn process_incoming_bytes(&self, bytes: Vec<u8>) -> Result<String, FfiError> {
        let packet = MeshPacket::deserialize(&bytes)
            .map_err(|e| FfiError::SerializationFailed(e.to_string()))?;

        let mut engine = self.routing_engine.lock().map_err(|_| FfiError::LockPoisoned)?;
        let decision = engine.process_packet(packet);

        let decision_summary = match decision {
            RoutingDecision::DeliverLocal(_) => "DELIVER_LOCAL",
            RoutingDecision::Relay(_) => "RELAY",
            RoutingDecision::StoreForOffline(_) => "STORE_OFFLINE",
            RoutingDecision::DropExpired => "DROP_EXPIRED",
            RoutingDecision::DropDuplicate => "DROP_DUPLICATE",
        };

        Ok(decision_summary.to_string())
    }

    /// Create and sign an outgoing mesh packet to recipient node ID
    pub fn create_outgoing_packet(
        &self,
        recipient_id: Vec<u8>,
        packet_id: Vec<u8>,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, FfiError> {
        let recipient_array: [u8; 32] = recipient_id
            .try_into()
            .map_err(|_| FfiError::InvalidLength("Recipient ID must be exactly 32 bytes".to_string()))?;
        let packet_id_array: [u8; 16] = packet_id
            .try_into()
            .map_err(|_| FfiError::InvalidLength("Packet ID must be exactly 16 bytes".to_string()))?;

        let header = PacketHeader::new(
            PacketType::DirectMessage,
            PacketFlags::new(PacketFlags::DIRECT_MESSAGE | PacketFlags::ENCRYPTED),
            self.identity.public_key_bytes(),
            recipient_array,
            packet_id_array,
            1700000000,
            payload.len() as u32,
        );

        let signature = self.identity.sign(&payload);
        let packet = MeshPacket::new(header, payload, signature);

        packet.serialize().map_err(|e| FfiError::SerializationFailed(e.to_string()))
    }
}

impl Default for LinkUpCoreEngineFFI {
    fn default() -> Self {
        Self::new()
    }
}
