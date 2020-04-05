<template>
  <div class="mt-5 container">
    <section class="jumbotron mt-5 bg-white border rounded">
      <h1 class="display-4">Best way to Central World from SCG Bangsue (Google API)</h1>
    </section>

    <section ref="mapContainer" class="border rounded p-5 mb-5">
    </section>
  </div>
</template>

<script>
const API_ENDPOINT = 'https://www.google.com/maps/embed/v1/directions'
const OPTION = new URLSearchParams({
  key: "AIzaSyBYtbzzry3PY5FviIpmwRxfw9XljsW6EfY",
  origin: "SCG+สำนักงานใหญ่+บางซื่อ",
  destination: "centralwOrld",
}).toString()

export default {
  mounted() {
    const iframe = document.createElement('iframe')
    iframe.src = `${API_ENDPOINT}?${OPTION}`
    iframe.classList.add('border', 'w-100')
    iframe.height = "600px"

    this.$refs.mapContainer.appendChild(iframe)
  }
};
</script>
