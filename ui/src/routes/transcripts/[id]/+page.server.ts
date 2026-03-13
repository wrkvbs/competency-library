import { error } from '@sveltejs/kit';
import { loadTranscripts, loadLatestRun } from '$lib/data.server';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params }) => {
	const transcripts = loadTranscripts();
	const transcript = transcripts.find((t) => t.id === params.id);
	if (!transcript) throw error(404, 'Transcript not found');

	const latestRun = loadLatestRun();
	const scoringResult = latestRun?.results.find((r) => r.response_id === params.id) ?? null;

	const isSelf = transcript.subjectId === `user:${transcript.responderUserId}`;

	// Tag matching: compare production tags to extracted skills
	const existingTags = transcript.tags.map((t) => t.tag);
	const extractedSkills =
		scoringResult?.extraction?.specific_skills.map((s) => s.name) ?? [];

	const tagMatches: Array<{
		tag: string;
		source: 'both' | 'tag_only' | 'skill_only';
	}> = [];

	const matchedSkills = new Set<string>();
	for (const tag of existingTags) {
		const match = extractedSkills.find(
			(skill) =>
				skill.toLowerCase().includes(tag.toLowerCase()) ||
				tag.toLowerCase().includes(skill.toLowerCase())
		);
		if (match) {
			tagMatches.push({ tag: `${tag} / ${match}`, source: 'both' });
			matchedSkills.add(match);
		} else {
			tagMatches.push({ tag, source: 'tag_only' });
		}
	}
	for (const skill of extractedSkills) {
		if (!matchedSkills.has(skill)) {
			tagMatches.push({ tag: skill, source: 'skill_only' });
		}
	}

	return {
		transcript: {
			id: transcript.id,
			responder: transcript.responder.fullName,
			promptText: transcript.prompt.selfText || transcript.prompt.otherUserText || 'Unknown',
			text: transcript.transcript?.text ?? null,
			tags: transcript.tags,
			isSelf
		},
		scoringResult: scoringResult
			? {
					prompt_version: scoringResult.prompt_version,
					llm_time_secs: scoringResult.llm_time_secs,
					extraction: scoringResult.extraction ?? null
				}
			: null,
		tagMatches
	};
};
