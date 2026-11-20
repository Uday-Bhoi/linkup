package com.example.linkup.core.domain.repository

import kotlinx.coroutines.flow.Flow

data class DomainMeshPacket(
    val packetId: ByteArray,
    val senderId: ByteArray,
    val recipientId: ByteArray,
    val payload: ByteArray,
    val hopCount: Int,
    val ttl: Int,
    val isDirect: Boolean
)

data class MeshPeerNode(
    val peerId: String,
    val publicKey: ByteArray,
    val rssi: Int?,
    val isConnected: Boolean
)

interface MeshRepository {
    fun getLocalPublicKey(): ByteArray
    suspend fun connectPeer(peerId: String, publicKey: ByteArray)
    suspend fun disconnectPeer(peerId: String)
    suspend fun sendDirectMessage(recipientPublicKey: ByteArray, messageText: String): Result<Unit>
    fun observeIncomingMessages(): Flow<DomainMeshPacket>
    fun observeConnectedPeers(): Flow<List<MeshPeerNode>>
}
