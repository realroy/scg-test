import Vue from "vue";
import VueRouter from "vue-router";

Vue.use(VueRouter);

export default new VueRouter({
  mode: "history",
  linkExactActiveClass: 'active',
  routes: [
    { path: "/", name: "home", component: () => import('./components/Home.vue') },
    { path: "/do-scg", name: "do-scg", component: () => import('./components/DoSCG.vue') },
  ],
});
