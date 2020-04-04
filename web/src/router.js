import Vue from "vue";
import VueRouter from "vue-router";

Vue.use(VueRouter);

export default new VueRouter({
  mode: "history",
  linkExactActiveClass: 'active',
  routes: [
    { path: "/", component: () => import('./components/Home.vue') },
    { path: "/do-scg", component: () => import('./components/DoSCG.vue') },
    { path: "/xyz", component: () => import('./components/XYZ.vue') },
    { path: "/bc", component: () => import('./components/BC.vue') },
    { path: "/google-api", component: () => import('./components/GoogleAPI.vue') },
    { path: "/line-message-api", component: () => import('./components/DoSCG.vue') },
  ],
});
