<script setup lang="ts">
import { ref, onMounted } from "vue"

const status = ref<string | null>(null)
const busy = ref(false)

const files = ref<string[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

function errorMessage(e: unknown): string {
  if (e instanceof Error) return e.message
  return String(e)
}

async function loadFiles() {
  loading.value = true
  error.value = null
  try {
    const res = await fetch("/list_textfiles", { 
    method: "GET",
    })
    const data = await res.json()
    files.value = data.files
  } catch (e) {
    error.value = errorMessage(e)
  } finally {
    loading.value = false
  }
}

function deleteTextfile(filename: string) {
    if (confirm("Are you sure you want to delete this file?")) {
        fetch("/delete_textfile", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ filename: filename }),
        }).then((_res) => {
          loadFiles()
        });
    }
}

async function onSubmit(e: Event) {
  const form = e.target as HTMLFormElement
  busy.value = true
  status.value = null
  try {
    const res = await fetch(form.action || window.location.pathname, {
      method: "POST",
      body: new FormData(form),
      credentials: "same-origin",
    })
    const data = await res.json()
    status.value = data.success ? "Uploaded." : "Upload failed."
    if (data.success) {
      form.reset()
      loadFiles()
    }
  } catch (err) {
    status.value = "Network error."
  } finally {
    busy.value = false
  }
}

onMounted(loadFiles)
</script>

<template>
<div>

  <p v-if="loading">Loading…</p>
  <p v-else-if="error">Couldn't load files: {{ error }}</p>
  <div v-else class="dev_div" style="display: inline">
    <div v-for="f in files" :key="f" class="dev_div" style="display: inline">
      <div class="dev_div" style="flex: 1">
          <div class="pan_btn">
            {{ f }}
          </div>
      </div>
      <div class="dev_div" style="flex: 1">
        <a :href="`/download_textfile/${encodeURIComponent(f)}`" download class="pan_btn">
          Download
        </a>
        <button @click="deleteTextfile(f)" class="pan_btn">
          Delete
        </button>
      </div>
    </div>
  </div>
  <div class="dev_div">
    <button @click="loadFiles" class="pan_btn" :disabled="loading">Refresh</button>
  </div>

  <br>

  <form action="/upload_textfile" method="POST" enctype="multipart/form-data" @submit.prevent="onSubmit">
    <div class="dev_div">
      <label for="file" class="pan_btn">Choose a textfile to upload.</label>
    </div>
    <div class="dev_div">
      <input id="file" class="pan_btn" type="file" name="file" />
    </div>
    <div class="dev_div">
      <button class="pan_btn" type="submit" :disabled="busy">Upload</button>
    </div>
    <p v-if="status">{{ status }}</p>
  </form>

</div>
</template>

<style scoped>
p {
  font-size: var(--fs);
  margin: 12px;
}
</style>
