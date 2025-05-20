<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

interface TransferTask {
  id: number
  fileName: string
  sourceServer: string
  destinationServer: string
  progress: number
  status: 'pending' | 'transferring' | 'completed' | 'failed'
  startTime: Date
}

defineProps<{
  isVisible: boolean
}>()

const emit = defineEmits(['close'])

// Список активных и завершенных передач
const transferTasks = ref<TransferTask[]>([])

// Состояние для управления панелью
const isPanelMinimized = ref(false)

// Закрытие панели
const closePanel = () => {
  emit('close')
}

// Сворачивание/разворачивание панели
const toggleMinimize = () => {
  isPanelMinimized.value = !isPanelMinimized.value
}

// Форматирование времени
const formatTime = (date: Date): string => {
  return date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
}

// Отмена передачи (заглушка)
const cancelTransfer = (taskId: number) => {
  const taskIndex = transferTasks.value.findIndex((task) => task.id === taskId)
  if (taskIndex !== -1) {
    transferTasks.value[taskIndex].status = 'failed'
  }
}

// Обработчик нажатия клавиши Escape
const handleKeyDown = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    closePanel()
  }
}

// Инициализация демо-данных
const initDemoTasks = () => {
  transferTasks.value = [
    {
      id: 1,
      fileName: 'config.json',
      sourceServer: 'Сервер разработки',
      destinationServer: 'Prod сервер',
      progress: 100,
      status: 'completed',
      startTime: new Date(Date.now() - 120000), // 2 минуты назад
    },
    {
      id: 2,
      fileName: 'database_backup.tar.gz',
      sourceServer: 'Prod сервер',
      destinationServer: 'Сервер разработки',
      progress: 75,
      status: 'transferring',
      startTime: new Date(),
    },
    {
      id: 3,
      fileName: 'logs/',
      sourceServer: 'Сервер разработки',
      destinationServer: 'Prod сервер',
      progress: 45,
      status: 'transferring',
      startTime: new Date(),
    },
  ]

  // Имитация прогресса передачи
  const progressInterval = setInterval(() => {
    transferTasks.value.forEach((task) => {
      if (task.status === 'transferring' && task.progress < 100) {
        task.progress += 5
        if (task.progress >= 100) {
          task.progress = 100
          task.status = 'completed'
        }
      }
    })

    // Если все задачи завершены, очищаем интервал
    if (transferTasks.value.every((task) => task.status !== 'transferring')) {
      clearInterval(progressInterval)
    }
  }, 1000)
}

onMounted(() => {
  document.addEventListener('keydown', handleKeyDown)
  initDemoTasks()
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeyDown)
})
</script>

<template>
  <div class="file-transfer-panel" v-if="isVisible" :class="{ minimized: isPanelMinimized }">
    <div class="panel-header">
      <div class="header-title">
        <span class="title-icon">📤</span>
        <h3>Передача файлов</h3>
        <span class="badge">{{ transferTasks.length }}</span>
      </div>
      <div class="header-actions">
        <button class="minimize-button" @click="toggleMinimize">
          {{ isPanelMinimized ? '▲' : '▼' }}
        </button>
        <button class="close-button" @click="closePanel">✕</button>
      </div>
    </div>

    <div class="panel-content" v-if="!isPanelMinimized">
      <div v-if="transferTasks.length === 0" class="no-tasks">
        <p>Нет активных передач</p>
      </div>

      <transition-group name="task-list" tag="div" class="tasks-list">
        <div
          v-for="task in transferTasks"
          :key="task.id"
          class="task-item"
          :class="{
            completed: task.status === 'completed',
            failed: task.status === 'failed',
            pending: task.status === 'pending',
          }"
        >
          <div class="task-header">
            <div class="task-title">
              <span class="file-icon">{{ task.fileName.endsWith('/') ? '📁' : '📄' }}</span>
              <span class="file-name">{{ task.fileName }}</span>
            </div>
            <div class="task-actions">
              <span class="task-time">{{ formatTime(task.startTime) }}</span>
              <button
                v-if="task.status === 'transferring' || task.status === 'pending'"
                class="cancel-button"
                @click="cancelTransfer(task.id)"
              >
                ✕
              </button>
            </div>
          </div>

          <div class="task-details">
            <div class="server-path">
              <span class="server-label">Откуда:</span>
              <span class="server-name">{{ task.sourceServer }}</span>
            </div>
            <div class="server-path">
              <span class="server-label">Куда:</span>
              <span class="server-name">{{ task.destinationServer }}</span>
            </div>
          </div>

          <div class="task-progress">
            <div class="progress-bar">
              <div class="progress-fill" :style="{ width: task.progress + '%' }"></div>
            </div>
            <span class="progress-text">{{ task.progress }}%</span>
          </div>

          <div class="task-status">
            <span
              class="status-badge"
              :class="{
                'status-completed': task.status === 'completed',
                'status-failed': task.status === 'failed',
                'status-pending': task.status === 'pending',
                'status-transferring': task.status === 'transferring',
              }"
            >
              {{
                task.status === 'completed'
                  ? 'Завершено'
                  : task.status === 'failed'
                    ? 'Ошибка'
                    : task.status === 'pending'
                      ? 'Ожидание'
                      : 'Передача'
              }}
            </span>
          </div>
        </div>
      </transition-group>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.file-transfer-panel {
  position: fixed;
  bottom: 20px;
  right: 20px;
  width: 450px;
  max-height: 80vh;
  background: linear-gradient(145deg, #1e293b, #0f172a);
  border-radius: 12px 12px 0 0;
  box-shadow: 0 -8px 30px rgba(0, 0, 0, 0.3);
  z-index: 1000;
  animation: slideUp 0.4s ease;
  transition: all 0.3s ease;
  overflow: hidden;

  &.minimized {
    height: auto;
  }
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 20px;
  background-color: rgba(15, 23, 42, 0.5);
  border-bottom: 1px solid rgba(148, 163, 184, 0.2);

  .header-title {
    display: flex;
    align-items: center;
    gap: 10px;

    .title-icon {
      font-size: 18px;
    }

    h3 {
      color: #f8fafc;
      font-size: 16px;
      font-weight: 600;
      margin: 0;
    }

    .badge {
      background-color: #3b82f6;
      color: white;
      font-size: 12px;
      font-weight: 500;
      min-width: 20px;
      height: 20px;
      border-radius: 10px;
      display: inline-flex;
      align-items: center;
      justify-content: center;
      padding: 0 6px;
    }
  }

  .header-actions {
    display: flex;
    gap: 8px;

    button {
      background-color: transparent;
      border: none;
      color: #94a3b8;
      font-size: 14px;
      width: 24px;
      height: 24px;
      display: flex;
      align-items: center;
      justify-content: center;
      cursor: pointer;
      transition: all 0.2s ease;

      &:hover {
        color: #f1f5f9;
      }
    }
  }
}

.panel-content {
  max-height: 60vh;
  overflow-y: auto;
  padding: 16px;

  &::-webkit-scrollbar {
    width: 6px;
  }

  &::-webkit-scrollbar-track {
    background: #1e293b;
  }

  &::-webkit-scrollbar-thumb {
    background-color: #475569;
    border-radius: 3px;
  }
}

.no-tasks {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100px;

  p {
    color: #94a3b8;
    font-size: 14px;
  }
}

.tasks-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.task-item {
  background-color: #1e293b;
  border-radius: 8px;
  padding: 14px;
  transition: all 0.3s ease;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 6px 14px rgba(0, 0, 0, 0.15);
  }

  &.completed {
    border-left: 3px solid #10b981;
  }

  &.failed {
    border-left: 3px solid #ef4444;
  }

  &.pending {
    border-left: 3px solid #f59e0b;
  }
}

.task-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;

  .task-title {
    display: flex;
    align-items: center;
    gap: 8px;

    .file-icon {
      font-size: 16px;
    }

    .file-name {
      color: #f1f5f9;
      font-size: 14px;
      font-weight: 500;
      white-space: nowrap;
      overflow: hidden;
      text-overflow: ellipsis;
      max-width: 200px;
    }
  }

  .task-actions {
    display: flex;
    align-items: center;
    gap: 10px;

    .task-time {
      color: #94a3b8;
      font-size: 12px;
    }

    .cancel-button {
      background-color: #334155;
      color: #94a3b8;
      border: none;
      width: 20px;
      height: 20px;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 4px;
      font-size: 12px;
      cursor: pointer;
      transition: all 0.2s ease;

      &:hover {
        background-color: #475569;
        color: #f1f5f9;
      }
    }
  }
}

.task-details {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-bottom: 12px;

  .server-path {
    display: flex;
    gap: 6px;

    .server-label {
      color: #94a3b8;
      font-size: 12px;
      min-width: 50px;
    }

    .server-name {
      color: #cbd5e1;
      font-size: 12px;
    }
  }
}

.task-progress {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 10px;

  .progress-bar {
    flex: 1;
    height: 6px;
    background-color: #334155;
    border-radius: 3px;
    overflow: hidden;

    .progress-fill {
      height: 100%;
      background: linear-gradient(to right, #3b82f6, #60a5fa);
      border-radius: 3px;
      transition: width 0.5s ease;
    }
  }

  .progress-text {
    color: #94a3b8;
    font-size: 12px;
    min-width: 40px;
    text-align: right;
  }
}

.task-status {
  display: flex;
  justify-content: flex-end;

  .status-badge {
    font-size: 12px;
    padding: 3px 8px;
    border-radius: 4px;
    font-weight: 500;

    &.status-completed {
      background-color: rgba(16, 185, 129, 0.2);
      color: #34d399;
    }

    &.status-failed {
      background-color: rgba(239, 68, 68, 0.2);
      color: #f87171;
    }

    &.status-pending {
      background-color: rgba(245, 158, 11, 0.2);
      color: #fbbf24;
    }

    &.status-transferring {
      background-color: rgba(59, 130, 246, 0.2);
      color: #60a5fa;
    }
  }
}

.task-list-enter-active,
.task-list-leave-active {
  transition: all 0.5s ease;
}

.task-list-enter-from,
.task-list-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.task-list-move {
  transition: transform 0.5s ease;
}

@keyframes slideUp {
  from {
    transform: translateY(100%);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}
</style>
