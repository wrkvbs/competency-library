<script lang="ts">
	let { data } = $props();

	let taxonomyCards = $derived([
		{ label: 'O*NET Abilities', count: data.counts.onetAbilities },
		{ label: 'O*NET Skills', count: data.counts.onetSkills },
		{ label: 'O*NET Knowledge', count: data.counts.onetKnowledge },
		{ label: 'O*NET Work Styles', count: data.counts.onetWorkStyles },
		{ label: 'O*NET Work Activities', count: data.counts.onetWorkActivities },
		{ label: 'OPM Competencies', count: data.counts.opmCompetencies },
		{ label: 'WORKBank Tasks', count: data.counts.workbankTasks }
	]);

	// Blue color scale for heatmap: 0 -> dark, 1 -> bright blue
	function heatColor(value: number | null): string {
		if (value === null) return '#1f2937'; // gray-800
		// Interpolate from dark navy to bright blue
		const r = Math.round(30 + value * 30);
		const g = Math.round(40 + value * 80);
		const b = Math.round(80 + value * 175);
		return `rgb(${r}, ${g}, ${b})`;
	}

	function formatCompName(name: string): string {
		return name
			.split('_')
			.map((w) => w[0].toUpperCase() + w.slice(1))
			.join(' ');
	}

	const cellSize = 32;
	const labelWidth = 160;
	const headerHeight = 120;
</script>

<div class="space-y-8">
	<div>
		<h2 class="text-2xl font-bold text-white">Dashboard</h2>
		<p class="mt-1 text-sm text-gray-400">Competency library exploration overview</p>
	</div>

	<!-- Stats cards -->
	<section>
		<h3 class="mb-3 text-sm font-semibold uppercase tracking-wider text-gray-500">allUP Data</h3>
		<div class="grid grid-cols-4 gap-4">
			<div class="rounded-lg border border-gray-800 bg-gray-900 p-4">
				<p class="text-3xl font-bold text-white">{data.counts.transcripts}</p>
				<p class="text-sm text-gray-400">Transcripts</p>
			</div>
			<div class="rounded-lg border border-gray-800 bg-gray-900 p-4">
				<p class="text-3xl font-bold text-white">{data.counts.transcriptsWithText}</p>
				<p class="text-sm text-gray-400">With text</p>
			</div>
			<div class="rounded-lg border border-gray-800 bg-gray-900 p-4">
				<p class="text-3xl font-bold text-white">{data.counts.scoredResponses}</p>
				<p class="text-sm text-gray-400">Scored</p>
			</div>
			<div class="rounded-lg border border-gray-800 bg-gray-900 p-4">
				<p class="text-3xl font-bold text-white">{data.counts.scoringRuns}</p>
				<p class="text-sm text-gray-400">Scoring runs</p>
			</div>
		</div>
	</section>

	<!-- Latest run info -->
	{#if data.latestRun}
		<section>
			<h3 class="mb-3 text-sm font-semibold uppercase tracking-wider text-gray-500">
				Latest Run
			</h3>
			<div class="rounded-lg border border-gray-800 bg-gray-900 p-4">
				<div class="flex items-baseline gap-4">
					<span class="font-mono text-sm text-gray-300">{data.latestRun.filename}</span>
					<span class="rounded bg-gray-800 px-2 py-0.5 text-xs text-gray-400"
						>{data.latestRun.prompt_version}</span
					>
				</div>
				<div class="mt-2 flex gap-6 text-sm">
					<span class="text-gray-400"
						>{data.latestRun.resultCount - data.latestRun.errorCount} scored</span
					>
					{#if data.latestRun.errorCount > 0}
						<span class="text-amber-400">{data.latestRun.errorCount} errors</span>
					{/if}
				</div>
			</div>
		</section>
	{/if}

	<!-- Behavioral Competency Heatmap -->
	{#if data.heatmapRows.length > 0}
		<section>
			<h3 class="mb-3 text-sm font-semibold uppercase tracking-wider text-gray-500">
				Behavioral Competency Heatmap
			</h3>
			<div class="overflow-x-auto rounded-lg border border-gray-800 bg-gray-900 p-4">
				<svg
					width={labelWidth + data.behavioralCompetencies.length * cellSize + 20}
					height={headerHeight + data.heatmapRows.length * cellSize + 40}
				>
					<!-- Column headers (rotated competency names) -->
					{#each data.behavioralCompetencies as comp, ci}
						<text
							x={labelWidth + ci * cellSize + cellSize / 2}
							y={headerHeight - 8}
							text-anchor="start"
							transform="rotate(-45, {labelWidth + ci * cellSize + cellSize / 2}, {headerHeight - 8})"
							class="fill-gray-400"
							font-size="11"
						>
							{formatCompName(comp)}
						</text>
					{/each}

					<!-- Rows -->
					{#each data.heatmapRows as row, ri}
						<!-- Responder name -->
						<text
							x={labelWidth - 8}
							y={headerHeight + ri * cellSize + cellSize / 2 + 4}
							text-anchor="end"
							class="fill-gray-300"
							font-size="12"
						>
							{row.responder.length > 18 ? row.responder.slice(0, 18) + '...' : row.responder}
						</text>

						<!-- Score cells -->
						{#each data.behavioralCompetencies as comp, ci}
							{@const score = row.scores[comp]}
							<a href="/transcripts/{row.responseId}">
								<rect
									x={labelWidth + ci * cellSize + 1}
									y={headerHeight + ri * cellSize + 1}
									width={cellSize - 2}
									height={cellSize - 2}
									rx="3"
									fill={heatColor(score)}
									class="cursor-pointer hover:opacity-80"
								>
									<title
										>{row.responder}: {formatCompName(comp)} = {score !== null ? score.toFixed(2) : 'N/A'}</title
									>
								</rect>
								{#if score !== null}
									<text
										x={labelWidth + ci * cellSize + cellSize / 2}
										y={headerHeight + ri * cellSize + cellSize / 2 + 4}
										text-anchor="middle"
										class="pointer-events-none fill-white"
										font-size="9"
										opacity="0.7"
									>
										{score.toFixed(1)}
									</text>
								{/if}
							</a>
						{/each}
					{/each}

					<!-- Average row -->
					<text
						x={labelWidth - 8}
						y={headerHeight + data.heatmapRows.length * cellSize + cellSize / 2 + 4}
						text-anchor="end"
						class="fill-gray-500"
						font-size="11"
						font-weight="bold"
					>
						Average
					</text>
					{#each data.behavioralCompetencies as comp, ci}
						{@const avg = data.compAverages[comp]}
						<rect
							x={labelWidth + ci * cellSize + 1}
							y={headerHeight + data.heatmapRows.length * cellSize + 6}
							width={cellSize - 2}
							height={cellSize - 2}
							rx="3"
							fill={heatColor(avg)}
							stroke="#374151"
							stroke-width="1"
						>
							<title>{formatCompName(comp)} avg: {avg !== null ? avg.toFixed(2) : 'N/A'}</title>
						</rect>
						{#if avg !== null}
							<text
								x={labelWidth + ci * cellSize + cellSize / 2}
								y={headerHeight + data.heatmapRows.length * cellSize + 6 + cellSize / 2 + 4}
								text-anchor="middle"
								class="fill-white"
								font-size="9"
								font-weight="bold"
								opacity="0.9"
							>
								{avg.toFixed(2)}
							</text>
						{/if}
					{/each}
				</svg>

				<!-- Color legend -->
				<div class="mt-3 flex items-center gap-3 text-xs text-gray-500">
					<span>Score:</span>
					<div class="flex items-center gap-1">
						<div class="h-3 w-3 rounded" style="background: {heatColor(0)}"></div>
						<span>0.0</span>
					</div>
					<div class="flex items-center gap-1">
						<div class="h-3 w-3 rounded" style="background: {heatColor(0.5)}"></div>
						<span>0.5</span>
					</div>
					<div class="flex items-center gap-1">
						<div class="h-3 w-3 rounded" style="background: {heatColor(1.0)}"></div>
						<span>1.0</span>
					</div>
					<div class="ml-4 flex items-center gap-1">
						<div class="h-3 w-3 rounded" style="background: {heatColor(null)}"></div>
						<span>Not detected</span>
					</div>
				</div>
			</div>
		</section>
	{:else}
		<section>
			<h3 class="mb-3 text-sm font-semibold uppercase tracking-wider text-gray-500">
				Scoring
			</h3>
			<p class="text-sm text-gray-500">
				No scoring runs yet. Run
				<code class="text-gray-400">tools/explore_scoring.py --all --save</code> to generate.
			</p>
		</section>
	{/if}

	<!-- Taxonomy Sources -->
	<section>
		<h3 class="mb-3 text-sm font-semibold uppercase tracking-wider text-gray-500">
			Taxonomy Sources
		</h3>
		<div class="grid grid-cols-4 gap-3">
			{#each taxonomyCards as card}
				<div class="rounded-lg border border-gray-800 bg-gray-900 p-3">
					<p class="text-2xl font-bold text-white">
						{card.count > 0 ? card.count.toLocaleString() : '--'}
					</p>
					<p class="text-xs text-gray-400">{card.label}</p>
					{#if card.count === 0}
						<p class="mt-1 text-xs text-gray-600">Not yet processed</p>
					{/if}
				</div>
			{/each}
		</div>
	</section>

	<!-- Quick Links -->
	<section>
		<h3 class="mb-3 text-sm font-semibold uppercase tracking-wider text-gray-500">
			Explore
		</h3>
		<div class="grid grid-cols-3 gap-4">
			<a
				href="/transcripts"
				class="rounded-lg border border-gray-800 bg-gray-900 p-4 transition-colors hover:border-gray-700"
			>
				<p class="font-medium text-white">Transcripts</p>
				<p class="mt-1 text-sm text-gray-400">Browse all response transcripts</p>
			</a>
			<a
				href="/taxonomy"
				class="rounded-lg border border-gray-800 bg-gray-900 p-4 transition-colors hover:border-gray-700"
			>
				<p class="font-medium text-white">Taxonomy Browser</p>
				<p class="mt-1 text-sm text-gray-400">Explore O*NET, OPM, and ESCO</p>
			</a>
			<a
				href="/scoring"
				class="rounded-lg border border-gray-800 bg-gray-900 p-4 transition-colors hover:border-gray-700"
			>
				<p class="font-medium text-white">Scoring Runs</p>
				<p class="mt-1 text-sm text-gray-400">View and compare scoring results</p>
			</a>
		</div>
	</section>
</div>
