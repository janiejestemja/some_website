import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export const useCounterStore = defineStore('counter', () => {
  const count = ref(0)
  function increment() {
    count.value++
  }
  function decrement() {
    count.value -= 1
  }

  return { count, decrement, increment }
})
