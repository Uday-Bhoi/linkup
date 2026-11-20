package com.example.linkup.feature.friends

import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.example.linkup.core.ui.components.GlassCard
import com.example.linkup.core.ui.theme.CyberCyan
import com.example.linkup.core.ui.theme.NeonGreen
import com.example.linkup.core.ui.theme.VoidDark

data class MockFriend(val id: String, val name: String, val fingerprint: String)

val mockFriendsList = listOf(
    MockFriend("1", "Alice Smith", "ed25519:7a8b...4f1e"),
    MockFriend("2", "Bob Johnson", "ed25519:1b9c...8e3f"),
    MockFriend("3", "Charlie Davis", "ed25519:9e4d...2a1c")
)

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun FriendsListScreen(
    onPairClick: () -> Unit
) {
    Scaffold(
        containerColor = VoidDark,
        topBar = {
            TopAppBar(
                title = { Text("Mesh Friends & Identity Keys", fontWeight = FontWeight.Bold) },
                colors = TopAppBarDefaults.topAppBarColors(containerColor = VoidDark)
            )
        },
        floatingActionButton = {
            ExtendedFloatingActionButton(
                onClick = onPairClick,
                containerColor = NeonGreen,
                contentColor = VoidDark
            ) {
                Text("Scan / Show QR",  fontWeight = FontWeight.Bold)
            }
        }
    ) { padding ->
        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(horizontal = 16.dp, vertical = 8.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            items(mockFriendsList) { friend ->
                GlassCard {
                    Column {
                        Text(
                            text = friend.name,
                            style = MaterialTheme.typography.titleMedium,
                            fontWeight = FontWeight.Bold
                        )
                        Spacer(modifier = Modifier.height(4.dp))
                        Text(
                            text = friend.fingerprint,
                            style = MaterialTheme.typography.bodySmall,
                            fontFamily = FontFamily.Monospace,
                            color = CyberCyan
                        )
                    }
                }
            }
        }
    }
}
