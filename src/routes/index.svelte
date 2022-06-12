<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	let value_bool = '';
	let value_int = '';
	let value_struct = '';

	function test_bool() {
		invoke('rust_test_bool').then((message) => {
			value_bool = JSON.stringify(message);
		});
	}

	function test_int() {
		invoke('rust_test_int').then((message) => {
			value_int = JSON.stringify(message);
		});
	}

	function test_struct() {
		invoke('rust_test_struct').then((message) => {
			value_struct = JSON.stringify(message);
		});
	}

	$: console.log('Bool from Svelte:', value_bool);
	$: console.log('Int from Svelte:', value_int);
	$: console.log('Struct from Svelte:', value_struct);
</script>

<section>
	<button on:click={test_bool}>Test Bool</button>
	<p>{value_bool}</p>
	<button on:click={test_int}>Test Int</button>
	<p>{value_int}</p>
	<button on:click={test_struct}>Test Struct</button>
	<p>{value_struct}</p>
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
