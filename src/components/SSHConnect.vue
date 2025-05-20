<script setup lang="ts">
import SSHContainer from './SSHContainer.vue'
</script>

<template>
  <div class="sshConnect">
    <p>
      Успешно подключились или Неудачное подключение(будет меняться в зависимости от подключения)
    </p>
    <SSHContainer />
    <form>
      <input type="text" placeholder="user@ipserver" />
      <input type="text" placeholder="passwordserver" />
      <button>Подключится</button>
    </form>
  </div>
</template>

<style lang="scss"></style>
