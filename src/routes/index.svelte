<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let value = '';

	function test() {
		invoke('test').then((message) => {
			value = JSON.stringify(message);
		});
	}

	$: console.log('Value from Svelte:', value);
</script>

<section>
	<button on:click={test}>Test</button>
	<p>{value}</p>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 1;
	}
</style>
