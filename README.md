# LinkUp 🛰️💬
### High-Performance Decentralized Offline Mesh Communication System

[![LinkUp CI](https://github.com/Uday-Bhoi/linkup/actions/workflows/ci.yml/badge.svg)](https://github.com/Uday-Bhoi/linkup/actions/workflows/ci.yml)
[![Platform](https://img.shields.io/badge/Platform-Android%20%7C%20Rust-brightgreen.svg)](https://developer.android.com)
[![Kotlin](https://img.shields.io/badge/Kotlin-2.2.10-purple.svg)](https://kotlinlang.org)
[![Compose](https://img.shields.io/badge/Jetpack%20Compose-BOM%202026.02.01-blue.svg)](https://developer.android.com/jetpack/compose)
[![Rust](https://img.shields.io/badge/Rust-2021%20Edition-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](#-license)

---

## 📖 Overview

**LinkUp** is an offline-first, peer-to-peer (P2P) mesh messaging application designed for resilient, encrypted communication in zero-connectivity environments. Whether during cellular network blackouts, natural disasters, stadium concerts, underground transit, flights, or wilderness expeditions, LinkUp allows devices to discover each other, form dynamic multi-hop ad-hoc mesh networks, and exchange encrypted messages without relying on cell towers, central servers, or an internet connection.

The project pairs a high-performance, secure **Rust Mesh Core** (handling binary packet framing, store-and-forward routing, cryptographic identity, and encryption) with a modern **Android Jetpack Compose UI** featuring a glassmorphic cyberpunk aesthetic, real-time node radar visualization, and QR-based public key exchange.

---

## ✨ Key Features

- 🌐 **Zero-Internet Multi-Hop Mesh**: Automatically relays encrypted packets through intermediary peer nodes using hop counting and TTL (Time-To-Live) decay.
- 🔐 **End-to-End Cryptography**:
  - **Ed25519** digital signatures for cryptographic identity and non-repudiation.
  - **X25519** ECDH key exchange with **HKDF-SHA256** key derivation.
  - **ChaCha20-Poly1305** AEAD authenticated payload encryption.
- 📦 **Compact Binary Wire Protocol**:
  - Fixed-framing binary protocol with magic header (`LU` -> `0x4C 0x55`), versioning, bitwise packet flags, and deduplication IDs.
- 🔄 **Store-and-Forward Offline Caching**:
  - Packets intended for currently offline contacts are cached in memory and automatically flushed once the destination node reconnects.
- 📡 **Radio & Transport Agnostic**:
  - Integrates Bluetooth Low Energy (BLE) Mesh and Google Nearby Connections for low-latency ad-hoc link formation.
- 🎯 **High-Density Stadium / Event Mode**:
  - Dynamically optimizes BLE scan intervals and duty cycles in crowded environments (e.g. festivals, rallies, sporting events).
- 🛰️ **Live Mesh Network Radar**:
  - Real-time animated canvas visualizing nearby nodes, signal strength (RSSI), active links, and network telemetry.
- ⚡ **QR-Based Key Exchange**:
  - Out-of-band identity verification and contact pairing via Ed25519 public key QR codes.

---

## 🏗️ Architecture & Project Structure

The project is structured as a modular monorepo combining native Android Gradle modules and a Rust Cargo workspace bridged via UniFFI:

```text
linkup/
├── .github/
│   └── workflows/
│       └── ci.yml                 # Automated CI for Rust workspace and Android lint/tests
├── android/                       # Android Multi-Module Project
│   ├── app/                       # Application entry point, NavHost, Main Activity
│   ├── core-ui/                   # Theme (VoidDark, NeonGreen), GlassCard, RadarCanvas, Onboarding
│   ├── core-domain/               # Domain entities (DomainMeshPacket, MeshPeerNode) & Repository interfaces
│   ├── core-data/                 # MeshRepositoryImpl & NativeLibLoader JNI/FFI bridge
│   ├── core-network/              # NearbyTransportDriver & radio abstractions
│   ├── feature-chat/              # Chat list, 1-on-1 conversation UI, message state
│   ├── feature-friends/           # Contact list & QR Code identity exchange
│   ├── feature-discovery/         # Real-time Radar screen & mesh diagnostics telemetry
│   ├── feature-settings/          # Stadium mode & store-and-forward mesh controls
│   ├── gradle/                    # Version catalog (libs.versions.toml) & wrapper
│   ├── build.gradle.kts           # Root Android build configuration
│   └── settings.gradle.kts        # Android module definitions
├── rust/                          # Rust Core Mesh Engine (Workspace)
│   ├── Cargo.toml                 # Rust workspace configuration
│   ├── linkup-protocol/           # Binary packet encoding/decoding, framing, magic verification
│   ├── linkup-crypto/             # Ed25519 identity, X25519 ECDH, ChaCha20-Poly1305 AEAD
│   ├── linkup-routing/            # Staged mesh routing engine & decision pipeline
│   ├── linkup-transport/          # Peer connection abstractions & transport drivers
│   ├── linkup-storage/            # In-memory deduplication filters & store-and-forward queue
│   └── linkup-ffi/                # UniFFI scaffolding bridging Rust core to Kotlin
├── build.gradle.kts               # Top-level workspace Gradle configuration
├── settings.gradle.kts            # Multi-project Gradle settings
└── README.md
```

### Staged Mesh Routing Pipeline

```text
[ Incoming Radio Packet ]
           │
           ▼
[ Magic Header & Version Check ] ──(Invalid)──► [ Drop: Invalid Packet ]
           │ (Valid)
           ▼
[ LRU Deduplication Cache ] ───────(Seen)─────► [ Drop: Duplicate Loop ]
           │ (New Packet)
           ▼
[ Recipient Check ] ──────────(Matches Local)─► [ Decrypt ChaCha20-Poly1305 & Deliver to UI ]
           │ (Different Node)
           ▼
[ TTL Check (TTL > 1) ] ───────────(TTL <= 1)─► [ Drop: Expired TTL ]
           │ (TTL Valid)
           ▼
[ Destination Node Reachability ]
     ├── (Peer is Online)  ──► Decrement TTL, Increment Hop ──► [ Relay to Next Peer ]
     └── (Peer is Offline) ──► [ Enqueue into Store-and-Forward Cache ]
```

---

## 🛠️ Tech Stack & Dependencies

### Android & Kotlin
- **Language**: Kotlin 2.2.10
- **UI Toolkit**: Jetpack Compose (BOM `2026.02.01`) + Material 3
- **Architecture**: MVI / MVVM with Clean Architecture principles
- **Asynchronous**: Kotlin Coroutines & Reactive Flows (`StateFlow`, `SharedFlow`)
- **Navigation**: Jetpack Navigation Compose
- **Target SDK**: Android 36 (Minimum SDK: 27 / Android 8.1 Oreo)
- **Build System**: Gradle 9.6.1 + Android Gradle Plugin 9.3.1

### Rust Core
- **Edition**: Rust 2021
- **Cryptography**: `ed25519-dalek`, `x25519-dalek`, `chacha20poly1305`, `hkdf`, `sha2`
- **Serialization**: `serde`, `bincode`
- **Error Handling**: `thiserror`
- **FFI Scaffolding**: Mozilla `uniffi`

---

## 🚀 Getting Started & Setup Guide

Follow these instructions to clone, build, and run LinkUp on your desktop.

### 1. Prerequisites
- **Git**: Installed and configured on your system.
- **Java Development Kit (JDK)**: **JDK 17** (Temurin, OpenJDK 17, or Android Studio's bundled JBR).
- **Android Studio**: Android Studio Koala / Ladybug (or newer) recommended.
- **Android SDK**: API 36 with Build-Tools and NDK installed via Android Studio SDK Manager.
- *(Optional)* **Rust**: Rust stable (`rustup toolchain install stable`) if you plan to modify or recompile the native Rust crates.

### 2. Clone the Repository
```bash
git clone https://github.com/Uday-Bhoi/linkup.git
cd linkup
```

### 3. Open in Android Studio
1. Launch Android Studio.
2. Select **Open** and browse to the cloned `linkup` directory (you can open the root folder or the `android` directory).
3. Allow Gradle to sync dependencies.

### 4. Build and Run via Command Line

**On Windows (PowerShell / Command Prompt):**
```powershell
.\gradlew.bat assembleDebug
```

**On macOS / Linux:**
```bash
chmod +x ./gradlew
./gradlew assembleDebug
```

To run unit tests:
```bash
./gradlew test
```

### 5. Running the Rust Workspace (Optional)
If you wish to test and lint the Rust core modules independently:
```bash
cd rust
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --all -- --check
```

---

## ⚙️ Configuration Notes & Troubleshooting

### Local JDK Configuration (`gradle.properties`)
- In `gradle.properties`, Gradle specifies the memory options and JVM environment.
- If you encounter a build error such as `The supplied javaHome seems to be invalid` or a toolchain discrepancy, verify that your `JAVA_HOME` environment variable points to a valid **JDK 17** installation, or configure the path in your local machine's `~/.gradle/gradle.properties`:
  ```properties
  org.gradle.java.home=/path/to/your/jdk-17
  ```
- In Android Studio, ensure that **Settings > Build, Execution, Deployment > Build Tools > Gradle > Gradle JDK** is set to **JDK 17** (or the Embedded JDK).

### Fallback Simulation Mode
- If `liblinkup_ffi.so` is not pre-compiled for your specific target architecture, `NativeLibLoader` automatically falls back to the high-level simulated mesh mode, allowing full UI design, telemetry, and workflow testing on standard emulators without requiring manual NDK compilation.

---

## 🤝 Contributing

We welcome contributions from the community! To contribute:

1. **Fork the Repository** on GitHub.
2. **Create a Feature Branch**:
   ```bash
   git checkout -b feature/awesome-mesh-feature
   ```
3. **Commit Your Changes**:
   ```bash
   git commit -m "Add awesome mesh routing feature"
   ```
4. **Push to Your Branch**:
   ```bash
   git push origin feature/awesome-mesh-feature
   ```
5. **Open a Pull Request**: Submit your PR with a clear summary of your enhancements.

---

## 📄 License

This project is open-source under the MIT License.
