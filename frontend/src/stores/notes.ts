import { ref, computed } from "vue"
import { defineStore } from "pinia"

export interface Note {
  note: string,
  salt: string | null,
  coun: string | null,
}

function hexToBytes(hex: string): Uint8Array {
    const hexValues = hex.trim().split(/\s+/);
    const bytes = new Uint8Array(hexValues.map(h => parseInt(h, 16)));
    return bytes;
}

export const useNotesStore = defineStore("notes", () => {
  const notes = ref<Note[]>([])

  function add_note(note: string, salt: string | null, counter: string | null) {
    console.log("Salt (add_note)")
    console.log(salt)
    if (salt !== null) {
    let salt_bytes = hexToBytes(salt)
    console.log(salt_bytes)
    }
    console.log("Count (add_note)")
    console.log(counter)
    let big_note = {
      note: note,
      salt: salt,
      coun: counter,
    }
    notes.value.push(big_note)
  }

  function remove_note(index: number) {
    notes.value.splice(index, 1)
  }

  return { notes, add_note, remove_note }
})
