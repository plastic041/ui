<script lang="ts">
	import { searchQuery } from '$lib/stores/query';
	import { invoke } from '@tauri-apps/api/tauri';

	type Post = {
		id: number;
		title: string;
	};

	const isPosts = (data: any): data is Post[] => {
		return (
			data[0]?.id !== undefined ||
			data[0]?.title !== undefined ||
			(Array.isArray(data) && data.length === 0) // data is an empty array
		);
	};

	const makePosts = () => invoke('make_tags');

	let posts: Post[] = [];
	$: {
		invoke('show_posts', {
			filterNames: $searchQuery.split(' ')
		}).then((res) => {
			if (isPosts(res)) {
				posts = res;
			}
		});
	}
</script>

<main>
	<button on:click={makePosts}>make</button>
	<input type="text" bind:value={$searchQuery} />

	<ul>
		{#each posts as post}
			<li>{post.title}</li>
		{/each}
	</ul>
</main>

<style>
	main {
		width: 50%;
		height: 100vh;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		gap: 1rem;
		padding-top: 4rem;
	}

	ul {
		padding: 0;
		margin: 0;
	}
</style>
