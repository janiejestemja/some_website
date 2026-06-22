<script setup lang="ts">
import { ref, onMounted } from "vue";
import { RouterView, Transition } from "vue-router";

const show_menu_pan = ref(false)
const show_ctrl = ref(false)

import init, { BoardWrapper } from "../../../rust-src/pkg/rust_src.js"

// canvas section for board in background
const canvas = ref(null)
const can_hei = 387
const can_wid = 387
const scale = 5
var paused = ref(false)
let board = ref(null)

// board control
function reset_board() {
  board.value.reset_board();
  board.value.set_diagonals();
  paused.value = false;
}

function clean_board() {
  board.value.reset_board();
  paused.value = false;
}

function set_cross() {
  board.value.set_cross();
  paused.value = false;
}

function set_diagonals() {
  board.value.set_diagonals();
  paused.value = false;
}

onMounted(async () => {
  // await board init
  await init()
  board = ref(new BoardWrapper(387n, 387n, 11n))
  board.value.set_diagonals()

  // setup canvas
  const ctx = canvas.value.getContext("2d")
  canvas.value.height = scale * can_hei
  canvas.value.width = scale * can_wid 

  function frame() {
    if (!paused.value) {
      // clear board and evaluate for next generation
      ctx.clearRect(0, 0, scale * can_wid, scale * can_hei)
      board.value.next_generation()

      // loop through cell states and fill rectangles
      for (let y = 0; y < can_hei; y++) {
        for (let x = 0; x < can_wid; x++) {
          let cell_state = board.value.index_into_board(x, y)
          // still alive
          if (cell_state == 2) {
            ctx.fillStyle = "aqua"
            ctx.fillRect(
              x * scale,
              y * scale,
              scale,
              scale
            )
          }
          // alive
          else if (cell_state == 1) {
            ctx.fillStyle = "chartreuse"
            ctx.fillRect(
              x * scale,
              y * scale,
              scale,
              scale
            )
          }
        }
      }
    }
    requestAnimationFrame(frame)
  }
  await frame()
})
</script>

<template>
  <canvas ref="canvas"></canvas>

  <div class="nav_container">
    <RouterLink to="/" class="menu_btn" @click='show_menu_pan = !show_menu_pan; show_ctrl = false'>
      Menu
    </RouterLink>
    <div class="menu_pan" :class="{ show: show_menu_pan }">
      <RouterLink to="/notes" @click="show_ctrl = false" class="pan_btn">
        Notes
      </RouterLink>
      <RouterLink to="/info" @click="show_ctrl = false" class="pan_btn">
        Info
      </RouterLink>
      <button class="menu_btn" :class="{ show: true }" @click="show_ctrl = !show_ctrl">
        Ctrl
      </button>
    </div>
  </div>

  <div class="ctrl_container" :class="{ show: show_ctrl }">
    <div class="menu_pan" :class="{ show: true}">
      <button class="pan_btn" @click="set_cross()">
        +
      </button>
      <button class="pan_btn" @click="set_diagonals()">
        x
      </button>
    </div>
    <div class="menu_pan" :class="{ show: true }">
      <button class="pan_btn" @click="paused = !paused">
        Paused: {{ paused? "True": "False" }}
      </button>
      <button class="pan_btn" @click="clean_board()">
        Clean
      </button>
    </div>
    <div class="menu_pan" :class="{ show: true }">
      <button class="pan_btn" @click="reset_board()">
        Restart
      </button>
    </div>
  </div>

  <div class="ctrl_container" :class="{ show: !show_ctrl }">
    <RouterView v-slot="{ Component }">
      <Transition name="page" mode="out-in">
        <component :is="Component" />
      </Transition>
    </RouterView>
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
  font-size: var(--fs);
  width: 64px;

  margin: 8px 8px 8px 8px;
  padding: 8px 0px 8px 0px;

  color: var(--text);
  text-decoration: underline;
  text-align: center;
  opacity: 0.8;

  border: 1px solid var(--text);
  border-radius: 10px;

  &:hover {
    font-weight: bold;
  }
}

.menu_pan {
  display: flex;
  flex: 1;

  opacity: 0.8;
  transform-origin: left;
  transform: scaleX(0);
  transition: transform 1s ease;
}
.menu_pan.show {
  transform: scaleX(1);
}

canvas {
  position: fixed;
  inset: 0;
  z-index: -1;
  opacity: 0.6;
  width: 100vmax;
  height: 100vmax;
}

@media (prefers-color-scheme: dark) {
  canvas {
    opacity: 0.25;
  }
}
</style>
