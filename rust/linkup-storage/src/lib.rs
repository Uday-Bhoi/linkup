//! LinkUp Storage Layer Crate (`linkup-storage`)
//!
//! Handles Store-and-Forward packet queueing for offline relaying,
//! LRU deduplication hash filters to prevent routing loops, and TTL eviction routines.

use linkup_protocol::MeshPacket;
use std::collections::{HashMap, VecDeque};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum StorageError {
    #[error("Storage capacity reached ({0} packets)")]
    CapacityFull(usize),

    #[error("Packet expired or invalid TTL")]
    ExpiredPacket,

    #[error("Duplicate packet rejected")]
    DuplicatePacket,
}

/// Deduplication Filter tracking recently seen packet IDs
pub struct DeduplicationStore {
    seen_ids: VecDeque<[u8; 16]>,
    capacity: usize,
}

impl DeduplicationStore {
    pub fn new(capacity: usize) -> Self {
        Self {
            seen_ids: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    /// Check if a packet ID has been seen recently. If not, record it.
    pub fn insert_or_check(&mut self, packet_id: &[u8; 16]) -> bool {
        if self.seen_ids.contains(packet_id) {
            true // Already seen (duplicate)
        } else {
            if self.seen_ids.len() >= self.capacity {
                self.seen_ids.pop_front();
            }
            self.seen_ids.push_back(*packet_id);
            false // New packet
        }
    }

    pub fn contains(&self, packet_id: &[u8; 16]) -> bool {
        self.seen_ids.contains(packet_id)
    }
}

/// Store-and-Forward Cache for offline multi-hop relaying
pub struct StoreAndForwardQueue {
    queue: HashMap<[u8; 32], Vec<MeshPacket>>,
    max_packets_per_peer: usize,
}

impl StoreAndForwardQueue {
    pub fn new(max_packets_per_peer: usize) -> Self {
        Self {
            queue: HashMap::new(),
            max_packets_per_peer,
        }
    }

    /// Store a packet destined for a recipient node ID
    pub fn enqueue(&mut self, packet: MeshPacket) -> Result<(), StorageError> {
        if packet.header.ttl == 0 {
            return Err(StorageError::ExpiredPacket);
        }

        let recipient = packet.header.recipient_id;
        let peer_queue = self.queue.entry(recipient).or_insert_with(Vec::new);

        if peer_queue.len() >= self.max_packets_per_peer {
            return Err(StorageError::CapacityFull(self.max_packets_per_peer));
        }

        peer_queue.push(packet);
        Ok(())
    }

    /// Drain and retrieve all stored packets for a re-connected peer
    pub fn drain_for_peer(&mut self, recipient: &[u8; 32]) -> Vec<MeshPacket> {
        self.queue.remove(recipient).unwrap_or_default()
    }

    /// Evict expired packets (`ttl <= 1`) across all queues
    pub fn evict_expired(&mut self) {
        for queue in self.queue.values_mut() {
            queue.retain(|p| p.header.ttl > 1);
        }
        self.queue.retain(|_, q| !q.is_empty());
    }

    /// Return total pending stored packets count
    pub fn len(&self) -> usize {
        self.queue.values().map(|q| q.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use linkup_protocol::{PacketFlags, PacketHeader, PacketType};

    fn create_packet(sender: [u8; 32], recipient: [u8; 32], packet_id: [u8; 16], ttl: u8) -> MeshPacket {
        let header = PacketHeader::new(
            PacketType::DirectMessage,
            PacketFlags::new(PacketFlags::DIRECT_MESSAGE),
            sender,
            recipient,
            packet_id,
            1700000000,
            5,
        );
        let mut packet = MeshPacket::new(header, b"test payload".to_vec(), [0x00; 64]);
        packet.header.ttl = ttl;
        packet
    }

    #[test]
    fn test_deduplication_store() {
        let mut dedup = DeduplicationStore::new(3);
        let id1 = [0x01; 16];
        let id2 = [0x02; 16];
        let id3 = [0x03; 16];
        let id4 = [0x04; 16];

        assert_eq!(dedup.insert_or_check(&id1), false);
        assert_eq!(dedup.insert_or_check(&id1), true); // Duplicate

        assert_eq!(dedup.insert_or_check(&id2), false);
        assert_eq!(dedup.insert_or_check(&id3), false);

        // Queue full (capacity 3), inserting id4 evicts id1
        assert_eq!(dedup.insert_or_check(&id4), false);
        assert_eq!(dedup.contains(&id1), false);
        assert_eq!(dedup.contains(&id4), true);
    }

    #[test]
    fn test_store_and_forward_enqueue_and_drain() {
        let mut sf = StoreAndForwardQueue::new(2);
        let sender = [0xAA; 32];
        let recipient = [0xBB; 32];

        let p1 = create_packet(sender, recipient, [0x01; 16], 7);
        let p2 = create_packet(sender, recipient, [0x02; 16], 5);
        let p3 = create_packet(sender, recipient, [0x03; 16], 3);

        assert!(sf.enqueue(p1.clone()).is_ok());
        assert!(sf.enqueue(p2.clone()).is_ok());

        // Exceeds max per peer (capacity 2)
        assert_eq!(sf.enqueue(p3), Err(StorageError::CapacityFull(2)));

        assert_eq!(sf.len(), 2);

        let drained = sf.drain_for_peer(&recipient);
        assert_eq!(drained.len(), 2);
        assert_eq!(sf.len(), 0);
    }

    #[test]
    fn test_store_and_forward_ttl_eviction() {
        let mut sf = StoreAndForwardQueue::new(5);
        let recipient = [0xCC; 32];

        let valid_packet = create_packet([0x01; 32], recipient, [0x01; 16], 5);
        let expired_packet = create_packet([0x01; 32], recipient, [0x02; 16], 1);

        sf.enqueue(valid_packet).unwrap();
        sf.enqueue(expired_packet).unwrap();

        assert_eq!(sf.len(), 2);
        sf.evict_expired();
        assert_eq!(sf.len(), 1);
    }
}
