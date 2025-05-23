<template>
  <div class="main">
    <div class="app-header">
      <div class="logo">
        <span class="logo-icon">⚡</span>
        <h1>SSH Connect</h1>
      </div>
      <div class="header-actions">
        <button class="theme-toggle">🌙</button>
      </div>
    </div>

    <div class="content-wrapper">
      <SSHconnectings @add-connection="handleAddConnection" />

      <div v-if="connections.length === 0" class="no-connections">
        <p>Зажмите <kbd>Ctrl</kbd> для добавления подключения</p>
      </div>

      <!-- Уведомление о трансфере файлов -->
      <div class="file-transfer-notification" v-if="fileTransferNotification.isVisible">
        <div class="notification-backdrop"></div>
        <div class="notification-content">
          <div class="notification-icon">
            <div class="icon-wrapper">
              <div class="circle-animation"></div>
              <span class="transfer-icon">📤</span>
            </div>
          </div>
          <div class="notification-info">
            <p class="notification-title">{{ fileTransferNotification.title }}</p>
            <p class="notification-description">{{ fileTransferNotification.description }}</p>
            <div class="notification-progress-wrapper">
              <div class="notification-progress">
                <div
                  class="progress-bar"
                  :style="{ width: fileTransferNotification.progress + '%' }"
                ></div>
              </div>
              <span class="progress-percentage"
                >{{ Math.round(fileTransferNotification.progress) }}%</span
              >
            </div>
          </div>
        </div>
      </div>

      <div class="connections-grid">
        <div v-for="connection in connections" :key="connection.id" class="connection-wrapper">
          <transition name="fade" mode="out-in">
            <ServerInfo
              v-if="connection.isConnected && connection.showServerInfo"
              :server="connection.server"
              :serverId="connection.id.toString()"
              :allConnectedServers="getAllConnectedServers()"
              @disconnect="() => handleDisconnect(connection.id)"
              @file-transfer="handleFileTransfer"
            />
            <SshConnect
              v-else
              @server-select="(server) => handleServerSelect(connection.id, server)"
            />
          </transition>
        </div>
      </div>
    </div>

    <div class="app-footer">
      <p>© 2025 SSH Connect. Все права защищены.</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import ServerInfo from '@/components/ServerInfo.vue'
import SshConnect from '@/components/SSHConnect.vue'
import SSHconnectings from '@/components/SSHconnectings.vue'
import { onMounted, onUnmounted } from 'vue'

interface ServerCredential {
  id?: number
  title: string
  user: string
  password: string
}

interface Connection {
  id: number
  server: ServerCredential | null
  isConnected: boolean
  showServerInfo: boolean
}

interface FileTransfer {
  fileName: string
  isFolder: boolean
  sourceServerId: string
  destinationServerId: string
}

const connections = ref<Connection[]>([])
const activeConnectionId = ref<number | null>(null)

const fileTransferNotification = reactive({
  isVisible: false,
  title: '',
  description: '',
  progress: 0,
})

const handleContextMenuPrevent = (e: MouseEvent) => {
  const isMenuElement =
    (e.target as HTMLElement).closest('.ssh-card') ||
    (e.target as HTMLElement).closest('.file-item')
  if (!isMenuElement) {
    e.preventDefault()
  }
}

onMounted(() => {
  const main = document.querySelector('.main')
  if (main) {
    main.classList.add('loaded')
  }

  document.addEventListener('contextmenu', handleContextMenuPrevent)
})

onUnmounted(() => {
  document.removeEventListener('contextmenu', handleContextMenuPrevent)
})

const handleServerSelect = (connectionId: number, server: ServerCredential) => {
  const connection = connections.value.find((c) => c.id === connectionId)
  if (connection) {
    connection.server = server
    connection.isConnected = true

    setTimeout(() => {
      connection.showServerInfo = true
    }, 100)
  }

  activeConnectionId.value = connectionId
}

const getAllConnectedServers = () => {
  return connections.value
    .filter((conn) => conn.isConnected && conn.server)
    .map((conn) => ({
      id: conn.id.toString(),
      server: conn.server!,
    }))
}

const handleDisconnect = (connectionId: number) => {
  const connection = connections.value.find((c) => c.id === connectionId)
  if (connection) {
    connection.showServerInfo = false

    setTimeout(() => {
      connection.isConnected = false
      connection.server = null
    }, 500)
  }
}

const handleAddConnection = (connectionData: { id: number; position: number }) => {
  if (!connections.value.some((c) => c.id === connectionData.id)) {
    connections.value.push({
      id: connectionData.id,
      server: null,
      isConnected: false,
      showServerInfo: false,
    })
  }
}

const handleFileTransfer = (transferData: FileTransfer) => {
  console.log('Передача файла между серверами:', transferData)

  const sourceConnection = connections.value.find(
    (c) => c.id.toString() === transferData.sourceServerId,
  )
  const destinationConnection = connections.value.find(
    (c) => c.id.toString() === transferData.destinationServerId,
  )

  if (
    sourceConnection &&
    destinationConnection &&
    sourceConnection.server &&
    destinationConnection.server
  ) {
    fileTransferNotification.isVisible = true
    fileTransferNotification.title = `Передача файла: ${transferData.fileName}`
    fileTransferNotification.description = `с ${sourceConnection.server.title} на ${destinationConnection.server.title}`
    fileTransferNotification.progress = 0

    const totalSteps = 10
    let currentStep = 0

    const transferInterval = setInterval(() => {
      currentStep++
      fileTransferNotification.progress = (currentStep / totalSteps) * 100

      if (currentStep >= totalSteps) {
        clearInterval(transferInterval)

        setTimeout(() => {
          fileTransferNotification.isVisible = false
        }, 2000)
      }
    }, 300)
  }
}
</script>

<style lang="scss">
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: 'Roboto', 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

body {
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
  color: #f8fafc;
  min-height: 100vh;
}

#app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.main {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  padding: 0 20px;
  opacity: 0;
  transform: translateY(20px);
  transition:
    opacity 0.8s ease,
    transform 0.8s ease;

  &.loaded {
    opacity: 1;
    transform: translateY(0);
  }
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 0;
  margin-bottom: 20px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.2);

  .logo {
    display: flex;
    align-items: center;
    gap: 12px;

    .logo-icon {
      font-size: 24px;
      background-color: #2563eb;
      color: white;
      width: 40px;
      height: 40px;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 10px;
      box-shadow: 0 0 15px rgba(37, 99, 235, 0.5);
    }

    h1 {
      font-size: 24px;
      font-weight: 700;
      background: linear-gradient(to right, #60a5fa, #3b82f6);
      background-clip: text;
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      margin: 0;
    }
  }

  .header-actions {
    .theme-toggle {
      background-color: #334155;
      border: none;
      border-radius: 8px;
      width: 40px;
      height: 40px;
      font-size: 18px;
      cursor: pointer;
      transition: all 0.2s ease;

      &:hover {
        background-color: #475569;
        transform: translateY(-2px);
      }
    }
  }
}

.content-wrapper {
  flex: 1;
  padding: 20px 0;
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  position: relative;
}

.connections-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
  margin-top: 20px;
}

.connection-wrapper {
  width: 100%;
}

.no-connections {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 300px;

  p {
    color: #94a3b8;
    font-size: 18px;
    text-align: center;

    kbd {
      background-color: #334155;
      padding: 2px 8px;
      border-radius: 4px;
      font-family: monospace;
      color: #f8fafc;
    }
  }
}

.app-footer {
  padding: 20px 0;
  text-align: center;
  margin-top: 40px;
  border-top: 1px solid rgba(148, 163, 184, 0.2);

  p {
    color: #94a3b8;
    font-size: 14px;
  }
}

.fade-enter-active,
.fade-leave-active {
  transition:
    opacity 0.5s ease,
    transform 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

@media (max-width: 768px) {
  .app-header {
    flex-direction: column;
    gap: 16px;
    align-items: flex-start;

    .header-actions {
      align-self: flex-end;
    }
  }

  .connections-grid {
    grid-template-columns: 1fr;
  }
}

@keyframes fadeUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.ssh-connect,
.ssh-container {
  animation: fadeUp 0.8s ease forwards;
}

.ssh-container {
  animation-delay: 0.2s;
}

.file-transfer-notification {
  position: fixed;
  top: 30px;
  right: 30px;
  z-index: 1100;
  width: 400px;
  max-width: 95vw;
  animation:
    slideInRight 0.4s ease-out forwards,
    elevate 0.3s ease-out 0.4s forwards;
}

.notification-backdrop {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: linear-gradient(135deg, rgba(37, 99, 235, 0.9), rgba(30, 64, 175, 0.92));
  border-radius: 16px;
  box-shadow: 0 12px 30px rgba(0, 10, 32, 0.35);
  opacity: 0.9;
  backdrop-filter: blur(10px);
  transform: scale(0.98);
  transition: transform 0.3s ease;
  z-index: -1;
  overflow: hidden;

  &:before {
    content: '';
    position: absolute;
    top: -50%;
    left: -50%;
    width: 200%;
    height: 200%;
    background: radial-gradient(circle, rgba(255, 255, 255, 0.15), transparent 60%);
    opacity: 0.6;
    pointer-events: none;
  }
}

.notification-content {
  display: flex;
  padding: 20px;
  align-items: center;
  gap: 16px;
}

.notification-icon {
  flex-shrink: 0;

  .icon-wrapper {
    position: relative;
    width: 50px;
    height: 50px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .circle-animation {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    border-radius: 50%;
    border: 2px solid rgba(255, 255, 255, 0.5);
    border-top-color: rgba(255, 255, 255, 1);
    animation: spin 1.5s linear infinite;
  }

  .transfer-icon {
    font-size: 24px;
    filter: drop-shadow(0 0 8px rgba(255, 255, 255, 0.5));
  }
}

.notification-info {
  flex: 1;
  overflow: hidden;
}

.notification-title {
  color: white;
  font-size: 16px;
  font-weight: 600;
  margin: 0 0 4px 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.notification-description {
  color: rgba(255, 255, 255, 0.9);
  font-size: 14px;
  margin: 0 0 12px 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.notification-progress-wrapper {
  display: flex;
  align-items: center;
  gap: 12px;
}

.notification-progress {
  flex: 1;
  height: 6px;
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 10px;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  background: linear-gradient(to right, rgba(255, 255, 255, 0.9), rgba(255, 255, 255, 0.7));
  border-radius: 10px;
  transition: width 0.3s ease;
  box-shadow: 0 0 6px rgba(255, 255, 255, 0.6);
  position: relative;
  overflow: hidden;

  &:after {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: linear-gradient(to right, transparent, rgba(255, 255, 255, 0.3), transparent);
    animation: shimmer 1.5s infinite;
  }
}

.progress-percentage {
  color: white;
  font-size: 14px;
  font-weight: 600;
  min-width: 36px;
  text-align: right;
}

@keyframes slideInRight {
  from {
    transform: translateX(100px);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@keyframes elevate {
  from {
    transform: translateY(0);
  }
  to {
    transform: translateY(-5px);
  }
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@keyframes shimmer {
  from {
    transform: translateX(-100%);
  }
  to {
    transform: translateX(100%);
  }
}

@media (max-width: 768px) {
  .file-transfer-notification {
    right: 10px;
    left: 10px;
    width: calc(100% - 20px);
    max-width: 100%;
  }
}
</style>
