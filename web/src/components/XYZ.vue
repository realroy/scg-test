<template>
  <div class="mt-5 container">
    <section class="jumbotron mt-5 bg-white border rounded">
      <h1 class="display-4">Finding XYZ</h1>
    </section>
    <section class="border rounded p-5 mb-5">
      <div v-if="isLoading">Loading ...</div>
      <div v-else>
        <strong>If</strong> X, Y, 5, 9, 15, 23, Z <br />
        <strong>Then</strong> X = {{ x }} Y = {{ y }}  Z = {{ z }}
      </div>
    </section>
  </div>
</template>

<script>
import { fetchDataService } from '../services'

export default {
  async mounted() {
    try {
      const { x, y, z } = await fetchDataService({
        path: 'xyz',
        key: 'XYZ',
        method: 'POST',
        body: {
          values: [5, 9, 15, 3]
        }
      })

      this.x = x
      this.y = y
      this.z = z

    } catch (error) {
      this.error = error
    } finally {
      this.isLoading = false
    }
  },
  data() {
    return {
      isLoading: true,
      x: 0,
      y: 0,
      z: 0,
      error: null
    }
  }
};
</script>
