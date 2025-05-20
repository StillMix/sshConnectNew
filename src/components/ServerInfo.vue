<script setup lang="ts">
import TextEdit from './TextEdit.vue'

defineProps<{
  server?: {
    title: string
    user: string
    password: string
  }
}>()

const emit = defineEmits(['disconnect'])

const handleDisconnect = () => {
  emit('disconnect')
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
          <span class="path-item active">/</span>
          <span class="path-separator">/</span>
          <span class="path-item">home</span>
        </div>
      </div>

      <div class="file-explorer">
        <div class="file-item folder">
          <span class="file-icon">📁</span>
          <span class="file-name">home</span>
        </div>
        <div class="file-item folder">
          <span class="file-icon">📁</span>
          <span class="file-name">var</span>
        </div>
        <div class="file-item folder">
          <span class="file-icon">📁</span>
          <span class="file-name">etc</span>
        </div>
        <div class="file-item file">
          <span class="file-icon">📄</span>
          <span class="file-name">config.json</span>
        </div>
        <div class="file-item file">
          <span class="file-icon">📄</span>
          <span class="file-name">app.log</span>
        </div>
      </div>
    </div>
    <TextEdit />
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

      .path-item {
        color: #94a3b8;
        font-size: 14px;
        cursor: pointer;
        transition: color 0.2s ease;

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
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 16px;
    padding: 16px;
    background-color: #1e293b;
    border-radius: 12px;

    .file-item {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 8px;
      padding: 12px;
      border-radius: 8px;
      cursor: pointer;
      transition: all 0.2s ease;

      &:hover {
        background-color: #334155;
        transform: translateY(-4px);
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
</style>
