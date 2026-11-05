<script setup lang="ts">
async function testFetch() {
  let response = await fetch("/test", {
    method: "POST",
    body: JSON.stringify({ body: "SomeBody" }),
    headers: { 
      "Content-Type": "application/json",
    },
  });

  console.log(response);
}
</script>

<template>
  <div class="dev_div">
    BaseDiv
  </div>
  <div class="dev_div">
    <button class="pan_btn"
            @click='testFetch()'
            >
            TestBtn
    </button>
  </div>
</template>

<style scoped>
</style>
