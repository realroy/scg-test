<template>
  <div class="mt-5 container">
    <section class="jumbotron mt-5 bg-white border rounded">
      <h1 class="display-4">Finding XYZ</h1>
    </section>
    <section class="border rounded p-5 mb-5">
      <p v-if="isLoading">Loading ...</p>
      <p v-else>
        {{ message }}
      </p>
    </section>
  </div>
</template>

<script>
import service from './xyzService'

export default {
  async mounted() {
    try {
      const result = await service()

      this.message = result
    } catch (error) {
      this.message = error
    } finally {
      this.isLoading = false
    }
  },
  data() {
    return {
      isLoading: true,
      message: ''
    }
  }
};
</script>
