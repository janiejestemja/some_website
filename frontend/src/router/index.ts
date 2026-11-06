import { createRouter, createWebHistory } from "vue-router";

import BaseView from "@/views/BaseView.vue";

const router = createRouter({
  // history: createWebHistory(import.meta.env.BASE_URL),
  history: createWebHistory("/"),
  routes: [
    {
      path: "/",
      name: "BaseView",
      component: BaseView,
      children: [
        {
          path: "",
          name: "base",
          component: () => import("@/pages/base.vue"),
        },
        {
          path: "/notes",
          name: "notes",
          component: () => import("@/pages/notes.vue"),
        },
        {
          path: "/info",
          name: "info",
          component: () => import("@/pages/info.vue"),
        },
        {
          path: "/login",
          name: "login",
          component: () => import("@/pages/login.vue"),
        },
      ],
    },
  ],
})

export default router
