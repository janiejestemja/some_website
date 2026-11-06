import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export const useUserSession = defineStore('session', () => {
  const logged_in = ref(false);
  function log_in() {
    logged_in.value = true;
  }
  function log_out() {
    logged_in.value = false;
  }

  return { logged_in, log_in, log_out }
})
