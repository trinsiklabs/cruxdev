# Project Type Competitive Gap Analysis

**Date:** 2026-03-28
**Status:** Research only — no build plans created
**Scope:** All 18 CruxDev project type pages

---

## 1. Greenfield Software (`software-new`)

**What CruxDev offers:** TDD-first lifecycle, 100% coverage enforcement, architecture alignment, doc sync, lint/format, security posture. Templates: README, CLAUDE.md, CHANGELOG, GAPS.md. Safety gates (3-failure rollback, 15-min timeout).

**Top competitors:**
1. **GitHub Copilot Workspace** — AI-driven greenfield scaffolding, issue-to-PR pipeline, integrated with GitHub
2. **Cursor / Windsurf** — AI-native IDEs with inline code generation, multi-file edits, project-aware context
3. **Replit Agent / Replit Ghostwriter** — Full-app generation from natural language, instant deploy
4. **v0 by Vercel** — UI-focused scaffolding, generates full React/Next.js apps from prompts
5. **Devin (Cognition)** — Autonomous software agent, full IDE sandbox, plans and executes multi-step tasks

**Gaps:**
- **No visual IDE integration.** CruxDev is CLI-only. Competitors embed into the editor experience (Copilot in VS Code, Cursor's native editor). Users who want a visual workflow have no path.
- **No instant deploy / preview.** Replit and v0 offer one-click deploy. CruxDev scaffolds but deployment is manual.
- **No UI generation.** v0 generates visual components. CruxDev has no opinion on UI scaffolding.
- **No multi-model orchestration visible to user.** Devin shows its planning process. CruxDev's convergence loop is opaque to users during execution.

**Priority:** **High** — This is CruxDev's core market. Every gap here is a reason someone picks a competitor.

---

## 2. Existing Codebase Adoption (`software-existing`)

**What CruxDev offers:** Inventory scan, gap analysis, prioritized plan to close gaps, progressive convergence. Audit dimensions: test coverage delta, doc completeness, CI/CD presence, dependency health, code quality, architecture debt.

**Top competitors:**
1. **SonarQube / SonarCloud** — Static analysis, code smells, coverage tracking, security hotspots
2. **CodeScene** — Behavioral code analysis, hotspot detection, team coupling, technical debt prioritization
3. **Codacy** — Automated code review, coverage tracking, security scanning
4. **Snyk** — Dependency vulnerability scanning, license compliance, container security
5. **Sourcery** — AI code review, refactoring suggestions, quality metrics

**Gaps:**
- **No real-time dashboard.** SonarQube provides a web dashboard with trend lines. CruxDev gap status is file-based.
- **No CI/CD integration out of the box.** SonarQube/Codacy plug into GitHub Actions, GitLab CI natively. CruxDev requires manual setup.
- **No dependency vulnerability database.** Snyk maintains a live CVE database. CruxDev audits dependency presence but does not scan for known vulnerabilities at database depth.
- **No team/collaboration metrics.** CodeScene analyzes team coupling and knowledge distribution. CruxDev is single-agent.
- **No PR-level checks.** Codacy and SonarCloud comment directly on PRs. CruxDev operates at project level, not PR level.

**Priority:** **High** — Existing codebase adoption is CruxDev's most common entry point. Dashboard and CI/CD integration are expected by teams.

---

## 3. Book (`book`)

**What CruxDev offers:** Outline convergence, draft generation, consistency audit (cross-chapter), edit convergence (multi-pass), production prep. Audit dimensions: structural completeness, argument coherence, terminology consistency, pacing/word balance, citation accuracy, voice consistency.

**Top competitors:**
1. **Scrivener** — Industry-standard book writing tool: corkboard, outliner, compile to multiple formats, research binder
2. **Atticus** — Book formatting + writing, direct export to epub/print-ready PDF
3. **ProWritingAid** — Style, grammar, readability analysis, consistency checking, pacing visualization
4. **Sudowrite** — AI writing assistant: brainstorm, expand, rewrite, describe, with tone/voice controls
5. **Plottr** — Outline and plot planning with timeline view, character arcs, series management

**Gaps:**
- **No manuscript formatting / export.** Scrivener and Atticus compile to epub, PDF, DOCX. CruxDev produces markdown — no typesetting.
- **No visual outline / corkboard.** Scrivener's corkboard and Plottr's timeline are visual tools. CruxDev outlines are text files.
- **No grammar/style engine.** ProWritingAid has a deep grammar engine with style rules. CruxDev audits structure and consistency but relies on LLM for prose quality.
- **No word processor integration.** Authors live in Scrivener, Word, or Google Docs. CruxDev operates in the terminal on markdown files.
- **No collaborative editing.** Google Docs allows real-time co-editing. CruxDev is single-agent.

**Priority:** **Medium** — Book authors are a real market, but CruxDev's strength (convergence auditing) is unique. The format/export gap is the most actionable.

---

## 4. Book Series (`book-series`)

**What CruxDev offers:** Series architecture, shared canon management, per-book convergence with cross-book audits. Audit dimensions: canon compliance, timeline consistency, character arc continuity, terminology drift, narrative escalation.

**Top competitors:**
1. **Scrivener** (multi-document projects) — Can link projects, shared research folders
2. **Campfire Write** — World-building + writing: encyclopedia, timelines, character relationship maps
3. **World Anvil** — World-building wiki: timelines, maps, family trees, article linking
4. **Plottr** — Series-level plot management, character arcs across books
5. **Bibisco** — Character management, location tracking, strand visualization

**Gaps:**
- **No visual world-building tools.** Campfire and World Anvil offer maps, relationship graphs, visual timelines. CruxDev is text-only canon.
- **No character relationship visualization.** Bibisco and Campfire show character webs. CruxDev's CHARACTERS.md is a flat document.
- **No interactive timeline.** World Anvil has an interactive timeline. CruxDev's TIMELINE.md is static text.
- **No reader-facing wiki export.** World Anvil generates a shareable wiki. CruxDev's canon is internal only.

**Priority:** **Low** — Niche market. CruxDev's cross-book consistency auditing is genuinely differentiated, but the visual tooling gap is not worth closing in the near term.

---

## 5. Online Course (`course`)

**What CruxDev offers:** Curriculum design, lesson development, assessment creation, sequencing audit, course convergence. Audit dimensions: learning objective coverage, prerequisite chain, difficulty progression, assessment alignment, content completeness, time estimates.

**Top competitors:**
1. **Teachable** — Course hosting, student management, payments, completion tracking
2. **Thinkific** — Course builder, community, quizzes, certificates, analytics
3. **Kajabi** — All-in-one: course hosting, email marketing, website, community
4. **LearnDash** (WordPress LMS) — Course builder, quizzes, certificates, drip content
5. **Articulate 360 / Rise** — SCORM-compliant course authoring, interactive content creation

**Gaps:**
- **No hosting or delivery platform.** Teachable, Thinkific, and Kajabi host and deliver courses. CruxDev produces content files, not a student experience.
- **No student progress tracking.** All competitors track completion, quiz scores, engagement. CruxDev has no student-facing layer.
- **No payment / monetization.** Kajabi and Teachable handle payments. CruxDev is a content quality tool.
- **No interactive content authoring.** Articulate creates interactive SCORM modules. CruxDev produces static markdown.
- **No video lesson support.** Course platforms integrate video hosting. CruxDev manages text-based content only.

**Priority:** **Low** — CruxDev is explicitly a content quality layer, not a delivery platform. The gap is by design. Focus on making the content pipeline excellent, not on building an LMS.

---

## 6. Podcast (`podcast`)

**What CruxDev offers:** Series planning, episode prep (5-pass research), post-production content (show notes, transcripts, social), publishing pipeline, series convergence. Audit dimensions: episode prep completeness, show notes quality, metadata consistency, content derivative sync, topic coverage, publishing cadence.

**Top competitors:**
1. **Descript** — AI-powered audio/video editing, transcription, show notes generation, clip creation
2. **Riverside.fm** — Recording, transcription, AI show notes, highlight clips
3. **Castmagic** — AI post-production: transcripts, show notes, social posts, blog posts from episodes
4. **Podium** — AI show notes, chapters, transcripts, social content
5. **Buzzsprout / Transistor** — Hosting, distribution, analytics, episode management

**Gaps:**
- **No audio/video processing.** Descript and Riverside handle recording and editing. CruxDev manages text artifacts only.
- **No automatic transcription.** Descript and Castmagic transcribe automatically. CruxDev assumes transcripts exist.
- **No AI clip generation.** Descript/Riverside identify highlight moments and generate clips. CruxDev has no media awareness.
- **No distribution / hosting.** Buzzsprout and Transistor distribute to Apple, Spotify, etc. CruxDev does not interact with podcast platforms.
- **No analytics.** Competitors provide download metrics, listener demographics. CruxDev tracks content quality, not audience data.

**Priority:** **Low** — CruxDev's value is in the content pipeline (prep, show notes, derivatives). The audio/hosting gap is by design. The opportunity is in tighter integration with tools like Descript for transcript ingestion.

---

## 7. Newsletter (`newsletter`)

**What CruxDev offers:** Strategy, editorial calendar, issue drafting, quality convergence, series consistency. Audit dimensions: content pillar balance, voice consistency, link health, CTA effectiveness, subject line quality, cadence adherence.

**Top competitors:**
1. **Beehiiv** — Newsletter platform: writing, growth tools, monetization, analytics, referral programs
2. **Substack** — Writing + publishing + payments + subscriber management
3. **ConvertKit (Kit)** — Email marketing: sequences, automations, subscriber tagging, landing pages
4. **Mailchimp** — Email design, audience segmentation, A/B testing, analytics
5. **Typefully** — Writing tool for Twitter/newsletter with scheduling, analytics, AI assistance

**Gaps:**
- **No email delivery.** All competitors send emails. CruxDev produces content, not deliveries.
- **No subscriber management.** Beehiiv, Substack, and ConvertKit manage subscriber lists. CruxDev has no audience layer.
- **No A/B testing.** Mailchimp and Beehiiv A/B test subject lines and content. CruxDev audits quality but cannot test with real audiences.
- **No growth/referral tools.** Beehiiv's referral program and ConvertKit's landing pages drive subscriber growth. CruxDev has no growth mechanics for newsletters.
- **No analytics.** Open rates, click rates, subscriber growth — all competitors provide this. CruxDev does not.

**Priority:** **Medium** — CruxDev already integrates with Typefully for social posting. Extending the integration pattern to newsletter platforms (Beehiiv API, ConvertKit API) for publish-from-CruxDev workflows would be high value.

---

## 8. YouTube Channel (`youtube`)

**What CruxDev offers:** Channel strategy, content calendar, script convergence, metadata optimization, derivative content. Audit dimensions: script structure, title/thumbnail alignment, SEO metadata, content pillar balance, upload cadence, cross-platform sync.

**Top competitors:**
1. **vidIQ** — YouTube SEO, keyword research, channel audit, competitor tracking, trend alerts
2. **TubeBuddy** — Tag suggestions, A/B testing thumbnails, SEO tools, bulk processing
3. **Opus Clip** — AI short-form clip extraction from long videos
4. **Jasper AI** — AI copywriting for titles, descriptions, scripts, social posts
5. **Notion / Airtable** — Content calendar and production pipeline management (manual)

**Gaps:**
- **No YouTube API integration.** vidIQ and TubeBuddy pull real channel data (views, CTR, rankings). CruxDev has no connection to YouTube analytics.
- **No keyword/SEO research tool.** vidIQ provides search volume, competition scores. CruxDev audits metadata quality but cannot research what to target.
- **No thumbnail analysis.** TubeBuddy A/B tests thumbnails. CruxDev acknowledges thumbnails exist but cannot evaluate them.
- **No video clip extraction.** Opus Clip identifies viral moments. CruxDev is text-only.
- **No channel analytics.** Competitors show which videos perform and why. CruxDev tracks content quality, not audience response.

**Priority:** **Medium** — YouTube is a large market. The script/strategy convergence is differentiated, but without YouTube data integration, CruxDev operates blind to what actually works.

---

## 9. Open Source (`open-source`)

**What CruxDev offers:** Full software convergence plus governance setup (LICENSE, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY), contributor experience, release management, community health tracking. Audit dimensions: all software dims + governance completeness, contributor onboarding, release discipline, issue hygiene, documentation quality.

**Top competitors:**
1. **GitHub (native features)** — Issues, PRs, Actions, Discussions, Security advisories, Dependabot, community health files
2. **Gitpod / GitHub Codespaces** — One-click contributor development environments
3. **All Contributors** — Contributor recognition bot
4. **Semantic Release** — Automated versioning and changelog from commits
5. **FOSSA / Snyk** — License compliance and vulnerability scanning for OSS

**Gaps:**
- **No contributor development environment.** Codespaces and Gitpod let contributors start coding in one click. CruxDev documents the setup but does not provision environments.
- **No automated release pipeline.** Semantic Release automates versioning from commit messages. CruxDev documents release process but does not execute it.
- **No license compliance scanning.** FOSSA scans dependency licenses for compatibility. CruxDev checks LICENSE exists but does not analyze transitive license compliance.
- **No community metrics dashboard.** GitHub Insights shows contributor activity, PR velocity. CruxDev tracks but does not visualize.
- **No bot integrations.** GitHub bots (stale, all-contributors, welcome) automate community tasks. CruxDev does not provide bots.

**Priority:** **Medium** — CruxDev's governance quality auditing is unique. The biggest actionable gap is automating release pipelines, which aligns with the engine's autonomous execution model.

---

## 10. New Business / Startup (`business-new`)

**What CruxDev offers:** Market research (5-pass), business plan convergence, financial modeling, operations setup, launch convergence. Audit dimensions: market validation, competitive positioning, financial coherence, operations completeness, go-to-market plan, risk assessment.

**Top competitors:**
1. **LivePlan** — Business plan software: templates, financial projections, pitch-ready documents, benchmark data
2. **Lean Canvas / Strategyzer** — Business model canvas tools, validation boards
3. **IdeaBuddy** — Business plan generator with financial projections, step-by-step guides
4. **Notion AI / ChatGPT** — General-purpose AI for brainstorming and drafting business plans
5. **Aha!** — Product strategy, roadmaps, idea management, OKR tracking

**Gaps:**
- **No benchmark / industry data.** LivePlan includes industry benchmark data for financial projections. CruxDev relies on the LLM's knowledge, not a data source.
- **No visual business model canvas.** Strategyzer's drag-and-drop canvas is an industry standard. CruxDev produces text documents.
- **No investor-ready export.** LivePlan generates pitch decks and formatted business plans. CruxDev outputs markdown.
- **No financial calculation engine.** LivePlan has built-in financial formulas (break-even, cash flow). CruxDev documents assumptions but does not compute projections natively.
- **No collaboration features.** Strategyzer supports team workshops. CruxDev is single-agent.

**Priority:** **Medium** — Business planning is a proven market. The biggest opportunity is connecting to benchmark data sources and generating formatted outputs (PDF business plans, pitch decks).

---

## 11. Existing Business Optimization (`business-existing`)

**What CruxDev offers:** Inventory of existing business docs, gap analysis, prioritized gap closure, convergence. Audit dimensions: process documentation, strategy-budget alignment, plan freshness, competitive awareness, risk tracking, KPI definition.

**Top competitors:**
1. **Monday.com / Asana** — Process management, task tracking, operational workflows
2. **Notion** — Knowledge base, process documentation, wikis, databases
3. **ProcessStreet** — Checklist-based process management, SOPs, workflow automation
4. **Lucidchart / Miro** — Process mapping, visual workflows, organizational diagrams
5. **Scoro / Planful** — Business management: budgets, projects, financial planning

**Gaps:**
- **No workflow automation.** Monday.com and ProcessStreet automate recurring processes. CruxDev documents processes but does not execute them.
- **No visual process mapping.** Lucidchart and Miro create flowcharts and diagrams. CruxDev is text-only.
- **No real-time KPI dashboard.** Scoro and Monday.com display live metrics. CruxDev defines KPIs but does not collect or display data.
- **No team task management.** Asana and Monday.com assign and track tasks across teams. CruxDev is single-agent.
- **No integrations with business tools.** Competitors connect to Slack, CRM, accounting software. CruxDev is isolated.

**Priority:** **Low** — CruxDev's value is in the audit/gap analysis layer, not in replacing operational tools. The strategy-budget alignment audit is differentiated.

---

## 12. Client Engagement (`consulting-client`)

**What CruxDev offers:** Engagement setup, proposal convergence, SOW generation, deliverable tracking, engagement convergence. Audit dimensions: scope clarity, deliverable completeness, proposal-SOW alignment, timeline tracking, communication quality, budget tracking.

**Top competitors:**
1. **HubSpot CRM** — Deal pipeline, proposal tracking, client communication history
2. **PandaDoc / Proposify** — Proposal and SOW generation, e-signatures, approval workflows
3. **Harvest / Toggl** — Time tracking, invoicing, budget vs. actual
4. **Accelo** — Client work management: projects, time, billing, retainers
5. **Qwilr** — Interactive proposals with embedded pricing, analytics on client viewing

**Gaps:**
- **No e-signature / approval workflow.** PandaDoc and Proposify handle signatures. CruxDev produces documents but cannot collect signatures.
- **No time tracking.** Harvest and Toggl track billable hours. CruxDev tracks deliverables, not time.
- **No invoicing.** Competitors generate and send invoices. CruxDev does not handle billing.
- **No CRM / client database.** HubSpot tracks client history across engagements. CruxDev manages one engagement at a time.
- **No client portal.** Accelo and Qwilr let clients view progress. CruxDev is internal-facing.

**Priority:** **Low** — CruxDev's proposal/SOW convergence is unique (no one else audits proposal-SOW alignment), but the operational gaps (billing, time tracking) are not worth building. Integration with PandaDoc or HubSpot APIs would be more valuable.

---

## 13. Marketing Campaign (`campaign`)

**What CruxDev offers:** Campaign strategy, messaging framework, creative production, channel consistency audit, campaign convergence. Audit dimensions: message consistency, audience alignment, CTA clarity, channel coverage, timeline feasibility, budget allocation.

**Top competitors:**
1. **HubSpot Marketing Hub** — Campaign management, email, social, ads, analytics, automation
2. **Jasper AI** — AI marketing copy: ads, emails, social posts, blog posts, brand voice
3. **Canva** — Design tool for marketing assets: social graphics, presentations, videos
4. **Hootsuite / Buffer** — Social media scheduling, multi-channel management, analytics
5. **Marketo / Pardot** — Enterprise marketing automation, lead scoring, campaign analytics

**Gaps:**
- **No ad platform integration.** HubSpot and Marketo connect to Google Ads, Facebook Ads. CruxDev produces copy but cannot publish or measure.
- **No design / visual asset creation.** Canva creates graphics. CruxDev is text-only.
- **No social media scheduling.** Hootsuite and Buffer schedule posts. CruxDev produces content but relies on external tools (Typefully integration exists for Twitter/X).
- **No campaign analytics.** HubSpot tracks conversions, ROI, attribution. CruxDev has no measurement layer.
- **No email automation.** Marketo and HubSpot automate drip sequences. CruxDev drafts emails but does not send them.

**Priority:** **Medium** — CruxDev's messaging consistency audit is unique. Extending the Typefully integration pattern to other channels (LinkedIn, email platforms) would close the most impactful gap.

---

## 14. Research (`research`)

**What CruxDev offers:** 5-pass iterative deepening, source verification, counter-research, quality-scored findings, convergence-detected completion. Audit dimensions: question coverage, source quality, counter-evidence, depth score, synthesis quality, reproducibility.

**Top competitors:**
1. **Elicit** — AI research assistant: paper search, data extraction, synthesis across papers
2. **Perplexity AI** — AI search with citations, follow-up questions, source linking
3. **Consensus** — AI-powered academic paper search with yes/no answers from literature
4. **Scite.ai** — Citation analysis: which papers support/contradict claims
5. **Notebook LM (Google)** — Upload sources, ask questions, generate summaries, audio overviews

**Gaps:**
- **No academic paper database access.** Elicit and Consensus search Semantic Scholar / PubMed. CruxDev uses web search, not structured academic databases.
- **No citation graph analysis.** Scite.ai maps supporting vs. contradicting citations. CruxDev verifies individual sources but does not analyze citation networks.
- **No source upload / ingestion.** Notebook LM lets users upload PDFs and query them. CruxDev works with web-accessible sources.
- **No structured data extraction.** Elicit extracts structured data from papers (sample size, methods, results). CruxDev synthesizes but does not extract structured data.
- **No collaborative research.** No way for multiple researchers to contribute to the same research project within CruxDev.

**Priority:** **High** — Research is a core CruxDev capability (5-pass methodology is genuinely differentiated). Adding academic database access and PDF ingestion would significantly strengthen the offering.

---

## 15. Financial Modeling (`financial`)

**What CruxDev offers:** Model definition, assumption documentation, model construction, scenario analysis, convergence. Audit dimensions: assumption coverage, internal consistency, scenario completeness, sensitivity analysis, time horizon coverage, presentation quality.

**Top competitors:**
1. **Excel / Google Sheets** — The industry standard for financial modeling, with formulas, charts, pivot tables
2. **Causal** — Visual financial modeling: drag-and-drop variables, scenario comparison, team collaboration
3. **Fathom** — Financial reporting and analysis connected to accounting data (Xero, QuickBooks)
4. **Mosaic** — Strategic finance platform: budgets, forecasts, actuals, departmental planning
5. **Jirav** — FP&A: driver-based planning, scenario analysis, dashboards, accounting integration

**Gaps:**
- **No calculation engine.** Excel and Causal compute formulas. CruxDev documents the model structure but does not natively compute numbers.
- **No accounting integration.** Fathom and Mosaic pull actual data from QuickBooks/Xero. CruxDev has no data source connections.
- **No interactive charts/dashboards.** Causal and Mosaic visualize models. CruxDev produces text.
- **No cell-level formula auditing.** Excel has trace precedents/dependents. CruxDev audits at the document level, not at formula level.
- **No collaborative modeling.** Causal and Google Sheets allow real-time collaboration.

**Priority:** **Low** — Financial modeling is deeply entrenched in spreadsheets. CruxDev's value is in assumption documentation and consistency auditing — a meta-layer, not a replacement. Not worth closing the computation gap.

---

## 16. Policy & Governance (`governance`)

**What CruxDev offers:** Policy inventory, framework mapping (SOC 2, ISO 27001, GDPR), policy drafting, cross-policy consistency, governance convergence. Audit dimensions: framework coverage, policy completeness, cross-policy consistency, version currency, enforceability, approval tracking.

**Top competitors:**
1. **Vanta** — Automated compliance (SOC 2, ISO 27001, HIPAA): evidence collection, policy templates, continuous monitoring
2. **Drata** — Compliance automation: control monitoring, employee training, vendor management
3. **Secureframe** — Compliance platform: policy generation, risk management, audit readiness
4. **OneTrust** — Privacy, security, and governance: GDPR consent, risk assessment, policy management
5. **PowerDMS** — Policy and procedure management: version control, distribution, acknowledgment tracking

**Gaps:**
- **No evidence collection automation.** Vanta and Drata automatically collect compliance evidence from AWS, GitHub, HR tools. CruxDev audits policy documents but does not collect evidence.
- **No continuous compliance monitoring.** Competitors monitor controls in real-time. CruxDev runs discrete audit passes.
- **No employee acknowledgment tracking.** PowerDMS tracks who has read and signed each policy. CruxDev has no distribution layer.
- **No vendor risk management.** Vanta and OneTrust assess third-party vendor risk. CruxDev does not.
- **No audit-ready report generation.** Competitors generate SOC 2 readiness reports for auditors. CruxDev produces policy documents but not auditor-facing reports.

**Priority:** **Medium** — Compliance is a large and growing market. CruxDev's policy drafting and cross-policy consistency checking is differentiated. The path forward is integration with Vanta/Drata for evidence collection, not replacing them.

---

## 17. Legal Document Management (`legal`)

**What CruxDev offers:** Document inventory, consistency audit, completeness check, draft convergence, document set convergence. Audit dimensions: defined term consistency, clause completeness, cross-document alignment, date/expiry tracking, plain language score, version control.

**Top competitors:**
1. **Ironclad** — Contract lifecycle management: creation, negotiation, approval, storage, analytics
2. **DocuSign CLM** — Contract management with e-signature, workflows, AI-assisted review
3. **Juro** — AI contract management: drafting, collaboration, approval, analytics
4. **Spellbook (by Rally)** — AI contract drafting, clause library, risk identification
5. **ContractPodAI** — Enterprise CLM with AI: extraction, obligation management, analytics

**Gaps:**
- **No contract lifecycle management.** Ironclad and DocuSign track contracts from creation through expiry with renewal alerts. CruxDev inventories and audits but does not manage the lifecycle.
- **No e-signature integration.** DocuSign is the market standard. CruxDev has no signing capability.
- **No clause library.** Spellbook and Juro maintain reusable clause libraries. CruxDev has a checklist but not a library of pre-approved clauses.
- **No obligation extraction / tracking.** ContractPodAI extracts obligations and deadlines from signed contracts. CruxDev's tracking is manual.
- **No negotiation workflow.** Ironclad and Juro support redline/comment/approve workflows. CruxDev is single-agent.

**Priority:** **Low** — Legal tech is heavily regulated and enterprise-focused. CruxDev's defined-term consistency and cross-document alignment auditing is niche but useful. Not worth building CLM features.

---

## 18. Composite (`composite`)

**What CruxDev offers:** Multi-type classification, dimension merging, per-type convergence, cross-type consistency, composite convergence. Audit dimensions: per-type dims + cross-type alignment, priority coherence, timeline sync.

**Top competitors:**
1. **Notion** — Flexible workspace that can model any project type: wikis, databases, kanban, calendars
2. **Monday.com** — Multi-project management with custom workflows per project type
3. **Airtable** — Database-driven project management, custom views per use case
4. **Linear** — Software project management that extends into roadmaps and initiatives
5. **Jira + Confluence** — Enterprise project management + knowledge management combo

**Gaps:**
- **No visual cross-project dashboard.** Notion and Monday.com show all projects in a unified view. CruxDev's composite convergence is file-based.
- **No drag-and-drop project organization.** Competitors offer kanban, calendar, and timeline views. CruxDev is terminal-only.
- **No team assignment / workload management.** Monday.com tracks who is doing what. CruxDev is single-agent.
- **No real-time collaboration.** All competitors support multi-user editing. CruxDev is designed for autonomous AI execution.
- **No third-party integrations.** Notion and Monday.com connect to hundreds of tools. CruxDev has limited integrations (Typefully, GitHub).

**Priority:** **Medium** — The composite type is CruxDev's unique differentiator — no competitor converges quality across project types. A lightweight web dashboard showing convergence status across sub-projects would make this tangible.

---

## Summary: Priority Matrix

| Priority | Project Types | Common Gap Theme |
|----------|--------------|-----------------|
| **High** | software-new, software-existing, research | Core competency areas where competitors are strong. IDE integration, CI/CD integration, academic data access needed. |
| **Medium** | newsletter, youtube, campaign, open-source, business-new, governance, composite | Integration with delivery platforms (email, YouTube, social, compliance tools) is the pattern. CruxDev excels at content quality but cannot publish or measure. |
| **Low** | book, book-series, course, podcast, business-existing, consulting-client, financial, legal | Niche markets or by-design gaps. CruxDev is a quality layer, not a replacement for domain-specific tools. |

## Cross-Cutting Gaps (Apply to Multiple Types)

1. **No web dashboard / visualization.** Every type suffers from being terminal-only. Convergence status, gap tracking, and audit results have no visual presentation.
2. **No platform integrations beyond GitHub and Typefully.** Newsletter, YouTube, podcast, campaign, compliance — all would benefit from API integrations with delivery platforms.
3. **No collaborative / multi-user support.** CruxDev is single-agent by design, but teams expect collaboration. Even read-only sharing of convergence status would help.
4. **No formatted export (PDF, DOCX, PPTX).** Book, business plan, proposal, financial model — all produce markdown. Formatted export would unlock value in every non-software type.
5. **No analytics / measurement feedback loop.** CruxDev measures content quality but cannot measure audience response. Connecting to analytics (YouTube, email, web) would enable data-driven convergence.
