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

	let posts: Post[] = [];
	$: {
		invoke('show_posts', {
			filterNames: $searchQuery.trim().split(' ')
		}).then((res) => {
			if (isPosts(res)) {
				posts = res;
			}
		});
	}
</script>

<main>
	<input type="text" bind:value={$searchQuery} />
	<code>{JSON.stringify($searchQuery.trim().split(' '), null, 2)}</code>

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
