<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  isVisible: boolean
  fileName: string
  isFolder: boolean
  currentX: number
  currentY: number
}>()

// Определяем отображаемое имя файла с учетом возможной длины
const displayFileName = computed(() => {
  if (props.fileName.length > 25) {
    return props.fileName.slice(0, 22) + '...'
  }
  return props.fileName
})
</script>

<template>
  <div
    v-if="isVisible"
    class="file-drag-indicator"
    :style="{ left: `${currentX}px`, top: `${currentY}px` }"
  >
    <div class="indicator-content">
      <span class="file-icon">{{ isFolder ? '📁' : '📄' }}</span>
      <span class="file-name">{{ displayFileName }}</span>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.file-drag-indicator {
  position: fixed;
  z-index: 1100;
  pointer-events: none;
  transform: translate(-50%, -50%);
  animation: fadeIn 0.2s ease;

  .indicator-content {
    display: flex;
    align-items: center;
    gap: 8px;
    background-color: rgba(15, 23, 42, 0.9);
    border: 1px solid #3b82f6;
    border-radius: 8px;
    padding: 8px 12px;
    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);

    .file-icon {
      font-size: 16px;
    }

    .file-name {
      color: #f1f5f9;
      font-size: 14px;
      font-weight: 500;
    }
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translate(-50%, -50%) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translate(-50%, -50%) scale(1);
  }
}
</style>
