<template>
  <div class="create-dialog-overlay" v-if="isVisible" @click.self="cancel">
    <div class="create-dialog">
      <div class="dialog-header">
        <h3>Создать новый элемент</h3>
        <button class="close-button" @click="cancel">✕</button>
      </div>

      <div class="dialog-content">
        <div class="type-selector">
          <button
            class="type-button"
            :class="{ active: itemType === 'file' }"
            @click="itemType = 'file'"
          >
            <span class="type-icon">📄</span>
            <span>Файл</span>
          </button>
          <button
            class="type-button"
            :class="{ active: itemType === 'folder' }"
            @click="itemType = 'folder'"
          >
            <span class="type-icon">📁</span>
            <span>Папка</span>
          </button>
        </div>

        <div class="input-group">
          <label for="itemName">Название {{ itemType === 'folder' ? 'папки' : 'файла' }}:</label>
          <input
            type="text"
            id="itemName"
            v-model="itemName"
            ref="inputField"
            @keyup.enter="confirm"
            :placeholder="
              itemType === 'folder' ? 'Введите название папки' : 'Введите название файла'
            "
          />
        </div>
      </div>

      <div class="dialog-actions">
        <button class="cancel-button" @click="cancel">Отмена</button>
        <button class="confirm-button" @click="confirm" :disabled="!isValid">
          Создать {{ itemType === 'folder' ? 'папку' : 'файл' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'

const props = defineProps<{
  isVisible: boolean
}>()

const emit = defineEmits(['confirm', 'cancel'])

const itemType = ref<'file' | 'folder'>('file')
const itemName = ref('')
const inputField = ref<HTMLInputElement | null>(null)

watch(
  () => props.isVisible,
  (visible) => {
    if (visible) {
      itemName.value = ''
      itemType.value = 'file'
      setTimeout(() => {
        if (inputField.value) {
          inputField.value.focus()
        }
      }, 100)
    }
  },
)

const isValid = computed(() => {
  return itemName.value.trim() !== ''
})

const confirm = () => {
  if (isValid.value) {
    emit('confirm', {
      name: itemName.value.trim(),
      isFolder: itemType.value === 'folder',
    })
  }
}

const cancel = () => {
  emit('cancel')
}
</script>

<style lang="scss" scoped>
.create-dialog-overlay {
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

.create-dialog {
  width: 90%;
  max-width: 500px;
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

  .type-selector {
    display: flex;
    gap: 12px;
    margin-bottom: 20px;

    .type-button {
      flex: 1;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 8px;
      padding: 16px 12px;
      border: 2px solid #334155;
      border-radius: 8px;
      background-color: #1e293b;
      color: #94a3b8;
      cursor: pointer;
      transition: all 0.2s ease;

      &:hover {
        border-color: #475569;
        color: #f1f5f9;
      }

      &.active {
        border-color: #3b82f6;
        background-color: rgba(59, 130, 246, 0.1);
        color: #3b82f6;
      }

      .type-icon {
        font-size: 24px;
      }
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

      &::placeholder {
        color: #64748b;
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
