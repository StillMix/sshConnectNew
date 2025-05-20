<script setup lang="ts">
defineProps<{
  title: string
  user: string
  password: string
}>()

const emit = defineEmits(['select', 'context-menu'])

const handleSelect = () => {
  emit('select')
}

const handleContextMenu = (event: MouseEvent) => {
  event.preventDefault()
  event.stopPropagation() // Останавливаем всплытие
  emit('context-menu', event)
}
</script>

<template>
  <button class="ssh-card" @click="handleSelect" @contextmenu="handleContextMenu">
    <div class="card-content">
      <div class="card-header">
        <div class="title">{{ title }}</div>
        <div class="connection-status"></div>
      </div>
      <div class="card-body">
        <div class="credential">
          <span class="label">Пользователь:</span>
          <span class="value">{{ user }}</span>
        </div>
        <div class="credential">
          <span class="label">Пароль:</span>
          <span class="value">{{ password.replace(/./g, '•') }}</span>
        </div>
      </div>
    </div>
  </button>
</template>

<style lang="scss" scoped>
.ssh-card {
  position: relative;
  width: 100%;
  background: linear-gradient(145deg, #1e293b, #0f172a);
  color: #e2e8f0;
  border: none;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 16px;
  text-align: left;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  cursor: pointer;

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    width: 4px;
    height: 100%;
    background: #3b82f6;
    transform: scaleY(0);
    transform-origin: bottom;
    transition: transform 0.3s ease;
  }

  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);

    &::before {
      transform: scaleY(1);
    }
  }

  &:active {
    transform: translateY(0);
  }

  .card-content {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;

    .title {
      font-size: 18px;
      font-weight: 600;
      color: #ffffff;
    }

    .connection-status {
      width: 10px;
      height: 10px;
      border-radius: 50%;
      background-color: #10b981;
      box-shadow: 0 0 12px #10b981;
    }
  }

  .card-body {
    display: flex;
    flex-direction: column;
    gap: 8px;

    .credential {
      display: flex;
      align-items: center;
      gap: 8px;

      .label {
        color: #94a3b8;
        font-size: 14px;
      }

      .value {
        font-family: 'Menlo', monospace;
        font-size: 14px;
        color: #cbd5e1;
        background-color: rgba(30, 41, 59, 0.7);
        padding: 2px 8px;
        border-radius: 4px;
      }
    }
  }
}
</style>
