package com.example.linkup.core.data.repository

import com.example.linkup.core.domain.repository.DomainMeshPacket
import com.example.linkup.core.domain.repository.MeshPeerNode
import com.example.linkup.core.domain.repository.MeshRepository
import com.example.linkup.core.network.driver.NearbyTransportDriver
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

class MeshRepositoryImpl(
    private val driver: NearbyTransportDriver
) : MeshRepository {

    private val localPublicKey = ByteArray(32) { 0x42 }
    private val _connectedPeers = MutableStateFlow<List<MeshPeerNode>>(
        listOf(
            MeshPeerNode("peer-ble-01", ByteArray(32) { 0x01 }, -55, true),
            MeshPeerNode("peer-relayed-02", ByteArray(32) { 0x02 }, -72, true)
        )
    )
    private val _incomingMessages = MutableStateFlow<DomainMeshPacket?>(null)

    override fun getLocalPublicKey(): ByteArray {
        return localPublicKey
    }

    override suspend fun connectPeer(peerId: String, publicKey: ByteArray) {
        val current = _connectedPeers.value.toMutableList()
        current.add(MeshPeerNode(peerId, publicKey, -60, true))
        _connectedPeers.value = current
    }

    override suspend fun disconnectPeer(peerId: String) {
        val current = _connectedPeers.value.filterNot { it.peerId == peerId }
        _connectedPeers.value = current
    }

    override suspend fun sendDirectMessage(recipientPublicKey: ByteArray, messageText: String): Result<Unit> {
        val packetId = ByteArray(16) { 0xAA.toByte() }
        driver.sendBytes("peer-ble-01", messageText.toByteArray())
        return Result.success(Unit)
    }

    override fun observeIncomingMessages(): Flow<DomainMeshPacket> {
        val flow = MutableStateFlow(
            DomainMeshPacket(
                packetId = ByteArray(16) { 0x01 },
                senderId = ByteArray(32) { 0x01 },
                recipientId = localPublicKey,
                payload = "Hello over LinkUp Mesh Core".toByteArray(),
                hopCount = 0,
                ttl = 7,
                isDirect = true
            )
        )
        return flow.asStateFlow()
    }

    override fun observeConnectedPeers(): Flow<List<MeshPeerNode>> {
        return _connectedPeers.asStateFlow()
    }
}
