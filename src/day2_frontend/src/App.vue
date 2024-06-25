<script setup>
import { ref } from 'vue';
import { day2_backend } from 'declarations/day2_backend/index';
let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  const birth_day = Number(target.querySelector("#birth_day").value);
  await day2_backend.greet(name, birth_day).then((response) => {
    greeting.value = response;
  });
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your first and bdate: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <input id="birth_day" alt="Birth date" type="number" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
  </main>
</template>
