<template>
  <div class="mt-5 container">
    <section class="jumbotron mt-5 bg-white border rounded">
      <h1 class="display-4">Finding B and C</h1>
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
import { fetchDataService } from '../services'

export default {
  async mounted() {
    try {
      const result = await fetchDataService('api/bc', 'BC')

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


