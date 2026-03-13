<script lang="ts">
	let { data } = $props();
</script>

<div class="space-y-4">
	<div>
		<h2 class="text-2xl font-bold text-white">Transcripts</h2>
		<p class="mt-1 text-sm text-gray-400">{data.rows.length} response transcripts</p>
	</div>

	<div class="overflow-x-auto rounded-lg border border-gray-800">
		<table class="w-full text-left text-sm">
			<thead class="border-b border-gray-800 bg-gray-900 text-xs uppercase tracking-wider text-gray-500">
				<tr>
					<th class="px-4 py-3">Responder</th>
					<th class="px-4 py-3">Prompt</th>
					<th class="px-4 py-3 text-right">Length</th>
					<th class="px-4 py-3 text-right">Tags</th>
					<th class="px-4 py-3 text-center">Scored</th>
				</tr>
			</thead>
			<tbody>
				{#each data.rows as row}
					<tr class="border-b border-gray-800/50 transition-colors hover:bg-gray-900/50">
						<td class="px-4 py-3">
							<a href="/transcripts/{row.id}" class="font-medium text-blue-400 hover:text-blue-300">
								{row.responder}
							</a>
						</td>
						<td class="max-w-xs truncate px-4 py-3 text-gray-400">
							{row.promptText}
						</td>
						<td class="px-4 py-3 text-right font-mono text-gray-400">
							{#if row.transcriptLength > 0}
								{row.transcriptLength.toLocaleString()}
							{:else}
								<span class="text-gray-600">--</span>
							{/if}
						</td>
						<td class="px-4 py-3 text-right text-gray-400">
							{row.tagCount}
						</td>
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
</div>
