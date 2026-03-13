<script lang="ts">
	let { data } = $props();

	let search = $state('');
	let expandedIds = $state<Set<string>>(new Set());

	let filteredByCategory = $derived.by(() => {
		const q = search.toLowerCase();
		if (!q) return data.byCategory;
		const result: Record<string, typeof data.tags> = {};
		for (const [cat, tags] of Object.entries(data.byCategory)) {
			const filtered = tags.filter(
				(t) =>
					t.tag.toLowerCase().includes(q) ||
					t.description?.toLowerCase().includes(q) ||
					t.internalDescription?.toLowerCase().includes(q)
			);
			if (filtered.length > 0) result[cat] = filtered;
		}
		return result;
	});

	let filteredCategories = $derived(Object.keys(filteredByCategory).sort());

	let totalShown = $derived(
		Object.values(filteredByCategory).reduce((sum, tags) => sum + tags.length, 0)
	);

	function toggleExpand(id: string) {
		const next = new Set(expandedIds);
		if (next.has(id)) next.delete(id);
		else next.add(id);
		expandedIds = next;
	}

	const categoryColors: Record<string, string> = {
		SKILLS: 'bg-blue-900/50 text-blue-300',
		ATTRIBUTES: 'bg-purple-900/50 text-purple-300',
		ACHIEVEMENTS: 'bg-green-900/50 text-green-300',
		DOMAINS: 'bg-amber-900/50 text-amber-300',
		INTERESTS: 'bg-pink-900/50 text-pink-300'
	};

	function catBadge(cat: string): string {
		return categoryColors[cat] ?? 'bg-gray-800 text-gray-400';
	}
</script>

<div class="space-y-4">
	<div>
		<h2 class="text-2xl font-bold text-white">Prompt Tags</h2>
		<p class="mt-1 text-sm text-gray-400">{data.tags.length} tags across {data.categories.length} categories</p>
	</div>

	<input
		type="text"
		bind:value={search}
		placeholder="Search tags..."
		class="w-full max-w-md rounded-md border border-gray-700 bg-gray-900 px-3 py-2 text-sm text-gray-200 placeholder-gray-500 focus:border-blue-500 focus:outline-none"
	/>

	{#if search}
		<p class="text-xs text-gray-500">Showing {totalShown} matching tags</p>
	{/if}

	{#each filteredCategories as category}
		{@const tags = filteredByCategory[category]}
		<section>
			<h3 class="mb-2 flex items-center gap-2 text-sm font-semibold uppercase tracking-wider text-gray-500">
				<span class="rounded px-2 py-0.5 text-xs {catBadge(category)}">{category}</span>
				<span class="text-gray-600">({tags.length})</span>
			</h3>
			<div class="overflow-x-auto rounded-lg border border-gray-800">
				<table class="w-full text-left text-sm">
					<thead class="border-b border-gray-800 bg-gray-900 text-xs uppercase tracking-wider text-gray-500">
						<tr>
							<th class="px-4 py-3">Tag</th>
							<th class="px-4 py-3 text-center">Selectable</th>
							<th class="px-4 py-3 text-center">Profile</th>
							<th class="px-4 py-3 text-right">Prompts</th>
							<th class="px-4 py-3 text-right">Children</th>
						</tr>
					</thead>
					<tbody>
						{#each tags as tag}
							<tr
								class="border-b border-gray-800/50 transition-colors hover:bg-gray-900/50 cursor-pointer"
								onclick={() => toggleExpand(tag.id)}
							>
								<td class="px-4 py-3 font-medium text-gray-200">{tag.tag}</td>
								<td class="px-4 py-3 text-center">
									{#if tag.userSelectable}
										<span class="text-green-400">Yes</span>
									{:else}
										<span class="text-gray-600">--</span>
									{/if}
								</td>
								<td class="px-4 py-3 text-center">
									{#if tag.visibleOnProfile}
										<span class="text-green-400">Yes</span>
									{:else}
										<span class="text-gray-600">--</span>
									{/if}
								</td>
								<td class="px-4 py-3 text-right font-mono text-gray-400">{tag.promptCount}</td>
								<td class="px-4 py-3 text-right font-mono text-gray-400">{tag.childTags.length}</td>
							</tr>
							{#if expandedIds.has(tag.id)}
								<tr class="bg-gray-900/30">
									<td colspan="5" class="px-4 py-3">
										<div class="space-y-2 text-sm">
											{#if tag.description}
												<p class="text-gray-300">{tag.description}</p>
											{/if}
											{#if tag.internalDescription}
												<p class="text-gray-500"><span class="text-gray-600">Internal:</span> {tag.internalDescription}</p>
											{/if}
											{#if tag.synonyms?.length}
												<p class="text-gray-500">
													<span class="text-gray-600">Synonyms:</span>
													{tag.synonyms.join(', ')}
												</p>
											{/if}
											{#if tag.childTags.length > 0}
												<div>
													<span class="text-gray-600">Children:</span>
													<div class="mt-1 flex flex-wrap gap-1">
														{#each tag.childTags as child}
															<span class="rounded bg-gray-800 px-2 py-0.5 text-xs text-gray-400">{child.tag}</span>
														{/each}
													</div>
												</div>
											{/if}
										</div>
									</td>
								</tr>
							{/if}
						{/each}
					</tbody>
				</table>
			</div>
		</section>
	{/each}
</div>
