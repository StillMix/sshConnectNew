<script setup lang="ts">
import { ref, reactive } from 'vue'
import SSHContainer from './SSHContainer.vue'

interface ServerCredential {
  id?: number
  title: string
  user: string
  password: string
}

const connectionState = ref<'none' | 'loading' | 'success' | 'error'>('none')
const connectionMessages = reactive({
  none: 'Ожидание подключения',
  loading: 'Выполняется подключение...',
  success: 'Успешно подключились',
  error: 'Неудачное подключение',
})

const credentials = reactive({
  user: '',
  password: '',
})

const emit = defineEmits(['server-select'])

const connect = () => {
  if (!credentials.user || !credentials.password) return

  connectionState.value = 'loading'

  // Имитация подключения
  setTimeout(() => {
    // Успех для демонстрации
    connectionState.value = 'success'

    // После успешного подключения передаем данные наверх
    setTimeout(() => {
      emit('server-select', {
        title: 'Быстрое подключение',
        user: credentials.user,
        password: credentials.password,
      })
    }, 1000)
  }, 2000)
}

const handleServerSelect = (server: ServerCredential) => {
  emit('server-select', server)
}
</script>

<template>
  <div class="ssh-connect">
    <div class="connection-status" :class="connectionState">
      <div class="status-icon"></div>
      <p class="status-message">{{ connectionMessages[connectionState] }}</p>
    </div>

    <SSHContainer @server-select="handleServerSelect" />

    <div class="connection-panel">
      <div class="panel-header">
        <h3>Быстрое подключение</h3>
      </div>

      <form @submit.prevent="connect" class="connection-form">
        <div class="form-group">
          <label for="userInput">Логин и сервер</label>
          <div class="input-container">
            <span class="input-icon">👤</span>
            <input
              id="userInput"
              type="text"
              v-model="credentials.user"
              placeholder="user@ipserver"
              required
            />
          </div>
        </div>

        <div class="form-group">
          <label for="passwordInput">Пароль</label>
          <div class="input-container">
            <span class="input-icon">🔒</span>
            <input
              id="passwordInput"
              type="password"
              v-model="credentials.password"
              placeholder="passwordserver"
              required
            />
          </div>
        </div>

        <button type="submit" class="connect-button" :disabled="connectionState === 'loading'">
          <span class="button-text">Подключиться</span>
          <span class="button-icon" v-if="connectionState !== 'loading'">→</span>
          <span class="loader" v-else></span>
        </button>
      </form>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.ssh-connect {
  display: flex;
  flex-direction: column;
  gap: 32px;
  max-width: 800px;
  margin: 0 auto;
  width: 100%;
}

.connection-status {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 16px 20px;
  border-radius: 12px;
  background-color: #1e293b;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;

  &.none {
    border-left: 4px solid #64748b;

    .status-icon {
      background-color: #64748b;
      box-shadow: 0 0 10px rgba(100, 116, 139, 0.5);
    }
  }

  &.loading {
    border-left: 4px solid #f59e0b;

    .status-icon {
      background-color: #f59e0b;
      box-shadow: 0 0 10px rgba(245, 158, 11, 0.5);
      animation: pulse 1.5s infinite;
    }
  }

  &.success {
    border-left: 4px solid #10b981;

    .status-icon {
      background-color: #10b981;
      box-shadow: 0 0 10px rgba(16, 185, 129, 0.5);
    }
  }

  &.error {
    border-left: 4px solid #ef4444;

    .status-icon {
      background-color: #ef4444;
      box-shadow: 0 0 10px rgba(239, 68, 68, 0.5);
    }
  }

  .status-icon {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    transition: all 0.3s ease;
  }

  .status-message {
    color: #f1f5f9;
    font-size: 16px;
    font-weight: 500;
    margin: 0;
  }
}

.connection-panel {
  background: linear-gradient(145deg, #1e293b, #0f172a);
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);

  .panel-header {
    margin-bottom: 20px;

    h3 {
      color: #f8fafc;
      font-size: 20px;
      font-weight: 600;
      margin: 0;
    }
  }
}

.connection-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 8px;

  label {
    color: #94a3b8;
    font-size: 14px;
    font-weight: 500;
  }

  .input-container {
    position: relative;

    .input-icon {
      position: absolute;
      left: 16px;
      top: 50%;
      transform: translateY(-50%);
      font-size: 16px;
    }

    input {
      width: 100%;
      background-color: #334155;
      color: #f1f5f9;
      border: 1px solid #475569;
      border-radius: 8px;
      padding: 12px 16px 12px 46px;
      font-size: 14px;
      transition: all 0.2s ease;

      &:focus {
        outline: none;
        border-color: #3b82f6;
        box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.3);
      }

      &::placeholder {
        color: #94a3b8;
      }
    }
  }
}

.connect-button {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  background-color: #2563eb;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 14px 20px;
  font-size: 16px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  margin-top: 10px;
  position: relative;
  overflow: hidden;

  &:hover {
    background-color: #1d4ed8;
    transform: translateY(-2px);
  }

  &:active {
    transform: translateY(0);
  }

  &:disabled {
    background-color: #3b82f6;
    cursor: not-allowed;
    transform: none;
  }

  .button-icon {
    font-size: 18px;
    transition: transform 0.3s ease;
  }

  &:hover .button-icon {
    transform: translateX(4px);
  }

  .loader {
    width: 20px;
    height: 20px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-radius: 50%;
    border-top-color: white;
    animation: spin 1s infinite linear;
  }
}

@keyframes pulse {
  0% {
    transform: scale(1);
    opacity: 1;
  }
  50% {
    transform: scale(1.2);
    opacity: 0.7;
  }
  100% {
    transform: scale(1);
    opacity: 1;
  }
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
