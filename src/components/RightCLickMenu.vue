<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue'

defineProps<{
  x: number
  y: number
  fileName?: string
  isFolder?: boolean
}>()

const emit = defineEmits(['rename', 'delete', 'close'])

const handleRename = () => {
  emit('rename')
}

const handleDelete = () => {
  emit('delete')
}

// Закрываем меню при клике вне него
const handleOutsideClick = (event: MouseEvent) => {
  const menu = document.querySelector('.right-click-menu')
  if (menu && !menu.contains(event.target as Node)) {
    emit('close')
  }
}

// Закрываем меню при нажатии Esc
const handleEscKey = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    emit('close')
  }
}

onMounted(() => {
  document.addEventListener('click', handleOutsideClick)
  document.addEventListener('keydown', handleEscKey)
})

onUnmounted(() => {
  document.removeEventListener('click', handleOutsideClick)
  document.removeEventListener('keydown', handleEscKey)
})
</script>

<template>
  <div class="right-click-menu" :style="{ top: `${y}px`, left: `${x}px` }" @click.stop>
    <button class="menu-item" @click="handleRename">
      <span class="item-icon">✏️</span>
      <span>Переименовать</span>
    </button>
    <button class="menu-item delete" @click="handleDelete">
      <span class="item-icon">🗑️</span>
      <span>Удалить</span>
    </button>
  </div>
</template>
<style lang="scss" scoped>
.right-click-menu {
  position: fixed;
  z-index: 1000;
  background: linear-gradient(145deg, #1e293b, #0f172a);
  border-radius: 8px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.3);
  overflow: hidden;
  border: 1px solid #334155;
  animation: menuAppear 0.15s ease-out;
  min-width: 180px;

  .menu-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 12px 16px;
    width: 100%;
    text-align: left;
    font-size: 14px;
    color: #f1f5f9;
    background: transparent;
    border: none;
    cursor: pointer;
    transition: all 0.15s ease;

    &:hover {
      background-color: #334155;
    }

    &:active {
      background-color: #475569;
    }

    &.delete {
      color: #f87171;

      &:hover {
        background-color: rgba(239, 68, 68, 0.2);
      }
    }

    .item-icon {
      font-size: 16px;
    }
  }

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 4px;
    height: 100%;
    background: #3b82f6;
  }
}

@keyframes menuAppear {
  from {
    opacity: 0;
    transform: scale(0.9);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
