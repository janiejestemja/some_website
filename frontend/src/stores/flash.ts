import { defineStore } from "pinia"
import { ref } from "vue"

export type FlashType = "success" | "error" | "warning" | "info"

export interface FlashMessage {
  id: number
  text: string
  type: FlashType
}

export const useFlashStore = defineStore("flash", () => {
  const messages = ref<FlashMessage[]>([])
  let nextId = 0

  function push(text: string, type: FlashType = "info"): number {
  // function push(text: string, type: FlashType = "info", timeout = 4000): number {
    const id = nextId++
    messages.value.push({ id, text, type })
    // if (timeout) setTimeout(() => remove(id), timeout)
    return id
  }

  function remove(id: number): void {
    messages.value = messages.value.filter(m => m.id !== id)
  }

  return { messages, push, remove }
})
