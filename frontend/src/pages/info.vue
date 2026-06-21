<script setup lang="ts">
import { useCounterStore } from "@/stores/counter.ts"
const store = useCounterStore()
</script>

<template>
  <div>
  <div class="dev_div">
    InfoDiv
  </div>
  
  <div class="dev_div">
    <p>
      {{ store.count }}
    </p>
  </div>
  <div class="dev_div">
    <button 
      class="pan_btn"
      @click="store.decrement()">
      decrement 
    </button>

    <button 
      class="pan_btn"
      @click="store.increment()">
      increment
    </button>
  </div>
  </div>
</template>

<style scoped>
p {
  font-size: var(--fs);
  margin: 12px;
}
</style>
