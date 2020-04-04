<template>
  <nav class="navbar navbar-expand-lg navbar-dark bg-dark fixed-top">
    <router-link class="navbar-brand" to="/">SCG Test</router-link>
    <ul class="navbar-nav">
      <li class="nav-item">
        <router-link class="nav-link" to="/">Home</router-link>
      </li>
      <li class="nav-item">
        <router-link class="nav-link" to="/do-scg">DOSCG</router-link>
      </li>
      <li class="nav-item">
        <router-link class="nav-link" to="/xyz">Finding X, Y and Z</router-link>
      </li>
      <li class="nav-item">
        <router-link class="nav-link" to="/finding-b-and-c">Finding B and C</router-link>
      </li>
      <div class="nav-item">
        <router-link class="nav-link" to="/google-api">Google API</router-link>
      </div>
      <div class="nav-item">
        <router-link class="nav-link" to="/line-message-api">Line Message API</router-link>
      </div>
    </ul>
  </nav>
</template>
