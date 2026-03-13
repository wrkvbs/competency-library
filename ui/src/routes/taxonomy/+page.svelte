<script lang="ts">
	let { data } = $props();

	type Tab = 'content-model' | 'tech-skills' | 'opm' | 'esco';
	let activeTab = $state<Tab>('content-model');

	// ---------- Content Model tree state ----------
	let expandedNodes = $state<Record<string, boolean>>({});
	let selectedNode = $state<{
		element_id: string;
		name: string;
		description: string;
		scale_anchors?: Array<{ level: number; description: string }>;
	} | null>(null);

	function toggleNode(id: string) {
		expandedNodes[id] = !expandedNodes[id];
	}

	// ---------- Tech Skills state ----------
	let techSearch = $state('');
	let techHotOnly = $state(false);
	let techDemandOnly = $state(false);
	let techExpandedNodes = $state<Record<string, boolean>>({});
	let selectedTechProduct = $state<string | null>(null);

	type TechTreeNode = typeof data.techSkillTree extends (infer T)[] ? T : never;

	function filterTechTree(nodes: TechTreeNode[]): TechTreeNode[] {
		const q = techSearch.toLowerCase();
		const hasFilter = q || techHotOnly || techDemandOnly;
		if (!hasFilter) return nodes;

		return nodes
			.map((node) => {
				if (node.type === 'product') {
					const p = node.product!;
					const matchesSearch =
						!q ||
						p.name.toLowerCase().includes(q) ||
						p.commodity_title.toLowerCase().includes(q) ||
						p.class_title.toLowerCase().includes(q);
					const matchesHot = !techHotOnly || p.hot_technology;
					const matchesDemand = !techDemandOnly || p.in_demand;
					return matchesSearch && matchesHot && matchesDemand ? node : null;
				}
				const filteredChildren = filterTechTree(node.children);
				if (filteredChildren.length === 0) return null;
				return {
					...node,
					children: filteredChildren,
					count: filteredChildren.reduce((s, c) => s + c.count, 0),
					hot_count: filteredChildren.reduce((s, c) => s + c.hot_count, 0),
					demand_count: filteredChildren.reduce((s, c) => s + c.demand_count, 0)
				};
			})
			.filter((n): n is TechTreeNode => n !== null);
	}

	let techTreeFiltered = $derived(filterTechTree(data.techSkillTree));
	let techFilteredCount = $derived(techTreeFiltered.reduce((s, c) => s + c.count, 0));

	// Auto-expand when filtering
	$effect(() => {
		const hasFilter = techSearch || techHotOnly || techDemandOnly;
		if (hasFilter) {
			const expanded: Record<string, boolean> = {};
			function expandAll(nodes: TechTreeNode[]) {
				for (const n of nodes) {
					if (n.children.length > 0) {
						expanded[n.id] = true;
						expandAll(n.children);
					}
				}
			}
			expandAll(techTreeFiltered);
			techExpandedNodes = expanded;
		}
	});

	let selectedTechProductData = $derived.by(() => {
		if (!selectedTechProduct) return null;
		function find(nodes: TechTreeNode[]): TechTreeNode | null {
			for (const n of nodes) {
				if (n.id === selectedTechProduct) return n;
				const found = find(n.children);
				if (found) return found;
			}
			return null;
		}
		return find(data.techSkillTree);
	});

	// ---------- OPM state ----------
	let opmSearch = $state('');
	let selectedOpm = $state<string | null>(null);

	let filteredOpm = $derived.by(() => {
		if (!opmSearch) return data.opmCompetencies;
		const q = opmSearch.toLowerCase();
		return data.opmCompetencies.filter(
			(c) => c.name.toLowerCase().includes(q) || c.definition.toLowerCase().includes(q)
		);
	});

	let selectedOpmComp = $derived(
		selectedOpm ? data.opmCompetencies.find((c) => c.name === selectedOpm) ?? null : null
	);

	// ---------- ESCO state ----------
	const PAGE_SIZE = 200;
	let escoSearch = $state('');
	let escoType = $state('');
	let escoExpandedNodes = $state<Record<string, boolean>>({});
	let selectedEscoSkill = $state<string | null>(null);

	type EscoTreeNode = typeof data.escoTree extends (infer T)[] ? T : never;

	// Whether we're in search/filter mode (flat list) vs tree browse mode
	let escoHasFilter = $derived(!!escoSearch || !!escoType);

	// Flat search results (used when filter is active)
	let escoFlatFiltered = $derived.by(() => {
		if (!escoHasFilter) return [];
		let skills = data.escoSkills;
		if (escoType) skills = skills.filter((s) => s.skill_type === escoType);
		if (escoSearch) {
			const q = escoSearch.toLowerCase();
			skills = skills.filter(
				(s) =>
					s.name.toLowerCase().includes(q) ||
					s.alt_labels?.some((l) => l.toLowerCase().includes(q))
			);
		}
		return skills;
	});

	let escoFlatCount = $derived(escoFlatFiltered.length);
	let escoFlatCapped = $derived(escoFlatFiltered.slice(0, PAGE_SIZE));

	let selectedEscoSkillData = $derived.by(() => {
		if (!selectedEscoSkill) return null;
		// Search in flat data directly
		return data.escoSkills.find((s) => s.uri === selectedEscoSkill) ?? null;
	});

	// ---------- Tabs ----------
	let tabs = $derived<Array<{ id: Tab; label: string; count: number }>>([
		{ id: 'content-model', label: 'O*NET Content Model', count: 0 },
		{ id: 'tech-skills', label: 'O*NET Tech Skills', count: data.techSkillCount },
		{ id: 'opm', label: 'OPM Competencies', count: data.opmCompetencies.length },
		{ id: 'esco', label: 'ESCO Skills', count: data.escoSkillCount }
	]);
</script>

<div class="space-y-4">
	<div>
		<h2 class="text-2xl font-bold text-white">Taxonomy Browser</h2>
		<p class="mt-1 text-sm text-gray-400">Explore competency taxonomies from O*NET, OPM, and ESCO</p>
	</div>

	<!-- Tabs -->
	<div class="flex gap-1 border-b border-gray-800">
		{#each tabs as tab}
			<button
				onclick={() => (activeTab = tab.id)}
				class="px-4 py-2 text-sm font-medium transition-colors
					{activeTab === tab.id
					? 'border-b-2 border-blue-500 text-white'
					: 'text-gray-500 hover:text-gray-300'}"
			>
				{tab.label}
				{#if tab.count > 0}
					<span class="ml-1 text-xs text-gray-600">{tab.count.toLocaleString()}</span>
				{/if}
			</button>
		{/each}
	</div>

	<!-- Tab: Content Model -->
	{#if activeTab === 'content-model'}
		<div class="flex gap-4">
			<!-- Tree -->
			<div class="flex-1 space-y-0.5">
				{#snippet treeNode(nodes: typeof data.contentTree, depth: number)}
					{#each nodes as node}
						<div style="padding-left: {depth * 16}px">
							<button
								onclick={() => {
									if (node.children.length > 0) toggleNode(node.element_id);
									selectedNode = node;
								}}
								class="flex w-full items-center gap-2 rounded px-2 py-1 text-left text-sm hover:bg-gray-800/50
									{selectedNode?.element_id === node.element_id ? 'bg-gray-800 text-white' : 'text-gray-300'}"
							>
								{#if node.children.length > 0}
									<span class="w-4 text-center text-xs text-gray-500">
										{expandedNodes[node.element_id] ? '\u25BC' : '\u25B6'}
									</span>
								{:else}
									<span class="w-4"></span>
								{/if}
								<span class="font-mono text-xs text-gray-600">{node.element_id}</span>
								<span>{node.name}</span>
							</button>
							{#if expandedNodes[node.element_id] && node.children.length > 0}
								{@render treeNode(node.children, depth + 1)}
							{/if}
						</div>
					{/each}
				{/snippet}

				{@render treeNode(data.contentTree, 0)}
			</div>

			<!-- Detail Panel -->
			<div class="w-80 shrink-0 rounded-lg border border-gray-800 bg-gray-900 p-4">
				{#if selectedNode}
					<h4 class="text-sm font-semibold text-white">{selectedNode.name}</h4>
					<p class="mt-1 font-mono text-xs text-gray-500">{selectedNode.element_id}</p>
					<p class="mt-3 text-sm leading-relaxed text-gray-300">{selectedNode.description}</p>

					{#if selectedNode.scale_anchors?.length}
						<div class="mt-4">
							<h5 class="mb-2 text-xs font-semibold uppercase text-gray-500">Scale Anchors</h5>
							<table class="w-full text-xs">
								<thead>
									<tr class="text-gray-500">
										<th class="py-1 pr-3 text-left">Level</th>
										<th class="py-1 text-left">Description</th>
									</tr>
								</thead>
								<tbody>
									{#each selectedNode.scale_anchors as anchor}
										<tr class="border-t border-gray-800">
											<td class="py-1.5 pr-3 font-mono text-gray-400">{anchor.level}</td>
											<td class="py-1.5 text-gray-300">{anchor.description}</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					{/if}
				{:else}
					<p class="text-sm text-gray-500">Select a node to view details.</p>
				{/if}
			</div>
		</div>
	{/if}

	<!-- Tab: Tech Skills -->
	{#if activeTab === 'tech-skills'}
		{#if data.techSkillCount === 0}
			<p class="text-sm text-gray-500">Tech skills not yet processed. Run <code class="text-gray-400">cargo run -- process onet</code> to generate.</p>
		{:else}
			<div class="flex flex-wrap items-center gap-3">
				<input
					type="text"
					placeholder="Search name, commodity, class..."
					bind:value={techSearch}
					class="rounded-md border border-gray-700 bg-gray-900 px-3 py-1.5 text-sm text-gray-200 placeholder-gray-600 outline-none focus:border-gray-500"
				/>
				<label class="flex items-center gap-1.5 text-sm text-gray-400">
					<input type="checkbox" bind:checked={techHotOnly} class="accent-orange-500" />
					Hot Technology
				</label>
				<label class="flex items-center gap-1.5 text-sm text-gray-400">
					<input type="checkbox" bind:checked={techDemandOnly} class="accent-green-500" />
					In Demand
				</label>
				<span class="text-xs text-gray-500">
					{techFilteredCount.toLocaleString()} results
				</span>
			</div>

			<div class="flex gap-4">
				<!-- Tree -->
				<div class="flex-1 space-y-0.5 overflow-y-auto" style="max-height: 70vh">
					{#snippet techTreeNode(nodes: TechTreeNode[], depth: number)}
						{#each nodes as node}
							<div style="padding-left: {depth * 16}px">
								{#if node.type === 'product'}
									<button
										onclick={() => (selectedTechProduct = node.id)}
										class="flex w-full items-center gap-2 rounded px-2 py-1 text-left text-sm hover:bg-gray-800/50
											{selectedTechProduct === node.id ? 'bg-gray-800 text-white' : 'text-gray-300'}"
									>
										<span class="w-4"></span>
										<span class="truncate">{node.label}</span>
										{#if node.product?.hot_technology}
											<span class="shrink-0 rounded bg-orange-900/50 px-1.5 py-0.5 text-xs text-orange-400">Hot</span>
										{/if}
										{#if node.product?.in_demand}
											<span class="shrink-0 rounded bg-green-900/50 px-1.5 py-0.5 text-xs text-green-400">In-demand</span>
										{/if}
									</button>
								{:else}
									<button
										onclick={() => (techExpandedNodes[node.id] = !techExpandedNodes[node.id])}
										class="flex w-full items-center gap-2 rounded px-2 py-1 text-left text-sm font-medium text-gray-200 hover:bg-gray-800/50"
									>
										<span class="w-4 text-center text-xs text-gray-500">
											{techExpandedNodes[node.id] ? '\u25BC' : '\u25B6'}
										</span>
										<span class="truncate">{node.label}</span>
										<span class="shrink-0 text-xs text-gray-600">{node.count.toLocaleString()}</span>
										{#if node.hot_count > 0}
											<span class="shrink-0 rounded bg-orange-900/30 px-1 py-0.5 text-xs text-orange-500">{node.hot_count}</span>
										{/if}
									</button>
									{#if techExpandedNodes[node.id]}
										{@render techTreeNode(node.children, depth + 1)}
									{/if}
								{/if}
							</div>
						{/each}
					{/snippet}

					{@render techTreeNode(techTreeFiltered, 0)}
				</div>

				<!-- Detail Panel -->
				<div class="w-80 shrink-0 rounded-lg border border-gray-800 bg-gray-900 p-4">
					{#if selectedTechProductData?.product}
						{@const p = selectedTechProductData.product}
						<h4 class="text-sm font-semibold text-white">{p.name}</h4>

						<div class="mt-2 flex flex-wrap gap-1.5">
							{#if p.hot_technology}
								<span class="rounded bg-orange-900/50 px-1.5 py-0.5 text-xs text-orange-400">Hot Technology</span>
							{/if}
							{#if p.in_demand}
								<span class="rounded bg-green-900/50 px-1.5 py-0.5 text-xs text-green-400">In Demand</span>
							{/if}
						</div>

						<div class="mt-3 space-y-2 text-xs">
							<div>
								<span class="text-gray-500">Commodity:</span>
								<span class="ml-1 text-gray-300">{p.commodity_title}</span>
								<span class="ml-1 font-mono text-gray-600">{p.commodity_code}</span>
							</div>
							<div>
								<span class="text-gray-500">Class:</span>
								<span class="ml-1 text-gray-300">{p.class_title}</span>
							</div>
							<div>
								<span class="text-gray-500">Family:</span>
								<span class="ml-1 text-gray-300">{p.family_title}</span>
							</div>
							<div>
								<span class="text-gray-500">Segment:</span>
								<span class="ml-1 text-gray-300">{p.segment_title}</span>
							</div>
							<div>
								<span class="text-gray-500">Occupations:</span>
								<span class="ml-1 font-mono text-gray-300">{p.occupation_count}</span>
							</div>
						</div>
					{:else}
						<p class="text-sm text-gray-500">Select a product to view details.</p>
					{/if}
				</div>
			</div>
		{/if}
	{/if}

	<!-- Tab: OPM -->
	{#if activeTab === 'opm'}
		{#if data.opmCompetencies.length === 0}
			<p class="text-sm text-gray-500">OPM competencies not yet processed.</p>
		{:else}
			<div class="flex gap-3">
				<input
					type="text"
					placeholder="Search competencies..."
					bind:value={opmSearch}
					class="rounded-md border border-gray-700 bg-gray-900 px-3 py-1.5 text-sm text-gray-200 placeholder-gray-600 outline-none focus:border-gray-500"
				/>
				<span class="self-center text-xs text-gray-500">
					{filteredOpm.length} results
				</span>
			</div>

			<div class="flex gap-4">
				<!-- Scrollable list -->
				<div class="flex-1 space-y-0.5 overflow-y-auto" style="max-height: 70vh">
					{#each filteredOpm as comp}
						<button
							onclick={() => (selectedOpm = comp.name)}
							class="flex w-full items-baseline gap-2 rounded px-2 py-1.5 text-left text-sm hover:bg-gray-800/50
								{selectedOpm === comp.name ? 'bg-gray-800 text-white' : 'text-gray-300'}"
						>
							<span class="truncate">{comp.name}</span>
							<span class="shrink-0 rounded bg-gray-800 px-1.5 py-0.5 text-xs text-gray-500">{comp.category}</span>
						</button>
					{/each}
				</div>

				<!-- Detail panel -->
				<div class="w-80 shrink-0 rounded-lg border border-gray-800 bg-gray-900 p-4">
					{#if selectedOpmComp}
						<h4 class="text-sm font-semibold text-white">{selectedOpmComp.name}</h4>
						<span class="mt-1 inline-block rounded bg-gray-800 px-1.5 py-0.5 text-xs text-gray-500">{selectedOpmComp.category}</span>
						<p class="mt-3 text-sm leading-relaxed text-gray-300">{selectedOpmComp.definition}</p>
						<p class="mt-3 text-xs text-gray-500">Source: {selectedOpmComp.source}</p>
					{:else}
						<p class="text-sm text-gray-500">Select a competency to view details.</p>
					{/if}
				</div>
			</div>
		{/if}
	{/if}

	<!-- Tab: ESCO -->
	{#if activeTab === 'esco'}
		{#if data.escoSkillCount === 0}
			<p class="text-sm text-gray-500">ESCO skills not yet downloaded/processed. Run <code class="text-gray-400">cargo run -- process esco</code> to generate.</p>
		{:else}
			<div class="flex flex-wrap items-center gap-3">
				<input
					type="text"
					placeholder="Search ESCO skills..."
					bind:value={escoSearch}
					class="rounded-md border border-gray-700 bg-gray-900 px-3 py-1.5 text-sm text-gray-200 placeholder-gray-600 outline-none focus:border-gray-500"
				/>
				<select
					bind:value={escoType}
					class="rounded-md border border-gray-700 bg-gray-900 px-3 py-1.5 text-sm text-gray-200 outline-none focus:border-gray-500"
				>
					<option value="">All types</option>
					<option value="skill">Skills</option>
					<option value="knowledge">Knowledge</option>
				</select>
				{#if escoHasFilter}
					<span class="text-xs text-gray-500">
						{escoFlatCount.toLocaleString()} results
						{#if escoFlatCount > PAGE_SIZE}
							<span class="text-gray-600">(showing first {PAGE_SIZE} — refine your search)</span>
						{/if}
					</span>
				{/if}
			</div>

			<div class="flex gap-4">
				<!-- Tree or Flat results -->
				<div class="flex-1 space-y-0.5 overflow-y-auto" style="max-height: 70vh">
					{#if escoHasFilter}
						<!-- Flat search results -->
						{#each escoFlatCapped as skill}
							<button
								onclick={() => (selectedEscoSkill = skill.uri)}
								class="flex w-full items-center gap-2 rounded px-2 py-1 text-left text-sm hover:bg-gray-800/50
									{selectedEscoSkill === skill.uri ? 'bg-gray-800 text-white' : 'text-gray-300'}"
							>
								<span class="w-4"></span>
								<span class="truncate">{skill.name}</span>
								<span class="shrink-0 rounded px-1.5 py-0.5 text-xs {skill.skill_type === 'knowledge' ? 'bg-purple-900/50 text-purple-400' : 'bg-blue-900/50 text-blue-400'}">
									{skill.skill_type}
								</span>
								{#if skill.alt_labels && skill.alt_labels.length > 0}
									<span class="truncate text-xs text-gray-600">{skill.alt_labels.slice(0, 2).join(', ')}</span>
								{/if}
							</button>
						{/each}
					{:else}
						<!-- Tree browse mode -->
						{#snippet escoTreeNode(nodes: EscoTreeNode[], depth: number)}
							{#each nodes as node}
								<div style="padding-left: {depth * 16}px">
									{#if node.type === 'skill'}
										<button
											onclick={() => (selectedEscoSkill = node.skill?.uri ?? null)}
											class="flex w-full items-center gap-2 rounded px-2 py-1 text-left text-sm hover:bg-gray-800/50
												{selectedEscoSkill === node.skill?.uri ? 'bg-gray-800 text-white' : 'text-gray-300'}"
										>
											<span class="w-4"></span>
											<span class="truncate">{node.label}</span>
											{#if node.skill}
												<span class="shrink-0 rounded px-1.5 py-0.5 text-xs {node.skill.skill_type === 'knowledge' ? 'bg-purple-900/50 text-purple-400' : 'bg-blue-900/50 text-blue-400'}">
													{node.skill.skill_type}
												</span>
											{/if}
										</button>
									{:else}
										<button
											onclick={() => (escoExpandedNodes[node.id] = !escoExpandedNodes[node.id])}
											class="flex w-full items-center gap-2 rounded px-2 py-1 text-left text-sm font-medium text-gray-200 hover:bg-gray-800/50"
										>
											<span class="w-4 text-center text-xs text-gray-500">
												{escoExpandedNodes[node.id] ? '\u25BC' : '\u25B6'}
											</span>
											<span class="truncate">{node.label}</span>
											<span class="shrink-0 text-xs text-gray-600">{node.count.toLocaleString()}</span>
										</button>
										{#if escoExpandedNodes[node.id]}
											{@render escoTreeNode(node.children, depth + 1)}
										{/if}
									{/if}
								</div>
							{/each}
						{/snippet}

						{@render escoTreeNode(data.escoTree, 0)}
					{/if}
				</div>

				<!-- Detail Panel -->
				<div class="w-96 shrink-0 rounded-lg border border-gray-800 bg-gray-900 p-4">
					{#if selectedEscoSkillData}
						{@const s = selectedEscoSkillData}
						<h4 class="text-sm font-semibold text-white">{s.name}</h4>
						<span class="mt-1 inline-block rounded px-1.5 py-0.5 text-xs {s.skill_type === 'knowledge' ? 'bg-purple-900/50 text-purple-400' : 'bg-blue-900/50 text-blue-400'}">
							{s.skill_type}
						</span>
						<p class="mt-3 text-sm leading-relaxed text-gray-300">{s.description}</p>

						{#if s.alt_labels && s.alt_labels.length > 0}
							<div class="mt-3">
								<h5 class="mb-1.5 text-xs font-semibold uppercase text-gray-500">Alternative Labels</h5>
								<div class="flex max-h-48 flex-wrap gap-1 overflow-y-auto">
									{#each s.alt_labels as label}
										<span class="rounded bg-gray-800 px-1.5 py-0.5 text-xs text-gray-400">{label}</span>
									{/each}
								</div>
							</div>
						{/if}

						<p class="mt-3 break-all text-xs text-gray-600">{s.uri}</p>
					{:else}
						<p class="text-sm text-gray-500">Select a skill to view details.</p>
					{/if}
				</div>
			</div>
		{/if}
	{/if}
</div>
