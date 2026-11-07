<script setup lang="ts">
</script>

<template>
  <div>
    <div class="pan_btn">
    <p>The game of flowsers has only four rules:</p>
    <p>- if a flower has less than two neighbours it is lonely, it dies,</p>
    <p>- if a flower has more than three neighbours it is crowded, it dies,</p>
    <p>- if a flower has two or three neighbours it continues living,</p>
    <p>- if a dead flower has exactly three neighbours it comes back to life</p>
    <p>The game once started doesn't really end - although it might end up in a stable configuration - and the only player action is the initial setup, nothing more.</p>
    </div>
  </div>
</template>

<style scoped>
p {
  font-size: var(--fs);
  margin: 12px;
}
</style>
