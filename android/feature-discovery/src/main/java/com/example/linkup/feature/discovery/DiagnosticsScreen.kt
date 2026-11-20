package com.example.linkup.feature.discovery

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.example.linkup.core.ui.components.GlassCard
import com.example.linkup.core.ui.components.RadarCanvas
import com.example.linkup.core.ui.components.RadarPeer
import com.example.linkup.core.ui.theme.CyberCyan
import com.example.linkup.core.ui.theme.NeonGreen
import com.example.linkup.core.ui.theme.VoidDark

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DiagnosticsScreen() {
    val mockPeers = listOf(
        RadarPeer(-0.4f, 0.3f, true),
        RadarPeer(0.5f, -0.6f, false),
        RadarPeer(0.2f, 0.7f, true)
    )

    Scaffold(
        containerColor = VoidDark,
        topBar = {
            TopAppBar(
                title = { Text("Mesh Network Radar & Telemetry", fontWeight = FontWeight.Bold) },
                colors = TopAppBarDefaults.topAppBarColors(containerColor = VoidDark)
            )
        }
    ) { padding ->
        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            item {
                GlassCard {
                    Text(
                        text = "Live Mesh Node Radar",
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.Bold,
                        color = NeonGreen
                    )
                    Spacer(modifier = Modifier.height(12.dp))
                    RadarCanvas(peers = mockPeers)
                }
            }

            item {
                GlassCard {
                    Text(text = "Active Transports", style = MaterialTheme.typography.titleMedium, fontWeight = FontWeight.Bold)
                    Spacer(modifier = Modifier.height(8.dp))
                    Text(text = "• Google Nearby Connections (Active)", color = NeonGreen)
                    Text(text = "• Bluetooth Low Energy (BLE Mesh)", color = CyberCyan)
                }
            }

            item {
                GlassCard {
                    Text(text = "Mesh Storage & Deduplication Stats", style = MaterialTheme.typography.titleMedium, fontWeight = FontWeight.Bold)
                    Spacer(modifier = Modifier.height(8.dp))
                    Text(text = "Active Peer Connections: 4 Nodes")
                    Text(text = "Store-and-Forward Buffer: 2 Encrypted Packets")
                    Text(text = "LRU Deduplication Filter: 18 Hashes")
                }
            }
        }
    }
}
