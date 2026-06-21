import { ref, computed } from "vue"
import { defineStore } from "pinia"

export const useNotesStore = defineStore("notes", () => {
  const notes = ref([])

  function add_note(note) {
    notes.value.push(note)
  }

  function remove_note(index) {
    notes.value.splice(index, 1)
  }

  return { notes, add_note, remove_note }
})
