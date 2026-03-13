import { readFileSync, readdirSync, existsSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import type {
	TranscriptRecord,
	ScoringResult,
	ScoringRun,
	OnetContentModelElement,
	OnetAbility,
	OnetSkill,
	OnetKnowledge,
	OnetWorkStyle,
	OnetWorkActivity,
	OnetTechSkill,
	UnspscCategory,
	OpmCompetency,
	EscoSkill,
	EscoSkillGroup,
	WorkbankTask,
	PromptTag,
	PromptTagFile,
	AllUpPrompt,
	PromptFile,
	UserProfile,
	UserProfileFile,
	PromptResponse,
	ResponseFile,
	Resume,
	ResumeFile
} from './types';

// Repo root is one level up from ui/
const __dirname = fileURLToPath(new URL('.', import.meta.url));
const REPO_ROOT = resolve(__dirname, '..', '..', '..');
const PROCESSED = join(REPO_ROOT, 'datasets', 'processed');
const RAW_ALLUP = join(REPO_ROOT, 'datasets', 'raw', 'allup');
const RUNS_EXPLORE = join(REPO_ROOT, 'runs', 'explore');

function readJson<T>(filepath: string, fallback: T): T {
	if (!existsSync(filepath)) return fallback;
	try {
		const raw = readFileSync(filepath, 'utf-8');
		return JSON.parse(raw) as T;
	} catch {
		return fallback;
	}
}

// ---------------------------------------------------------------------------
// allUP data
// ---------------------------------------------------------------------------

export function loadTranscripts(): TranscriptRecord[] {
	return readJson<TranscriptRecord[]>(join(RAW_ALLUP, 'transcripts.json'), []);
}

// ---------------------------------------------------------------------------
// Scoring runs
// ---------------------------------------------------------------------------

function parseRunFilename(filename: string): { timestamp: string; promptVersion: string } {
	// Format: run-YYYYMMDD-HHMMSS-v1.json
	const match = filename.match(/^run-(\d{8}-\d{6})-(.+)\.json$/);
	if (!match) return { timestamp: '', promptVersion: '' };
	return { timestamp: match[1], promptVersion: match[2] };
}

export function loadAllRuns(): ScoringRun[] {
	if (!existsSync(RUNS_EXPLORE)) return [];
	const files = readdirSync(RUNS_EXPLORE)
		.filter((f) => f.endsWith('.json'))
		.sort()
		.reverse();

	return files.map((filename) => {
		const { timestamp, promptVersion } = parseRunFilename(filename);
		const results = readJson<ScoringResult[]>(join(RUNS_EXPLORE, filename), []);
		return {
			filename,
			timestamp,
			prompt_version: promptVersion,
			results
		};
	});
}

export function loadLatestRun(): ScoringRun | null {
	const runs = loadAllRuns();
	return runs.length > 0 ? runs[0] : null;
}

// ---------------------------------------------------------------------------
// O*NET
// ---------------------------------------------------------------------------

export function loadOnetContentModel(): OnetContentModelElement[] {
	return readJson<OnetContentModelElement[]>(join(PROCESSED, 'onet-content-model.json'), []);
}

export function loadOnetAbilities(): OnetAbility[] {
	return readJson<OnetAbility[]>(join(PROCESSED, 'onet-abilities.json'), []);
}

export function loadOnetSkills(): OnetSkill[] {
	return readJson<OnetSkill[]>(join(PROCESSED, 'onet-skills.json'), []);
}

export function loadOnetKnowledge(): OnetKnowledge[] {
	return readJson<OnetKnowledge[]>(join(PROCESSED, 'onet-knowledge.json'), []);
}

export function loadOnetWorkStyles(): OnetWorkStyle[] {
	return readJson<OnetWorkStyle[]>(join(PROCESSED, 'onet-work-styles.json'), []);
}

export function loadOnetWorkActivities(): OnetWorkActivity[] {
	return readJson<OnetWorkActivity[]>(join(PROCESSED, 'onet-work-activities.json'), []);
}

export function loadOnetTechSkills(): OnetTechSkill[] {
	return readJson<OnetTechSkill[]>(join(PROCESSED, 'onet-tech-skills.json'), []);
}

export function loadUnspscCategories(): UnspscCategory[] {
	return readJson<UnspscCategory[]>(join(PROCESSED, 'onet-unspsc.json'), []);
}

// ---------------------------------------------------------------------------
// OPM
// ---------------------------------------------------------------------------

export function loadOpmCompetencies(): OpmCompetency[] {
	return readJson<OpmCompetency[]>(join(PROCESSED, 'opm-competencies.json'), []);
}

// ---------------------------------------------------------------------------
// ESCO (future)
// ---------------------------------------------------------------------------

export function loadEscoSkills(): EscoSkill[] {
	return readJson<EscoSkill[]>(join(PROCESSED, 'esco-skills.json'), []);
}

export function loadEscoSkillGroups(): EscoSkillGroup[] {
	return readJson<EscoSkillGroup[]>(join(PROCESSED, 'esco-skill-groups.json'), []);
}

// ---------------------------------------------------------------------------
// WORKBank
// ---------------------------------------------------------------------------

export function loadWorkbankTasks(): WorkbankTask[] {
	return readJson<WorkbankTask[]>(join(PROCESSED, 'workbank-tasks.json'), []);
}

// ---------------------------------------------------------------------------
// allUP Production Data
// ---------------------------------------------------------------------------

export function loadPromptTags(): PromptTag[] {
	const file = readJson<PromptTagFile | null>(join(RAW_ALLUP, 'prompt-tags.json'), null);
	return file?.tags ?? [];
}

export function loadAllUpPrompts(): AllUpPrompt[] {
	const file = readJson<PromptFile | null>(join(RAW_ALLUP, 'prompts.json'), null);
	return file?.prompts ?? [];
}

export function loadUserProfiles(): UserProfile[] {
	const file = readJson<UserProfileFile | null>(join(RAW_ALLUP, 'user-profiles.json'), null);
	const profiles = file?.profiles ?? [];
	// Strip "user:" prefix from userId for consistency with response data
	for (const p of profiles) {
		if (p.userId.startsWith('user:')) p.userId = p.userId.slice(5);
	}
	return profiles;
}

export function loadResponses(): PromptResponse[] {
	const file = readJson<ResponseFile | null>(join(RAW_ALLUP, 'responses.json'), null);
	return file?.responses ?? [];
}

export function loadResumes(): Resume[] {
	const file = readJson<ResumeFile | null>(join(RAW_ALLUP, 'resumes.json'), null);
	return file?.resumes ?? [];
}
