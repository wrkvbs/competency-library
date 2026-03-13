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
