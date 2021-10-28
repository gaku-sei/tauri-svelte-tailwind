<script lang="ts">
	import Counter from "./components/Counter.svelte";

	export let title: string;
</script>

<main class="flex flex-col p-4 w-full items-center">
	<div class="flex flex-col space-y-8 w-[300px]">
		<h1 class="text-[#ff3e00] uppercase font-thin text-6xl text-center">
			{title}
		</h1>
		<Counter />
	</div>
</main>
