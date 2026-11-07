<script setup lang="ts">
import { ref } from "vue"
import { useNotesStore } from "@/stores/notes.ts"

const note_passwd = ref("password")
const text_input = ref("...")
const notes = useNotesStore()

function bytesToHex(bytes: Uint8Array): string {
    return Array.from(bytes)
        .map(byte => byte.toString(16).padStart(2, "0"))
        .join(" ")
        .toUpperCase();
}
function hexToBytes(hex: string): Uint8Array {
    const hexValues = hex.toString().trim().split(/\s+/);
    const bytes = new Uint8Array(hexValues.map(h => parseInt(h, 16)));
    return bytes;
}

async function some_other_function(hex_note: {
  note: string,
  salt: string | null,
  coun: string | null,
}, password: string) {
  // import key 
  const salt = hexToBytes(hex_note?.salt ?? "")
  const counter = hexToBytes(hex_note?.coun ?? "")
  console.log("Salt (decrypt)")
  console.log(salt)
  console.log("Counter (decrypt)")
  console.log(counter)

  const encoded = new TextEncoder().encode(password)
  const pass_key = await window.crypto.subtle.importKey(
    "raw",
    encoded,
    "PBKDF2",
    false,
    ["deriveKey"]
  )
  const pbkdf2_params = {
    name: "PBKDF2",
    salt,
    iterations: 100000,
    hash: "SHA-256",
  }
  const aes_ctr_keygen_params = {
    name: "AES-CTR",
    length: 256,
  }
  const key = await window.crypto.subtle.deriveKey(
    pbkdf2_params,
    pass_key,
    aes_ctr_keygen_params,
    true,
    ["encrypt", "decrypt"],
  )
  const aes_ctr_crypto_params = {
    name: "AES-CTR",
    counter,
    length: 64,
  }
  const dehexed_cipher = hexToBytes(hex_note?.note ?? "")
  const decrypted = await window.crypto.subtle.decrypt(
    aes_ctr_crypto_params,
    key,
    dehexed_cipher as unknown as ArrayBuffer,
  )

  const decoded = new TextDecoder().decode(decrypted)
  console.log(decoded)
  notes.add_note(decoded, null, null)
}

async function some_function(note: { note: string }, password: string) {
  // setup aes ctr params
  const aes_ctr_keygen_params = {
    name: "AES-CTR",
    length: 256,
  }

  const counter = window.crypto.getRandomValues(new Uint8Array(16))
  const aes_ctr_crypto_params = {
    name: "AES-CTR",
    counter,
    length: 64,
  }

  // encode password and generate a hash of it
  const encoded = new TextEncoder().encode(password)
  /*
  const pass_hash = await window.crypto.subtle.digest(
    "SHA-256", encoded
  )
  console.log(pass_hash)
  */

  // import key 
  const pass_key = await window.crypto.subtle.importKey(
    "raw",
    encoded,
    "PBKDF2",
    false,
    ["deriveKey"]
  )

  // generate salt, setup and derive pbkdf key
  const salt = window.crypto.getRandomValues(new Uint8Array(16))
  const pbkdf2_params = {
    name: "PBKDF2",
    salt,
    iterations: 100000,
    hash: "SHA-256",
  }
  const key = await window.crypto.subtle.deriveKey(
    pbkdf2_params,
    pass_key,
    aes_ctr_keygen_params,
    true,
    ["encrypt", "decrypt"],
  )

  /*
  // generate random key
  const key = await window.crypto.subtle.generateKey(
    aes_ctr_keygen_params,
    true,
    ["encrypt", "decrypt"]
  )
  */

  // encrypt note and encode it
  const encoded_note = new TextEncoder().encode(note.note)
  const cipher = new Uint8Array(await window.crypto.subtle.encrypt(
    aes_ctr_crypto_params,
    key,
    encoded_note,
  ))
  const ciphered_hex = bytesToHex(cipher)
  notes.add_note(ciphered_hex, bytesToHex(salt), bytesToHex(counter))
}
</script>

<template>
  <div>
  <div class="pan_btn">
    <p>Client side cryptography demo</p>
  </div>
  <div class="dev_div">
    <input v-model="note_passwd" 
    class="pan_btn"
    >
    </input>
  </div>

  <div class="dev_div" style="display: inline">
    <div v-for="(note, index) in notes.notes">
      <div class="dev_div" style="display: inline">
        <!--
        <div class="dev_div" style="flex: 1">
          <p>{{ index }}</p>
        </div>
        -->
        <div class="dev_div" style="flex: 8">
          <p>{{ note.note }}</p>
          <p>{{ note.salt }}</p>
          <p>{{ note.coun }}</p>
        </div>
        <div class="dev_div">
          <button 
            class="dev_div"
            style="flex: 1"
            @click='some_function(note, note_passwd)'
            >
            <p>Encrypt</p>
          </button>
          <button 
            class="dev_div"
            style="flex: 1"
            @click='some_other_function(note, note_passwd)'
            >
            <p>Decrypt</p>
          </button>
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
      @click='notes.add_note(text_input, null, null)'
      >
      Submit
    </button>
  </div>

  </div>
</template>

<style scoped>
p {
  font-size: var(--fs);
  margin: 12px;
}
textarea {
  opacity: 0.6;
  color: var(--text);
  background-color: var(--bg);
}
</style>
