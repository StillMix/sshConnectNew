<template>
  <div class="confirm-dialog-overlay" v-if="isVisible" @click.self="cancel">
    <div class="confirm-dialog">
      <div class="dialog-header">
        <h3>{{ title }}</h3>
        <button class="close-button" @click="cancel">✕</button>
      </div>
      <div class="dialog-content">
        <p>{{ message }}</p>
      </div>
      <div class="dialog-actions">
        <button class="cancel-button" @click="cancel">Отмена</button>
        <button class="confirm-button" :class="{ delete: isDelete }" @click="confirm">
          {{ confirmText }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  isVisible: boolean
  title: string
  message: string
  confirmText: string
  isDelete?: boolean
}>()

const emit = defineEmits(['confirm', 'cancel'])

const confirm = () => {
  emit('confirm')
}

const cancel = () => {
  emit('cancel')
}
</script>

<style lang="scss" scoped>
.confirm-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(15, 23, 42, 0.75);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.2s ease-out;
}

.confirm-dialog {
  width: 90%;
  max-width: 400px;
  background: linear-gradient(145deg, #1e293b, #0f172a);
  border-radius: 12px;
  box-shadow: 0 15px 40px rgba(0, 0, 0, 0.3);
  overflow: hidden;
  animation: scaleIn 0.3s ease-out;
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.2);

  h3 {
    color: #f8fafc;
    font-size: 18px;
    font-weight: 600;
    margin: 0;
  }

  .close-button {
    background-color: transparent;
    color: #94a3b8;
    border: none;
    font-size: 18px;
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

.dialog-content {
  padding: 20px;

  p {
    color: #e2e8f0;
    font-size: 16px;
    line-height: 1.6;
    margin: 0;
  }
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 20px;
  border-top: 1px solid rgba(148, 163, 184, 0.2);

  button {
    padding: 10px 16px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .cancel-button {
    background-color: #334155;
    color: #e2e8f0;
    border: none;

    &:hover {
      background-color: #475569;
      transform: translateY(-2px);
    }

    &:active {
      transform: translateY(0);
    }
  }

  .confirm-button {
    background-color: #2563eb;
    color: white;
    border: none;

    &:hover {
      background-color: #1d4ed8;
      transform: translateY(-2px);
    }

    &:active {
      transform: translateY(0);
    }

    &.delete {
      background-color: #ef4444;

      &:hover {
        background-color: #dc2626;
      }
    }
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

@keyframes scaleIn {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}
</style>
