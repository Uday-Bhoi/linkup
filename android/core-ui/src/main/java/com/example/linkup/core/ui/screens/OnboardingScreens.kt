package com.example.linkup.core.ui.screens

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material3.Button
import androidx.compose.material3.ButtonDefaults
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.example.linkup.core.ui.components.GlassCard
import com.example.linkup.core.ui.theme.NeonGreen
import com.example.linkup.core.ui.theme.VoidDark

@Composable
fun SplashScreen(onNext: () -> Unit) {
    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(VoidDark),
        contentAlignment = Alignment.Center
    ) {
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Text(
                text = "LinkUp",
                style = MaterialTheme.typography.displayLarge,
                color = NeonGreen,
                fontWeight = FontWeight.Bold
            )
            Spacer(modifier = Modifier.height(8.dp))
            Text(
                text = "Decentralized Offline Mesh System",
                style = MaterialTheme.typography.titleMedium,
                color = MaterialTheme.colorScheme.onBackground
            )
            Spacer(modifier = Modifier.height(48.dp))
            Button(
                onClick = onNext,
                colors = ButtonDefaults.buttonColors(containerColor = NeonGreen, contentColor = VoidDark)
            ) {
                Text("Get Started", fontWeight = FontWeight.Bold)
            }
        }
    }
}

@Composable
fun OnboardingScreen(onContinue: () -> Unit) {
    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(VoidDark)
            .padding(24.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        GlassCard {
            Text(
                text = "Welcome to LinkUp Mesh",
                style = MaterialTheme.typography.headlineSmall,
                color = NeonGreen,
                fontWeight = FontWeight.Bold
            )
            Spacer(modifier = Modifier.height(12.dp))
            Text(
                text = "Communicate during concerts, stadium events, and internet blackouts using peer-to-peer device mesh routing.",
                style = MaterialTheme.typography.bodyLarge
            )
        }
        Spacer(modifier = Modifier.height(32.dp))
        Button(
            onClick = onContinue,
            colors = ButtonDefaults.buttonColors(containerColor = NeonGreen, contentColor = VoidDark)
        ) {
            Text("Create Mesh Profile", fontWeight = FontWeight.Bold)
        }
    }
}

@Composable
fun CreateProfileScreen(onProfileCreated: () -> Unit) {
    var username by remember { mutableStateOf("") }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(VoidDark)
            .padding(24.dp),
        horizontalAlignment = Alignment.CenterHorizontally,
        verticalArrangement = Arrangement.Center
    ) {
        GlassCard {
            Text(
                text = "Set Up Profile",
                style = MaterialTheme.typography.headlineSmall,
                color = NeonGreen,
                fontWeight = FontWeight.Bold
            )
            Spacer(modifier = Modifier.height(4.dp))
            Text(text = "Your identity remains 100% offline on this device.")
            Spacer(modifier = Modifier.height(20.dp))
            OutlinedTextField(
                value = username,
                onValueChange = { username = it },
                label = { Text("Display Name") },
                modifier = Modifier.fillMaxWidth()
            )
        }
        Spacer(modifier = Modifier.height(24.dp))
        Button(
            onClick = onProfileCreated,
            enabled = username.isNotBlank(),
            modifier = Modifier.fillMaxWidth(),
            colors = ButtonDefaults.buttonColors(containerColor = NeonGreen, contentColor = VoidDark)
        ) {
            Text("Enter LinkUp Mesh Network", fontWeight = FontWeight.Bold)
        }
    }
}
