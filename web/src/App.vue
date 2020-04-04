<template>
  <div>
    <header>
      <Navbar />
    </header>
    <main class="main">
      <router-view></router-view>
    </main>
    <Footer></Footer>
  </div>
</template>

<script>
import Navbar from "./components/commons/Navbar";
import Footer from "./components/commons/Footer";

export default {
  name: "App",
  components: {
    Navbar, Footer
  }
};
</script>

<style scoped>
  .main {
    height: 100vh;
    width: 100vw;
  }
</style>