<script setup lang="ts">
import { ref } from "vue"
import { useNotesStore } from "@/stores/notes.ts"

const ref_counter = ref([])
const ref_salt = ref([])
const note_passwd = ref("password")
const text_input = ref("...")
const notes = useNotesStore()

/* Conversions */
function stringToHex(str: string): string {
    return Array.from(new TextEncoder().encode(str))
        .map(byte => byte.toString(16).padStart(2, "0"))
        .join(" ")
        .toUpperCase();
}

function bytesToHex(bytes: Uint8Array): string {
    return Array.from(bytes)
        .map(byte => byte.toString(16).padStart(2, "0"))
        .join(" ")
        .toUpperCase();
}

function hexToString(hex: string): string {
    const hexValues = hex.trim().split(/\s+/);
    const bytes = new Uint8Array(hexValues.map(h => parseInt(h, 16)));
    return new TextDecoder().decode(bytes);
}

function hexToBytes(hex: string): Uint8Array {
    const hexValues = hex.trim().split(/\s+/);
    const bytes = new Uint8Array(hexValues.map(h => parseInt(h, 16)));
    return bytes;
}

async function some_other_function(hex_note, password) {
  // import key 
  const salt = ref_salt.value
  const counter = ref_counter.value

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
  const dehexed_cipher = hexToBytes(hex_note)
  const decrypted = await window.crypto.subtle.decrypt(
    aes_ctr_crypto_params,
    key,
    dehexed_cipher,
  )

  const decoded = new TextDecoder().decode(decrypted)
  console.log(decoded)
  notes.add_note(decoded)
}

async function some_function(note, password) {
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
  const encoded_note = new TextEncoder().encode(note)
  const cipher = new Uint8Array(await window.crypto.subtle.encrypt(
    aes_ctr_crypto_params,
    key,
    encoded_note,
  ))
  const ciphered_hex = bytesToHex(cipher)
  const dehexed_cipher = hexToBytes(ciphered_hex)
  const decrypted = await window.crypto.subtle.decrypt(
    aes_ctr_crypto_params,
    key,
    dehexed_cipher,
  )

  const decoded = new TextDecoder().decode(decrypted)
  
  console.log("Counter...")
  console.log(counter)
  console.log("Salt...")
  console.log(salt)
  console.log("Original: " + note)
  console.log("Encrypted and hexed: " + ciphered_hex)
  console.log("Decrypted: " + decoded)

  ref_counter.value = counter
  ref_salt.value = salt

  notes.add_note(ciphered_hex)
}
</script>

<template>
  <div>
  <div class="dev_div">
    NotesDiv
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
          <p>{{ note }}</p>
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
      @click='notes.add_note(text_input)'
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
