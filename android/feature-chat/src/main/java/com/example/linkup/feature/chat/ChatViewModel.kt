package com.example.linkup.feature.chat

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.linkup.core.domain.repository.DomainMeshPacket
import com.example.linkup.core.domain.repository.MeshRepository
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.launch

class ChatViewModel(
    private val repository: MeshRepository
) : ViewModel() {

    val incomingMessages: StateFlow<DomainMeshPacket?> = repository.observeIncomingMessages()
        .stateIn(
            scope = viewModelScope,
            started = SharingStarted.WhileSubscribed(5000),
            initialValue = null
        )

    fun sendMessage(recipientPublicKey: ByteArray, messageText: String) {
        viewModelScope.launch {
            repository.sendDirectMessage(recipientPublicKey, messageText)
        }
    }
}
