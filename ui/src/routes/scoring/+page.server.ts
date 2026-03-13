import { loadAllRuns } from '$lib/data.server';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const allRuns = loadAllRuns();

	const runSummaries = allRuns.map((run) => {
		const scored = run.results.filter((r) => !r.error && r.extraction);

		// Competency frequency
		const compCounts: Record<string, number> = {};
		const allConfidences: number[] = [];
		let totalComps = 0;
		let totalSkills = 0;
		const latencies: number[] = [];

		for (const r of scored) {
			if (!r.extraction) continue;

			totalComps += r.extraction.behavioral_competencies.length;
			totalSkills += r.extraction.specific_skills.length;

			for (const c of r.extraction.behavioral_competencies) {
				compCounts[c.name] = (compCounts[c.name] || 0) + 1;
				allConfidences.push(c.confidence);
			}
			for (const s of r.extraction.specific_skills) {
				allConfidences.push(s.confidence);
			}

			if (r.llm_time_secs) latencies.push(r.llm_time_secs);
		}

		// Confidence histogram: 5 buckets
		const confBuckets = [0, 0, 0, 0, 0];
		for (const c of allConfidences) {
			const idx = Math.min(Math.floor(c * 5), 4);
			confBuckets[idx]++;
		}

		// Competency frequency sorted
		const compFreq = Object.entries(compCounts)
			.map(([name, count]) => ({ name, count }))
			.sort((a, b) => b.count - a.count);

		// Latency stats
		latencies.sort((a, b) => a - b);
		const latencyStats =
			latencies.length > 0
				? {
						min: latencies[0],
						max: latencies[latencies.length - 1],
						avg: latencies.reduce((a, b) => a + b, 0) / latencies.length,
						p50: latencies[Math.floor(latencies.length / 2)]
					}
				: null;

		return {
			filename: run.filename,
			timestamp: run.timestamp,
			prompt_version: run.prompt_version,
			totalResults: run.results.length,
			scoredCount: scored.length,
			errorCount: run.results.filter((r) => r.error).length,
			avgCompsPerTranscript: scored.length > 0 ? totalComps / scored.length : 0,
			avgSkillsPerTranscript: scored.length > 0 ? totalSkills / scored.length : 0,
			confBuckets,
			compFreq,
			latencyStats
		};
	});

	return { runSummaries };
};
