package com.example.linkup.feature.chat

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.example.linkup.core.ui.components.GlassCard
import com.example.linkup.core.ui.theme.CyberCyan
import com.example.linkup.core.ui.theme.NeonGreen
import com.example.linkup.core.ui.theme.VoidDark

data class MockChat(val id: String, val name: String, val lastMsg: String, val isDirect: Boolean, val hopCount: Int)

val mockChatsList = listOf(
    MockChat("1", "Alice (BLE Direct)", "Offline message via BLE radio.", true, 0),
    MockChat("2", "Bob (Relayed Mesh)", "Meeting near Stage 2.", false, 2),
    MockChat("3", "Festival Emergency Hub", "Welcome to LinkUp Mesh Network!", false, 1)
)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ChatListScreen(
    onChatClick: (String) -> Unit
) {
    Scaffold(
        containerColor = VoidDark,
        topBar = {
            TopAppBar(
                title = { Text("LinkUp Mesh Chats", fontWeight = FontWeight.Bold) },
                colors = TopAppBarDefaults.topAppBarColors(containerColor = VoidDark)
            )
        }
    ) { padding ->
        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(horizontal = 16.dp, vertical = 8.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            items(mockChatsList) { chat ->
                GlassCard(
                    modifier = Modifier.clickable { onChatClick(chat.id) }
                ) {
                    Row(
                        verticalAlignment = Alignment.CenterVertically
                    ) {
                        Surface(
                            modifier = Modifier
                                .size(42.dp)
                                .clip(CircleShape),
                            color = if (chat.isDirect) NeonGreen else CyberCyan
                        ) {
                            Box(contentAlignment = Alignment.Center) {
                                Text(
                                    text = chat.name.take(1),
                                    color = VoidDark,
                                    fontWeight = FontWeight.Bold
                                )
                            }
                        }
                        Spacer(modifier = Modifier.width(12.dp))
                        Column(modifier = Modifier.weight(1f)) {
                            Text(
                                text = chat.name,
                                style = MaterialTheme.typography.titleMedium,
                                fontWeight = FontWeight.Bold
                            )
                            Spacer(modifier = Modifier.height(2.dp))
                            Text(
                                text = chat.lastMsg,
                                style = MaterialTheme.typography.bodyMedium,
                                color = MaterialTheme.colorScheme.onSurfaceVariant
                            )
                        }
                        Spacer(modifier = Modifier.width(8.dp))
                        AssistChip(
                            onClick = { },
                            label = { Text(if (chat.isDirect) "BLE" else "${chat.hopCount} Hops") }
                        )
                    }
                }
            }
        }
    }
}
