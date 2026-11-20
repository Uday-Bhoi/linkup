//! LinkUp Binary Protocol Crate
//!
//! Handles compact binary packet encoding, decoding, magic header verification,
//! and packet framing for offline mesh networking.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Protocol Identifier Magic Byte ('L' 'U' -> 0x4C 0x55)
pub const MAGIC_BYTES: [u8; 2] = [0x4C, 0x55];

/// Current Protocol Version
pub const PROTOCOL_VERSION: u8 = 1;

/// Default Time-To-Live (TTL) for multi-hop packet forwarding
pub const DEFAULT_TTL: u8 = 7;

#[derive(Error, Debug, PartialEq)]
pub enum ProtocolError {
    #[error("Invalid magic header bytes")]
    InvalidMagic,

    #[error("Unsupported protocol version: {0}")]
    UnsupportedVersion(u8),

    #[error("Serialization failure: {0}")]
    SerializationFailed(String),

    #[error("Deserialization failure: {0}")]
    DeserializationFailed(String),

    #[error("Packet TTL expired")]
    TtlExpired,

    #[error("Payload exceeds maximum MTU size ({0} bytes)")]
    MtuExceeded(usize),
}

/// Bitwise Header Flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketFlags(pub u8);

impl PacketFlags {
    pub const DIRECT_MESSAGE: u8 = 1 << 0;
    pub const BROADCAST: u8 = 1 << 1;
    pub const ACK_REQUEST: u8 = 1 << 2;
    pub const ENCRYPTED: u8 = 1 << 3;

    pub fn new(flags: u8) -> Self {
        Self(flags)
    }

    pub fn is_direct(&self) -> bool {
        (self.0 & Self::DIRECT_MESSAGE) != 0
    }

    pub fn is_broadcast(&self) -> bool {
        (self.0 & Self::BROADCAST) != 0
    }

    pub fn is_ack_requested(&self) -> bool {
        (self.0 & Self::ACK_REQUEST) != 0
    }

    pub fn is_encrypted(&self) -> bool {
        (self.0 & Self::ENCRYPTED) != 0
    }
}

/// High-Level Packet Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum PacketType {
    Beacon = 1,
    DirectMessage = 2,
    BroadcastMessage = 3,
    Acknowledgement = 4,
    RouteDiscovery = 5,
}

/// Core Binary Packet Header (Fixed Framing)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketHeader {
    pub magic: [u8; 2],
    pub version: u8,
    pub packet_type: PacketType,
    pub flags: PacketFlags,
    pub ttl: u8,
    pub hop_count: u8,
    pub sender_id: [u8; 32],
    pub recipient_id: [u8; 32],
    pub packet_id: [u8; 16],
    pub timestamp: u64,
    pub payload_len: u32,
}

impl PacketHeader {
    pub fn new(
        packet_type: PacketType,
        flags: PacketFlags,
        sender_id: [u8; 32],
        recipient_id: [u8; 32],
        packet_id: [u8; 16],
        timestamp: u64,
        payload_len: u32,
    ) -> Self {
        Self {
            magic: MAGIC_BYTES,
            version: PROTOCOL_VERSION,
            packet_type,
            flags,
            ttl: DEFAULT_TTL,
            hop_count: 0,
            sender_id,
            recipient_id,
            packet_id,
            timestamp,
            payload_len,
        }
    }

    pub fn validate(&self) -> Result<(), ProtocolError> {
        if self.magic != MAGIC_BYTES {
            return Err(ProtocolError::InvalidMagic);
        }
        if self.version != PROTOCOL_VERSION {
            return Err(ProtocolError::UnsupportedVersion(self.version));
        }
        if self.ttl == 0 {
            return Err(ProtocolError::TtlExpired);
        }
        Ok(())
    }

    pub fn decrement_ttl(&mut self) -> Result<(), ProtocolError> {
        if self.ttl <= 1 {
            self.ttl = 0;
            return Err(ProtocolError::TtlExpired);
        }
        self.ttl -= 1;
        self.hop_count += 1;
        Ok(())
    }
}

/// LinkUp Wire Packet Container
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MeshPacket {
    pub header: PacketHeader,
    pub payload: Vec<u8>,
    pub signature: [u8; 64],
}

impl MeshPacket {
    pub fn new(header: PacketHeader, payload: Vec<u8>, signature: [u8; 64]) -> Self {
        Self {
            header,
            payload,
            signature,
        }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, ProtocolError> {
        self.header.validate()?;
        bincode::serialize(self).map_err(|e| ProtocolError::SerializationFailed(e.to_string()))
    }

    pub fn deserialize(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let packet: Self = bincode::deserialize(bytes)
            .map_err(|e| ProtocolError::DeserializationFailed(e.to_string()))?;
        packet.header.validate()?;
        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_sender() -> [u8; 32] {
        [0x01; 32]
    }

    fn sample_recipient() -> [u8; 32] {
        [0x02; 32]
    }

    fn sample_packet_id() -> [u8; 16] {
        [0xAA; 16]
    }

    #[test]
    fn test_valid_packet_serialization() {
        let header = PacketHeader::new(
            PacketType::DirectMessage,
            PacketFlags::new(PacketFlags::DIRECT_MESSAGE | PacketFlags::ENCRYPTED),
            sample_sender(),
            sample_recipient(),
            sample_packet_id(),
            1700000000,
            12,
        );

        let packet = MeshPacket::new(header, b"Hello LinkUp".to_vec(), [0x77; 64]);
        let serialized = packet.serialize().expect("Serialization failed");
        let deserialized = MeshPacket::deserialize(&serialized).expect("Deserialization failed");

        assert_eq!(packet, deserialized);
        assert_eq!(deserialized.header.ttl, DEFAULT_TTL);
        assert_eq!(deserialized.header.hop_count, 0);
        assert!(deserialized.header.flags.is_direct());
        assert!(deserialized.header.flags.is_encrypted());
    }

    #[test]
    fn test_invalid_magic_bytes() {
        let mut header = PacketHeader::new(
            PacketType::BroadcastMessage,
            PacketFlags::new(PacketFlags::BROADCAST),
            sample_sender(),
            sample_recipient(),
            sample_packet_id(),
            1700000000,
            5,
        );
        header.magic = [0x00, 0x00];

        let packet = MeshPacket::new(header, b"test".to_vec(), [0x00; 64]);
        let serialized = bincode::serialize(&packet).unwrap();
        let result = MeshPacket::deserialize(&serialized);

        assert_eq!(result.unwrap_err(), ProtocolError::InvalidMagic);
    }

    #[test]
    fn test_ttl_decrement() {
        let mut header = PacketHeader::new(
            PacketType::DirectMessage,
            PacketFlags::new(PacketFlags::DIRECT_MESSAGE),
            sample_sender(),
            sample_recipient(),
            sample_packet_id(),
            1700000000,
            0,
        );

        assert_eq!(header.ttl, 7);
        assert_eq!(header.hop_count, 0);

        header.decrement_ttl().unwrap();
        assert_eq!(header.ttl, 6);
        assert_eq!(header.hop_count, 1);

        // Decrement down to 0
        for _ in 0..5 {
            header.decrement_ttl().unwrap();
        }

        assert_eq!(header.ttl, 1);
        let err = header.decrement_ttl().unwrap_err();
        assert_eq!(err, ProtocolError::TtlExpired);
    }
}
