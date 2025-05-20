<template>
  <div class="rename-dialog-overlay" v-if="isVisible" @click.self="cancel">
    <div class="rename-dialog">
      <div class="dialog-header">
        <h3>Переименовать {{ isFolder ? 'папку' : 'файл' }}</h3>
        <button class="close-button" @click="cancel">✕</button>
      </div>
      <div class="dialog-content">
        <div class="file-info">
          <span class="file-icon">{{ isFolder ? '📁' : '📄' }}</span>
          <span class="file-name">{{ fileName }}</span>
        </div>
        <div class="input-group">
          <label for="newFileName">Новое имя:</label>
          <input
            type="text"
            id="newFileName"
            v-model="newFileName"
            ref="inputField"
            @keyup.enter="confirm"
          />
        </div>
      </div>
      <div class="dialog-actions">
        <button class="cancel-button" @click="cancel">Отмена</button>
        <button class="confirm-button" @click="confirm" :disabled="!isValid">Переименовать</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, computed } from 'vue'

const props = defineProps<{
  isVisible: boolean
  fileName: string
  isFolder?: boolean
}>()

const emit = defineEmits(['confirm', 'cancel'])

const newFileName = ref('')
const inputField = ref<HTMLInputElement | null>(null)

// Следим за изменениями имени файла
watch(
  () => props.fileName,
  (newValue) => {
    newFileName.value = newValue
  },
)

// Устанавливаем фокус на поле ввода при открытии окна
watch(
  () => props.isVisible,
  (visible) => {
    if (visible) {
      newFileName.value = props.fileName
      setTimeout(() => {
        if (inputField.value) {
          inputField.value.focus()
          inputField.value.select()
        }
      }, 100)
    }
  },
)

// Проверка валидности имени файла
const isValid = computed(() => {
  return newFileName.value.trim() !== '' && newFileName.value.trim() !== props.fileName
})

const confirm = () => {
  if (isValid.value) {
    emit('confirm', newFileName.value.trim())
  }
}

const cancel = () => {
  emit('cancel')
}

onMounted(() => {
  if (props.isVisible && inputField.value) {
    inputField.value.focus()
    inputField.value.select()
  }
})
</script>

<style lang="scss" scoped>
.rename-dialog-overlay {
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

.rename-dialog {
  width: 90%;
  max-width: 450px;
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

  .file-info {
    display: flex;
    align-items: center;
    gap: 12px;
    margin-bottom: 20px;
    padding: 12px;
    border-radius: 8px;
    background-color: #0f172a;

    .file-icon {
      font-size: 24px;
    }

    .file-name {
      color: #e2e8f0;
      font-size: 16px;
      word-break: break-word;
    }
  }

  .input-group {
    display: flex;
    flex-direction: column;
    gap: 8px;

    label {
      color: #94a3b8;
      font-size: 14px;
    }

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
    }
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

    &:hover:not(:disabled) {
      background-color: #1d4ed8;
      transform: translateY(-2px);
    }

    &:active:not(:disabled) {
      transform: translateY(0);
    }

    &:disabled {
      background-color: #3b82f670;
      cursor: not-allowed;
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
