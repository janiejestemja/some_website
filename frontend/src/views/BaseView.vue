<script setup lang="ts">
import { ref } from "vue";
import { RouterView } from "vue-router";

const show_menu_pan = ref(false)
const show_ctrl = ref(false)
</script>

<template>
  <div class="nav_container">
    <button class="menu_btn" @click='show_menu_pan = !show_menu_pan; show_ctrl = false'>
      Menu
    </button>
    <div class="menu_pan" :class="{ show: show_menu_pan }">
      <div class="menu_pan_btn">
        <RouterLink to="/" @click="show_ctrl = false">
          Home
        </RouterLink>
      </div>
      <div class="menu_pan_btn">
        <RouterLink to="/info" @click="show_ctrl = false">
          Info
        </RouterLink>
      </div>
      <button class="menu_btn" :class="{ show: true }" @click="show_ctrl = !show_ctrl">
        Ctrl
      </button>
    </div>
  </div>

  <div class="ctrl_container" :class="{ show: show_ctrl }">
    <div class="menu_pan" :class="{ show: true}">
      <button class="menu_pan_btn">
        +
      </button>
      <button class="menu_pan_btn">
        x
      </button>
    </div>
    <div class="menu_pan" :class="{ show: true}">
      <button class="menu_pan_btn">
        Pause
      </button>
      <button class="menu_pan_btn">
        Reset
      </button>
    </div>
    <div class="menu_pan" :class="{ show: true}">
      <button class="menu_pan_btn">
        Restart
      </button>
    </div>
  </div>

  <div class="page_container" :class="{ show: !show_ctrl }">
    <RouterView />
  </div>
</template>

<style scoped>
.ctrl_container {
  position: fixed;

  top: 65px;
  left: 0px;
  right: 0px;
  bottom: 1px;

  border: 1px solid var(--text);

  transform-origin: top;
  transform: scaleY(0);
  transition: transform 1s ease;
}
.ctrl_container.show {
  transform: scaleY(1);
}
.page_container {
  position: fixed;

  top: 65px;
  left: 0px;
  right: 0px;
  bottom: 1px;

  border: 1px solid var(--text);

  transform-origin: top;
  transform: scaleY(0);
  transition: transform 1s ease;
}
.page_container.show {
  transform: scaleY(1);
}

.nav_container {
  position: fixed;

  top: 0px;
  left: 0px;
  right: 0px;
  height: 64px;

  display: flex;
  flex: 1;

  border: 1px solid var(--text);
  border-radius: 10px;
}

.menu_btn {
  font-size: 18px;
  width: 64px;
  margin: 8px 8px 8px 8px;

  text-decoration: none;
  text-align: center;

  border: 1px solid var(--text);
  border-radius: 10px;
}

.menu_pan {
  display: flex;
  flex: 1;

  transform-origin: left;
  transform: scaleX(0);
  transition: transform 1s ease;
}
.menu_pan.show {
  transform: scaleX(1);
}

.menu_pan_btn {
  font-size: 18px;
  flex: 1;

  margin: 8px 8px 8px 8px;
  padding: 8px 8px 8px 8px;

  text-decoration: none;
  text-align: center;

  border: 1px solid var(--text);
  border-radius: 10px;
}
</style>
