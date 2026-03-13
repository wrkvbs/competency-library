<script lang="ts">
	let { data } = $props();

	let search = $state('');

	let filtered = $derived.by(() => {
		const q = search.toLowerCase();
		if (!q) return data.rows;
		return data.rows.filter(
			(r) =>
				r.fullName.toLowerCase().includes(q) ||
				r.headline?.toLowerCase().includes(q) ||
				r.currentOrg?.toLowerCase().includes(q) ||
				r.currentTitle?.toLowerCase().includes(q)
		);
	});
</script>

<div class="space-y-4">
	<div>
		<h2 class="text-2xl font-bold text-white">Users</h2>
		<p class="mt-1 text-sm text-gray-400">{data.total} user profiles</p>
	</div>

	<input
		type="text"
		bind:value={search}
		placeholder="Search by name, headline, or org..."
		class="w-full max-w-md rounded-md border border-gray-700 bg-gray-900 px-3 py-2 text-sm text-gray-200 placeholder-gray-500 focus:border-blue-500 focus:outline-none"
	/>

	{#if search}
		<p class="text-xs text-gray-500">Showing {filtered.length} matches on this page</p>
	{/if}

	<div class="overflow-x-auto rounded-lg border border-gray-800">
		<table class="w-full text-left text-sm">
			<thead class="border-b border-gray-800 bg-gray-900 text-xs uppercase tracking-wider text-gray-500">
				<tr>
					<th class="px-4 py-3">Name</th>
					<th class="px-4 py-3">Headline</th>
					<th class="px-4 py-3">Current Org</th>
					<th class="px-4 py-3 text-right">Tags</th>
					<th class="px-4 py-3 text-right">Responses</th>
					<th class="px-4 py-3 text-right">Resumes</th>
					<th class="px-4 py-3 text-center">Scored</th>
				</tr>
			</thead>
			<tbody>
				{#each filtered as row}
					<tr class="border-b border-gray-800/50 transition-colors hover:bg-gray-900/50">
						<td class="px-4 py-3">
							<a href="/users/{row.userId.replace('user:', '')}" class="font-medium text-blue-400 hover:text-blue-300">
								{row.fullName}
							</a>
						</td>
						<td class="max-w-xs truncate px-4 py-3 text-gray-400">
							{row.headline ?? '--'}
						</td>
						<td class="px-4 py-3 text-gray-400">
							{#if row.currentOrg}
								<span class="text-gray-300">{row.currentOrg}</span>
								{#if row.currentTitle}
									<span class="text-gray-600"> / {row.currentTitle}</span>
								{/if}
							{:else}
								<span class="text-gray-600">--</span>
							{/if}
						</td>
						<td class="px-4 py-3 text-right font-mono text-gray-400">{row.tagCount}</td>
						<td class="px-4 py-3 text-right font-mono text-gray-400">{row.responseCount}</td>
						<td class="px-4 py-3 text-right font-mono text-gray-400">{row.resumeCount}</td>
						<td class="px-4 py-3 text-center">
							{#if row.scored}
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

	<!-- Pagination -->
	{#if data.totalPages > 1}
		<div class="flex items-center justify-between text-sm text-gray-400">
			<span>Page {data.page} of {data.totalPages}</span>
			<div class="flex gap-2">
				{#if data.page > 1}
					<a
						href="/users?page={data.page - 1}"
						class="rounded border border-gray-700 px-3 py-1 hover:bg-gray-800"
					>
						Prev
					</a>
				{/if}
				{#if data.page < data.totalPages}
					<a
						href="/users?page={data.page + 1}"
						class="rounded border border-gray-700 px-3 py-1 hover:bg-gray-800"
					>
						Next
					</a>
				{/if}
			</div>
		</div>
	{/if}
</div>
