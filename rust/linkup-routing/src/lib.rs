//! LinkUp Routing Engine Crate (`linkup-routing`)
//!
//! Implements Staged Mesh Routing logic:
//! Stage 1: Local delivery check (Recipient == Local Node ID).
//! Stage 2: Duplicate packet filtering via `DeduplicationStore`.
//! Stage 3: TTL decay & hop counter increment.
//! Stage 4: Direct delivery to connected peer vs multi-hop broadcast.
//! Stage 5: Store-and-Forward caching for offline peers.

use linkup_protocol::MeshPacket;
use linkup_storage::{DeduplicationStore, StoreAndForwardQueue};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Outcome of processing an incoming or outgoing MeshPacket
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoutingDecision {
    /// Packet is addressed to local node -> Deliver to Application UI
    DeliverLocal(MeshPacket),

    /// Packet should be relayed over radio transport to adjacent peer nodes
    Relay(MeshPacket),

    /// Recipient is currently offline -> Cached in Store-and-Forward Queue
    StoreForOffline(MeshPacket),

    /// Packet dropped due to expired TTL (`ttl == 0`)
    DropExpired,

    /// Packet dropped because it has already been processed by this node
    DropDuplicate,
}

/// Core Staged Mesh Routing Engine
pub struct RoutingEngine {
    local_node_id: [u8; 32],
    dedup_store: DeduplicationStore,
    storage_queue: StoreAndForwardQueue,
    connected_peers: HashSet<[u8; 32]>,
}

impl RoutingEngine {
    pub fn new(local_node_id: [u8; 32], dedup_capacity: usize, max_queue_per_peer: usize) -> Self {
        Self {
            local_node_id,
            dedup_store: DeduplicationStore::new(dedup_capacity),
            storage_queue: StoreAndForwardQueue::new(max_queue_per_peer),
            connected_peers: HashSet::new(),
        }
    }

    /// Register a newly connected active peer node ID
    pub fn on_peer_connected(&mut self, peer_id: [u8; 32]) -> Vec<MeshPacket> {
        self.connected_peers.insert(peer_id);
        // Flush any stored packets queued while peer was offline
        self.storage_queue.drain_for_peer(&peer_id)
    }

    /// Register a peer disconnection
    pub fn on_peer_disconnected(&mut self, peer_id: &[u8; 32]) {
        self.connected_peers.remove(peer_id);
    }

    /// Process incoming mesh packet through Staged Pipeline
    pub fn process_packet(&mut self, mut packet: MeshPacket) -> RoutingDecision {
        // Stage 1: Deduplication Check
        if self.dedup_store.insert_or_check(&packet.header.packet_id) {
            return RoutingDecision::DropDuplicate;
        }

        // Stage 2: TTL Validation & Decrement
        if packet.header.ttl <= 1 {
            return RoutingDecision::DropExpired;
        }
        packet.header.ttl -= 1;
        packet.header.hop_count += 1;

        // Stage 3: Local Node Recipient Check
        if packet.header.recipient_id == self.local_node_id {
            return RoutingDecision::DeliverLocal(packet);
        }

        // Stage 4: Active Peer Reachability vs Store & Forward Caching
        if self.connected_peers.contains(&packet.header.recipient_id) || packet.header.flags.is_broadcast() {
            RoutingDecision::Relay(packet)
        } else {
            match self.storage_queue.enqueue(packet.clone()) {
                Ok(_) => RoutingDecision::StoreForOffline(packet),
                Err(_) => RoutingDecision::Relay(packet), // Fallback flood if storage queue full
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use linkup_protocol::{PacketFlags, PacketHeader, PacketType};

    fn sample_node(id: u8) -> [u8; 32] {
        [id; 32]
    }

    fn sample_packet(sender: [u8; 32], recipient: [u8; 32], packet_id: [u8; 16], ttl: u8) -> MeshPacket {
        let header = PacketHeader::new(
            PacketType::DirectMessage,
            PacketFlags::new(PacketFlags::DIRECT_MESSAGE),
            sender,
            recipient,
            packet_id,
            1700000000,
            5,
        );
        let mut packet = MeshPacket::new(header, b"routing test".to_vec(), [0x00; 64]);
        packet.header.ttl = ttl;
        packet
    }

    #[test]
    fn test_local_delivery_routing() {
        let local_id = sample_node(1);
        let sender_id = sample_node(2);
        let mut engine = RoutingEngine::new(local_id, 100, 10);

        let packet = sample_packet(sender_id, local_id, [0x01; 16], 7);
        let decision = engine.process_packet(packet);

        if let RoutingDecision::DeliverLocal(delivered) = decision {
            assert_eq!(delivered.header.recipient_id, local_id);
            assert_eq!(delivered.header.hop_count, 1);
            assert_eq!(delivered.header.ttl, 6);
        } else {
            panic!("Expected DeliverLocal decision");
        }
    }

    #[test]
    fn test_duplicate_packet_drop() {
        let local_id = sample_node(1);
        let mut engine = RoutingEngine::new(local_id, 100, 10);

        let packet = sample_packet(sample_node(2), sample_node(3), [0xAA; 16], 7);

        let decision1 = engine.process_packet(packet.clone());
        assert!(matches!(decision1, RoutingDecision::StoreForOffline(_)));

        let decision2 = engine.process_packet(packet);
        assert_eq!(decision2, RoutingDecision::DropDuplicate);
    }

    #[test]
    fn test_expired_ttl_drop() {
        let local_id = sample_node(1);
        let mut engine = RoutingEngine::new(local_id, 100, 10);

        let packet = sample_packet(sample_node(2), sample_node(3), [0x01; 16], 1);
        let decision = engine.process_packet(packet);

        assert_eq!(decision, RoutingDecision::DropExpired);
    }

    #[test]
    fn test_peer_reconnect_store_and_forward_flush() {
        let local_id = sample_node(1);
        let offline_peer = sample_node(5);
        let mut engine = RoutingEngine::new(local_id, 100, 10);

        let packet = sample_packet(local_id, offline_peer, [0x99; 16], 7);
        let decision = engine.process_packet(packet);
        assert!(matches!(decision, RoutingDecision::StoreForOffline(_)));

        // Peer comes online -> drain stored packets
        let flushed = engine.on_peer_connected(offline_peer);
        assert_eq!(flushed.len(), 1);
        assert_eq!(flushed[0].header.recipient_id, offline_peer);
    }
}
