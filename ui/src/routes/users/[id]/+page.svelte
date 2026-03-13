<script lang="ts">
	let { data } = $props();

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

	function formatDate(d: string | null): string {
		if (!d) return '--';
		return new Date(d).toLocaleDateString('en-US', { year: 'numeric', month: 'short' });
	}
</script>

<div class="space-y-6">
	<!-- Header -->
	<div>
		<a href="/users" class="text-sm text-gray-500 hover:text-gray-300">&larr; Back to Users</a>
		<h2 class="mt-2 text-2xl font-bold text-white">{data.profile.fullName}</h2>
		{#if data.profile.headline}
			<p class="mt-1 text-gray-400">{data.profile.headline}</p>
		{/if}
		{#if data.currentOrg}
			<p class="mt-1 text-sm text-gray-500">
				{data.currentTitle ? `${data.currentTitle} at ` : ''}{data.currentOrg}
			</p>
		{/if}
		<div class="mt-2 flex flex-wrap gap-3 text-xs text-gray-500">
			{#if data.profile.pronouns}<span>{data.profile.pronouns.subjective}/{data.profile.pronouns.objective}</span>{/if}
			<span>Joined {formatDate(data.profile.createdAt)}</span>
			{#if data.profile.trending}<span class="text-amber-400">Trending</span>{/if}
		</div>
		{#if data.profile.socialLinks.length > 0}
			<div class="mt-2 flex flex-wrap gap-2">
				{#each data.profile.socialLinks as link}
					{#if link.url}
						<a href={link.url} target="_blank" rel="noopener" class="text-xs text-blue-400 hover:text-blue-300">{link.name}</a>
					{:else if link.username}
						<span class="text-xs text-gray-500">{link.name}: {link.username}</span>
					{/if}
				{/each}
			</div>
		{/if}
	</div>

	<!-- Tags -->
	{#if Object.keys(data.tagsByCategory).length > 0}
		<section>
			<h3 class="mb-2 text-sm font-semibold uppercase tracking-wider text-gray-500">Tags</h3>
			<div class="space-y-2">
				{#each Object.entries(data.tagsByCategory) as [category, tags]}
					<div class="flex flex-wrap items-center gap-1">
						<span class="rounded px-2 py-0.5 text-xs {catBadge(category)}">{category}</span>
						{#each tags as tag}
							<span class="rounded bg-gray-800 px-2 py-0.5 text-xs text-gray-300">{tag}</span>
						{/each}
					</div>
				{/each}
			</div>
		</section>
	{/if}

	<!-- Work History -->
	{#if data.affiliations.length > 0}
		<section>
			<h3 class="mb-2 text-sm font-semibold uppercase tracking-wider text-gray-500">Work History</h3>
			<div class="space-y-2">
				{#each data.affiliations as aff}
					<div class="rounded-lg border border-gray-800 bg-gray-900 px-4 py-3">
						<div class="flex items-baseline justify-between">
							<div>
								<span class="font-medium text-gray-200">{aff.orgName}</span>
								{#if aff.title}
									<span class="text-gray-500"> / {aff.title}</span>
								{/if}
							</div>
							<span class="text-xs text-gray-500">
								{formatDate(aff.startedAt)} - {aff.endedAt ? formatDate(aff.endedAt) : 'Present'}
							</span>
						</div>
						{#if aff.orgLocation}
							<p class="text-xs text-gray-600">{aff.orgLocation}</p>
						{/if}
					</div>
				{/each}
			</div>
		</section>
	{/if}

	<!-- Self-Responses -->
	{#if data.selfResponses.length > 0}
		<section>
			<h3 class="mb-2 text-sm font-semibold uppercase tracking-wider text-gray-500">
				Self-Responses ({data.selfResponses.length})
			</h3>
			<div class="overflow-x-auto rounded-lg border border-gray-800">
				<table class="w-full text-left text-sm">
					<thead class="border-b border-gray-800 bg-gray-900 text-xs uppercase tracking-wider text-gray-500">
						<tr>
							<th class="px-4 py-3">Prompt</th>
							<th class="px-4 py-3">Preview</th>
							<th class="px-4 py-3 text-center">Scored</th>
							<th class="px-4 py-3 text-right">Date</th>
						</tr>
					</thead>
					<tbody>
						{#each data.selfResponses as resp}
							<tr class="border-b border-gray-800/50 transition-colors hover:bg-gray-900/50">
								<td class="max-w-xs truncate px-4 py-3 text-gray-300">
									{resp.promptText?.slice(0, 60) ?? '--'}
								</td>
								<td class="max-w-sm truncate px-4 py-3 text-gray-500">
									{resp.transcriptPreview ?? 'No transcript'}
								</td>
								<td class="px-4 py-3 text-center">
									{#if resp.scored}
										<a href="/transcripts/{resp.id}" class="text-green-400 hover:text-green-300">View</a>
									{:else}
										<span class="text-gray-600">--</span>
									{/if}
								</td>
								<td class="px-4 py-3 text-right text-xs text-gray-500">{formatDate(resp.createdAt)}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</section>
	{/if}

	<!-- References Given -->
	{#if data.referencesGiven.length > 0}
		<section>
			<h3 class="mb-2 text-sm font-semibold uppercase tracking-wider text-gray-500">
				References Given ({data.referencesGiven.length})
			</h3>
			<div class="overflow-x-auto rounded-lg border border-gray-800">
				<table class="w-full text-left text-sm">
					<thead class="border-b border-gray-800 bg-gray-900 text-xs uppercase tracking-wider text-gray-500">
						<tr>
							<th class="px-4 py-3">Prompt</th>
							<th class="px-4 py-3">Preview</th>
							<th class="px-4 py-3 text-right">Date</th>
						</tr>
					</thead>
					<tbody>
						{#each data.referencesGiven as ref}
							<tr class="border-b border-gray-800/50 transition-colors hover:bg-gray-900/50">
								<td class="max-w-xs truncate px-4 py-3 text-gray-300">
									{ref.promptText?.slice(0, 60) ?? '--'}
								</td>
								<td class="max-w-sm truncate px-4 py-3 text-gray-500">
									{ref.transcriptPreview ?? 'No transcript'}
								</td>
								<td class="px-4 py-3 text-right text-xs text-gray-500">{formatDate(ref.createdAt)}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</section>
	{/if}

	<!-- References Received -->
	{#if data.referencesReceived.length > 0}
		<section>
			<h3 class="mb-2 text-sm font-semibold uppercase tracking-wider text-gray-500">
				References Received ({data.referencesReceived.length})
			</h3>
			<div class="overflow-x-auto rounded-lg border border-gray-800">
				<table class="w-full text-left text-sm">
					<thead class="border-b border-gray-800 bg-gray-900 text-xs uppercase tracking-wider text-gray-500">
						<tr>
							<th class="px-4 py-3">From</th>
							<th class="px-4 py-3">Prompt</th>
							<th class="px-4 py-3">Preview</th>
							<th class="px-4 py-3 text-right">Date</th>
						</tr>
					</thead>
					<tbody>
						{#each data.referencesReceived as ref}
							<tr class="border-b border-gray-800/50 transition-colors hover:bg-gray-900/50">
								<td class="px-4 py-3 text-gray-300">{ref.responderName}</td>
								<td class="max-w-xs truncate px-4 py-3 text-gray-400">
									{ref.promptText?.slice(0, 60) ?? '--'}
								</td>
								<td class="max-w-sm truncate px-4 py-3 text-gray-500">
									{ref.transcriptPreview ?? 'No transcript'}
								</td>
								<td class="px-4 py-3 text-right text-xs text-gray-500">{formatDate(ref.createdAt)}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</section>
	{/if}

	<!-- Resumes -->
	{#if data.resumes.length > 0}
		<section>
			<h3 class="mb-2 text-sm font-semibold uppercase tracking-wider text-gray-500">
				Resumes ({data.resumes.length})
			</h3>
			<div class="space-y-2">
				{#each data.resumes as resume}
					<div class="rounded-lg border border-gray-800 bg-gray-900 px-4 py-3">
						<span class="font-mono text-xs text-gray-400">{resume.id}</span>
						{#if resume.url}
							<a href={resume.url} target="_blank" rel="noopener" class="ml-3 text-sm text-blue-400 hover:text-blue-300">
								Download
							</a>
						{/if}
					</div>
				{/each}
			</div>
		</section>
	{/if}
</div>
