package com.example.linkup.feature.chat

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.example.linkup.core.ui.theme.CyberCyan
import com.example.linkup.core.ui.theme.GlassSurface
import com.example.linkup.core.ui.theme.NeonGreen
import com.example.linkup.core.ui.theme.VoidDark

data class MockMessage(val id: String, val text: String, val isMe: Boolean, val timestamp: String)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ConversationScreen(
    chatId: String,
    onBack: () -> Unit
) {
    var textInput by remember { mutableStateOf("") }
    val messages = remember {
        mutableStateListOf(
            MockMessage("1", "Hey! Can you hear me over BLE mesh?", false, "17:05"),
            MockMessage("2", "Yes! Encrypted offline message working great.", true, "17:06")
        )
    }

    Scaffold(
        containerColor = VoidDark,
        topBar = {
            TopAppBar(
                title = {
                    Column {
                        Text("Conversation #$chatId", fontWeight = FontWeight.Bold)
                        Text("🔒 End-to-End Encrypted (Ed25519 / ChaCha20)", style = MaterialTheme.typography.labelSmall, color = NeonGreen)
                    }
                },
                navigationIcon = {
                    IconButton(onClick = onBack) {
                        Text("◄", color = NeonGreen, fontWeight = FontWeight.Bold)
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(containerColor = VoidDark)
            )
        },
        bottomBar = {
            Surface(
                color = GlassSurface,
                modifier = Modifier.padding(8.dp),
                shape = RoundedCornerShape(24.dp)
            ) {
                Row(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 12.dp, vertical = 6.dp),
                    verticalAlignment = Alignment.CenterVertically
                ) {
                    OutlinedTextField(
                        value = textInput,
                        onValueChange = { textInput = it },
                        modifier = Modifier.weight(1f),
                        placeholder = { Text("Type E2EE mesh message...") },
                        shape = RoundedCornerShape(20.dp)
                    )
                    Spacer(modifier = Modifier.width(8.dp))
                    Button(
                        onClick = {
                            if (textInput.isNotBlank()) {
                                messages.add(MockMessage(System.currentTimeMillis().toString(), textInput, true, "Now"))
                                textInput = ""
                            }
                        },
                        colors = ButtonDefaults.buttonColors(containerColor = NeonGreen, contentColor = VoidDark)
                    ) {
                        Text("Send", fontWeight = FontWeight.Bold)
                    }
                }
            }
        }
    ) { padding ->
        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(horizontal = 16.dp),
            verticalArrangement = Arrangement.spacedBy(10.dp)
        ) {
            items(messages) { msg ->
                Box(
                    modifier = Modifier.fillMaxWidth(),
                    contentAlignment = if (msg.isMe) Alignment.CenterEnd else Alignment.CenterStart
                ) {
                    Surface(
                        shape = RoundedCornerShape(
                            topStart = 16.dp,
                            topEnd = 16.dp,
                            bottomStart = if (msg.isMe) 16.dp else 4.dp,
                            bottomEnd = if (msg.isMe) 4.dp else 16.dp
                        ),
                        color = if (msg.isMe) NeonGreen else GlassSurface
                    ) {
                        Column(modifier = Modifier.padding(12.dp)) {
                            Text(
                                text = msg.text,
                                style = MaterialTheme.typography.bodyLarge,
                                color = if (msg.isMe) VoidDark else Color.White
                            )
                            Spacer(modifier = Modifier.height(2.dp))
                            Text(
                                text = msg.timestamp,
                                style = MaterialTheme.typography.labelSmall,
                                color = if (msg.isMe) VoidDark.copy(alpha = 0.7f) else CyberCyan
                            )
                        }
                    }
                }
            }
        }
    }
}
