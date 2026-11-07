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
          path: "/demo",
          name: "demo",
          component: () => import("@/pages/demo.vue"),
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
        {
          path: "/texts",
          name: "texts",
          component: () => import("@/pages/texts.vue"),
        },
      ],
    },
  ],
})

export default router
