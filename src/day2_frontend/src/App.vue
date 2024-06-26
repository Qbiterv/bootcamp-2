<script setup>
import { ref } from 'vue';
import { day2_backend } from 'declarations/day2_backend/index';
import Blog from "./components/Blog.vue";
</script>

<template>
  <main class=" m-20">
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <Blog />
  </main>
</template>
