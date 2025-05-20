<script setup lang="ts">
import SSHCard from './SSHCard.vue'
</script>

<template>
  <div class="sshContainer">
    <p>Ваши адреса</p>
    <button>Добавить</button>
    <form>
      <input type="text" placeholder="Название" />
      <input type="text" placeholder="user@ipserver" />
      <input type="text" placeholder="passwordserver" />
      <button>Добавить</button>
    </form>
    <div class="cards">
      <SSHCard />
    </div>
  </div>
</template>

<style lang="scss"></style>
