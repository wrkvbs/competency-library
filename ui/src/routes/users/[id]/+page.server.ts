import { loadUserProfiles, loadResponses, loadResumes, loadLatestRun } from '$lib/data.server';
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const profiles = loadUserProfiles();
	// Profiles have bare UUIDs (user: prefix stripped by loader)
	const bareId = params.id.replace('user:', '');
	const profile = profiles.find((p) => p.userId === bareId);
	if (!profile) throw error(404, 'User not found');

	const allResponses = loadResponses();
	const resumes = loadResumes();
	const latestRun = loadLatestRun();

	// Response data uses bare UUID for responderUserId, user: prefix for subjectId
	const prefixedId = `user:${bareId}`;

	// Responses where this user is the responder about themselves
	const selfResponses = allResponses.filter(
		(r) => r.responderUserId === bareId && r.subjectId === prefixedId
	);

	// References this user gave about others
	const referencesGiven = allResponses.filter(
		(r) => r.responderUserId === bareId && r.subjectId !== prefixedId
	);

	// References others gave about this user
	const referencesReceived = allResponses.filter(
		(r) => r.subjectId === prefixedId && r.responderUserId !== bareId
	);

	const userResumes = resumes.filter((r) => r.userId === bareId || r.userId === prefixedId);

	// Scored response IDs
	const scoredIds = new Set(
		latestRun?.results.filter((r) => !r.error).map((r) => r.response_id) ?? []
	);

	// Group tags by category
	const tagsByCategory: Record<string, string[]> = {};
	for (const t of profile.promptTags) {
		const cat = t.category || 'OTHER';
		(tagsByCategory[cat] ??= []).push(t.tag);
	}

	// Current affiliation
	const currentAff = profile.affiliations.find((a) => !a.endedAt);

	return {
		profile: {
			userId: profile.userId,
			fullName: profile.fullName,
			shortName: profile.shortName,
			headline: profile.headline,
			pronouns: profile.pronouns,
			slug: profile.slug,
			createdAt: profile.createdAt,
			trending: profile.trending,
			socialLinks: profile.socialLinks,
			intentFlags: profile.intentFlags
		},
		currentOrg: currentAff?.organization?.name ?? null,
		currentTitle: currentAff?.title ?? null,
		tagsByCategory,
		affiliations: profile.affiliations.map((a) => ({
			orgName: a.organization?.name ?? 'Unknown',
			orgLocation: a.organization?.location,
			title: a.title,
			startedAt: a.startedAt,
			endedAt: a.endedAt
		})),
		selfResponses: selfResponses.map((r) => ({
			id: r.id,
			promptText: r.promptText,
			transcriptPreview: r.transcript?.text?.slice(0, 120) ?? null,
			scored: scoredIds.has(r.id),
			createdAt: r.createdAt
		})),
		referencesGiven: referencesGiven.map((r) => ({
			id: r.id,
			promptText: r.promptText,
			responderName: r.responder?.fullName ?? 'Unknown',
			transcriptPreview: r.transcript?.text?.slice(0, 120) ?? null,
			createdAt: r.createdAt
		})),
		referencesReceived: referencesReceived.map((r) => ({
			id: r.id,
			promptText: r.promptText,
			responderName: r.responder?.fullName ?? 'Unknown',
			transcriptPreview: r.transcript?.text?.slice(0, 120) ?? null,
			createdAt: r.createdAt
		})),
		resumes: userResumes
	};
};
