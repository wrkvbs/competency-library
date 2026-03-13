<script lang="ts">
	let { data } = $props();

	let search = $state('');
	let expandedIds = $state<Set<string>>(new Set());

	let filtered = $derived.by(() => {
		const q = search.toLowerCase();
		if (!q) return data.rows;
		return data.rows.filter(
			(r) =>
				r.selfText?.toLowerCase().includes(q) ||
				r.otherUserText?.toLowerCase().includes(q) ||
				r.tags.some((t) => t.toLowerCase().includes(q)) ||
				r.purpose?.toLowerCase().includes(q)
		);
	});

	function toggleExpand(id: string) {
		const next = new Set(expandedIds);
		if (next.has(id)) next.delete(id);
		else next.add(id);
		expandedIds = next;
	}

	function truncate(text: string | null, len: number): string {
		if (!text) return '--';
		return text.length > len ? text.slice(0, len) + '...' : text;
	}
</script>

<div class="space-y-4">
	<div>
		<h2 class="text-2xl font-bold text-white">Prompts</h2>
		<p class="mt-1 text-sm text-gray-400">{data.total} prompts</p>
	</div>

	<input
		type="text"
		bind:value={search}
		placeholder="Search prompts..."
		class="w-full max-w-md rounded-md border border-gray-700 bg-gray-900 px-3 py-2 text-sm text-gray-200 placeholder-gray-500 focus:border-blue-500 focus:outline-none"
	/>

	{#if search}
		<p class="text-xs text-gray-500">Showing {filtered.length} of {data.total}</p>
	{/if}

	<div class="overflow-x-auto rounded-lg border border-gray-800">
		<table class="w-full text-left text-sm">
			<thead class="border-b border-gray-800 bg-gray-900 text-xs uppercase tracking-wider text-gray-500">
				<tr>
					<th class="px-4 py-3">Prompt Text</th>
					<th class="px-4 py-3">Subject</th>
					<th class="px-4 py-3">Tags</th>
					<th class="px-4 py-3">Relationships</th>
					<th class="px-4 py-3 text-center">Suggested</th>
				</tr>
			</thead>
			<tbody>
				{#each filtered as row}
					<tr
						class="border-b border-gray-800/50 transition-colors hover:bg-gray-900/50 cursor-pointer"
						onclick={() => toggleExpand(row.id)}
					>
						<td class="max-w-md px-4 py-3 text-gray-200">
							{truncate(row.selfText, 80)}
						</td>
						<td class="px-4 py-3">
							<span class="rounded bg-gray-800 px-2 py-0.5 text-xs text-gray-400">{row.subjectType}</span>
						</td>
						<td class="px-4 py-3">
							<div class="flex flex-wrap gap-1">
								{#each row.tags.slice(0, 3) as tag}
									<span class="rounded bg-blue-900/40 px-2 py-0.5 text-xs text-blue-300">{tag}</span>
								{/each}
								{#if row.tags.length > 3}
									<span class="text-xs text-gray-500">+{row.tags.length - 3}</span>
								{/if}
							</div>
						</td>
						<td class="px-4 py-3 text-xs text-gray-400">
							{row.relationshipTypes.join(', ') || '--'}
						</td>
						<td class="px-4 py-3 text-center">
							{#if row.suggested}
								<span class="text-green-400">Yes</span>
							{:else}
								<span class="text-gray-600">--</span>
							{/if}
						</td>
					</tr>
					{#if expandedIds.has(row.id)}
						<tr class="bg-gray-900/30">
							<td colspan="5" class="px-4 py-3">
								<div class="space-y-2 text-sm">
									{#if row.selfText}
										<p><span class="text-gray-500">Self:</span> <span class="text-gray-300">{row.selfText}</span></p>
									{/if}
									{#if row.otherUserText}
										<p><span class="text-gray-500">Other:</span> <span class="text-gray-300">{row.otherUserText}</span></p>
									{/if}
									{#if row.purpose}
										<p><span class="text-gray-500">Purpose:</span> <span class="text-gray-400">{row.purpose}</span></p>
									{/if}
									<div class="flex gap-4 text-xs text-gray-500">
										<span>Type: {row.promptType}</span>
										{#if row.order != null}<span>Order: {row.order}</span>{/if}
										{#if row.priority != null}<span>Priority: {row.priority}</span>{/if}
									</div>
									{#if row.tags.length > 0}
										<div>
											<span class="text-gray-500">All tags:</span>
											<div class="mt-1 flex flex-wrap gap-1">
												{#each row.tags as tag}
													<span class="rounded bg-blue-900/40 px-2 py-0.5 text-xs text-blue-300">{tag}</span>
												{/each}
											</div>
										</div>
									{/if}
									{#if row.topicTags.length > 0}
										<div>
											<span class="text-gray-500">Topic tags:</span>
											<div class="mt-1 flex flex-wrap gap-1">
												{#each row.topicTags as tag}
													<span class="rounded bg-purple-900/40 px-2 py-0.5 text-xs text-purple-300">{tag}</span>
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
</div>
