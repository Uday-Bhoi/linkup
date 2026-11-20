//! LinkUp Transport Abstraction Crate (`linkup-transport`)
//!
//! Provides hardware-agnostic traits, event definitions, capability descriptors,
//! and multi-driver management for underlying peer-to-peer radio transports
//! (Google Nearby Connections, Bluetooth LE Mesh, Wi-Fi Aware).

use async_trait::async_trait;
use linkup_protocol::MeshPacket;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum TransportError {
    #[error("Driver initialization failed: {0}")]
    InitFailed(String),

    #[error("Peer not reachable: {0}")]
    PeerUnreachable(String),

    #[error("Transmission failed: {0}")]
    SendFailed(String),

    #[error("Transport driver disabled or inactive: {0}")]
    DriverInactive(String),

    #[error("Packet exceeds transport MTU limit ({0} bytes)")]
    MtuExceeded(usize),
}

/// Medium Category for underlying radio hardware
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportMedium {
    NearbyConnections,
    BluetoothLe,
    WifiAware,
    Simulated,
}

/// Radio Driver Capability Metadata
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportCapabilities {
    pub medium: TransportMedium,
    pub max_mtu: usize,
    pub max_concurrent_peers: usize,
    pub supports_broadcast: bool,
}

/// Discovered Peer Device Descriptor
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerAddress {
    pub peer_id: String,
    pub medium: TransportMedium,
    pub rssi: Option<i8>,
}

/// Events emitted by hardware transport drivers to the Core Mesh Engine
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportEvent {
    PeerDiscovered(PeerAddress),
    PeerLost(String),
    PacketReceived { sender: PeerAddress, packet: MeshPacket },
    Error(String),
}

/// Event listener callback trait for receiving transport events asynchronously
pub trait TransportEventListener: Send + Sync {
    fn on_event(&self, event: TransportEvent);
}

/// Abstract Hardware Transport Driver Trait
#[async_trait]
pub trait TransportDriver: Send + Sync {
    /// Return transport capabilities (MTU, Medium type)
    fn capabilities(&self) -> TransportCapabilities;

    /// Start peer discovery & advertising
    async fn start(&self) -> Result<(), TransportError>;

    /// Stop peer discovery & release radio resources
    async fn stop(&self) -> Result<(), TransportError>;

    /// Send a serialized mesh packet to a specific peer endpoint
    async fn send_packet(&self, peer_id: &str, packet: &MeshPacket) -> Result<(), TransportError>;

    /// Broadcast a mesh packet to all currently connected nearby peers
    async fn broadcast_packet(&self, packet: &MeshPacket) -> Result<(), TransportError>;
}

/// Manager handling registration and routing across multiple transport drivers
pub struct TransportManager {
    drivers: Vec<Arc<dyn TransportDriver>>,
    listeners: Vec<Arc<dyn TransportEventListener>>,
}

impl Default for TransportManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TransportManager {
    pub fn new() -> Self {
        Self {
            drivers: Vec::new(),
            listeners: Vec::new(),
        }
    }

    /// Register a new hardware transport driver
    pub fn register_driver(&mut self, driver: Arc<dyn TransportDriver>) {
        self.drivers.push(driver);
    }

    /// Add an event listener for receiving peer & packet events
    pub fn add_listener(&mut self, listener: Arc<dyn TransportEventListener>) {
        self.listeners.push(listener);
    }

    /// Dispatch an incoming event from any active driver to registered listeners
    pub fn dispatch_event(&self, event: TransportEvent) {
        for listener in &self.listeners {
            listener.on_event(event.clone());
        }
    }

    /// Broadcast a mesh packet across all active registered drivers
    pub async fn broadcast_all(&self, packet: &MeshPacket) -> Result<(), TransportError> {
        if self.drivers.is_empty() {
            return Err(TransportError::DriverInactive("No active drivers registered".to_string()));
        }
        for driver in &self.drivers {
            driver.broadcast_packet(packet).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use linkup_protocol::{PacketFlags, PacketHeader, PacketType};
    use std::sync::Mutex;

    struct MockListener {
        events: Arc<Mutex<Vec<TransportEvent>>>,
    }

    impl TransportEventListener for MockListener {
        fn on_event(&self, event: TransportEvent) {
            self.events.lock().unwrap().push(event);
        }
    }

    struct MockDriver {
        medium: TransportMedium,
        sent_packets: Arc<Mutex<Vec<MeshPacket>>>,
    }

    #[async_trait]
    impl TransportDriver for MockDriver {
        fn capabilities(&self) -> TransportCapabilities {
            TransportCapabilities {
                medium: self.medium,
                max_mtu: 512,
                max_concurrent_peers: 8,
                supports_broadcast: true,
            }
        }

        async fn start(&self) -> Result<(), TransportError> {
            Ok(())
        }

        async fn stop(&self) -> Result<(), TransportError> {
            Ok(())
        }

        async fn send_packet(&self, _peer_id: &str, packet: &MeshPacket) -> Result<(), TransportError> {
            self.sent_packets.lock().unwrap().push(packet.clone());
            Ok(())
        }

        async fn broadcast_packet(&self, packet: &MeshPacket) -> Result<(), TransportError> {
            self.sent_packets.lock().unwrap().push(packet.clone());
            Ok(())
        }
    }

    fn create_test_packet() -> MeshPacket {
        let header = PacketHeader::new(
            PacketType::DirectMessage,
            PacketFlags::new(PacketFlags::DIRECT_MESSAGE),
            [0x01; 32],
            [0x02; 32],
            [0xAA; 16],
            1700000000,
            5,
        );
        MeshPacket::new(header, b"hello".to_vec(), [0x00; 64])
    }

    #[test]
    fn test_transport_manager_registration_and_dispatch() {
        let mut manager = TransportManager::new();
        let events = Arc::new(Mutex::new(Vec::new()));
        let listener = Arc::new(MockListener { events: events.clone() });

        manager.add_listener(listener);

        let peer = PeerAddress {
            peer_id: "node-123".to_string(),
            medium: TransportMedium::BluetoothLe,
            rssi: Some(-65),
        };

        manager.dispatch_event(TransportEvent::PeerDiscovered(peer.clone()));

        let received_events = events.lock().unwrap();
        assert_eq!(received_events.len(), 1);
        assert_eq!(received_events[0], TransportEvent::PeerDiscovered(peer));
    }

    #[tokio::test]
    async fn test_transport_manager_broadcast() {
        let mut manager = TransportManager::new();
        let sent = Arc::new(Mutex::new(Vec::new()));
        let driver = Arc::new(MockDriver {
            medium: TransportMedium::NearbyConnections,
            sent_packets: sent.clone(),
        });

        manager.register_driver(driver);
        let packet = create_test_packet();

        manager.broadcast_all(&packet).await.unwrap();

        let packets = sent.lock().unwrap();
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0], packet);
    }
}
