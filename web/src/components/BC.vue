<template>
  <div class="mt-5 container">
    <section class="jumbotron mt-5 bg-white border rounded">
      <h1 class="display-4">Finding B and C</h1>
    </section>
    <section class="border rounded p-5 mb-5">
      <div v-if="isLoading">Loading ...</div>
      <div v-else>
        <p>
          <strong>Let</strong>
        </p>
        <p>
          A = {{ a }}
        </p>
        <p>
          A + B = {{ aPlusB }}
        </p>
        <p>
          A + C = {{ aPlusC }}
        </p>
        <p><strong>Then</strong></p>
        <p>
          B = {{ b }} and C = {{ c }}
        </p>
      </div>
    </section>
  </div>
</template>

<script>
import { fetchDataService } from '../services'

export default {
  async mounted() {
    try {
      const { b, c } = await fetchDataService({
        path: 'bc',
        key: 'BC',
        method: 'POST',
        body: {
          a: this.a,
          a_plus_b: this.aPlusB,
          a_plus_c: this.aPlusC,
        }
      })

      this.b = b
      this.c = c
    } catch (error) {
      this.error = error
    } finally {
      this.isLoading = false
    }
  },
  data() {
    return {
      isLoading: true,
      a: 21,
      aPlusB: 23,
      aPlusC: -21,
      b: 0,
      c: 0,
      error: null
    }
  }
};
</script>


