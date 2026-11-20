package com.example.linkup

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import com.example.linkup.core.ui.components.LinkUpBottomNavBar
import com.example.linkup.core.ui.components.Screen
import com.example.linkup.core.ui.screens.CreateProfileScreen
import com.example.linkup.core.ui.screens.OnboardingScreen
import com.example.linkup.core.ui.screens.SplashScreen
import com.example.linkup.core.ui.theme.LinkupTheme
import com.example.linkup.feature.chat.ChatListScreen
import com.example.linkup.feature.chat.ConversationScreen
import com.example.linkup.feature.discovery.DiagnosticsScreen
import com.example.linkup.feature.friends.FriendsListScreen
import com.example.linkup.feature.friends.QrPairingScreen
import com.example.linkup.feature.settings.SettingsScreen

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            LinkupTheme {
                MainAppNavigation()
            }
        }
    }
}

@Composable
fun MainAppNavigation() {
    val navController = rememberNavController()
    val navBackStackEntry by navController.currentBackStackEntryAsState()
    val currentRoute = navBackStackEntry?.destination?.route

    val bottomBarRoutes = listOf(Screen.Chats.route, Screen.Friends.route, Screen.Diagnostics.route, Screen.Settings.route)
    val showBottomBar = currentRoute in bottomBarRoutes

    Scaffold(
        bottomBar = {
            if (showBottomBar) {
                LinkUpBottomNavBar(
                    currentRoute = currentRoute,
                    onNavigate = { route ->
                        navController.navigate(route) {
                            popUpTo(Screen.Chats.route) { saveState = true }
                            launchSingleTop = true
                            restoreState = true
                        }
                    }
                )
            }
        }
    ) { innerPadding ->
        NavHost(
            navController = navController,
            startDestination = "splash",
            modifier = Modifier.padding(innerPadding)
        ) {
            composable("splash") {
                SplashScreen(onNext = { navController.navigate("onboarding") })
            }
            composable("onboarding") {
                OnboardingScreen(onContinue = { navController.navigate("create_profile") })
            }
            composable("create_profile") {
                CreateProfileScreen(onProfileCreated = {
                    navController.navigate(Screen.Chats.route) {
                        popUpTo("splash") { inclusive = true }
                    }
                })
            }
            composable(Screen.Chats.route) {
                ChatListScreen(onChatClick = { chatId -> navController.navigate("conversation/$chatId") })
            }
            composable("conversation/{chatId}") { backStackEntry ->
                val chatId = backStackEntry.arguments?.getString("chatId") ?: "1"
                ConversationScreen(chatId = chatId, onBack = { navController.popBackStack() })
            }
            composable(Screen.Friends.route) {
                FriendsListScreen(onPairClick = { navController.navigate("qr_pairing") })
            }
            composable("qr_pairing") {
                QrPairingScreen(onBack = { navController.popBackStack() })
            }
            composable(Screen.Diagnostics.route) {
                DiagnosticsScreen()
            }
            composable(Screen.Settings.route) {
                SettingsScreen()
            }
        }
    }
}