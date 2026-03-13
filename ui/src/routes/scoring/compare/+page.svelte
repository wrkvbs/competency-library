<script lang="ts">
	let { data } = $props();

	let runAIdx = $state(0);
	let runBIdx = $state(0);

	$effect(() => {
		if (data.runs.length > 1) runBIdx = 1;
	});

	let comparison = $derived.by(() => {
		const runA = data.runs[runAIdx];
		const runB = data.runs[runBIdx];
		if (!runA || !runB) return null;

		// Find common response_ids
		const aMap = new Map(runA.results.map((r) => [r.response_id, r]));
		const bMap = new Map(runB.results.map((r) => [r.response_id, r]));
		const commonIds = [...aMap.keys()].filter((id) => bMap.has(id));

		const perTranscript = commonIds.map((id) => {
			const a = aMap.get(id)!;
			const b = bMap.get(id)!;

			// Score deltas per competency
			const aScores = new Map(a.behavioral_competencies.map((c) => [c.name, c.score]));
			const bScores = new Map(b.behavioral_competencies.map((c) => [c.name, c.score]));
			const allComps = new Set([...aScores.keys(), ...bScores.keys()]);
			const deltas = [...allComps].map((name) => ({
				name,
				scoreA: aScores.get(name) ?? null,
				scoreB: bScores.get(name) ?? null,
				delta: (bScores.get(name) ?? 0) - (aScores.get(name) ?? 0)
			}));

			// Skills gained/lost
			const aSkills = new Set(a.specific_skills);
			const bSkills = new Set(b.specific_skills);
			const gained = [...bSkills].filter((s) => !aSkills.has(s));
			const lost = [...aSkills].filter((s) => !bSkills.has(s));

			return { responseId: id, responder: a.responder, deltas, gained, lost };
		});

		// Aggregate
		let totalDelta = 0;
		let deltaCount = 0;
		for (const t of perTranscript) {
			for (const d of t.deltas) {
				totalDelta += d.delta;
				deltaCount++;
			}
		}

		return {
			commonCount: commonIds.length,
			perTranscript,
			avgDelta: deltaCount > 0 ? totalDelta / deltaCount : 0
		};
	});

	function formatCompName(name: string): string {
		return name
			.split('_')
			.map((w) => w[0].toUpperCase() + w.slice(1))
			.join(' ');
	}
</script>

<div class="space-y-6">
	<div>
		<a href="/scoring" class="text-sm text-gray-500 hover:text-gray-300">&larr; All Runs</a>
		<h2 class="mt-2 text-2xl font-bold text-white">Compare Scoring Runs</h2>
	</div>

	{#if data.runs.length < 2}
		<p class="text-sm text-gray-500">Need at least 2 runs to compare. Currently have {data.runs.length}.</p>
	{:else}
		<div class="flex items-center gap-4">
			<div>
				<label for="run-a-select" class="mb-1 block text-xs text-gray-500">Run A</label>
				<select
					id="run-a-select"
					bind:value={runAIdx}
					class="rounded-md border border-gray-700 bg-gray-900 px-3 py-1.5 text-sm text-gray-200"
				>
					{#each data.runs as run, i}
						<option value={i}>{run.filename}</option>
					{/each}
				</select>
			</div>
			<span class="mt-5 text-gray-500">vs</span>
			<div>
				<label for="run-b-select" class="mb-1 block text-xs text-gray-500">Run B</label>
				<select
					id="run-b-select"
					bind:value={runBIdx}
					class="rounded-md border border-gray-700 bg-gray-900 px-3 py-1.5 text-sm text-gray-200"
				>
					{#each data.runs as run, i}
						<option value={i}>{run.filename}</option>
					{/each}
				</select>
			</div>
		</div>

		{#if comparison}
			<div class="rounded-lg border border-gray-800 bg-gray-900 p-4">
				<div class="flex gap-6 text-sm">
					<span class="text-gray-400">{comparison.commonCount} common transcripts</span>
					<span class={comparison.avgDelta >= 0 ? 'text-green-400' : 'text-red-400'}>
						Avg score delta: {comparison.avgDelta >= 0 ? '+' : ''}{comparison.avgDelta.toFixed(3)}
					</span>
				</div>
			</div>

			{#each comparison.perTranscript as t}
				<div class="rounded-lg border border-gray-800 bg-gray-900 p-4">
					<h4 class="mb-3 text-sm font-medium text-white">{t.responder}</h4>

					{#if t.deltas.length > 0}
						<div class="mb-3 space-y-1">
							{#each t.deltas as d}
								<div class="flex items-center gap-3 text-xs">
									<span class="w-36 text-gray-400">{formatCompName(d.name)}</span>
									<span class="w-12 text-right font-mono text-gray-500">
										{d.scoreA !== null ? d.scoreA.toFixed(2) : '--'}
									</span>
									<span class="text-gray-600">&rarr;</span>
									<span class="w-12 text-right font-mono text-gray-500">
										{d.scoreB !== null ? d.scoreB.toFixed(2) : '--'}
									</span>
									<span
										class="w-16 text-right font-mono {d.delta > 0
											? 'text-green-400'
											: d.delta < 0
												? 'text-red-400'
												: 'text-gray-600'}"
									>
										{d.delta > 0 ? '+' : ''}{d.delta.toFixed(2)}
									</span>
								</div>
							{/each}
						</div>
					{/if}

					<div class="flex gap-6 text-xs">
						{#if t.gained.length > 0}
							<div>
								<span class="text-green-500">+ Skills:</span>
								<span class="text-gray-400">{t.gained.join(', ')}</span>
							</div>
						{/if}
						{#if t.lost.length > 0}
							<div>
								<span class="text-red-500">- Skills:</span>
								<span class="text-gray-400">{t.lost.join(', ')}</span>
							</div>
						{/if}
						{#if t.gained.length === 0 && t.lost.length === 0}
							<span class="text-gray-600">No skill changes</span>
						{/if}
					</div>
				</div>
			{/each}
		{/if}
	{/if}
</div>
