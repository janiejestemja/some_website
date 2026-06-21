import { createRouter, createWebHistory } from "vue-router";

import BaseView from "@/views/BaseView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "BaseView",
      displayName: "BaseView",
      component: BaseView,
      children: [
        {
          path: "",
          name: "base",
          displayName: "base",
          component: () => import("@/pages/base.vue"),
        },
        {
          path: "/info",
          name: "info",
          displayName: "info",
          component: () => import("@/pages/info.vue"),
        },
      ],
    },
  ],
})

export default router
