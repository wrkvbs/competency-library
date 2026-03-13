import { loadLatestRun, loadOnetTechSkills, loadTranscripts } from '$lib/data.server';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const latestRun = loadLatestRun();
	const techSkills = loadOnetTechSkills();
	const transcripts = loadTranscripts();

	// Build O*NET tech skill name lookup (lowercase)
	const onetSkillNames = new Set(techSkills.map((s) => s.name.toLowerCase()));

	// Build production tag lookup (lowercase)
	const allTags = new Set<string>();
	for (const t of transcripts) {
		for (const tag of t.tags) {
			allTags.add(tag.tag.toLowerCase());
		}
	}

	// Aggregate skills across all scored transcripts
	const skillAgg: Record<
		string,
		{
			name: string;
			category: string;
			count: number;
			totalDepth: number;
			totalConfidence: number;
		}
	> = {};

	if (latestRun) {
		for (const r of latestRun.results) {
			if (r.error || !r.extraction) continue;
			for (const s of r.extraction.specific_skills) {
				const key = s.name.toLowerCase();
				if (!skillAgg[key]) {
					skillAgg[key] = {
						name: s.name,
						category: s.category,
						count: 0,
						totalDepth: 0,
						totalConfidence: 0
					};
				}
				skillAgg[key].count++;
				skillAgg[key].totalDepth += s.depth_score;
				skillAgg[key].totalConfidence += s.confidence;
			}
		}
	}

	const skillRows = Object.values(skillAgg)
		.map((s) => ({
			name: s.name,
			category: s.category,
			count: s.count,
			avgDepth: s.totalDepth / s.count,
			avgConfidence: s.totalConfidence / s.count,
			inOnet: onetSkillNames.has(s.name.toLowerCase()),
			inTags: allTags.has(s.name.toLowerCase())
		}))
		.sort((a, b) => b.count - a.count);

	return {
		skillRows,
		runFilename: latestRun?.filename ?? null
	};
};
