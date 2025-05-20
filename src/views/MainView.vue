<script setup lang="ts">
import SshConnect from '@/components/SSHConnect.vue'
import { onMounted } from 'vue'

// Анимация при загрузке страницы
onMounted(() => {
  const main = document.querySelector('.main')
  if (main) {
    main.classList.add('loaded')
  }
})
</script>

<template>
  <div class="main">
    <div class="app-header">
      <div class="logo">
        <span class="logo-icon">⚡</span>
        <h1>SSH Connect</h1>
      </div>
      <div class="header-actions">
        <button class="theme-toggle">🌙</button>
      </div>
    </div>

    <div class="content-wrapper">
      <SshConnect />
    </div>

    <div class="app-footer">
      <p>© 2025 SSH Connect. Все права защищены.</p>
    </div>
  </div>
</template>

<style lang="scss">
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
  font-family: 'Roboto', 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

body {
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
  color: #f8fafc;
  min-height: 100vh;
}

#app {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.main {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  padding: 0 20px;
  opacity: 0;
  transform: translateY(20px);
  transition:
    opacity 0.8s ease,
    transform 0.8s ease;

  &.loaded {
    opacity: 1;
    transform: translateY(0);
  }
}

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 0;
  margin-bottom: 20px;
  border-bottom: 1px solid rgba(148, 163, 184, 0.2);

  .logo {
    display: flex;
    align-items: center;
    gap: 12px;

    .logo-icon {
      font-size: 24px;
      background-color: #2563eb;
      color: white;
      width: 40px;
      height: 40px;
      display: flex;
      align-items: center;
      justify-content: center;
      border-radius: 10px;
      box-shadow: 0 0 15px rgba(37, 99, 235, 0.5);
    }

    h1 {
      font-size: 24px;
      font-weight: 700;
      background: linear-gradient(to right, #60a5fa, #3b82f6);
      background-clip: text;
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      margin: 0;
    }
  }

  .header-actions {
    .theme-toggle {
      background-color: #334155;
      border: none;
      border-radius: 8px;
      width: 40px;
      height: 40px;
      font-size: 18px;
      cursor: pointer;
      transition: all 0.2s ease;

      &:hover {
        background-color: #475569;
        transform: translateY(-2px);
      }
    }
  }
}

.content-wrapper {
  flex: 1;
  padding: 20px 0;
  width: 100%;
  max-width: 900px;
  margin: 0 auto;
}

.app-footer {
  padding: 20px 0;
  text-align: center;
  margin-top: 40px;
  border-top: 1px solid rgba(148, 163, 184, 0.2);

  p {
    color: #94a3b8;
    font-size: 14px;
  }
}

@media (max-width: 768px) {
  .app-header {
    flex-direction: column;
    gap: 16px;
    align-items: flex-start;

    .header-actions {
      align-self: flex-end;
    }
  }
}

@keyframes fadeUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.ssh-connect,
.ssh-container {
  animation: fadeUp 0.8s ease forwards;
}

.ssh-container {
  animation-delay: 0.2s;
}
</style>
