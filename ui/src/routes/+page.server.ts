import {
	loadTranscripts,
	loadLatestRun,
	loadAllRuns,
	loadOnetAbilities,
	loadOnetSkills,
	loadOnetKnowledge,
	loadOnetWorkStyles,
	loadOnetWorkActivities,
	loadOpmCompetencies,
	loadWorkbankTasks,
	loadPromptTags,
	loadAllUpPrompts,
	loadUserProfiles,
	loadResponses,
	loadResumes
} from '$lib/data.server';
import type { PageServerLoad } from './$types';

const BEHAVIORAL_COMPETENCIES = [
	'problem_solving',
	'communication',
	'leadership',
	'collaboration',
	'initiative',
	'adaptability',
	'analytical_thinking',
	'customer_focus',
	'attention_to_detail',
	'strategic_thinking'
] as const;

export const load: PageServerLoad = async () => {
	const transcripts = loadTranscripts();
	const latestRun = loadLatestRun();
	const allRuns = loadAllRuns();

	const onetAbilities = loadOnetAbilities();
	const onetSkills = loadOnetSkills();
	const onetKnowledge = loadOnetKnowledge();
	const onetWorkStyles = loadOnetWorkStyles();
	const onetWorkActivities = loadOnetWorkActivities();
	const opmCompetencies = loadOpmCompetencies();
	const workbankTasks = loadWorkbankTasks();

	const promptTags = loadPromptTags();
	const allUpPrompts = loadAllUpPrompts();
	const userProfiles = loadUserProfiles();
	const responses = loadResponses();
	const resumes = loadResumes();

	// Build heatmap data from latest run
	const heatmapRows: Array<{
		responseId: string;
		responder: string;
		scores: Record<string, number | null>;
	}> = [];

	const compStats: Record<string, { count: number; total: number }> = {};
	for (const name of BEHAVIORAL_COMPETENCIES) {
		compStats[name] = { count: 0, total: 0 };
	}

	if (latestRun) {
		for (const result of latestRun.results) {
			if (result.error || !result.extraction) continue;

			const scores: Record<string, number | null> = {};
			for (const name of BEHAVIORAL_COMPETENCIES) {
				scores[name] = null;
			}

			for (const comp of result.extraction.behavioral_competencies) {
				if (comp.name in scores) {
					scores[comp.name] = comp.score;
					compStats[comp.name].count++;
					compStats[comp.name].total += comp.score;
				}
			}

			heatmapRows.push({
				responseId: result.response_id,
				responder: result.responder,
				scores
			});
		}
	}

	const compAverages: Record<string, number | null> = {};
	for (const name of BEHAVIORAL_COMPETENCIES) {
		const s = compStats[name];
		compAverages[name] = s.count > 0 ? s.total / s.count : null;
	}

	return {
		counts: {
			transcripts: transcripts.length,
			transcriptsWithText: transcripts.filter((t) => t.transcript?.text).length,
			scoringRuns: allRuns.length,
			scoredResponses: latestRun?.results.filter((r) => !r.error).length ?? 0,
			onetAbilities: onetAbilities.length,
			onetSkills: onetSkills.length,
			onetKnowledge: onetKnowledge.length,
			onetWorkStyles: onetWorkStyles.length,
			onetWorkActivities: onetWorkActivities.length,
			opmCompetencies: opmCompetencies.length,
			workbankTasks: workbankTasks.length,
			promptTags: promptTags.length,
			allUpPrompts: allUpPrompts.length,
			userProfiles: userProfiles.length,
			responses: responses.length,
			resumes: resumes.length
		},
		latestRun: latestRun
			? {
					filename: latestRun.filename,
					timestamp: latestRun.timestamp,
					prompt_version: latestRun.prompt_version,
					resultCount: latestRun.results.length,
					errorCount: latestRun.results.filter((r) => r.error).length
				}
			: null,
		behavioralCompetencies: [...BEHAVIORAL_COMPETENCIES],
		heatmapRows,
		compAverages
	};
};
