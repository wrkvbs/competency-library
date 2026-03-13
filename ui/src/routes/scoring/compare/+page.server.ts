import { loadAllRuns } from '$lib/data.server';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const allRuns = loadAllRuns();

	// Serialize just the data needed for comparison (runs can be large)
	const runs = allRuns.map((run) => ({
		filename: run.filename,
		timestamp: run.timestamp,
		prompt_version: run.prompt_version,
		results: run.results.map((r) => ({
			response_id: r.response_id,
			responder: r.responder,
			behavioral_competencies: r.extraction?.behavioral_competencies ?? [],
			specific_skills: (r.extraction?.specific_skills ?? []).map((s) => s.name),
			error: r.error
		}))
	}));

	return { runs };
};
