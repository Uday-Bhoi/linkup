package com.example.linkup.core.ui.components

import androidx.compose.animation.animateColorAsState
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Icon
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.example.linkup.core.ui.theme.GlassSurface
import com.example.linkup.core.ui.theme.NeonGreen
import com.example.linkup.core.ui.theme.TextSecondary

sealed class Screen(val route: String, val title: String) {
    object Chats : Screen("chats", "Chats")
    object Friends : Screen("friends", "Friends")
    object Diagnostics : Screen("diagnostics", "Mesh Radar")
    object Settings : Screen("settings", "Settings")
}

val bottomNavScreens = listOf(
    Screen.Chats,
    Screen.Friends,
    Screen.Diagnostics,
    Screen.Settings
)

@Composable
fun LinkUpBottomNavBar(
    currentRoute: String?,
    onNavigate: (String) -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .background(GlassSurface)
            .padding(vertical = 8.dp, horizontal = 16.dp),
        horizontalArrangement = Arrangement.SpaceAround,
        verticalAlignment = Alignment.CenterVertically
    ) {
        bottomNavScreens.forEach { screen ->
            val isSelected = currentRoute == screen.route
            val animColor by animateColorAsState(
                targetValue = if (isSelected) NeonGreen else TextSecondary,
                label = "NavColor"
            )

            Box(
                modifier = Modifier
                    .clip(RoundedCornerShape(12.dp))
                    .clickable { onNavigate(screen.route) }
                    .padding(vertical = 8.dp, horizontal = 12.dp),
                contentAlignment = Alignment.Center
            ) {
                Text(
                    text = screen.title,
                    color = animColor,
                    fontSize = 13.sp,
                    fontWeight = if (isSelected) FontWeight.Bold else FontWeight.Normal
                )
            }
        }
    }
}
