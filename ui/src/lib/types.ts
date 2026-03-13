// ---------------------------------------------------------------------------
// allUP Transcript data
// ---------------------------------------------------------------------------

export interface TranscriptRecord {
	id: string;
	responderUserId: string;
	subjectId: string;
	responder: { fullName: string };
	prompt: { selfText: string | null; otherUserText: string | null };
	transcript: { text: string | null } | null;
	tags: Array<{ tag: string; category: string }>;
}

// ---------------------------------------------------------------------------
// Scoring output (from explore_scoring.py)
// ---------------------------------------------------------------------------

export interface BehavioralCompetency {
	name: string;
	score: number;
	confidence: number;
	evidence: string;
}

export interface SpecificSkill {
	name: string;
	category: 'technology' | 'methodology' | 'domain_knowledge' | 'tool' | 'soft_skill' | string;
	depth: 'surface' | 'working' | 'deep' | 'expert';
	depth_score: number;
	confidence: number;
	evidence: string;
}

export interface OverallAssessment {
	strongest_signals: string[];
	gaps: string[];
	suggested_followup_questions: string[];
}

export interface ScoringExtraction {
	behavioral_competencies: BehavioralCompetency[];
	specific_skills: SpecificSkill[];
	overall_assessment: OverallAssessment;
}

export interface ScoringResult {
	response_id: string;
	responder: string;
	prompt_text?: string;
	transcript_length?: number;
	prompt_version: string;
	llm_time_secs?: number;
	extraction?: ScoringExtraction;
	error?: string;
}

export interface ScoringRun {
	filename: string;
	timestamp: string;
	prompt_version: string;
	results: ScoringResult[];
}

// ---------------------------------------------------------------------------
// O*NET
// ---------------------------------------------------------------------------

export interface OnetContentModelElement {
	element_id: string;
	name: string;
	description: string;
}

export interface OnetAbility {
	element_id: string;
	name: string;
	description: string;
	parent_id?: string;
	parent_name?: string;
	scale_anchors?: Array<{ level: number; description: string }>;
}

export interface OnetSkill {
	element_id: string;
	name: string;
	description: string;
}

export interface OnetKnowledge {
	element_id: string;
	name: string;
	description: string;
}

export interface OnetWorkStyle {
	element_id: string;
	name: string;
	description: string;
}

export interface OnetWorkActivity {
	element_id: string;
	name: string;
	description: string;
}

export interface OnetTechSkill {
	name: string;
	commodity_code: string;
	commodity_title: string;
	class_title: string;
	family_title: string;
	segment_title: string;
	hot_technology: boolean;
	in_demand: boolean;
	occupation_count: number;
}

export interface UnspscCategory {
	commodity_code: string;
	commodity_title: string;
	class_code: string;
	class_title: string;
	family_code: string;
	family_title: string;
	segment_code: string;
	segment_title: string;
}

// ---------------------------------------------------------------------------
// OPM
// ---------------------------------------------------------------------------

export interface OpmCompetency {
	name: string;
	definition: string;
	source: string;
	category: string;
}

// ---------------------------------------------------------------------------
// ESCO (future)
// ---------------------------------------------------------------------------

export interface EscoSkill {
	uri: string;
	name: string;
	description: string;
	skill_type: string;
	group_uri?: string;
	alt_labels?: string[];
	broader?: string[];
	narrower?: string[];
}

export interface EscoSkillGroup {
	uri: string;
	name: string;
	code: string;
	description: string;
	level: number;
	parent_uri?: string;
}

// ---------------------------------------------------------------------------
// WORKBank
// ---------------------------------------------------------------------------

export interface WorkbankTask {
	task_id: string;
	occupation: string;
	o_net_soc: string;
	task_statement: string;
	worker_desire_mean: number | null;
	ai_capability_mean: number | null;
	human_agency_level: string;
}

// ---------------------------------------------------------------------------
// allUP Production Data
// ---------------------------------------------------------------------------

export interface PromptTag {
	id: string;
	tag: string;
	description: string | null;
	internalDescription: string | null;
	category: string;
	userSelectable: boolean;
	structural: boolean;
	parentTagId: string | null;
	definesCategory: boolean;
	visibleOnProfile: boolean;
	featuredRole: boolean;
	color: string | null;
	synonyms: string[];
	promptCount: number;
	properties: Record<string, unknown> | null;
	childTags: Array<{ id: string; tag: string; category: string }>;
}

export interface PromptTagFile {
	fetched_at: string;
	count: number;
	tags: PromptTag[];
}

export interface AllUpPrompt {
	id: string;
	promptType: string;
	subjectType: string;
	selfText: string | null;
	otherUserText: string | null;
	tagIds: string[];
	topicTagIds: string[];
	order: number | null;
	priority: number | null;
	suggested: boolean;
	useForInterviewGeneration: boolean;
	relationshipTypes: string[];
	recommendedDuration: number | null;
	maximumDuration: number | null;
	purpose: string | null;
}

export interface PromptFile {
	fetched_at: string;
	count: number;
	prompts: AllUpPrompt[];
}

export interface Affiliation {
	id: string;
	organization: { id: string; name: string; shortDescription: string | null; location: string | null };
	title: string | null;
	startedAt: string | null;
	endedAt: string | null;
	status: string;
}

export interface UserProfile {
	userId: string;
	fullName: string;
	shortName: string | null;
	headline: string | null;
	pronouns: { form: string; subjective: string; objective: string; posessive: string } | null;
	slug: string | null;
	createdAt: string;
	trending: boolean;
	promptTags: Array<{ id: string; tag: string; category: string }>;
	affiliations: Affiliation[];
	intentFlags: Array<{ intentFlagId: string; text: string; color: string | null; expiresAt: string | null }>;
	socialLinks: Array<{ socialLinkId: string; name: string; username: string | null; url: string | null }>;
}

export interface UserProfileFile {
	fetched_at: string;
	count: number;
	profiles: UserProfile[];
}

export interface PromptResponse {
	id: string;
	promptId: string;
	subjectId: string;
	responderUserId: string;
	approval: string;
	visibility: string;
	promptText: string | null;
	transcript: { text: string | null } | null;
	responder: { fullName: string } | null;
	tagIds: string[];
	topicIds: string[];
	createdAt: string;
}

export interface ResponseFile {
	fetched_at: string;
	count: number;
	responses: PromptResponse[];
}

export interface Resume {
	id: string;
	userId: string;
	assetId: string | null;
	url: string | null;
}

export interface ResumeFile {
	fetched_at: string;
	count: number;
	resumes: Resume[];
}
