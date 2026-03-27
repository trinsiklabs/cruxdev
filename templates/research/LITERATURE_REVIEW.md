---
title: "Literature Review: [Topic]"
conducted: [YYYY-MM-DD]
valid_until: [YYYY-MM-DD]
decision_context: "[What decision does this review support?]"
status: [in-progress | complete | stale]
author: "[Name]"
plan_reference: "[PLAN-XXXX]"
---

# Literature Review: [Topic]

**Decision Context:** [What decision does this review support? Or: "Baseline knowledge establishment for [domain]."]
**Conducted:** [YYYY-MM-DD]
**Valid Until:** [YYYY-MM-DD]

---

## 1. Executive Summary

[3-5 sentences. What topic was surveyed, how many sources were reviewed, what are the key findings, what is the state of knowledge.]

**State of Knowledge:** [Well-established / Active research / Emerging / Contested / Sparse]

---

## 2. Scope and Search Methodology

### 2.1 Research Questions

1. [Question 1: e.g., "What are the established approaches to distributed file synchronization?"]
2. [Question 2: e.g., "What are the known failure modes and their mitigations?"]
3. [Question 3: e.g., "What performance characteristics have been measured in practice?"]

### 2.2 Search Strategy

| Source Type | Sources Searched | Search Terms | Results Found | Included |
|---|---|---|---|---|
| Academic databases | [e.g., Google Scholar, ACM, IEEE] | [Terms used] | [Count] | [Count] |
| Industry reports | [e.g., Gartner, Forrester] | [Terms used] | [Count] | [Count] |
| Technical blogs / whitepapers | [e.g., company engineering blogs] | [Terms used] | [Count] | [Count] |
| Open source projects | [e.g., GitHub, GitLab] | [Terms used] | [Count] | [Count] |
| Books / reference works | [Specific titles] | — | [Count] | [Count] |
| Conference proceedings | [e.g., USENIX, OSDI, SOSP] | [Terms used] | [Count] | [Count] |

### 2.3 Inclusion/Exclusion Criteria

**Include:**
- [Criterion: e.g., "Published after 2020"]
- [Criterion: e.g., "Addresses distributed systems with offline operation"]
- [Criterion: e.g., "Includes empirical evaluation or formal analysis"]

**Exclude:**
- [Criterion: e.g., "Cloud-only solutions without local-first capability"]
- [Criterion: e.g., "Marketing materials without technical substance"]
- [Criterion: e.g., "Duplicate publications of the same research"]

### 2.4 Limitations

- [Limitation: e.g., "Search limited to English-language publications"]
- [Limitation: e.g., "Proprietary vendor research not accessible"]

---

## 3. Background and Context

[Provide necessary context for understanding the reviewed literature. Define key terms. Describe the problem domain.]

### 3.1 Key Definitions

| Term | Definition |
|---|---|
| [Term 1] | [Definition as used in this review] |
| [Term 2] | [Definition as used in this review] |
| [Term 3] | [Definition as used in this review] |

### 3.2 Historical Context

[Brief history of the field/topic. How did we get to the current state of knowledge?]

---

## 4. Thematic Analysis

### 4.1 [Theme 1: e.g., "Conflict Resolution Strategies"]

**Summary:** [2-3 sentence summary of what the literature says about this theme.]

| Source | Key Finding | Methodology | Strength of Evidence |
|---|---|---|---|
| [Author, Year] | [Finding] | [How they determined this] | [Strong/Moderate/Weak] |
| [Author, Year] | [Finding] | [Methodology] | [Strength] |
| [Author, Year] | [Finding] | [Methodology] | [Strength] |

**Consensus:** [What most sources agree on]
**Disagreement:** [Where sources disagree and why]
**Gaps:** [What is not addressed in the literature on this theme]

---

### 4.2 [Theme 2: e.g., "Performance at Scale"]

[Same structure as Theme 1]

---

### 4.3 [Theme 3: e.g., "Security Models"]

[Same structure as Theme 1]

---

### 4.4 [Theme 4: e.g., "Practical Deployment Patterns"]

[Same structure as Theme 1]

---

## 5. Comparative Analysis

### 5.1 Approaches Comparison

| Approach | Strengths | Weaknesses | Best Suited For | Sources |
|---|---|---|---|---|
| [Approach A] | [Strengths] | [Weaknesses] | [Use case] | [Citations] |
| [Approach B] | [Strengths] | [Weaknesses] | [Use case] | [Citations] |
| [Approach C] | [Strengths] | [Weaknesses] | [Use case] | [Citations] |

### 5.2 Methodology Quality Assessment

| Source | Sample Size / Scale | Reproducibility | Bias Risk | Overall Quality |
|---|---|---|---|---|
| [Author, Year] | [Assessment] | [High/Med/Low] | [High/Med/Low] | [High/Med/Low] |
| [Author, Year] | [Assessment] | [High/Med/Low] | [High/Med/Low] | [High/Med/Low] |

---

## 6. Synthesis

### 6.1 Key Findings

[Synthesize across all themes. What does the body of literature, taken together, tell us?]

1. **[Finding 1]:** [Statement with supporting evidence from multiple sources]
2. **[Finding 2]:** [Statement with supporting evidence]
3. **[Finding 3]:** [Statement with supporting evidence]

### 6.2 Areas of Consensus

[What the literature broadly agrees on:]
- [Consensus point with citations]
- [Consensus point with citations]

### 6.3 Areas of Debate

[Where the literature disagrees:]

| Debate | Position A | Position B | Current Weight of Evidence |
|---|---|---|---|
| [Debate topic] | [Position + proponents] | [Position + proponents] | [Which side has stronger evidence] |

### 6.4 Identified Gaps

[What the literature does NOT address:]

| Gap | Significance | How It Affects Our Decision |
|---|---|---|
| [Gap description] | [Why it matters] | [Impact on our context] |

---

## 7. Relevance to Our Context

### 7.1 Applicability Assessment

[How well does the literature apply to our specific situation?]

| Finding | Applicability to Our Context | Adaptation Needed |
|---|---|---|
| [Finding] | [High/Med/Low — why] | [What would change in our context] |

### 7.2 Lessons Learned from Others

[Practical takeaways from the literature that apply to our situation:]

1. [Lesson with source]
2. [Lesson with source]
3. [Lesson with source]

### 7.3 Pitfalls to Avoid

[Mistakes or failure modes documented in the literature that we should guard against:]

1. [Pitfall with source and our mitigation]
2. [Pitfall with source and our mitigation]

---

## 8. Recommendations

### 8.1 Based on the Literature

[What the body of evidence suggests we should do:]

1. [Recommendation with supporting evidence]
2. [Recommendation with supporting evidence]

### 8.2 Further Research Needed

[Questions that remain open and need investigation beyond literature review:]

| Question | Why It Matters | Suggested Method |
|---|---|---|
| [Question] | [Impact on decision] | [How to investigate: POC, benchmark, expert consultation] |

---

## 9. Annotated Bibliography

### Primary Sources

#### [Author(s), "Title," Publication, Year]

- **URL/DOI:** [Link]
- **Summary:** [2-3 sentence summary of the work]
- **Key contribution:** [What this source uniquely contributes]
- **Relevance:** [High/Medium/Low] — [Why relevant to our context]
- **Limitations:** [Known limitations of this work]

#### [Next source]

[Same structure]

---

### Secondary Sources

#### [Author(s), "Title," Publication, Year]

[Same structure but for supporting/background sources]

---

## 10. Sources Summary Table

| # | Author | Title | Year | Type | Relevance | Quality |
|---|---|---|---|---|---|---|
| 1 | [Author] | [Title] | [Year] | [Paper/Report/Blog/Book] | [H/M/L] | [H/M/L] |
| 2 | [Author] | [Title] | [Year] | [Type] | [H/M/L] | [H/M/L] |
| 3 | [Author] | [Title] | [Year] | [Type] | [H/M/L] | [H/M/L] |

---

## Related Documents

- [Link to technology evaluation, feasibility study, or other research this feeds]
