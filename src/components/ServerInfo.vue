<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import TextEdit from './TextEdit.vue'
import RightClickMenu from './RightCLickMenu.vue'
import ServerFile from './ServerFile.vue'
import RenameDialog from './RenameDialog.vue'
import ConfirmDialog from './ConfirmDialog.vue'
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

interface FileData {
  fileName: string
  isFolder: boolean
  path: string
}

const emit = defineEmits(['disconnect', 'fileTransfer'])

// Состояние для диалога переименования
const renameDialogState = ref({
  isVisible: false,
  fileName: '',
  isFolder: false,
})

// Состояние для диалога подтверждения удаления
const deleteDialogState = ref({
  isVisible: false,
  fileName: '',
  isFolder: false,
})

const handleDisconnect = () => {
  emit('disconnect')
}

// Состояние для перетаскивания
const isDragOver = ref(false)

// Состояние для текстового редактора
const textEditorState = ref({
  isOpen: false,
  fileName: '',
  content: '',
})

// Состояние для контекстного меню
const contextMenu = ref({
  isVisible: false,
  x: 0,
  y: 0,
  fileName: '',
  isFolder: false,
})

// Состояние для отображения индикатора перетаскивания
const transferIndicator = ref({
  isVisible: false,
  fileName: '',
  sourceServerId: '',
  destinationServerId: '',
})

// Данные о файловой системе
const fileSystem = ref<FileData[]>([])
const currentPath = ref('/')
const isLoading = ref(false)

// Загрузка содержимого директории
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

    // Преобразуем результат
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
    fileSystem.value = [] // Очищаем при ошибке
  } finally {
    isLoading.value = false
  }
}

// Обработка клика на папку
const handleFolderClick = (fileName: string) => {
  const file = fileSystem.value.find((f) => f.fileName === fileName)
  if (file && file.isFolder) {
    loadDirectory(file.path)
  }
}

// Навигация по пути
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

// Загрузка директории при монтировании компонента
onMounted(() => {
  if (props.server) {
    loadDirectory('/')
  }
})

// Открывает текстовый редактор для файла
const openTextEditor = (fileName: string, content: string = '') => {
  textEditorState.value = {
    isOpen: true,
    fileName,
    content,
  }
}

// Обработчик сохранения файла
const handleSaveFile = (fileData: { fileName: string; content: string }) => {
  console.log('Сохранение файла:', fileData)
  // Здесь будет логика сохранения файла
}

// Закрытие текстового редактора
const closeTextEditor = () => {
  textEditorState.value.isOpen = false
}

// Открывает контекстное меню
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
// Закрывает контекстное меню
const closeContextMenu = () => {
  contextMenu.value.isVisible = false
}

// Обработчики для контекстного меню
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

// Функции для работы с диалогом переименования
const confirmRename = (newFileName: string) => {
  if (newFileName && newFileName.trim() !== '') {
    const fileIndex = fileSystem.value.findIndex(
      (file) => file.fileName === renameDialogState.value.fileName,
    )

    if (fileIndex !== -1) {
      fileSystem.value[fileIndex].fileName = newFileName.trim()
    }
  }

  renameDialogState.value.isVisible = false
}

const cancelRename = () => {
  renameDialogState.value.isVisible = false
}

// Функции для работы с диалогом подтверждения удаления
const confirmDelete = () => {
  const fileIndex = fileSystem.value.findIndex(
    (file) => file.fileName === deleteDialogState.value.fileName,
  )

  if (fileIndex !== -1) {
    fileSystem.value.splice(fileIndex, 1)
  }

  deleteDialogState.value.isVisible = false
}

const cancelDelete = () => {
  deleteDialogState.value.isVisible = false
}

// Обработка двойного клика на файле
const handleFileDoubleClick = (fileName: string) => {
  const file = fileSystem.value.find((f) => f.fileName === fileName)

  if (!file) return

  if (file.isFolder) {
    handleFolderClick(fileName)
    return
  }

  // Для файлов открываем текстовый редактор
  const fileContent = `Это содержимое файла ${fileName}.\nВы можете редактировать этот текст.`
  openTextEditor(fileName, fileContent)
}

// Обработчики drag and drop
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

const handleDrop = (event: DragEvent) => {
  event.preventDefault()
  isDragOver.value = false

  if (event.dataTransfer) {
    try {
      const transferData = JSON.parse(event.dataTransfer.getData('text/plain'))

      if (transferData.serverId !== props.serverId) {
        console.log('Файл перетащен с одного сервера на другой:', transferData)

        transferIndicator.value = {
          isVisible: true,
          fileName: transferData.fileName,
          sourceServerId: transferData.serverId,
          destinationServerId: props.serverId,
        }

        setTimeout(() => {
          emit('fileTransfer', {
            fileName: transferData.fileName,
            isFolder: transferData.isFolder,
            sourceServerId: transferData.serverId,
            destinationServerId: props.serverId,
          })

          transferIndicator.value.isVisible = false

          fileSystem.value.push({
            fileName: transferData.fileName,
            isFolder: transferData.isFolder,
            path:
              currentPath.value +
              (currentPath.value.endsWith('/') ? '' : '/') +
              transferData.fileName,
          })
        }, 2000)
      }
    } catch (error) {
      console.error('Ошибка при обработке перетаскиваемых данных:', error)
    }
  }
}

// Начало перетаскивания файла
const handleFileDragStart = (fileData: FileDragData) => {
  console.log('Начало перетаскивания файла:', fileData)
}

// Вычисляемое свойство для класса file-explorer
const fileExplorerClass = computed(() => {
  return {
    'file-explorer': true,
    'drag-over': isDragOver.value,
    'is-loading': isLoading.value,
  }
})
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
          @context-menu="showContextMenu"
          @double-click="handleFileDoubleClick"
          @drag-start="handleFileDragStart"
        />

        <div v-if="!isLoading && fileSystem.length === 0" class="empty-folder">
          <p>Папка пуста</p>
        </div>

        <!-- Индикатор перетаскивания -->
        <div class="transfer-indicator" v-if="transferIndicator.isVisible">
          <div class="indicator-content">
            <div class="spinner"></div>
            <p>Передача: {{ transferIndicator.fileName }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Текстовый редактор -->
    <TextEdit
      v-if="textEditorState.isOpen"
      :fileName="textEditorState.fileName"
      :content="textEditorState.content"
      :isOpen="textEditorState.isOpen"
      @save="handleSaveFile"
      @close="closeTextEditor"
    />

    <!-- Контекстное меню -->
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

    <!-- Диалог переименования -->
    <RenameDialog
      :isVisible="renameDialogState.isVisible"
      :fileName="renameDialogState.fileName"
      :isFolder="renameDialogState.isFolder"
      @confirm="confirmRename"
      @cancel="cancelRename"
    />

    <!-- Диалог подтверждения удаления -->
    <ConfirmDialog
      :isVisible="deleteDialogState.isVisible"
      :title="'Удаление ' + (deleteDialogState.isFolder ? 'папки' : 'файла')"
      :message="deleteConfirmMessage"
      :confirmText="'Удалить'"
      :isDelete="true"
      @confirm="confirmDelete"
      @cancel="cancelDelete"
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
      max-width: 60%;
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
