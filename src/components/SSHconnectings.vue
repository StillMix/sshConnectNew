<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import SshConnect from './SSHConnect.vue'

// Массив активных подключений
const connections = ref<{ id: number; position: number; active: boolean }[]>([])
const showConnectings = ref(false)
const availablePositions = ref([0, 1, 2, 3])

// Отслеживание нажатия Ctrl
const isCtrlPressed = ref(false)

const emit = defineEmits(['add-connection'])

// Следим за нажатием и отпусканием клавиши Ctrl
const handleKeyDown = (e: KeyboardEvent) => {
  if (e.key === 'Control') {
    isCtrlPressed.value = true
    showConnectings.value = true
  }
}

const handleKeyUp = (e: KeyboardEvent) => {
  if (e.key === 'Control') {
    isCtrlPressed.value = false
    showConnectings.value = false
  }
}

// Добавление нового подключения
const addConnection = (position: number) => {
  // Проверяем, что позиция свободна и мы не превысили лимит в 4 подключения
  if (
    connections.value.length < 4 &&
    !connections.value.some((conn) => conn.position === position)
  ) {
    const newId = Date.now()

    // Если это первое подключение и не позиция 0, размещаем его в центре (позиция 0)
    if (connections.value.length === 0 && position !== 0) {
      connections.value.push({
        id: newId,
        position: 0,
        active: true,
      })

      // Оповещаем родительский компонент о новом подключении
      emit('add-connection', { id: newId, position: 0 })
    } else {
      connections.value.push({
        id: newId,
        position,
        active: true,
      })

      // Оповещаем родительский компонент о новом подключении
      emit('add-connection', { id: newId, position })
    }

    // Обновляем доступные позиции
    updateAvailablePositions()
  }
}

// Обновление доступных позиций
const updateAvailablePositions = () => {
  const takenPositions = connections.value.map((conn) => conn.position)
  availablePositions.value = [0, 1, 2, 3].filter((pos) => !takenPositions.includes(pos))
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown)
  window.addEventListener('keyup', handleKeyUp)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
  window.removeEventListener('keyup', handleKeyUp)
})
</script>

<template>
  <div class="ssh-connectings-overlay" v-if="showConnectings">
    <div class="ssh-grid">
      <div
        v-for="position in 4"
        :key="position - 1"
        class="ssh-cell"
        :class="{ occupied: !availablePositions.includes(position - 1) }"
        @click="addConnection(position - 1)"
      >
        <div class="add-connection">+</div>
      </div>
    </div>
  </div>

  <div class="active-connections">
    <div
      v-for="connection in connections"
      :key="connection.id"
      class="connection-container"
      :class="`position-${connection.position}`"
    >
      <SshConnect @server-select="() => {}" />
    </div>
  </div>
</template>

<style lang="scss" scoped>
.ssh-connectings-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(15, 23, 42, 0.8);
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  animation: fadeIn 0.2s ease;
}

.ssh-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  grid-template-rows: 1fr 1fr;
  gap: 20px;
  width: 80%;
  height: 80%;
  max-width: 1200px;
}

.ssh-cell {
  position: relative;
  border: 2px dashed #3b82f6;
  border-radius: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: rgba(30, 41, 59, 0.5);
  cursor: pointer;
  transition: all 0.3s ease;

  &:hover {
    background-color: rgba(59, 130, 246, 0.2);
    transform: scale(1.02);
  }

  &.occupied {
    border: 2px dashed #64748b;
    background-color: rgba(30, 41, 59, 0.3);
    cursor: not-allowed;

    &:hover {
      transform: none;
    }

    .add-connection {
      color: #64748b;
    }
  }
}

.add-connection {
  font-size: 48px;
  color: #3b82f6;
  font-weight: 300;
  transition: all 0.3s ease;
}

.active-connections {
  position: relative;
  width: 100%;
  height: 100%;
}

.connection-container {
  position: absolute;
  width: 50%;
  height: 50%;
  transition: all 0.5s ease;

  // Для одного подключения (центральная позиция)
  &:only-child {
    width: 100%;
    height: 100%;
    left: 0;
    top: 0;
  }

  // Позиционирование для разных слотов
  &.position-0 {
    left: 0;
    top: 0;
  }

  &.position-1 {
    left: 50%;
    top: 0;
  }

  &.position-2 {
    left: 0;
    top: 50%;
  }

  &.position-3 {
    left: 50%;
    top: 50%;
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}
</style>
