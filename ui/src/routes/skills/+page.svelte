<script lang="ts">
	let { data } = $props();

	let search = $state('');
	let minCount = $state(false);

	type SortKey = 'name' | 'category' | 'count' | 'avgDepth' | 'avgConfidence';
	let sortBy = $state<SortKey>('count');
	let sortAsc = $state(false);

	let filtered = $derived.by(() => {
		let rows = data.skillRows;
		if (minCount) rows = rows.filter((r) => r.count >= 2);
		if (search) {
			const q = search.toLowerCase();
			rows = rows.filter((r) => r.name.toLowerCase().includes(q));
		}
		const dir = sortAsc ? 1 : -1;
		rows = rows.slice().sort((a, b) => {
			const av = a[sortBy];
			const bv = b[sortBy];
			if (typeof av === 'string' && typeof bv === 'string') return av.localeCompare(bv) * dir;
			return ((av as number) - (bv as number)) * dir;
		});
		return rows;
	});

	function toggleSort(key: SortKey) {
		if (sortBy === key) {
			sortAsc = !sortAsc;
		} else {
			sortBy = key;
			sortAsc = key === 'name' || key === 'category';
		}
	}

	function sortIndicator(key: SortKey): string {
		if (sortBy !== key) return '';
		return sortAsc ? ' \u25B2' : ' \u25BC';
	}
</script>

<div class="space-y-4">
	<div>
		<h2 class="text-2xl font-bold text-white">Skills Analysis</h2>
		<p class="mt-1 text-sm text-gray-400">
			Extracted skills from {data.runFilename ?? 'no run'}
		</p>
	</div>

	{#if data.skillRows.length === 0}
		<p class="text-sm text-gray-500">No scoring data available.</p>
	{:else}
		<div class="flex items-center gap-4">
			<input
				type="text"
				placeholder="Search skills..."
				bind:value={search}
				class="rounded-md border border-gray-700 bg-gray-900 px-3 py-1.5 text-sm text-gray-200 placeholder-gray-600 outline-none focus:border-gray-500"
			/>
			<label class="flex items-center gap-2 text-sm text-gray-400">
				<input type="checkbox" bind:checked={minCount} class="rounded" />
				2+ extractions only
			</label>
			<span class="text-xs text-gray-500">{filtered.length} skills</span>
		</div>

		<div class="overflow-x-auto rounded-lg border border-gray-800">
			<table class="w-full text-left text-sm">
				<thead class="border-b border-gray-800 bg-gray-900 text-xs uppercase tracking-wider text-gray-500">
					<tr>
						<th class="cursor-pointer px-3 py-2 hover:text-gray-300" onclick={() => toggleSort('name')}>
							Skill{sortIndicator('name')}
						</th>
						<th class="cursor-pointer px-3 py-2 hover:text-gray-300" onclick={() => toggleSort('category')}>
							Category{sortIndicator('category')}
						</th>
						<th class="cursor-pointer px-3 py-2 text-right hover:text-gray-300" onclick={() => toggleSort('count')}>
							Count{sortIndicator('count')}
						</th>
						<th class="cursor-pointer px-3 py-2 text-right hover:text-gray-300" onclick={() => toggleSort('avgDepth')}>
							Avg Depth{sortIndicator('avgDepth')}
						</th>
						<th class="cursor-pointer px-3 py-2 text-right hover:text-gray-300" onclick={() => toggleSort('avgConfidence')}>
							Avg Conf{sortIndicator('avgConfidence')}
						</th>
						<th class="px-3 py-2 text-center">O*NET</th>
						<th class="px-3 py-2 text-center">Tag</th>
					</tr>
				</thead>
				<tbody>
					{#each filtered as row}
						<tr class="border-b border-gray-800/50 hover:bg-gray-900/50">
							<td class="px-3 py-2 text-gray-200">{row.name}</td>
							<td class="px-3 py-2 text-xs text-gray-400">{row.category.replace('_', ' ')}</td>
							<td class="px-3 py-2 text-right font-mono text-gray-300">{row.count}</td>
							<td class="px-3 py-2 text-right font-mono text-gray-400">{row.avgDepth.toFixed(2)}</td>
							<td class="px-3 py-2 text-right font-mono text-gray-400">{row.avgConfidence.toFixed(2)}</td>
							<td class="px-3 py-2 text-center">
								{#if row.inOnet}
									<span class="text-green-400">Yes</span>
								{:else}
									<span class="text-gray-600">--</span>
								{/if}
							</td>
							<td class="px-3 py-2 text-center">
								{#if row.inTags}
									<span class="text-green-400">Yes</span>
								{:else}
									<span class="text-gray-600">--</span>
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
