package com.example.linkup.feature.settings

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.example.linkup.core.ui.components.GlassCard
import com.example.linkup.core.ui.theme.NeonGreen
import com.example.linkup.core.ui.theme.VoidDark

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun SettingsScreen() {
    var eventModeEnabled by remember { mutableStateOf(true) }
    var storeForwardEnabled by remember { mutableStateOf(true) }

    Scaffold(
        containerColor = VoidDark,
        topBar = {
            TopAppBar(
                title = { Text("Settings & Stadium Mode", fontWeight = FontWeight.Bold) },
                colors = TopAppBarDefaults.topAppBarColors(containerColor = VoidDark)
            )
        }
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            GlassCard {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Column(modifier = Modifier.weight(1f)) {
                        Text("High-Density Stadium / Event Mode", fontWeight = FontWeight.Bold)
                        Text("Optimizes BLE scan intervals for stadium crowds and concerts.", style = MaterialTheme.typography.bodySmall)
                    }
                    Switch(
                        checked = eventModeEnabled,
                        onCheckedChange = { eventModeEnabled = it },
                        colors = SwitchDefaults.colors(checkedThumbColor = NeonGreen)
                    )
                }
            }

            GlassCard {
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween
                ) {
                    Column(modifier = Modifier.weight(1f)) {
                        Text("Store-and-Forward Mesh Cache", fontWeight = FontWeight.Bold)
                        Text("Relay encrypted packets for offline contacts.", style = MaterialTheme.typography.bodySmall)
                    }
                    Switch(
                        checked = storeForwardEnabled,
                        onCheckedChange = { storeForwardEnabled = it },
                        colors = SwitchDefaults.colors(checkedThumbColor = NeonGreen)
                    )
                }
            }
        }
    }
}
