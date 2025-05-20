<script setup lang="ts">
const props = defineProps<{
  fileName: string
  isFolder?: boolean
  serverId?: string // ID сервера, к которому относится файл
}>()

const emit = defineEmits(['contextMenu', 'doubleClick', 'dragStart'])

const handleContextMenu = (event: MouseEvent) => {
  emit('contextMenu', event, props.fileName, props.isFolder)
}

const handleDoubleClick = () => {
  if (!props.isFolder) {
    emit('doubleClick', props.fileName)
  }
}

const handleDragStart = (event: DragEvent) => {
  if (event.dataTransfer) {
    // Сохраняем информацию о перетаскиваемом файле
    event.dataTransfer.setData(
      'text/plain',
      JSON.stringify({
        fileName: props.fileName,
        isFolder: props.isFolder,
        serverId: props.serverId,
      }),
    )

    // Устанавливаем эффект копирования
    event.dataTransfer.effectAllowed = 'copyMove'

    // Информируем родительский компонент о начале перетаскивания
    emit('dragStart', {
      fileName: props.fileName,
      isFolder: props.isFolder,
      serverId: props.serverId,
    })
  }
}
</script>

<template>
  <div
    class="file-item"
    :class="{ folder: isFolder, file: !isFolder }"
    @contextmenu="handleContextMenu"
    @dblclick="handleDoubleClick"
    draggable="true"
    @dragstart="handleDragStart"
  >
    <span class="file-icon">{{ isFolder ? '📁' : '📄' }}</span>
    <span class="file-name">{{ fileName }}</span>
  </div>
</template>

<style lang="scss" scoped>
.file-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s ease;
  user-select: none;

  &:hover {
    background-color: #334155;
    transform: translateY(-4px);
  }

  &:active {
    cursor: grabbing;
  }

  .file-icon {
    font-size: 24px;
  }

  .file-name {
    color: #e2e8f0;
    font-size: 14px;
    text-align: center;
    word-break: break-word;
  }

  &.folder .file-name {
    color: #60a5fa;
  }

  &.file {
    &:active {
      transform: scale(0.98);
      background-color: #475569;
    }
  }
}
</style>
