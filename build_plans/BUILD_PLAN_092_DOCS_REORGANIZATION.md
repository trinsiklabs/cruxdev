# BUILD_PLAN_092: Docs Reorganization — Pattern Files into Folders

**Status:** NOT STARTED
**Priority:** High (80 files in flat directory is unmanageable)

## Context

`docs/` has 80+ files in a flat directory — 21 stack patterns, 15+ methodology patterns, competitive docs, guides, references. Finding anything requires knowing the exact filename. This needs folder structure.

## Proposed Structure

```
docs/
├── patterns/
│   ├── methodology/           # How CruxDev works
│   │   ├── CONVERGENCE.md     # (was DEVELOPMENT_PATTERNS_CRUXDEV.md)
│   │   ├── FORM_PATTERNS.md
│   │   ├── METRICS_PATTERNS.md
│   │   ├── DASHBOARD_PATTERNS.md
│   │   ├── BLOG_PATTERNS.md
│   │   ├── BLOG_PAGINATION_PATTERNS.md
│   │   ├── BLOG_POST_PATTERNS.md
│   │   ├── BLOG_TAGGING_PATTERNS.md
│   │   ├── COLOR_CONTRAST_PATTERNS.md
│   │   ├── DRY_UI_COMPONENT_PATTERNS.md
│   │   ├── MOBILE_WEB_PATTERNS.md
│   │   ├── POST_DEPLOYMENT_PATTERNS.md
│   │   ├── KV_CACHE_PATTERNS.md
│   │   ├── REGRESSION_DETECTION_PATTERNS.md
│   │   ├── CROSS_MODEL_VALIDATION_PATTERNS.md
│   │   ├── LIFECYCLE_HOOK_PATTERNS.md
│   │   ├── VISUAL_VERIFICATION_PATTERNS.md
│   │   ├── MULTI_AGENT_PATTERNS.md
│   │   ├── KERNEL_SANDBOXING_PATTERNS.md
│   │   ├── SKILLS_AUTO_ACTIVATION_PATTERNS.md
│   │   └── GEO_PATTERNS.md
│   ├── stacks/                # Stack-specific development patterns
│   │   ├── PETAL.md
│   │   ├── PADL.md
│   │   ├── DJANGO.md
│   │   ├── NEXTJS.md
│   │   ├── RAILS.md
│   │   ├── TALL.md
│   │   ├── FASTAPI.md
│   │   ├── SPRING.md
│   │   ├── BLAZOR.md
│   │   ├── EXPO.md
│   │   ├── FLUTTER.md
│   │   ├── SVELTEKIT.md
│   │   ├── NESTJS.md
│   │   ├── ANGULAR.md
│   │   ├── GOTH.md
│   │   ├── NUXT.md
│   │   ├── SWIFTUI.md
│   │   ├── AXUM.md
│   │   ├── ASTRO.md
│   │   └── KMP.md
│   └── verticals/             # Vertical-specific patterns (NEW — from BP091 gaps)
│       ├── AUTHORS.md         # Patterns for book/writing projects
│       ├── PODCASTERS.md      # Patterns for podcast production
│       ├── COURSE_CREATORS.md # Patterns for online courses
│       ├── NEWSLETTERS.md     # Patterns for email newsletters
│       ├── COACHES.md         # Patterns for coaching practices
│       └── (more as verticals are researched)
├── competitive/
│   ├── COMPETITORS.md
│   ├── COMPETITORS_PATTERN.md
│   ├── AI_HARNESS_LANDSCAPE.md
│   └── BDD_REFERENCE.md
├── guides/
│   ├── ADOPTION_PROCESS.md
│   ├── ADOPTION_PLAYBOOK.md
│   ├── AUTONOMOUS_SELF_IMPROVEMENT_PATTERNS.md
│   ├── SEARCH_REGISTRATION_GUIDE.md
│   ├── CLOUDFLARE_SETUP_GUIDE.md
│   └── CLOUDFLARE_SETUP_REPORT.md
├── growth/
│   ├── GROWTH_STRATEGY.md
│   ├── GROWTH_CYCLE_PUBLIC_INTERACTIONS.md
│   ├── WEBSITE_PLANNING.md
│   ├── WEBSITE_LOGO_PATTERNS.md
│   ├── X_POST_PATTERNS.md
│   └── SEO/GEO docs
├── architecture/
│   ├── ARCHITECTURE.md
│   ├── CruxDev.md
│   └── SESSION_UPGRADE.md
└── testing/
    ├── E2E_TEST_PATTERNS.md
    ├── UAT_TEST_PATTERNS.md
    └── RESEARCH_PATTERNS.md
```

## Phase 1: Move Files

- [ ] 1.1 Create folder structure
- [ ] 1.2 Move files with git mv (preserves history)
- [ ] 1.3 Update ALL internal references (grep for old paths, fix)
- [ ] 1.4 Update growth.toml methodology_docs paths
- [ ] 1.5 Update router.rs paths (detect_website, auto_discover_docs)
- [ ] 1.6 Update CLAUDE.md references
- [ ] 1.7 Update build plan references
- [ ] 1.8 Update adoption process references
- [ ] 1.9 Run full test suite — nothing should break

## Phase 2: Vertical Pattern Files (NEW)

As BP091 persona pages surface gaps, create vertical-specific patterns:
- [ ] 2.1 AUTHORS_PATTERNS.md — manuscript tracking, voice audit, chapter convergence, publishing workflow
- [ ] 2.2 PODCAST_PATTERNS.md — episode format, show notes, guest management, distribution
- [ ] 2.3 COURSE_PATTERNS.md — curriculum design, module structure, assessment, learner journey
- [ ] 2.4 NEWSLETTER_PATTERNS.md — edition template, subscriber growth, content calendar
- [ ] 2.5 COACHING_PATTERNS.md — framework delivery, session structure, client progress
- [ ] 2.6 Each pattern doc follows the PETAL model (17 sections + Report Improvements)

## Phase 3: Update Existing Patterns

As tool/type/persona pages expose gaps, update existing patterns:
- [ ] 3.1 Track every "Coming soon" and "Build plan needed" in the generated pages
- [ ] 3.2 Create build plans for each gap
- [ ] 3.3 Update patterns docs as gaps are closed

## Risk

Moving 80+ files breaks every internal reference. Must grep exhaustively:
- `docs/FORM_PATTERNS.md` → `docs/patterns/methodology/FORM_PATTERNS.md`
- Every CLAUDE.md, build plan, patterns doc, and Rust source file needs updating

## Alternative: Symlinks

Instead of moving, create the folder structure with symlinks to the original files:
```bash
ln -s ../../FORM_PATTERNS.md docs/patterns/methodology/FORM_PATTERNS.md
```
Pros: nothing breaks. Cons: two ways to reference every file.

## Recommendation

Do the full move (Phase 1) in one atomic commit. Update all references. Run tests. If anything breaks, fix it before pushing. The symlink approach creates confusion long-term.
