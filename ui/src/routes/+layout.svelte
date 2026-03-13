<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import { page } from '$app/state';

	let { children } = $props();

	const nav = [
		{ href: '/', label: 'Dashboard' },
		{ href: '/transcripts', label: 'Transcripts' },
		{ href: '/taxonomy', label: 'Taxonomy' },
		{ href: '/scoring', label: 'Scoring' },
		{ href: '/skills', label: 'Skills' }
	];

	const allupNav = [
		{ href: '/tags', label: 'Tags' },
		{ href: '/prompts', label: 'Prompts' },
		{ href: '/users', label: 'Users' }
	];

	function isActive(href: string): boolean {
		if (href === '/') return page.url.pathname === '/';
		return page.url.pathname.startsWith(href);
	}
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="flex h-screen bg-gray-950 text-gray-100">
	<!-- Sidebar -->
	<nav class="flex w-56 flex-shrink-0 flex-col border-r border-gray-800 bg-gray-900">
		<div class="border-b border-gray-800 px-5 py-4">
			<h1 class="text-lg font-semibold text-white">Competency Explorer</h1>
			<p class="mt-0.5 text-xs text-gray-500">allUP Research Tool</p>
		</div>
		<div class="flex flex-col gap-1 p-3">
			{#each nav as item}
				<a
					href={item.href}
					class="rounded-md px-3 py-2 text-sm font-medium transition-colors hover:bg-gray-800 hover:text-white
						{isActive(item.href) ? 'bg-gray-800 text-white' : 'text-gray-400'}"
				>
					{item.label}
				</a>
			{/each}
			<div class="my-2 border-t border-gray-800"></div>
			<span class="px-3 py-1 text-xs font-semibold uppercase tracking-wider text-gray-600">allUP Data</span>
			{#each allupNav as item}
				<a
					href={item.href}
					class="rounded-md px-3 py-2 text-sm font-medium transition-colors hover:bg-gray-800 hover:text-white
						{isActive(item.href) ? 'bg-gray-800 text-white' : 'text-gray-400'}"
				>
					{item.label}
				</a>
			{/each}
		</div>
		<div class="mt-auto border-t border-gray-800 px-5 py-3">
			<p class="text-xs text-gray-600">Phase 2 Explorer</p>
		</div>
	</nav>

	<!-- Main content -->
	<main class="flex-1 overflow-y-auto p-6">
		{@render children()}
	</main>
</div>
