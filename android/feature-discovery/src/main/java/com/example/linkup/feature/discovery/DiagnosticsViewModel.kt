package com.example.linkup.feature.discovery

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.example.linkup.core.domain.repository.MeshPeerNode
import com.example.linkup.core.domain.repository.MeshRepository
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.stateIn

class DiagnosticsViewModel(
    repository: MeshRepository
) : ViewModel() {

    val activePeers: StateFlow<List<MeshPeerNode>> = repository.observeConnectedPeers()
        .stateIn(
            scope = viewModelScope,
            started = SharingStarted.WhileSubscribed(5000),
            initialValue = emptyList()
        )
}
