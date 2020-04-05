<template>
  <div class="mt-5 container">
    <section class="jumbotron mt-5 bg-white border rounded">
      <h1 class="display-4">Best way to Central World from SCG Bangsue (Google API)</h1>
    </section>

    <section ref="mapContainer" class="border rounded p-5 mb-5">
      <div id="map"></div>
    </section>
  </div>
</template>

<script>
const scgBangsue = { lat: 13.8050023, lng: 100.5327976 };
const centralWorld = { lat: 13.7466356, lng: 100.5371464 };

export default {
  mounted() {
    const origin = JSON.stringify(scgBangsue)
    const destination = JSON.stringify(centralWorld)
    const apiKey = "AIzaSyBYtbzzry3PY5FviIpmwRxfw9XljsW6EfY"
    const callbackName = "initMap"
    const selectorId = "map"

    const googleMapScript = document.createElement("script");
    googleMapScript.async = true;
    googleMapScript.defer = true;
    googleMapScript.src = `https://maps.googleapis.com/maps/api/js?key=${apiKey}&callback=${callbackName}`;

    const initMapScript = document.createElement("script");
    initMapScript.textContent = `
    function ${callbackName}() {
      const directionsRenderer = new google.maps.DirectionsRenderer();
      const directionsService = new google.maps.DirectionsService();

      const $map = document.getElementById("${selectorId}");

      const map = new google.maps.Map($map, { zoom: 14, center: ${origin} });
      
      directionsRenderer.setMap(map);

      const travelMode = "DRIVING";

      function callback(response, status) {
        (status == "OK")
          ? directionsRenderer.setDirections(response)
          : alert(status);
      }

      directionsService.route({ origin: ${origin}, destination: ${destination}, travelMode }, callback);
    }
  `;

    this.$refs.mapContainer
      .appendChild(googleMapScript)
      .appendChild(initMapScript);
  }
};
</script>