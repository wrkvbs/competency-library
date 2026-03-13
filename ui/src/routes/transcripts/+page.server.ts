import { loadTranscripts, loadLatestRun } from '$lib/data.server';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const transcripts = loadTranscripts();
	const latestRun = loadLatestRun();

	const scoredIds = new Set(
		latestRun?.results.filter((r) => !r.error).map((r) => r.response_id) ?? []
	);

	const rows = transcripts.map((t) => ({
		id: t.id,
		responder: t.responder.fullName,
		promptText: (t.prompt.selfText || t.prompt.otherUserText || 'Unknown').slice(0, 60),
		transcriptLength: t.transcript?.text?.length ?? 0,
		tagCount: t.tags.length,
		scored: scoredIds.has(t.id)
	}));

	return { rows };
};
