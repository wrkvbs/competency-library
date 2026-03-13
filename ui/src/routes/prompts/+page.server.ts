import { loadAllUpPrompts, loadPromptTags } from '$lib/data.server';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const prompts = loadAllUpPrompts();
	const tags = loadPromptTags();

	const tagMap = new Map(tags.map((t) => [t.id, t.tag]));

	const rows = prompts.map((p) => ({
		id: p.id,
		selfText: p.selfText,
		otherUserText: p.otherUserText,
		subjectType: p.subjectType,
		promptType: p.promptType,
		tags: p.tagIds.map((id) => tagMap.get(id) ?? id),
		topicTags: p.topicTagIds.map((id) => tagMap.get(id) ?? id),
		relationshipTypes: p.relationshipTypes,
		suggested: p.suggested,
		purpose: p.purpose,
		order: p.order,
		priority: p.priority
	}));

	return { rows, total: prompts.length };
};
