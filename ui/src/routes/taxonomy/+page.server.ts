import {
	loadOnetContentModel,
	loadOnetAbilities,
	loadOnetSkills,
	loadOnetKnowledge,
	loadOnetWorkStyles,
	loadOnetWorkActivities,
	loadOnetTechSkills,
	loadOpmCompetencies,
	loadEscoSkills,
	loadEscoSkillGroups
} from '$lib/data.server';
import type { PageServerLoad } from './$types';

interface ContentModelNode {
	element_id: string;
	name: string;
	description: string;
	scale_anchors?: Array<{ level: number; description: string }>;
	children: ContentModelNode[];
}

interface TechSkillTreeNode {
	id: string;
	label: string;
	type: 'class' | 'commodity' | 'product';
	count: number;
	hot_count: number;
	demand_count: number;
	children: TechSkillTreeNode[];
	product?: {
		name: string;
		commodity_code: string;
		commodity_title: string;
		class_title: string;
		family_title: string;
		segment_title: string;
		hot_technology: boolean;
		in_demand: boolean;
		occupation_count: number;
	};
}

function buildContentTree(): ContentModelNode[] {
	const contentModel = loadOnetContentModel();
	const abilities = loadOnetAbilities();
	const skills = loadOnetSkills();
	const knowledge = loadOnetKnowledge();
	const workStyles = loadOnetWorkStyles();
	const workActivities = loadOnetWorkActivities();

	// Merge all items by element_id
	const items = new Map<string, ContentModelNode>();

	for (const item of contentModel) {
		items.set(item.element_id, { ...item, children: [] });
	}
	// Add detailed items (abilities with scale_anchors, etc.)
	for (const a of abilities) {
		const existing = items.get(a.element_id);
		if (existing) {
			existing.scale_anchors = a.scale_anchors;
		} else {
			items.set(a.element_id, { ...a, children: [] });
		}
	}
	for (const list of [skills, knowledge, workStyles, workActivities]) {
		for (const item of list) {
			if (!items.has(item.element_id)) {
				items.set(item.element_id, { ...item, children: [] });
			}
		}
	}

	// Build tree by parent relationships using element_id dot-notation
	const roots: ContentModelNode[] = [];
	for (const [id, node] of items) {
		const parentId = id.substring(0, id.lastIndexOf('.'));
		const parent = items.get(parentId);
		if (parent) {
			parent.children.push(node);
		} else {
			roots.push(node);
		}
	}

	// Sort children by element_id
	function sortTree(nodes: ContentModelNode[]) {
		nodes.sort((a, b) => a.element_id.localeCompare(b.element_id, undefined, { numeric: true }));
		for (const n of nodes) sortTree(n.children);
	}
	sortTree(roots);

	return roots;
}

interface EscoTreeNode {
	id: string;
	label: string;
	type: 'group' | 'skill';
	level: number;
	count: number;
	children: EscoTreeNode[];
	skill?: {
		uri: string;
		name: string;
		description: string;
		skill_type: string;
		alt_labels: string[];
	};
}

function buildEscoTree(): EscoTreeNode[] {
	const skillGroups = loadEscoSkillGroups();
	const skills = loadEscoSkills();

	// Build group nodes map
	const groupNodes = new Map<string, EscoTreeNode>();
	for (const g of skillGroups) {
		groupNodes.set(g.uri, {
			id: `group:${g.uri}`,
			label: g.name,
			type: 'group',
			level: g.level,
			count: 0,
			children: []
		});
	}

	// Wire parent → child for groups
	for (const g of skillGroups) {
		if (g.parent_uri) {
			const parent = groupNodes.get(g.parent_uri);
			const child = groupNodes.get(g.uri);
			if (parent && child) {
				parent.children.push(child);
			}
		}
	}

	// Attach skills to their group
	const uncategorized: EscoTreeNode[] = [];
	for (const s of skills) {
		const skillNode: EscoTreeNode = {
			id: `skill:${s.uri}`,
			label: s.name,
			type: 'skill',
			level: -1,
			count: 1,
			children: [],
			skill: {
				uri: s.uri,
				name: s.name,
				description: s.description,
				skill_type: s.skill_type,
				alt_labels: s.alt_labels ?? []
			}
		};

		if (s.group_uri && groupNodes.has(s.group_uri)) {
			groupNodes.get(s.group_uri)!.children.push(skillNode);
		} else {
			uncategorized.push(skillNode);
		}
	}

	// Roll up counts
	function rollUpCount(node: EscoTreeNode): number {
		if (node.type === 'skill') return 1;
		let count = 0;
		for (const child of node.children) {
			count += rollUpCount(child);
		}
		node.count = count;
		return count;
	}

	// Sort children: groups first (by label), then skills (by label)
	function sortTree(node: EscoTreeNode) {
		node.children.sort((a, b) => {
			if (a.type !== b.type) return a.type === 'group' ? -1 : 1;
			return a.label.localeCompare(b.label);
		});
		for (const child of node.children) {
			if (child.type === 'group') sortTree(child);
		}
	}

	// Get L0 roots (groups with no parent)
	const roots: EscoTreeNode[] = [];
	for (const g of skillGroups) {
		if (!g.parent_uri) {
			const node = groupNodes.get(g.uri);
			if (node) roots.push(node);
		}
	}

	// Add uncategorized bucket if there are orphans
	if (uncategorized.length > 0) {
		uncategorized.sort((a, b) => a.label.localeCompare(b.label));
		roots.push({
			id: 'group:uncategorized',
			label: 'Uncategorized',
			type: 'group',
			level: 0,
			count: uncategorized.length,
			children: uncategorized
		});
	}

	for (const root of roots) {
		rollUpCount(root);
		sortTree(root);
	}
	roots.sort((a, b) => a.label.localeCompare(b.label));

	return roots;
}

function buildTechSkillTree(techSkills: ReturnType<typeof loadOnetTechSkills>): TechSkillTreeNode[] {
	// Group by class_title → commodity_title → products
	const classMap = new Map<string, Map<string, typeof techSkills>>();

	for (const skill of techSkills) {
		if (!classMap.has(skill.class_title)) {
			classMap.set(skill.class_title, new Map());
		}
		const commodityMap = classMap.get(skill.class_title)!;
		if (!commodityMap.has(skill.commodity_title)) {
			commodityMap.set(skill.commodity_title, []);
		}
		commodityMap.get(skill.commodity_title)!.push(skill);
	}

	const tree: TechSkillTreeNode[] = [];

	for (const [classTitle, commodityMap] of [...classMap.entries()].sort((a, b) => a[0].localeCompare(b[0]))) {
		const commodityNodes: TechSkillTreeNode[] = [];

		for (const [commodityTitle, products] of [...commodityMap.entries()].sort((a, b) => a[0].localeCompare(b[0]))) {
			const sortedProducts = products.sort((a, b) => a.name.localeCompare(b.name));
			const productNodes: TechSkillTreeNode[] = sortedProducts.map((p) => ({
				id: `product:${p.commodity_code}:${p.name}`,
				label: p.name,
				type: 'product' as const,
				count: 1,
				hot_count: p.hot_technology ? 1 : 0,
				demand_count: p.in_demand ? 1 : 0,
				children: [],
				product: p
			}));

			commodityNodes.push({
				id: `commodity:${products[0].commodity_code}`,
				label: commodityTitle,
				type: 'commodity',
				count: products.length,
				hot_count: products.filter((p) => p.hot_technology).length,
				demand_count: products.filter((p) => p.in_demand).length,
				children: productNodes
			});
		}

		tree.push({
			id: `class:${classTitle}`,
			label: classTitle,
			type: 'class',
			count: commodityNodes.reduce((s, c) => s + c.count, 0),
			hot_count: commodityNodes.reduce((s, c) => s + c.hot_count, 0),
			demand_count: commodityNodes.reduce((s, c) => s + c.demand_count, 0),
			children: commodityNodes
		});
	}

	return tree;
}

export const load: PageServerLoad = async () => {
	const contentTree = buildContentTree();
	const techSkills = loadOnetTechSkills();
	const opmCompetencies = loadOpmCompetencies();
	const escoSkills = loadEscoSkills();

	const techSkillTree = buildTechSkillTree(techSkills);
	const escoTree = buildEscoTree();

	return {
		contentTree,
		techSkillTree,
		techSkillCount: techSkills.length,
		opmCompetencies,
		escoSkills,
		escoTree,
		escoSkillCount: escoSkills.length
	};
};
