package com.example.linkup.core.ui.components

import androidx.compose.animation.core.*
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.unit.dp
import com.example.linkup.core.ui.theme.CyberCyan
import com.example.linkup.core.ui.theme.NeonGreen

data class RadarPeer(val xRatio: Float, val yRatio: Float, val isDirect: Boolean)

@Composable
fun RadarCanvas(
    peers: List<RadarPeer>,
    modifier: Modifier = Modifier
) {
    val infiniteTransition = rememberInfiniteTransition(label = "RadarSweep")
    val pulseRadius by infiniteTransition.animateFloat(
        initialValue = 0f,
        targetValue = 1f,
        animationSpec = infiniteRepeatable(
            animation = tween(2500, easing = LinearEasing),
            repeatMode = RepeatMode.Restart
        ),
        label = "Pulse"
    )

    Canvas(
        modifier = modifier
            .fillMaxWidth()
            .height(240.dp)
    ) {
        val center = Offset(size.width / 2f, size.height / 2f)
        val maxRadius = minOf(size.width, size.height) / 2.2f

        // Draw concentric radar rings
        for (i in 1..3) {
            drawCircle(
                color = Color(0x3D00FF9D),
                radius = maxRadius * (i / 3f),
                center = center,
                style = Stroke(width = 1.dp.toPx())
            )
        }

        // Draw animated sweep pulse
        drawCircle(
            color = Color(0x2600FF9D),
            radius = maxRadius * pulseRadius,
            center = center,
            style = Stroke(width = 2.dp.toPx())
        )

        // Draw local node center marker
        drawCircle(
            color = NeonGreen,
            radius = 6.dp.toPx(),
            center = center
        )

        // Draw peer nodes
        peers.forEach { peer ->
            val peerOffset = Offset(
                x = center.x + (peer.xRatio * maxRadius),
                y = center.y + (peer.yRatio * maxRadius)
            )
            val peerColor = if (peer.isDirect) NeonGreen else CyberCyan
            drawCircle(
                color = peerColor,
                radius = 8.dp.toPx(),
                center = peerOffset
            )
            drawCircle(
                color = peerColor.copy(alpha = 0.3f),
                radius = 14.dp.toPx(),
                center = peerOffset
            )
        }
    }
}
