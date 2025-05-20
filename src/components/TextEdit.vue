<script setup lang="ts">
import { ref, onMounted } from 'vue'

const props = defineProps<{
  fileName?: string
  content?: string
  isOpen?: boolean
}>()

const emit = defineEmits(['save', 'close'])

const fileContent = ref(props.content || '')

const handleSave = () => {
  emit('save', {
    fileName: props.fileName,
    content: fileContent.value,
  })
}

const handleClose = () => {
  emit('close')
}

onMounted(() => {
  // Устанавливаем фокус на текстовую область при открытии
  const textarea = document.querySelector('.editor-textarea')
  if (textarea) {
    ;(textarea as HTMLTextAreaElement).focus()
  }
})
</script>

<template>
  <div class="text-edit" v-if="isOpen">
    <div class="text-edit-header">
      <div class="file-info">
        <span class="file-icon">📄</span>
        <h3 class="file-name">{{ fileName || 'Новый файл' }}</h3>
      </div>
      <div class="header-actions">
        <button class="save-button" @click="handleSave">
          <span class="button-icon">💾</span>
          <span>Сохранить</span>
        </button>
        <button class="close-button" @click="handleClose">✕</button>
      </div>
    </div>
    <div class="editor-container">
      <textarea
        v-model="fileContent"
        class="editor-textarea"
        :placeholder="'Введите текст здесь...'"
        spellcheck="false"
      ></textarea>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.text-edit {
  position: fixed;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 80%;
  max-width: 800px;
  max-height: 80vh;
  background: linear-gradient(145deg, #1e293b, #0f172a);
  border-radius: 12px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
  display: flex;
  flex-direction: column;
  z-index: 100;
  animation: fadeIn 0.2s ease-out;
  overflow: hidden;

  .text-edit-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 16px 20px;
    border-bottom: 1px solid rgba(148, 163, 184, 0.2);

    .file-info {
      display: flex;
      align-items: center;
      gap: 12px;

      .file-icon {
        font-size: 20px;
      }

      .file-name {
        color: #f8fafc;
        font-size: 18px;
        font-weight: 600;
        margin: 0;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        max-width: 300px;
      }
    }

    .header-actions {
      display: flex;
      align-items: center;
      gap: 12px;

      .save-button {
        display: flex;
        align-items: center;
        gap: 8px;
        background-color: #2563eb;
        color: white;
        border: none;
        border-radius: 6px;
        padding: 8px 16px;
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
          font-size: 16px;
        }
      }

      .close-button {
        background-color: #334155;
        color: #94a3b8;
        border: none;
        border-radius: 6px;
        width: 36px;
        height: 36px;
        font-size: 16px;
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
          background-color: #475569;
          color: #f1f5f9;
        }
      }
    }
  }

  .editor-container {
    flex: 1;
    padding: 16px;
    overflow: hidden;
    display: flex;
    min-height: 300px;

    .editor-textarea {
      flex: 1;
      height: 100%;
      min-height: 300px;
      background-color: #1e293b;
      color: #f8fafc;
      border: 1px solid #334155;
      border-radius: 8px;
      padding: 16px;
      font-family: 'Menlo', monospace;
      font-size: 14px;
      line-height: 1.6;
      resize: none;

      &:focus {
        outline: none;
        border-color: #3b82f6;
        box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
      }

      &::placeholder {
        color: #64748b;
      }
    }
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translate(-50%, -45%);
  }
  to {
    opacity: 1;
    transform: translate(-50%, -50%);
  }
}

@media (max-width: 768px) {
  .text-edit {
    width: 95%;
    max-height: 85vh;

    .text-edit-header {
      .file-info {
        .file-name {
          max-width: 150px;
        }
      }

      .header-actions {
        .save-button span:not(.button-icon) {
          display: none;
        }
      }
    }
  }
}
</style>
