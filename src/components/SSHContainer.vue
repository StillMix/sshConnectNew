<script setup lang="ts">
import { ref, reactive } from 'vue'
import SSHCard from './SSHCard.vue'

interface SSHCredential {
  id: number
  title: string
  user: string
  password: string
}

const showForm = ref(false)
const credentials = reactive<SSHCredential[]>([
  { id: 1, title: 'Сервер разработки', user: 'dev@192.168.1.10', password: 'dev2024!' },
  { id: 2, title: 'Prod сервер', user: 'admin@10.0.15.25', password: 'Pr0d$ecure' },
])

const newCredential = reactive({
  title: '',
  user: '',
  password: '',
})

const emit = defineEmits(['server-select', 'connecting'])

const toggleForm = () => {
  showForm.value = !showForm.value
}

const addCredential = () => {
  if (newCredential.title && newCredential.user && newCredential.password) {
    credentials.push({
      id: Date.now(),
      title: newCredential.title,
      user: newCredential.user,
      password: newCredential.password,
    })

    newCredential.title = ''
    newCredential.user = ''
    newCredential.password = ''

    showForm.value = false
  }
}

const selectCredential = (credential: SSHCredential) => {
  // Сообщаем родителю, что начали подключение
  emit('connecting')

  // Имитация подключения
  setTimeout(() => {
    // Передаем данные наверх через событие
    emit('server-select', credential)
  }, 1000)
}
</script>

<template>
  <div class="ssh-container">
    <div class="container-header">
      <h2>Ваши адреса</h2>
      <button class="add-button" @click="toggleForm">
        <span class="button-icon">+</span>
        <span>Добавить</span>
      </button>
    </div>

    <transition name="slide-fade">
      <form v-if="showForm" class="add-form" @submit.prevent="addCredential">
        <div class="form-group">
          <input type="text" v-model="newCredential.title" placeholder="Название" required />
        </div>
        <div class="form-group">
          <input type="text" v-model="newCredential.user" placeholder="user@ipserver" required />
        </div>
        <div class="form-group">
          <input
            type="password"
            v-model="newCredential.password"
            placeholder="passwordserver"
            required
          />
        </div>
        <div class="form-actions">
          <button type="button" class="cancel-button" @click="toggleForm">Отмена</button>
          <button type="submit" class="submit-button">Добавить</button>
        </div>
      </form>
    </transition>

    <transition-group name="list" tag="div" class="cards">
      <SSHCard
        v-for="credential in credentials"
        :key="credential.id"
        :title="credential.title"
        :user="credential.user"
        :password="credential.password"
        @select="selectCredential(credential)"
      />
    </transition-group>
  </div>
</template>

<style lang="scss" scoped>
.ssh-container {
  background-color: #0f172a;
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
  max-width: 600px;
  width: 100%;
  margin: 0 auto;
}

.container-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;

  h2 {
    color: #f8fafc;
    font-size: 24px;
    font-weight: 600;
    margin: 0;
  }
}

.add-button {
  display: flex;
  align-items: center;
  gap: 8px;
  background-color: #2563eb;
  color: white;
  border: none;
  border-radius: 8px;
  padding: 10px 16px;
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
    font-size: 18px;
    font-weight: bold;
  }
}

.add-form {
  background-color: #1e293b;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 24px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.form-group {
  margin-bottom: 16px;

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
      color: #94a3b8;
    }
  }
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;

  button {
    padding: 10px 16px;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .cancel-button {
    background-color: transparent;
    color: #94a3b8;
    border: 1px solid #475569;

    &:hover {
      background-color: #1e293b;
      color: #f1f5f9;
    }
  }

  .submit-button {
    background-color: #2563eb;
    color: white;
    border: none;

    &:hover {
      background-color: #1d4ed8;
    }
  }
}

.cards {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

// Анимации
.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: all 0.3s ease;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
  transform: translateY(-20px);
  opacity: 0;
}

.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}

.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateX(30px);
}

.list-move {
  transition: transform 0.5s ease;
}
</style>
