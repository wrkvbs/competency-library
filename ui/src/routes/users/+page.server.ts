import { loadUserProfiles, loadResponses, loadResumes, loadLatestRun } from '$lib/data.server';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ url }) => {
	const profiles = loadUserProfiles();
	const responses = loadResponses();
	const resumes = loadResumes();
	const latestRun = loadLatestRun();

	// Build count maps
	const responseCounts = new Map<string, number>();
	for (const r of responses) {
		responseCounts.set(r.responderUserId, (responseCounts.get(r.responderUserId) ?? 0) + 1);
	}

	const resumeCounts = new Map<string, number>();
	for (const r of resumes) {
		// Normalize to bare UUID (profiles have bare UUIDs)
		const key = r.userId.replace('user:', '');
		resumeCounts.set(key, (resumeCounts.get(key) ?? 0) + 1);
	}

	const scoredUserIds = new Set<string>();
	if (latestRun) {
		for (const r of latestRun.results) {
			if (!r.error) {
				const resp = responses.find((resp) => resp.id === r.response_id);
				if (resp) scoredUserIds.add(resp.responderUserId);
			}
		}
	}

	const rows = profiles.map((p) => {
		const currentAff = p.affiliations.find((a) => !a.endedAt);
		return {
			userId: p.userId,
			fullName: p.fullName,
			headline: p.headline,
			currentOrg: currentAff?.organization?.name ?? null,
			currentTitle: currentAff?.title ?? null,
			tagCount: p.promptTags.length,
			responseCount: responseCounts.get(p.userId) ?? 0,
			resumeCount: resumeCounts.get(p.userId) ?? 0,
			scored: scoredUserIds.has(p.userId),
			createdAt: p.createdAt
		};
	});

	// Sort by response count desc by default
	rows.sort((a, b) => b.responseCount - a.responseCount);

	// Pagination
	const pageNum = Math.max(1, parseInt(url.searchParams.get('page') ?? '1'));
	const perPage = 50;
	const totalPages = Math.ceil(rows.length / perPage);
	const paged = rows.slice((pageNum - 1) * perPage, pageNum * perPage);

	return {
		rows: paged,
		total: rows.length,
		page: pageNum,
		totalPages,
		perPage
	};
};
