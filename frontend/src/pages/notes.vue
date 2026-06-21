<script setup lang="ts">
import { ref } from "vue"
import { useNotesStore } from "@/stores/notes.ts"

const text_input = ref("...")
const notes = useNotesStore()
</script>

<template>
  <div class="dev_div">
    NotesDiv
  </div>

  <div class="dev_div" style="display: inline">
    <div v-for="(note, index) in notes.notes">
      <div class="dev_div">
        <!--
        <div class="dev_div" style="flex: 1">
          <p>{{ index }}</p>
        </div>
        -->
        <div class="dev_div" style="flex: 8">
          <p>{{ note }}</p>
        </div>
        <button 
          class="dev_div"
          style="flex: 1"
          @click='notes.remove_note(index)'
          >
          <p>Delete</p>
        </button>
      </div>
    </div>
  </div>

  <div class="dev_div">
    <textarea v-model="text_input" 
    class="pan_btn" 
    >
    </textarea>
  </div>
  <div class="dev_div">
    <button
      class="pan_btn"
      @click='notes.add_note(text_input)'
      >
      Submit
    </button>
  </div>

</template>

<style scoped>
p {
  font-size: var(--fs);
  margin: 12px;
}
</style>
