<script setup lang="ts">
import { ref, onMounted } from "vue";
import { RouterView } from "vue-router";

import { useUserSession } from "@/stores/session.ts"
const session = useUserSession()

import { useLogout } from "@/composables/useLogout.ts"

const show_menu_pan = ref(false)
const show_ctrl = ref(false)

import { useFlashStore } from "@/stores/flash";
const flashMsgQ = useFlashStore();
flashMsgQ.push("Fancy Msg!");
flashMsgQ.push("Fancy Msg!", "success");
flashMsgQ.push("Fancy Msg!", "error");
flashMsgQ.push("Fancy Msg!", "warning");

import init, { BoardWrapper } from "../../../rust-src/pkg/rust_src.js"

// canvas section for board in background
const canvas = ref<HTMLCanvasElement | null>(null)
const can_hei = 387
const can_wid = 387
const scale = 5
var paused = ref(false)
let board = ref<BoardWrapper | null>(null)

// board control
function reset_board() {
  board.value?.reset_board();
  board.value?.set_diagonals();
  paused.value = false;
}

function clean_board() {
  board.value?.reset_board();
  paused.value = false;
}

function set_cross() {
  board.value?.set_cross();
  paused.value = false;
}

function set_diagonals() {
  board.value?.set_diagonals();
  paused.value = false;
}

onMounted(async () => {
  // await board init
  await init()
  board.value = new BoardWrapper(387n, 387n, 11n)
  board.value.set_diagonals()

  // setup canvas
  if (!canvas.value) return
  const ctx = canvas.value.getContext("2d")
  if (!ctx) return
  canvas.value.height = scale * can_hei
  canvas.value.width = scale * can_wid 

  function frame() {
    if (!paused.value && ctx && board.value) {
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
      <RouterLink to="/demo" @click="show_ctrl = false" class="pan_btn">
        Demo
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

    <h3 class="pan_btn">Control</h3>
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
    <h3 class="pan_btn">Navigation</h3>
    <div class="menu_pan" :class="{ show: show_menu_pan }">
      <RouterLink to="/login" @click="show_ctrl = false" class="pan_btn">
        Login
      </RouterLink>
    </div>
    <div class="menu_pan" :class="{ show: session.logged_in }">
      <button @click="useLogout(session); flashMsgQ.push('Logout successful.', 'success');" class="pan_btn" style="flex: 1">Logout</button>
    </div>
    <div class="menu_pan" :class="{ show: session.logged_in }">
      <RouterLink to="/texts" @click="show_ctrl = false" class="pan_btn">
        Textfiles
      </RouterLink>
    </div>
  </div>

  <div class="flash-stack" aria-live="polite" aria-atomic="false">
    <TransitionGroup name="flash">
      <div
        v-for="msg in flashMsgQ.messages"
        :key="msg.id"
        class="flash"
        :class="`flash--${msg.type}`"
        role="status"
      >
        <span class="flash__icon" aria-hidden="true">
          <template v-if="msg.type === 'success'">✓</template>
          <template v-else-if="msg.type === 'error'">✕</template>
          <template v-else-if="msg.type === 'warning'">!</template>
          <template v-else>i</template>
        </span>
        <p class="flash__text">{{ msg.text }}</p>
        <button
          class="flash__close"
          @click="flashMsgQ.remove(msg.id)"
          aria-label="Dismiss"
        >×</button>
      </div>
    </TransitionGroup>
  </div>

  <div class="page_container" :class="{ show: !show_ctrl }">
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

  overflow: scroll;
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

.flash-stack {
  position: fixed;
  top: 64px;
  right: 1rem;
  z-index: 9999;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  max-width: min(24rem, calc(100vw - 2rem));
  pointer-events: none; /* so empty stack doesn't block clicks */
}

.flash {
  top: 65px;
  pointer-events: auto;
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  border-radius: 0.5rem;
  border: 1px solid;
  background: var(--flash-bg);
  color: var(--flash-fg);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  font-size: 0.9rem;
  line-height: 1.4;
}

/* Per-type color tokens */
.flash--success { --flash-bg: #ecfdf5; --flash-fg: #065f46; --flash-border: #a7f3d0; border-color: var(--flash-border); }
.flash--error   { --flash-bg: #fef2f2; --flash-fg: #991b1b; --flash-border: #fecaca; border-color: var(--flash-border); }
.flash--warning { --flash-bg: #fffbeb; --flash-fg: #92400e; --flash-border: #fde68a; border-color: var(--flash-border); }
.flash--info    { --flash-bg: #eff6ff; --flash-fg: #1e40af; --flash-border: #bfdbfe; border-color: var(--flash-border); }

.flash__icon {
  flex-shrink: 0;
  width: 1.25rem;
  height: 1.25rem;
  border-radius: 50%;
  display: grid;
  place-items: center;
  font-size: 0.75rem;
  font-weight: 700;
  color: white;
}
.flash--success .flash__icon { background: #059669; }
.flash--error   .flash__icon { background: #dc2626; }
.flash--warning .flash__icon { background: #d97706; }
.flash--info    .flash__icon { background: #2563eb; }

.flash__text {
  flex: 1;
  margin: 0;
  word-wrap: break-word;
}

.flash__close {
  flex-shrink: 0;
  background: transparent;
  border: none;
  cursor: pointer;
  font-size: 1.1rem;
  line-height: 1;
  color: inherit;
  opacity: 0.5;
  padding: 0 0.25rem;
}
.flash__close:hover { opacity: 1; }

/* TransitionGroup animations */
.flash-enter-active,
.flash-leave-active {
  transition: all 0.25s ease;
}
.flash-enter-from {
  opacity: 0;
  transform: translateX(1rem);
}
.flash-leave-to {
  opacity: 0;
  transform: translateX(1rem);
}
.flash-leave-active {
  position: absolute; /* so remaining items slide up smoothly */
}

/* Dark mode friendly */
@media (prefers-color-scheme: dark) {
  .flash--success { --flash-bg: #052e2b; --flash-fg: #6ee7b7; --flash-border: #065f46; border-color: var(--flash-border); }
  .flash--error   { --flash-bg: #3f1414; --flash-fg: #fca5a5; --flash-border: #991b1b; border-color: var(--flash-border); }
  .flash--warning { --flash-bg: #3b2606; --flash-fg: #fcd34d; --flash-border: #92400e; border-color: var(--flash-border); }
  .flash--info    { --flash-bg: #0f1d3d; --flash-fg: #93c5fd; --flash-border: #1e40af; border-color: var(--flash-border); }
}
</style>
