<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import TextEdit from './TextEdit.vue'
import RightClickMenu from './RightCLickMenu.vue'
import ServerFile from './ServerFile.vue'
import RenameDialog from './RenameDialog.vue'
import ConfirmDialog from './ConfirmDialog.vue'
import CreateFileDialog from './CreateFileDialog.vue'
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  server?: {
    title: string
    user: string
    password: string
  } | null
  serverId: string
}>()

interface FileDragData {
  fileName: string
  isFolder: boolean
  serverId: string
}

interface FileContent {
  content: string
  is_editable: boolean
  file_type: string
}

interface FileData {
  fileName: string
  isFolder: boolean
  path: string
}

const emit = defineEmits(['disconnect', 'fileTransfer'])

const renameDialogState = ref({
  isVisible: false,
  fileName: '',
  isFolder: false,
})

const deleteDialogState = ref({
  isVisible: false,
  fileName: '',
  isFolder: false,
})

const createDialogState = ref({
  isVisible: false,
})

const handleDisconnect = () => {
  emit('disconnect')
}

const isDragOver = ref(false)

const textEditorState = ref({
  isOpen: false,
  fileName: '',
  content: '',
})

const contextMenu = ref({
  isVisible: false,
  x: 0,
  y: 0,
  fileName: '',
  isFolder: false,
})

const transferIndicator = ref({
  isVisible: false,
  fileName: '',
  sourceServerId: '',
  destinationServerId: '',
})

const fileSystem = ref<FileData[]>([])
const currentPath = ref('/')
const isLoading = ref(false)

const loadDirectory = async (path = '/') => {
  if (!props.server) return

  isLoading.value = true

  try {
    const username = props.server.user.includes('@')
      ? props.server.user.split('@')[0]
      : props.server.user

    const files = await invoke('list_directory', {
      connectionInfo: {
        username: username,
        host: props.server.user,
        password: props.server.password,
      },
      path: path,
    })

    fileSystem.value = Array.isArray(files)
      ? files.map((file) => ({
          fileName: file.name,
          isFolder: file.is_folder,
          path: file.path,
        }))
      : []

    currentPath.value = path
  } catch (error) {
    console.error('Ошибка при загрузке директории:', error)
    fileSystem.value = []
  } finally {
    isLoading.value = false
  }
}

const handleFolderClick = (fileName: string) => {
  const file = fileSystem.value.find((f) => f.fileName === fileName)
  if (file && file.isFolder) {
    loadDirectory(file.path)
  }
}

const pathParts = computed(() => {
  const parts = currentPath.value.split('/').filter(Boolean)
  return ['/', ...parts]
})

const navigateTo = (index: number) => {
  if (index === 0) {
    loadDirectory('/')
    return
  }

  const parts = pathParts.value.slice(1, index + 1)
  const path = '/' + parts.join('/')
  loadDirectory(path)
}

const deleteConfirmMessage = computed(() => {
  const fileType = deleteDialogState.value.isFolder ? 'папку' : 'файл'
  return `Вы уверены, что хотите удалить ${fileType} "${deleteDialogState.value.fileName}"?`
})

onMounted(() => {
  if (props.server) {
    loadDirectory('/')
  }
})

const openTextEditor = (fileName: string, content: string = '') => {
  textEditorState.value = {
    isOpen: true,
    fileName,
    content,
  }
}

const handleSaveFile = async (fileData: { fileName: string; content: string }) => {
  if (!props.server) return

  const file = fileSystem.value.find((f) => f.fileName === fileData.fileName)
  if (!file) return

  try {
    const username = props.server.user.includes('@')
      ? props.server.user.split('@')[0]
      : props.server.user

    await invoke('save_file_content', {
      connectionInfo: {
        username: username,
        host: props.server.user,
        password: props.server.password,
      },
      filePath: file.path,
      content: fileData.content,
    })

    console.log('Файл успешно сохранен')
  } catch (error) {
    console.error('Ошибка сохранения файла:', error)
  }
}

const closeTextEditor = () => {
  textEditorState.value.isOpen = false
}

const showContextMenu = (event: MouseEvent, fileName: string, isFolder: boolean = false) => {
  event.preventDefault()
  if (event.target && (event.target as HTMLElement).closest('.file-item')) {
    contextMenu.value = {
      isVisible: true,
      x: event.clientX,
      y: event.clientY,
      fileName,
      isFolder,
    }
  }
}

const closeContextMenu = () => {
  contextMenu.value.isVisible = false
}

const handleRename = () => {
  renameDialogState.value = {
    isVisible: true,
    fileName: contextMenu.value.fileName,
    isFolder: contextMenu.value.isFolder,
  }
  closeContextMenu()
}

const handleDelete = () => {
  deleteDialogState.value = {
    isVisible: true,
    fileName: contextMenu.value.fileName,
    isFolder: contextMenu.value.isFolder,
  }
  closeContextMenu()
}

const confirmRename = async (newFileName: string) => {
  if (!newFileName || newFileName.trim() === '' || !props.server) return

  const file = fileSystem.value.find((f) => f.fileName === renameDialogState.value.fileName)
  if (!file) return

  const oldPath = file.path
  const newPath =
    currentPath.value + (currentPath.value.endsWith('/') ? '' : '/') + newFileName.trim()

  try {
    const username = props.server.user.includes('@')
      ? props.server.user.split('@')[0]
      : props.server.user

    await invoke('rename_file', {
      connectionInfo: {
        username: username,
        host: props.server.user,
        password: props.server.password,
      },
      oldPath: oldPath,
      newPath: newPath,
    })

    const fileIndex = fileSystem.value.findIndex(
      (f) => f.fileName === renameDialogState.value.fileName,
    )
    if (fileIndex !== -1) {
      fileSystem.value[fileIndex].fileName = newFileName.trim()
      fileSystem.value[fileIndex].path = newPath
    }
  } catch (error) {
    console.error('Ошибка переименования:', error)
  }

  renameDialogState.value.isVisible = false
}

const cancelRename = () => {
  renameDialogState.value.isVisible = false
}

const confirmDelete = async () => {
  if (!props.server) return

  const file = fileSystem.value.find((f) => f.fileName === deleteDialogState.value.fileName)
  if (!file) return

  try {
    const username = props.server.user.includes('@')
      ? props.server.user.split('@')[0]
      : props.server.user

    if (file.isFolder) {
      await invoke('delete_directory', {
        connectionInfo: {
          username: username,
          host: props.server.user,
          password: props.server.password,
        },
        dirPath: file.path,
      })
    } else {
      await invoke('delete_file', {
        connectionInfo: {
          username: username,
          host: props.server.user,
          password: props.server.password,
        },
        filePath: file.path,
      })
    }

    const fileIndex = fileSystem.value.findIndex(
      (f) => f.fileName === deleteDialogState.value.fileName,
    )
    if (fileIndex !== -1) {
      fileSystem.value.splice(fileIndex, 1)
    }
  } catch (error) {
    console.error('Ошибка удаления:', error)
  }

  deleteDialogState.value.isVisible = false
}
const cancelDelete = () => {
  deleteDialogState.value.isVisible = false
}

const handleFileDoubleClick = async (fileName: string) => {
  const file = fileSystem.value.find((f) => f.fileName === fileName)

  if (!file) return

  if (file.isFolder) {
    handleFolderClick(fileName)
    return
  }

  if (!props.server) return

  try {
    const username = props.server.user.includes('@')
      ? props.server.user.split('@')[0]
      : props.server.user

    const fileContent = (await invoke('read_file_content', {
      connectionInfo: {
        username: username,
        host: props.server.user,
        password: props.server.password,
      },
      filePath: file.path,
    })) as FileContent

    if (fileContent.is_editable) {
      openTextEditor(fileName, fileContent.content)
    } else {
      openTextEditor(fileName, fileContent.content)
    }
  } catch (error) {
    console.error('Ошибка чтения файла:', error)
    const errorContent = `Ошибка чтения файла: ${error}`
    openTextEditor(fileName, errorContent)
  }
}

const handleDragOver = (event: DragEvent) => {
  event.preventDefault()
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'copy'
  }
  isDragOver.value = true
}

const handleDragLeave = () => {
  isDragOver.value = false
}

const handleDrop = async (event: DragEvent) => {
  event.preventDefault()
  isDragOver.value = false

  if (!event.dataTransfer || !props.server) return

  try {
    const transferData = JSON.parse(event.dataTransfer.getData('text/plain'))

    if (transferData.serverId === props.serverId) {
      console.log('Перемещение файла в пределах одного сервера')
      return
    }

    console.log('Файл перетащен с одного сервера на другой:', transferData)

    transferIndicator.value = {
      isVisible: true,
      fileName: transferData.fileName,
      sourceServerId: transferData.serverId,
      destinationServerId: props.serverId,
    }

    const sourceServer = getServerInfoById(transferData.serverId)
    if (!sourceServer) {
      throw new Error('Не удалось найти информацию об исходном сервере')
    }

    const sourceUsername = sourceServer.user.includes('@')
      ? sourceServer.user.split('@')[0]
      : sourceServer.user

    const destUsername = props.server.user.includes('@')
      ? props.server.user.split('@')[0]
      : props.server.user

    const destinationPath =
      currentPath.value + (currentPath.value.endsWith('/') ? '' : '/') + transferData.fileName

    const transferRequest = {
      source_connection: {
        username: sourceUsername,
        host: sourceServer.user,
        password: sourceServer.password,
      },
      destination_connection: {
        username: destUsername,
        host: props.server.user,
        password: props.server.password,
      },
      file_path: transferData.fullPath || transferData.basePath + transferData.fileName,
      is_folder: transferData.isFolder,
      destination_path: destinationPath,
    }

    await invoke('transfer_file_between_servers', { transferRequest })

    fileSystem.value.push({
      fileName: transferData.fileName,
      isFolder: transferData.isFolder,
      path: destinationPath,
    })

    transferIndicator.value.isVisible = false

    console.log('Файл успешно передан между серверами')
  } catch (error) {
    console.error('Ошибка при передаче файла:', error)
    transferIndicator.value.isVisible = false

    alert(`Ошибка передачи файла: ${error}`)
  }
}

const handleFileDragStart = (fileData: FileDragData) => {
  console.log('Начало перетаскивания файла:', fileData)
}

const fileExplorerClass = computed(() => {
  return {
    'file-explorer': true,
    'drag-over': isDragOver.value,
    'is-loading': isLoading.value,
  }
})

const openCreateDialog = () => {
  createDialogState.value.isVisible = true
}

const confirmCreate = async (data: { name: string; isFolder: boolean }) => {
  if (!props.server) return

  const newPath = currentPath.value + (currentPath.value.endsWith('/') ? '' : '/') + data.name

  try {
    const username = props.server.user.includes('@')
      ? props.server.user.split('@')[0]
      : props.server.user

    const connectionInfo = {
      username: username,
      host: props.server.user,
      password: props.server.password,
    }

    console.log('Создание элемента:', {
      type: data.isFolder ? 'папка' : 'файл',
      path: newPath,
      connectionInfo: { ...connectionInfo, password: '***' },
    })

    if (data.isFolder) {
      await invoke('create_directory', {
        connectionInfo: connectionInfo,
        dirPath: newPath,
      })
      console.log('Папка успешно создана')
    } else {
      await invoke('create_file', {
        connectionInfo: connectionInfo,
        filePath: newPath,
      })
      console.log('Файл успешно создан')
    }

    fileSystem.value.push({
      fileName: data.name,
      isFolder: data.isFolder,
      path: newPath,
    })

    createDialogState.value.isVisible = false
  } catch (error) {
    console.error(`Подробная ошибка создания ${data.isFolder ? 'папки' : 'файла'}:`, error)
    console.error('Тип ошибки:', typeof error)

    alert(`Ошибка создания ${data.isFolder ? 'папки' : 'файла'}: ${error}`)
  }
}

const cancelCreate = () => {
  createDialogState.value.isVisible = false
}
</script>

<template>
  <div class="serverinfo">
    <div class="server-header">
      <div class="server-title">
        <h2>{{ server?.title }}</h2>
        <span class="connection-badge">Подключено</span>
      </div>
      <button class="disconnect-button" @click="handleDisconnect">
        <span class="button-icon">⏻</span>
        <span>Отключиться</span>
      </button>
    </div>

    <div class="server-credentials">
      <div class="credential-item">
        <span class="credential-label">Пользователь:</span>
        <span class="credential-value">{{ server?.user }}</span>
      </div>
    </div>

    <div class="server-content">
      <div class="content-header">
        <h3>Файловая система</h3>
        <div class="path-navigation">
          <span
            v-for="(part, index) in pathParts"
            :key="index"
            class="path-item"
            :class="{ active: index === pathParts.length - 1 }"
            @click="navigateTo(index)"
          >
            {{ part === '/' ? '/' : part }}
          </span>
          <span v-if="pathParts.length > 1" class="path-separator">/</span>
        </div>
        <button class="create-button" @click="openCreateDialog">
          <span class="button-icon">+</span>
          <span>Создать</span>
        </button>
      </div>

      <div
        :class="fileExplorerClass"
        @dragover="handleDragOver"
        @dragleave="handleDragLeave"
        @drop="handleDrop"
      >
        <div v-if="isLoading" class="loading-overlay">
          <div class="spinner"></div>
          <p>Загрузка файлов...</p>
        </div>

        <ServerFile
          v-for="file in fileSystem"
          :key="file.fileName"
          :fileName="file.fileName"
          :isFolder="file.isFolder"
          :serverId="serverId"
          :fullPath="file.path"
          :basePath="currentPath"
          @context-menu="showContextMenu"
          @double-click="handleFileDoubleClick"
          @drag-start="handleFileDragStart"
        />

        <div v-if="!isLoading && fileSystem.length === 0" class="empty-folder">
          <p>Папка пуста</p>
        </div>

        <div class="transfer-indicator" v-if="transferIndicator.isVisible">
          <div class="indicator-content">
            <div class="spinner"></div>
            <p>Передача: {{ transferIndicator.fileName }}</p>
          </div>
        </div>
      </div>
    </div>

    <TextEdit
      v-if="textEditorState.isOpen"
      :fileName="textEditorState.fileName"
      :content="textEditorState.content"
      :isOpen="textEditorState.isOpen"
      @save="handleSaveFile"
      @close="closeTextEditor"
    />

    <RightClickMenu
      v-if="contextMenu.isVisible"
      :x="contextMenu.x"
      :y="contextMenu.y"
      :fileName="contextMenu.fileName"
      :isFolder="contextMenu.isFolder"
      @rename="handleRename"
      @delete="handleDelete"
      @close="closeContextMenu"
    />

    <RenameDialog
      :isVisible="renameDialogState.isVisible"
      :fileName="renameDialogState.fileName"
      :isFolder="renameDialogState.isFolder"
      @confirm="confirmRename"
      @cancel="cancelRename"
    />

    <ConfirmDialog
      :isVisible="deleteDialogState.isVisible"
      :title="'Удаление ' + (deleteDialogState.isFolder ? 'папки' : 'файла')"
      :message="deleteConfirmMessage"
      :confirmText="'Удалить'"
      :isDelete="true"
      @confirm="confirmDelete"
      @cancel="cancelDelete"
    />

    <CreateFileDialog
      :isVisible="createDialogState.isVisible"
      @confirm="confirmCreate"
      @cancel="cancelCreate"
    />
  </div>
</template>

<style lang="scss" scoped>
.serverinfo {
  display: flex;
  flex-direction: column;
  gap: 24px;
  max-width: 900px;
  margin: 0 auto;
  background-color: #0f172a;
  border-radius: 16px;
  padding: 30px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
  animation: fadeIn 0.5s ease-out;
}

.server-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 16px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.2);

  .server-title {
    display: flex;
    align-items: center;
    gap: 16px;

    h2 {
      color: #f8fafc;
      font-size: 24px;
      font-weight: 600;
      margin: 0;
    }

    .connection-badge {
      background-color: #10b981;
      color: white;
      font-size: 12px;
      font-weight: 500;
      padding: 4px 10px;
      border-radius: 12px;
      display: inline-block;
    }
  }
}

.disconnect-button {
  display: flex;
  align-items: center;
  gap: 8px;
  background-color: #ef4444;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 10px 16px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;

  &:hover {
    background-color: #dc2626;
    transform: translateY(-2px);
  }

  &:active {
    transform: translateY(0);
  }

  .button-icon {
    font-size: 18px;
  }
}

.server-credentials {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 16px;
  background-color: #1e293b;
  border-radius: 12px;

  .credential-item {
    display: flex;
    align-items: center;
    gap: 12px;

    .credential-label {
      color: #94a3b8;
      font-size: 14px;
      min-width: 120px;
    }

    .credential-value {
      font-family: 'Menlo', monospace;
      font-size: 14px;
      color: #cbd5e1;
      background-color: rgba(30, 41, 59, 0.7);
      padding: 4px 12px;
      border-radius: 4px;
    }
  }
}

.server-content {
  display: flex;
  flex-direction: column;
  gap: 16px;

  .content-header {
    display: flex;
    justify-content: space-between;
    align-items: center;

    h3 {
      color: #f1f5f9;
      font-size: 18px;
      font-weight: 600;
      margin: 0;
    }

    .path-navigation {
      display: flex;
      align-items: center;
      gap: 4px;
      background-color: #1e293b;
      padding: 6px 12px;
      border-radius: 8px;
      flex-wrap: wrap;
      max-width: 50%;
      overflow-x: auto;

      .path-item {
        color: #94a3b8;
        font-size: 14px;
        cursor: pointer;
        transition: color 0.2s ease;
        white-space: nowrap;

        &:hover {
          color: #f1f5f9;
        }

        &.active {
          color: #3b82f6;
          font-weight: 500;
        }
      }

      .path-separator {
        color: #64748b;
        font-size: 14px;
      }
    }

    .create-button {
      display: flex;
      align-items: center;
      gap: 8px;
      background-color: #10b981;
      color: white;
      border: none;
      border-radius: 8px;
      padding: 8px 16px;
      font-size: 14px;
      font-weight: 500;
      cursor: pointer;
      transition: all 0.2s ease;

      &:hover {
        background-color: #059669;
        transform: translateY(-2px);
      }

      &:active {
        transform: translateY(0);
      }

      .button-icon {
        font-size: 16px;
        font-weight: bold;
      }
    }
  }

  .file-explorer {
    position: relative;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 16px;
    padding: 16px;
    background-color: #1e293b;
    border-radius: 12px;
    transition: all 0.3s ease;
    min-height: 300px;

    &.drag-over {
      background-color: #2d3a4f;
      box-shadow: 0 0 0 2px #3b82f6;
      transform: scale(1.01);
    }

    &.is-loading {
      opacity: 0.7;
    }
  }
}

.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  background-color: rgba(15, 23, 42, 0.7);
  border-radius: 12px;
  z-index: 5;

  .spinner {
    width: 40px;
    height: 40px;
    border: 3px solid rgba(59, 130, 246, 0.3);
    border-radius: 50%;
    border-top-color: #3b82f6;
    animation: spin 1s linear infinite;
  }

  p {
    color: #f1f5f9;
    font-size: 16px;
    font-weight: 500;
  }
}

.empty-folder {
  grid-column: 1 / -1;
  display: flex;
  justify-content: center;
  align-items: center;
  height: 150px;

  p {
    color: #94a3b8;
    font-size: 16px;
  }
}

.transfer-indicator {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(15, 23, 42, 0.8);
  border-radius: 12px;
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 10;
  animation: fadeIn 0.3s ease;

  .indicator-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;

    p {
      color: #f1f5f9;
      font-size: 16px;
      font-weight: 500;
      margin: 0;
    }

    .spinner {
      width: 40px;
      height: 40px;
      border: 3px solid rgba(59, 130, 246, 0.3);
      border-radius: 50%;
      border-top-color: #3b82f6;
      animation: spin 1s linear infinite;
    }
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
