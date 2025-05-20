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
        <div class="notification-content">
          <div class="notification-icon">📤</div>
          <div class="notification-message">
            <p class="notification-title">{{ fileTransferNotification.title }}</p>
            <p class="notification-description">{{ fileTransferNotification.description }}</p>
          </div>
        </div>
        <div class="notification-progress">
          <div
            class="progress-bar"
            :style="{ width: fileTransferNotification.progress + '%' }"
          ></div>
        </div>
      </div>

      <div class="connections-grid">
        <div v-for="connection in connections" :key="connection.id" class="connection-wrapper">
          <transition name="fade" mode="out-in">
            <ServerInfo
              v-if="connection.isConnected && connection.showServerInfo"
              :server="connection.server"
              :serverId="connection.id.toString()"
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
import { onMounted } from 'vue'

// Интерфейс для сервера
interface ServerCredential {
  id?: number
  title: string
  user: string
  password: string
}

// Интерфейс для подключения
interface Connection {
  id: number
  server: ServerCredential | null
  isConnected: boolean
  showServerInfo: boolean
}

// Интерфейс для трансфера файлов
interface FileTransfer {
  fileName: string
  isFolder: boolean
  sourceServerId: string
  destinationServerId: string
}

// Состояние подключений
const connections = ref<Connection[]>([])
const activeConnectionId = ref<number | null>(null)

// Состояние уведомления о трансфере файлов
const fileTransferNotification = reactive({
  isVisible: false,
  title: '',
  description: '',
  progress: 0,
})

// Анимация при загрузке страницы
onMounted(() => {
  const main = document.querySelector('.main')
  if (main) {
    main.classList.add('loaded')
  }
})

// Обработчик выбора сервера для определенного подключения
const handleServerSelect = (connectionId: number, server: ServerCredential) => {
  const connection = connections.value.find((c) => c.id === connectionId)
  if (connection) {
    connection.server = server
    connection.isConnected = true

    // Небольшая задержка перед показом ServerInfo
    setTimeout(() => {
      connection.showServerInfo = true
    }, 100)
  }

  // Устанавливаем активное подключение
  activeConnectionId.value = connectionId
}

// Обработчик отключения от сервера
const handleDisconnect = (connectionId: number) => {
  const connection = connections.value.find((c) => c.id === connectionId)
  if (connection) {
    connection.showServerInfo = false

    // Задержка перед сбросом состояния подключения
    setTimeout(() => {
      connection.isConnected = false
      connection.server = null
    }, 500)
  }
}

// Обработчик добавления нового подключения
const handleAddConnection = (connectionData: { id: number; position: number }) => {
  // Проверяем, что такого подключения еще нет
  if (!connections.value.some((c) => c.id === connectionData.id)) {
    connections.value.push({
      id: connectionData.id,
      server: null,
      isConnected: false,
      showServerInfo: false,
    })
  }
}

// Обработчик трансфера файлов между серверами
const handleFileTransfer = (transferData: FileTransfer) => {
  console.log('Передача файла между серверами:', transferData)

  // Получаем информацию о серверах
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
    // Показываем уведомление о начале передачи
    fileTransferNotification.isVisible = true
    fileTransferNotification.title = `Передача файла: ${transferData.fileName}`
    fileTransferNotification.description = `с ${sourceConnection.server.title} на ${destinationConnection.server.title}`
    fileTransferNotification.progress = 0

    // Имитируем процесс передачи файла
    const totalSteps = 10
    let currentStep = 0

    const transferInterval = setInterval(() => {
      currentStep++
      fileTransferNotification.progress = (currentStep / totalSteps) * 100

      if (currentStep >= totalSteps) {
        clearInterval(transferInterval)

        // После завершения передачи оставляем уведомление на короткое время
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
</style>
