<script setup lang="ts">
import { ref } from "vue";

const user_name = ref("");
const email = ref("");
const password = ref("");
const passwordA = ref("");
const passwordB = ref("");

async function login() {
  const request = await fetch("/login", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ email: email.value, password: password.value }),
  });
  const response = await request.json();
  console.log(response);
}

async function signup() {
  const request = await fetch("/signup", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ user_name: user_name.value, email: email.value, passwordA: passwordA.value, passwordB: passwordB.value }),
  });
  const response = await request.json();
  console.log(response);
}

async function logout() {
  const request = await fetch("/logout", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ user_name: user_name.value }),
  });
  try {
    const response = await request.json();
    console.log(response);
  } catch(error) {
    console.log("Current user not logged in.");
  }
}

</script>

<template>
  <div>

  <div class="dev_div">
    <h3>Logout</h3>
  </div>
  <div class="dev_div">
    <button @click="logout()" class="dev_div" style="flex: 1">Logout</button>
  </div>

  <div class="dev_div">
    <h3>Login</h3>
  </div>

  <form @submit.prevent="login" class="dev_div; display: inline">
    <div class="dev_div" style="flex: 1">
      <input v-model="email" class="dev_div" style="flex: 1" type="email" name="email" placeholder="email" required><br>
    </div>
    <div class="dev_div" style="flex: 1">
      <input v-model="password" class="dev_div" style="flex: 1" type="password" name="passwordA" placeholder="Password" required><br>
    </div>
    <div class="dev_div" style="flex: 1">
      <button type="submit" class="dev_div" style="flex: 1">Submit</button>
    </div>
  </form>

  <div class="dev_div">
    <h3>Signup</h3>
  </div>
  <form @submit.prevent="signup" class="dev_div; display: inline">
    <div class="dev_div" style="flex: 1">
      <input v-model="user_name" class="dev_div" style="flex: 1" type="text" name="user_name" placeholder="Username" required><br>
    </div>
    <div class="dev_div" style="flex: 1">
      <input v-model="email" class="dev_div" style="flex: 1" type="email" name="email" placeholder="email" required><br>
    </div>
    <div class="dev_div" style="flex: 1">
      <input v-model="passwordA" class="dev_div" style="flex: 1" type="password" name="passwordA" placeholder="Password" required><br>
    </div>
    <div class="dev_div" style="flex: 1">
      <input v-model="passwordB" class="dev_div" style="flex: 1" type="password" name="passwordB" placeholder="Confirm password" required><br>
    </div>
    <div class="dev_div" style="flex: 1">
      <button type="submit" class="dev_div" style="flex: 1">Submit</button>
    </div>
  </form>

  </div>
</template>

<style scoped>
p {
  font-size: var(--fs);
  margin: 12px;
}
</style>
