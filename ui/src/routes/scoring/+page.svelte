<script lang="ts">
	let { data } = $props();

	function formatCompName(name: string): string {
		return name
			.split('_')
			.map((w) => w[0].toUpperCase() + w.slice(1))
			.join(' ');
	}

	const confLabels = ['0.0-0.2', '0.2-0.4', '0.4-0.6', '0.6-0.8', '0.8-1.0'];
	const barHeight = 18;
	const maxBarWidth = 200;
</script>

<div class="space-y-6">
	<div class="flex items-baseline gap-4">
		<div>
			<h2 class="text-2xl font-bold text-white">Scoring Runs</h2>
			<p class="mt-1 text-sm text-gray-400">{data.runSummaries.length} runs found</p>
		</div>
		<a href="/scoring/compare" class="rounded-md bg-gray-800 px-3 py-1.5 text-sm text-gray-300 hover:bg-gray-700">
			Compare Runs
		</a>
	</div>

	{#if data.runSummaries.length === 0}
		<p class="text-sm text-gray-500">No scoring runs yet. Run <code class="text-gray-400">tools/explore_scoring.py --all --save</code> to generate.</p>
	{/if}

	{#each data.runSummaries as run}
		<section class="rounded-lg border border-gray-800 bg-gray-900 p-5">
			<div class="mb-4 flex items-baseline gap-3">
				<h3 class="font-mono text-sm text-gray-300">{run.filename}</h3>
				<span class="rounded bg-gray-800 px-2 py-0.5 text-xs text-gray-400">{run.prompt_version}</span>
			</div>

			<!-- Stats -->
			<div class="mb-6 grid grid-cols-5 gap-3">
				<div>
					<p class="text-xl font-bold text-white">{run.scoredCount}</p>
					<p class="text-xs text-gray-500">Scored</p>
				</div>
				<div>
					<p class="text-xl font-bold text-white">{run.avgCompsPerTranscript.toFixed(1)}</p>
					<p class="text-xs text-gray-500">Avg competencies</p>
				</div>
				<div>
					<p class="text-xl font-bold text-white">{run.avgSkillsPerTranscript.toFixed(1)}</p>
					<p class="text-xs text-gray-500">Avg skills</p>
				</div>
				{#if run.latencyStats}
					<div>
						<p class="text-xl font-bold text-white">{run.latencyStats.avg.toFixed(1)}s</p>
						<p class="text-xs text-gray-500">Avg latency</p>
					</div>
					<div>
						<p class="text-xl font-bold text-white">{run.latencyStats.p50.toFixed(1)}s</p>
						<p class="text-xs text-gray-500">p50 latency</p>
					</div>
				{/if}
			</div>

			<div class="grid grid-cols-2 gap-6">
				<!-- Competency frequency -->
				<div>
					<h4 class="mb-2 text-xs font-semibold uppercase text-gray-500">Competency Frequency</h4>
					<svg width={maxBarWidth + 160} height={run.compFreq.length * (barHeight + 4) + 4}>
						{#each run.compFreq as comp, i}
							{@const maxCount = run.compFreq[0]?.count || 1}
							<text
								x="0"
								y={i * (barHeight + 4) + barHeight / 2 + 4}
								font-size="11"
								class="fill-gray-400"
							>
								{formatCompName(comp.name)}
							</text>
							<rect
								x="130"
								y={i * (barHeight + 4)}
								width={(comp.count / maxCount) * maxBarWidth}
								height={barHeight}
								rx="3"
								fill="#3b82f6"
								opacity="0.7"
							/>
							<text
								x={130 + (comp.count / maxCount) * maxBarWidth + 6}
								y={i * (barHeight + 4) + barHeight / 2 + 4}
								font-size="10"
								class="fill-gray-500"
							>
								{comp.count}
							</text>
						{/each}
					</svg>
				</div>

				<!-- Confidence distribution -->
				<div>
					<h4 class="mb-2 text-xs font-semibold uppercase text-gray-500">Confidence Distribution</h4>
					<svg width={maxBarWidth + 100} height={confLabels.length * (barHeight + 4) + 4}>
						{#each confLabels as label, i}
							{@const maxBucket = Math.max(...run.confBuckets, 1)}
							<text
								x="0"
								y={i * (barHeight + 4) + barHeight / 2 + 4}
								font-size="11"
								class="fill-gray-400"
							>
								{label}
							</text>
							<rect
								x="60"
								y={i * (barHeight + 4)}
								width={(run.confBuckets[i] / maxBucket) * maxBarWidth}
								height={barHeight}
								rx="3"
								fill="#8b5cf6"
								opacity="0.7"
							/>
							<text
								x={60 + (run.confBuckets[i] / maxBucket) * maxBarWidth + 6}
								y={i * (barHeight + 4) + barHeight / 2 + 4}
								font-size="10"
								class="fill-gray-500"
							>
								{run.confBuckets[i]}
							</text>
						{/each}
					</svg>
				</div>
			</div>

			{#if run.latencyStats}
				<div class="mt-4 text-xs text-gray-500">
					Latency: min {run.latencyStats.min.toFixed(1)}s / max {run.latencyStats.max.toFixed(1)}s / avg {run.latencyStats.avg.toFixed(1)}s / p50 {run.latencyStats.p50.toFixed(1)}s
				</div>
			{/if}
		</section>
	{/each}
</div>
