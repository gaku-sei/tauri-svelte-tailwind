<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Button from "./Button.svelte";

  let counter = 0;

  const inc = () => {
    counter++;
  };

  const dec = () => {
    counter--;
  };

  const randomize = async () => {
    counter = await invoke("random_int");
  };

  $: double = counter * 2;
</script>

<div class="flex flex-col space-y-2">
  <p class="border rounded border-gray-200 px-2 py-1 text-center">
    Counter: {counter} / Double: {double}
  </p>
  <Button on:click={inc}>Increment</Button>
  <Button on:click={dec}>Decrement</Button>
  <Button on:click={randomize}>Random</Button>
</div>
