<script lang="ts">
	let { data } = $props();

	let expandedEvidence = $state<Record<string, boolean>>({});
	let expandedSkillEvidence = $state<Record<string, boolean>>({});

	function toggleEvidence(name: string) {
		expandedEvidence[name] = !expandedEvidence[name];
	}
	function toggleSkillEvidence(name: string) {
		expandedSkillEvidence[name] = !expandedSkillEvidence[name];
	}

	function formatCompName(name: string): string {
		return name
			.split('_')
			.map((w) => w[0].toUpperCase() + w.slice(1))
			.join(' ');
	}

	function depthColor(depth: string): string {
		switch (depth) {
			case 'surface':
				return 'bg-gray-600 text-gray-200';
			case 'working':
				return 'bg-blue-800 text-blue-200';
			case 'deep':
				return 'bg-indigo-700 text-indigo-200';
			case 'expert':
				return 'bg-purple-700 text-purple-200';
			default:
				return 'bg-gray-700 text-gray-300';
		}
	}

	function matchColor(source: string): string {
		switch (source) {
			case 'both':
				return 'text-green-400 bg-green-900/30 border-green-800';
			case 'tag_only':
				return 'text-red-400 bg-red-900/30 border-red-800';
			case 'skill_only':
				return 'text-amber-400 bg-amber-900/30 border-amber-800';
			default:
				return 'text-gray-400';
		}
	}

	let sortedCompetencies = $derived(
		(data.scoringResult?.extraction?.behavioral_competencies ?? [])
			.slice()
			.sort((a, b) => b.score - a.score)
	);

	let skillsByCategory = $derived.by(() => {
		const skills = data.scoringResult?.extraction?.specific_skills ?? [];
		const groups: Record<string, typeof skills> = {};
		for (const s of skills) {
			const cat = s.category || 'uncategorized';
			if (!groups[cat]) groups[cat] = [];
			groups[cat].push(s);
		}
		return groups;
	});

	const maxBarWidth = 300;
</script>

<div class="space-y-8">
	<!-- Back link + header -->
	<div>
		<a href="/transcripts" class="text-sm text-gray-500 hover:text-gray-300">
			&larr; All Transcripts
		</a>
		<h2 class="mt-2 text-2xl font-bold text-white">{data.transcript.responder}</h2>
		<div class="mt-1 flex items-center gap-3 text-sm text-gray-400">
			<span class="rounded bg-gray-800 px-2 py-0.5 text-xs">
				{data.transcript.isSelf ? 'Self-response' : 'Reference'}
			</span>
			{#if data.scoringResult}
				<span class="rounded bg-gray-800 px-2 py-0.5 text-xs">
					Scored ({data.scoringResult.prompt_version})
				</span>
			{/if}
		</div>
	</div>

	<!-- Context -->
	<section class="rounded-lg border border-gray-800 bg-gray-900 p-5">
		<h3 class="mb-3 text-sm font-semibold uppercase tracking-wider text-gray-500">Prompt</h3>
		<p class="text-sm text-gray-300">{data.transcript.promptText}</p>

		<h3 class="mb-3 mt-6 text-sm font-semibold uppercase tracking-wider text-gray-500">
			Transcript
		</h3>
		{#if data.transcript.text}
			<p class="whitespace-pre-wrap text-sm leading-relaxed text-gray-300">
				{data.transcript.text}
			</p>
		{:else}
			<p class="text-sm text-gray-600">No transcript text available.</p>
		{/if}
	</section>

	<!-- Scoring Results -->
	{#if data.scoringResult?.extraction}
		{@const extraction = data.scoringResult.extraction}

		<!-- Behavioral Competencies -->
		<section class="rounded-lg border border-gray-800 bg-gray-900 p-5">
			<h3 class="mb-4 text-sm font-semibold uppercase tracking-wider text-gray-500">
				Behavioral Competencies ({sortedCompetencies.length})
			</h3>

			<div class="space-y-2">
				{#each sortedCompetencies as comp}
					<div>
						<button
							onclick={() => toggleEvidence(comp.name)}
							class="flex w-full items-center gap-3 rounded px-2 py-1.5 text-left hover:bg-gray-800/50"
						>
							<span class="w-40 shrink-0 text-sm text-gray-300">{formatCompName(comp.name)}</span>
							<div class="flex-1">
								<svg width={maxBarWidth + 40} height="20">
									<!-- Background bar -->
									<rect x="0" y="4" width={maxBarWidth} height="12" rx="3" fill="#1f2937" />
									<!-- Score bar -->
									<rect
										x="0"
										y="4"
										width={comp.score * maxBarWidth}
										height="12"
										rx="3"
										fill="#3b82f6"
										opacity={0.3 + comp.confidence * 0.7}
									/>
									<!-- Score text -->
									<text
										x={maxBarWidth + 6}
										y="14"
										font-size="11"
										fill="#9ca3af"
									>
										{comp.score.toFixed(2)}
									</text>
								</svg>
							</div>
							<span class="shrink-0 text-xs text-gray-500">conf {comp.confidence.toFixed(2)}</span>
						</button>
						{#if expandedEvidence[comp.name]}
							<p class="ml-44 mt-1 rounded bg-gray-800/50 px-3 py-2 text-xs leading-relaxed text-gray-400">
								{comp.evidence}
							</p>
						{/if}
					</div>
				{/each}
			</div>
		</section>

		<!-- Specific Skills -->
		<section class="rounded-lg border border-gray-800 bg-gray-900 p-5">
			<h3 class="mb-4 text-sm font-semibold uppercase tracking-wider text-gray-500">
				Specific Skills ({extraction.specific_skills.length})
			</h3>

			{#each Object.entries(skillsByCategory) as [category, skills]}
				<div class="mb-4">
					<h4 class="mb-2 text-xs font-semibold uppercase text-gray-500">
						{category.replace('_', ' ')}
					</h4>
					<div class="space-y-1">
						{#each skills as skill}
							<div>
								<button
									onclick={() => toggleSkillEvidence(skill.name)}
									class="flex w-full items-center gap-3 rounded px-2 py-1.5 text-left hover:bg-gray-800/50"
								>
									<span class="w-40 shrink-0 text-sm text-gray-300">{skill.name}</span>
									<span class="shrink-0 rounded px-2 py-0.5 text-xs {depthColor(skill.depth)}">
										{skill.depth}
									</span>
									<div class="flex-1">
										<div class="flex items-center gap-2">
											<div class="h-1.5 w-24 rounded-full bg-gray-800">
												<div
													class="h-1.5 rounded-full bg-blue-500"
													style="width: {skill.depth_score * 100}%"
												></div>
											</div>
											<span class="text-xs text-gray-500">{skill.depth_score.toFixed(2)}</span>
										</div>
									</div>
									<span class="shrink-0 text-xs text-gray-500">conf {skill.confidence.toFixed(2)}</span>
								</button>
								{#if expandedSkillEvidence[skill.name]}
									<p class="ml-44 mt-1 rounded bg-gray-800/50 px-3 py-2 text-xs leading-relaxed text-gray-400">
										{skill.evidence}
									</p>
								{/if}
							</div>
						{/each}
					</div>
				</div>
			{/each}
		</section>

		<!-- Overall Assessment -->
		{#if extraction.overall_assessment}
			<section class="rounded-lg border border-gray-800 bg-gray-900 p-5">
				<h3 class="mb-4 text-sm font-semibold uppercase tracking-wider text-gray-500">
					Overall Assessment
				</h3>
				{#if extraction.overall_assessment.strongest_signals?.length}
					<div class="mb-4">
						<h4 class="mb-1 text-xs font-semibold text-gray-500">Strongest Signals</h4>
						<ul class="list-inside list-disc text-sm text-gray-300">
							{#each extraction.overall_assessment.strongest_signals as signal}
								<li>{signal}</li>
							{/each}
						</ul>
					</div>
				{/if}
				{#if extraction.overall_assessment.gaps?.length}
					<div class="mb-4">
						<h4 class="mb-1 text-xs font-semibold text-gray-500">Gaps</h4>
						<ul class="list-inside list-disc text-sm text-amber-300/80">
							{#each extraction.overall_assessment.gaps as gap}
								<li>{gap}</li>
							{/each}
						</ul>
					</div>
				{/if}
				{#if extraction.overall_assessment.suggested_followup_questions?.length}
					<div>
						<h4 class="mb-1 text-xs font-semibold text-gray-500">Suggested Follow-up</h4>
						<ul class="list-inside list-disc text-sm text-gray-400">
							{#each extraction.overall_assessment.suggested_followup_questions as q}
								<li>{q}</li>
							{/each}
						</ul>
					</div>
				{/if}
			</section>
		{/if}
	{:else if !data.scoringResult}
		<section class="rounded-lg border border-gray-800 bg-gray-900 p-5">
			<p class="text-sm text-gray-500">This transcript has not been scored yet.</p>
		</section>
	{/if}

	<!-- Tag Comparison -->
	{#if data.tagMatches.length > 0}
		<section class="rounded-lg border border-gray-800 bg-gray-900 p-5">
			<h3 class="mb-4 text-sm font-semibold uppercase tracking-wider text-gray-500">
				Tag Comparison
			</h3>
			<div class="mb-3 flex gap-4 text-xs text-gray-500">
				<span class="flex items-center gap-1">
					<span class="inline-block h-2 w-2 rounded bg-green-500"></span> Both agree
				</span>
				<span class="flex items-center gap-1">
					<span class="inline-block h-2 w-2 rounded bg-red-500"></span> Tag only
				</span>
				<span class="flex items-center gap-1">
					<span class="inline-block h-2 w-2 rounded bg-amber-500"></span> LLM only
				</span>
			</div>
			<div class="flex flex-wrap gap-2">
				{#each data.tagMatches as match}
					<span
						class="rounded border px-2 py-1 text-xs {matchColor(match.source)}"
					>
						{match.tag}
					</span>
				{/each}
			</div>
		</section>
	{/if}
</div>
