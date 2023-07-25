import { createRouter, createWebHistory } from "vue-router";

export const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "HomePage",
      component: () => import("./views/home"),
      meta: {
        title: "首页"
      }
    }
  ]
});

router.beforeEach((to, _from, next) => {
  if (to.path === "/") {
    next();
    // "/live-state"
  } else {
    next();
  }
});

export default router;
