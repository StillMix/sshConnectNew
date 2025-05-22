<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import SSHCard from './SSHCard.vue'
import RightClickMenu from './RightCLickMenu.vue'
import { invoke } from '@tauri-apps/api/core'

interface SSHCredential {
  id: number
  title: string
  user: string
  password: string
}

const showForm = ref(false)
const credentials = reactive<SSHCredential[]>([])

const newCredential = reactive({
  title: '',
  user: '',
  password: '',
})

const contextMenu = ref({
  isVisible: false,
  x: 0,
  y: 0,
  serverId: 0,
})

const renameState = ref({
  isVisible: false,
  serverId: 0,
  newTitle: '',
})

const emit = defineEmits(['server-select', 'connecting'])
const debugInfo = ref({
  configPath: '',
  showDebug: false,
})

const toggleDebug = async () => {
  debugInfo.value.showDebug = !debugInfo.value.showDebug
  if (debugInfo.value.showDebug) {
    try {
      debugInfo.value.configPath = await invoke('get_config_path')
    } catch (error) {
      console.error('Ошибка получения пути конфигурации:', error)
    }
  }
}

const reloadServers = () => {
  loadServers()
}
const loadServers = async () => {
  try {
    const servers = (await invoke('load_servers_from_config')) as SSHCredential[]
    credentials.splice(0, credentials.length, ...servers)
  } catch (error) {
    console.error('Ошибка загрузки серверов:', error)
    credentials.splice(0, credentials.length)
  }
}

const saveServers = async () => {
  try {
    await invoke('save_servers', { servers: credentials })
  } catch (error) {
    console.error('Ошибка сохранения серверов:', error)
  }
}

const toggleForm = () => {
  showForm.value = !showForm.value
}

const addCredential = async () => {
  if (newCredential.title && newCredential.user && newCredential.password) {
    try {
      const newServer = await invoke('add_server_to_config', {
        title: newCredential.title,
        user: newCredential.user,
        password: newCredential.password,
      }) as SSHCredential

      credentials.push(newServer)

      newCredential.title = ''
      newCredential.user = ''
      newCredential.password = ''

      showForm.value = false
    } catch (error) {
      console.error('Ошибка добавления сервера:', error)
    }
  }
}

const selectCredential = (credential: SSHCredential) => {
  emit('connecting')

  setTimeout(() => {
    emit('server-select', credential)
  }, 1000)
}

const showContextMenu = (event: MouseEvent, serverId: number) => {
  event.preventDefault()
  event.stopPropagation()
  contextMenu.value = {
    isVisible: true,
    x: event.clientX,
    y: event.clientY,
    serverId: serverId,
  }
}

const closeContextMenu = () => {
  contextMenu.value.isVisible = false
}

const handleDelete = async () => {
  try {
    await invoke('remove_server_from_config', { id: contextMenu.value.serverId })
    const index = credentials.findIndex((cred) => cred.id === contextMenu.value.serverId)
    if (index !== -1) {
      credentials.splice(index, 1)
    }
  } catch (error) {
    console.error('Ошибка удаления сервера:', error)
  }
  closeContextMenu()
}


const handleRename = () => {
  const server = credentials.find((cred) => cred.id === contextMenu.value.serverId)
  if (server) {
    renameState.value = {
      isVisible: true,
      serverId: server.id,
      newTitle: server.title,
    }
  }
  closeContextMenu()
}

const applyRename = async () => {
  if (renameState.value.newTitle.trim() === '') return

  const server = credentials.find((cred) => cred.id === renameState.value.serverId)
  if (server) {
    try {
      const updatedServer = await invoke('update_server_in_config', {
        id: server.id,
        title: renameState.value.newTitle,
        user: server.user,
        password: server.password,
      }) as SSHCredential

      server.title = updatedServer.title
    } catch (error) {
      console.error('Ошибка переименования сервера:', error)
    }
  }

  renameState.value.isVisible = false
}

const cancelRename = () => {
  renameState.value.isVisible = false
}

onMounted(() => {
  loadServers()
})
</script>
<template>
  <div class="ssh-container">
    <div class="container-header">
      <h2>Ваши адреса</h2>
      <button class="debug-button" @click="toggleDebug">🔧</button>
      <button class="add-button" @click="toggleForm">
        <span class="button-icon">+</span>
        <span>Добавить</span>
      </button>
    </div>

    <transition name="slide-fade">
      <form v-if="showForm" class="add-form" @submit.prevent="addCredential">
        <div class="form-group">
          <input type="text" v-model="newCredential.title" placeholder="Название" required />
        </div>
        <div class="form-group">
          <input type="text" v-model="newCredential.user" placeholder="user@ipserver" required />
        </div>
        <div class="form-group">
          <input
            type="password"
            v-model="newCredential.password"
            placeholder="passwordserver"
            required
          />
        </div>
        <div class="form-actions">
          <button type="button" class="cancel-button" @click="toggleForm">Отмена</button>
          <button type="submit" class="submit-button">Добавить</button>
        </div>
      </form>
    </transition>

    <transition name="slide-fade">
      <form v-if="renameState.isVisible" class="add-form rename-form" @submit.prevent="applyRename">
        <div class="form-group">
          <label>Переименовать сервер</label>
          <input type="text" v-model="renameState.newTitle" placeholder="Новое название" required />
        </div>
        <div class="form-actions">
          <button type="button" class="cancel-button" @click="cancelRename">Отмена</button>
          <button type="submit" class="submit-button">Сохранить</button>
        </div>
      </form>
    </transition>

    <transition name="slide-fade">
      <div v-if="debugInfo.showDebug" class="debug-panel">
        <p><strong>Путь к конфигурации:</strong> {{ debugInfo.configPath }}</p>
        <p><strong>Количество серверов:</strong> {{ credentials.length }}</p>
        <div class="debug-actions">
          <button @click="reloadServers" class="debug-action-btn">Перезагрузить</button>
          <button @click="saveServers" class="debug-action-btn">Сохранить</button>
        </div>
      </div>
    </transition>

    <transition-group name="list" tag="div" class="cards">
      <SSHCard
        v-for="credential in credentials"
        :key="credential.id"
        :title="credential.title"
        :user="credential.user"
        :password="credential.password"
        @select="selectCredential(credential)"
        @context-menu="(event) => showContextMenu(event, credential.id)"
      />
    </transition-group>

    <RightClickMenu
      v-if="contextMenu.isVisible"
      :x="contextMenu.x"
      :y="contextMenu.y"
      @rename="handleRename"
      @delete="handleDelete"
      @close="closeContextMenu"
    />
  </div>
</template>

<style lang="scss" scoped>
.ssh-container {
  background-color: #0f172a;
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
  max-width: 600px;
  width: 100%;
  margin: 0 auto;
}

.container-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;

  h2 {
    color: #f8fafc;
    font-size: 24px;
    font-weight: 600;
    margin: 0;
  }
}

.add-button {
  display: flex;
  align-items: center;
  gap: 8px;
  background-color: #2563eb;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 10px 16px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    background-color: #1d4ed8;
    transform: translateY(-2px);
  }

  &:active {
    transform: translateY(0);
  }

  .button-icon {
    font-size: 18px;
    font-weight: bold;
  }
}

.add-form,
.rename-form {
  background-color: #1e293b;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 24px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.rename-form {
  border-left: 3px solid #3b82f6;

  label {
    display: block;
    color: #94a3b8;
    font-size: 14px;
    margin-bottom: 8px;
    font-weight: 500;
  }
}

.form-group {
  margin-bottom: 16px;

  input {
    width: 100%;
    background-color: #334155;
    color: #f1f5f9;
    border: 1px solid #475569;
    border-radius: 8px;
    padding: 12px 16px;
    font-size: 14px;
    transition: all 0.2s ease;

    &:focus {
      outline: none;
      border-color: #3b82f6;
      box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.3);
    }

    &::placeholder {
      color: #94a3b8;
    }
  }
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;

  button {
    padding: 10px 16px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .cancel-button {
    background-color: transparent;
    color: #94a3b8;
    border: 1px solid #475569;

    &:hover {
      background-color: #1e293b;
      color: #f1f5f9;
    }
  }

  .submit-button {
    background-color: #2563eb;
    color: white;
    border: none;

    &:hover {
      background-color: #1d4ed8;
    }
  }
}

.cards {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: all 0.3s ease;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateY(-20px);
  opacity: 0;
}

.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(30px);
}

.list-move {
  transition: transform 0.5s ease;
}

.header-actions {
  display: flex;
  gap: 12px;
  align-items: center;
}

.debug-button {
  background-color: #374151;
  color: #9ca3af;
  border: none;
  border-radius: 6px;
  width: 36px;
  height: 36px;
  font-size: 16px;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    background-color: #4b5563;
    color: #f3f4f6;
  }
}

.debug-panel {
  background-color: #374151;
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 20px;
  font-size: 14px;
  color: #e5e7eb;

  p {
    margin: 8px 0;
  }

  .debug-actions {
    display: flex;
    gap: 8px;
    margin-top: 12px;
  }

  .debug-action-btn {
    background-color: #6b7280;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 6px 12px;
    font-size: 12px;
    cursor: pointer;

    &:hover {
      background-color: #9ca3af;
    }
  }
}
</style>
