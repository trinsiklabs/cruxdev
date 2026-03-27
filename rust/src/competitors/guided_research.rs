//! Guided competitor research — enforces 5-pass methodology via state machine.
//!
//! The engine enforces the methodology by issuing specific search instructions
//! per pass, refusing to advance until the current pass is complete, requiring
//! contrarian evidence before accepting results, and validating completeness.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Research pass stages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ResearchPass {
    Broad,
    Academic,
    Practitioner,
    Contrarian,
    Primary,
    Verify,
    Done,
}

impl ResearchPass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Broad => "broad",
            Self::Academic => "academic",
            Self::Practitioner => "practitioner",
            Self::Contrarian => "contrarian",
            Self::Primary => "primary",
            Self::Verify => "verify",
            Self::Done => "done",
        }
    }
}

const PASS_ORDER: &[ResearchPass] = &[
    ResearchPass::Broad,
    ResearchPass::Academic,
    ResearchPass::Practitioner,
    ResearchPass::Contrarian,
    ResearchPass::Primary,
    ResearchPass::Verify,
    ResearchPass::Done,
];

/// Result submitted for a single research pass.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassResult {
    pub pass_name: String,
    pub findings: Vec<String>,
    pub sources: Vec<String>,
    pub search_queries_used: Vec<String>,
}

/// State of a guided research session for one competitor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchState {
    pub competitor_name: String,
    pub competitor_url: String,
    pub category: String,
    pub current_pass: ResearchPass,
    pub pass_results: Vec<PassResult>,
    pub profile_data: HashMap<String, Value>,
    pub started_at: f64,
}

impl ResearchState {
    pub fn is_done(&self) -> bool {
        self.current_pass == ResearchPass::Done
    }

    pub fn passes_completed(&self) -> usize {
        self.pass_results.len()
    }

    pub fn to_dict(&self) -> Value {
        json!({
            "competitor_name": self.competitor_name,
            "competitor_url": self.competitor_url,
            "category": self.category,
            "current_pass": self.current_pass.as_str(),
            "passes_completed": self.passes_completed(),
            "is_done": self.is_done(),
        })
    }
}

struct PassInstructions {
    goal: &'static str,
    search_queries: Vec<&'static str>,
    required_fields: Vec<&'static str>,
    instructions: &'static str,
}

fn get_pass_instructions(pass: &ResearchPass) -> PassInstructions {
    match pass {
        ResearchPass::Broad => PassInstructions {
            goal: "Establish what this competitor is and does",
            search_queries: vec!["{name} AI coding tool", "{name} {url} features", "{name} vs alternatives"],
            required_fields: vec!["description", "features"],
            instructions: "Search for the competitor. Find: what it does, key features, pricing, tech stack. Return raw findings.",
        },
        ResearchPass::Academic => PassInstructions {
            goal: "Find authoritative evidence — official docs, benchmarks, technical details",
            search_queries: vec!["site:{domain} documentation", "{name} benchmark performance", "{name} architecture technical"],
            required_fields: vec!["tech_stack"],
            instructions: "Find official documentation, benchmarks, technical architecture details. Look for specific numbers (users, stars, downloads). Return findings with source URLs.",
        },
        ResearchPass::Practitioner => PassInstructions {
            goal: "Find real-world user experience — reviews, complaints, praise",
            search_queries: vec!["{name} review 2025 2026", "{name} problems issues", "{name} reddit experience"],
            required_fields: vec!["strengths", "weaknesses"],
            instructions: "Find real user reviews, Reddit discussions, blog posts about using this tool. What do users praise? What do they complain about? Return findings with sources.",
        },
        ResearchPass::Contrarian => PassInstructions {
            goal: "Find evidence AGAINST this competitor — failures, limitations, criticism",
            search_queries: vec!["{name} problems limitations criticism", "{name} not good why avoid", "{name} alternative better than"],
            required_fields: vec![],
            instructions: "THIS PASS IS MANDATORY. Search for criticism, failures, and limitations. Find at least one negative finding. If you cannot find any criticism, state that explicitly — do not skip this pass.",
        },
        ResearchPass::Primary => PassInstructions {
            goal: "Verify claims from primary sources — official site, GitHub, pricing page",
            search_queries: vec!["{url}", "{name} github stars", "{name} pricing"],
            required_fields: vec!["pricing"],
            instructions: "Go to the primary sources: official website, GitHub repo, pricing page. Verify the claims from previous passes. Check: is the pricing accurate? Are the features real? Return verified data.",
        },
        ResearchPass::Verify => PassInstructions {
            goal: "Cross-check all findings and compile final profile",
            search_queries: vec![],
            required_fields: vec![],
            instructions: "No new searches needed. Review all findings from passes 1-5. Flag any contradictions. Compile the final profile with: description, features (list), strengths (list), weaknesses (list), pricing, revenue_model, tech_stack, differentiation. Return as structured data.",
        },
        ResearchPass::Done => PassInstructions {
            goal: "Research complete",
            search_queries: vec![],
            required_fields: vec![],
            instructions: "Research complete.",
        },
    }
}

/// Start a guided research session for one competitor.
pub fn start_research(competitor_name: &str, competitor_url: &str, category: &str) -> ResearchState {
    ResearchState {
        competitor_name: competitor_name.to_string(),
        competitor_url: competitor_url.to_string(),
        category: category.to_string(),
        current_pass: ResearchPass::Broad,
        pass_results: Vec::new(),
        profile_data: HashMap::new(),
        started_at: 0.0, // Simplified: not using real time for portability
    }
}

/// Get the next research instruction for the LLM.
pub fn get_next_step(state: &ResearchState) -> Value {
    if state.is_done() {
        return json!({
            "pass_name": "done",
            "is_done": true,
            "instructions": "Research complete. Call setup_competitive_analysis with the compiled data.",
            "profile_data": state.profile_data,
        });
    }

    let info = get_pass_instructions(&state.current_pass);
    let domain = state
        .competitor_url
        .replace("https://", "")
        .replace("http://", "")
        .split('/')
        .next()
        .unwrap_or("")
        .to_string();

    let queries: Vec<String> = info
        .search_queries
        .iter()
        .map(|q| {
            q.replace("{name}", &state.competitor_name)
                .replace("{url}", &state.competitor_url)
                .replace("{domain}", &domain)
        })
        .collect();

    let pass_idx = PASS_ORDER
        .iter()
        .position(|p| *p == state.current_pass)
        .unwrap_or(0);

    let previous_findings: Vec<Value> = state
        .pass_results
        .iter()
        .map(|r| {
            json!({
                "pass": r.pass_name,
                "finding_count": r.findings.len(),
            })
        })
        .collect();

    json!({
        "pass_name": state.current_pass.as_str(),
        "pass_number": pass_idx + 1,
        "total_passes": PASS_ORDER.len() - 1,
        "goal": info.goal,
        "search_queries": queries,
        "instructions": info.instructions,
        "required_fields": info.required_fields,
        "is_done": false,
        "competitor": state.competitor_name,
        "previous_findings": previous_findings,
    })
}

/// Submit results for the current pass and advance to next.
///
/// Returns a dict with validation result and next step info.
/// Contrarian pass enforcement: rejects empty findings.
pub fn submit_pass_result(
    state: &mut ResearchState,
    findings: Vec<String>,
    sources: Option<Vec<String>>,
    search_queries_used: Option<Vec<String>>,
    profile_updates: Option<HashMap<String, Value>>,
) -> Value {
    // Validate contrarian pass has actual findings
    if state.current_pass == ResearchPass::Contrarian && findings.is_empty() {
        return json!({
            "accepted": false,
            "error": "Contrarian pass MUST have at least one finding. Search for criticism, limitations, or failures. This pass cannot be skipped.",
            "current_pass": state.current_pass.as_str(),
        });
    }

    // Record the pass result
    state.pass_results.push(PassResult {
        pass_name: state.current_pass.as_str().to_string(),
        findings,
        sources: sources.unwrap_or_default(),
        search_queries_used: search_queries_used.unwrap_or_default(),
    });

    // Update profile data
    if let Some(updates) = profile_updates {
        state.profile_data.extend(updates);
    }

    // Advance to next pass
    let current_idx = PASS_ORDER
        .iter()
        .position(|p| *p == state.current_pass)
        .unwrap_or(0);
    state.current_pass = PASS_ORDER[current_idx + 1].clone();

    json!({
        "accepted": true,
        "passes_completed": state.passes_completed(),
        "next": get_next_step(state),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_research() {
        let state = start_research("Cursor", "https://cursor.sh", "official");
        assert_eq!(state.competitor_name, "Cursor");
        assert_eq!(state.current_pass, ResearchPass::Broad);
        assert!(!state.is_done());
        assert_eq!(state.passes_completed(), 0);
    }

    #[test]
    fn test_get_next_step_broad() {
        let state = start_research("Cursor", "https://cursor.sh", "");
        let step = get_next_step(&state);
        assert_eq!(step["pass_name"], "broad");
        assert_eq!(step["is_done"], false);
        assert_eq!(step["pass_number"], 1);
        assert!(step["search_queries"].as_array().unwrap().len() > 0);
    }

    #[test]
    fn test_submit_pass_result_advances() {
        let mut state = start_research("Test", "https://test.com", "");
        let result = submit_pass_result(
            &mut state,
            vec!["Found some info".to_string()],
            None,
            None,
            None,
        );
        assert_eq!(result["accepted"], true);
        assert_eq!(result["passes_completed"], 1);
        assert_eq!(state.current_pass, ResearchPass::Academic);
    }

    #[test]
    fn test_contrarian_pass_rejects_empty_findings() {
        let mut state = start_research("Test", "https://test.com", "");
        // Advance to contrarian
        state.current_pass = ResearchPass::Contrarian;

        let result = submit_pass_result(&mut state, vec![], None, None, None);
        assert_eq!(result["accepted"], false);
        assert!(result["error"].as_str().unwrap().contains("Contrarian pass MUST"));
        // State should NOT have advanced
        assert_eq!(state.current_pass, ResearchPass::Contrarian);
    }

    #[test]
    fn test_contrarian_pass_accepts_findings() {
        let mut state = start_research("Test", "https://test.com", "");
        state.current_pass = ResearchPass::Contrarian;

        let result = submit_pass_result(
            &mut state,
            vec!["Users report high latency".to_string()],
            None,
            None,
            None,
        );
        assert_eq!(result["accepted"], true);
        assert_eq!(state.current_pass, ResearchPass::Primary);
    }

    #[test]
    fn test_full_research_flow_to_done() {
        let mut state = start_research("Test", "https://test.com", "");

        // Advance through all passes
        for i in 0..6 {
            let findings = vec![format!("Finding from pass {i}")];
            submit_pass_result(&mut state, findings, None, None, None);
        }
        assert!(state.is_done());
        assert_eq!(state.passes_completed(), 6);

        let step = get_next_step(&state);
        assert_eq!(step["is_done"], true);
    }

    #[test]
    fn test_to_dict() {
        let state = start_research("Cursor", "https://cursor.sh", "official");
        let d = state.to_dict();
        assert_eq!(d["competitor_name"], "Cursor");
        assert_eq!(d["is_done"], false);
        assert_eq!(d["passes_completed"], 0);
    }
}
