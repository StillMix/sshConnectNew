<script setup lang="ts"></script>

<template>
  <div class="sshConnectings">
    <div>+</div>
    <div>+</div>
    <div>+</div>
    <div>+</div>
  </div>
</template>

<style lang="scss" scoped>
.sshConnectings {
  position: absolute;
  width: 100%;
  height: 100%;
  display: flex;
  flex-wrap: wrap;
  div {
    width: 100%;
    height: 100%;
    border: 1px solid gray;
    display: flex;
    align-items: center;
    justify-content: center;
  }
}
</style>
