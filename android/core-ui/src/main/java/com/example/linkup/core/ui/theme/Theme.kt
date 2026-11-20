package com.example.linkup.core.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Typography
import androidx.compose.material3.darkColorScheme
import androidx.compose.runtime.Composable

private val LinkUpDarkColorScheme = darkColorScheme(
    primary = NeonGreen,
    onPrimary = VoidDark,
    secondary = CyberCyan,
    onSecondary = VoidDark,
    tertiary = MeshPurple,
    background = VoidDark,
    onBackground = TextPrimary,
    surface = GlassSurface,
    onSurface = TextPrimary,
    surfaceVariant = GlassSurfaceVariant,
    onSurfaceVariant = TextSecondary,
    outline = GlassBorder
)

@Composable
fun LinkupTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    content: @Composable () -> Unit
) {
    MaterialTheme(
        colorScheme = LinkUpDarkColorScheme,
        typography = Typography(),
        content = content
    )
}
