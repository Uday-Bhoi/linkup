package com.example.linkup.feature.friends

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.example.linkup.core.ui.components.GlassCard
import com.example.linkup.core.ui.theme.CyberCyan
import com.example.linkup.core.ui.theme.NeonGreen
import com.example.linkup.core.ui.theme.VoidDark

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun QrPairingScreen(
    onBack: () -> Unit
) {
    Scaffold(
        containerColor = VoidDark,
        topBar = {
            TopAppBar(
                title = { Text("Offline Identity Exchange", fontWeight = FontWeight.Bold) },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Text("◄", color = NeonGreen, fontWeight = FontWeight.Bold)
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(containerColor = VoidDark)
            )
        }
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(24.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center
        ) {
            GlassCard {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    Text(
                        text = "Ed25519 Public Key QR",
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.Bold,
                        color = NeonGreen
                    )
                    Spacer(modifier = Modifier.height(16.dp))
                    Box(
                        modifier = Modifier
                            .size(200.dp)
                            .background(Color.White),
                        contentAlignment = Alignment.Center
                    ) {
                        Text(
                            text = "[ LinkUp QR Code ]",
                            color = Color.Black,
                            fontWeight = FontWeight.Bold
                        )
                    }
                    Spacer(modifier = Modifier.height(16.dp))
                    Text(
                        text = "ed25519:4f9b8a...12ce",
                        fontFamily = FontFamily.Monospace,
                        color = CyberCyan
                    )
                }
            }
            Spacer(modifier = Modifier.height(24.dp))
            Text(
                text = "Scan peer QR code to exchange public keys without internet access.",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
    }
}
