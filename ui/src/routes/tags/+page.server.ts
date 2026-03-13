import { loadPromptTags } from '$lib/data.server';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const tags = loadPromptTags();

	const byCategory: Record<string, typeof tags> = {};
	for (const tag of tags) {
		const cat = tag.category || 'UNCATEGORIZED';
		(byCategory[cat] ??= []).push(tag);
	}

	// Sort categories, sort tags within each by promptCount desc
	const categories = Object.keys(byCategory).sort();
	for (const cat of categories) {
		byCategory[cat].sort((a, b) => b.promptCount - a.promptCount);
	}

	return { tags, byCategory, categories };
};
