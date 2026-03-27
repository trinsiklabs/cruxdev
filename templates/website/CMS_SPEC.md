---
title: CMS Selection & Configuration
last_updated: [YYYY-MM-DD]
project: [Project Name]
---

# CMS Selection & Configuration

> CMS platform choice, content model, editorial workflow, and configuration.

## 1. CMS Selection

### 1.1 Requirements

| Requirement | Priority | Notes |
|---|---|---|
| [e.g., Non-technical content editing] | P0 | [Marketing team edits without dev help] |
| [e.g., Git-based content] | P1 | [Content versioned in Git] |
| [e.g., API / headless support] | P0 | [Decoupled from front-end] |
| [e.g., Image optimization] | P1 | [Built-in or plugin] |
| [e.g., Markdown support] | P1 | [Native markdown editing] |
| [e.g., Scheduling / drafts] | P1 | [Schedule future publications] |
| [e.g., Multi-user roles] | P2 | [Editor, admin, reviewer roles] |
| [e.g., Cost < $X/month] | P0 | [Budget constraint] |

### 1.2 Evaluation Matrix

| Criterion | [CMS Option A] | [CMS Option B] | [CMS Option C] |
|---|---|---|---|
| Type | [Headless / Traditional / Git-based] | [Type] | [Type] |
| Pricing | [Free tier / $X/month] | [Pricing] | [Pricing] |
| Content editing UX | [Rating + notes] | [Rating] | [Rating] |
| Developer experience | [Rating + notes] | [Rating] | [Rating] |
| API quality | [Rating + notes] | [Rating] | [Rating] |
| Image handling | [Rating + notes] | [Rating] | [Rating] |
| Vendor lock-in risk | [Low/Medium/High] | [Risk] | [Risk] |
| Community / docs | [Rating] | [Rating] | [Rating] |
| **Decision** | **[Selected / Rejected]** | **[Selected / Rejected]** | **[Selected / Rejected]** |

**Selected CMS:** [Name]
**Rationale:** [Why this CMS was chosen over alternatives]

---

## 2. Content Model

### 2.1 Content Types

| Content Type | Fields | Collection/Single | Notes |
|---|---|---|---|
| Page | [title, slug, body, meta_title, meta_description, og_image] | Collection | [Marketing pages] |
| Blog Post | [title, slug, excerpt, body, author, category, tags, featured_image, publish_date, updated_date] | Collection | [Blog content] |
| Author | [name, bio, headshot, social_links] | Collection | [Blog post authors] |
| Category | [name, slug, description] | Collection | [Blog categories] |
| Testimonial | [quote, person_name, title, company, headshot] | Collection | [Social proof] |
| FAQ Item | [question, answer, category] | Collection | [FAQ sections] |
| Navigation | [items: [{label, url, children}]] | Single | [Site navigation structure] |
| Site Settings | [site_title, description, social_links, analytics_id] | Single | [Global settings] |

### 2.2 Field Definitions

[Detail the fields for your most important content type.]

**Blog Post Fields:**

| Field | Type | Required | Validation | Notes |
|---|---|---|---|---|
| title | String | Yes | Max 100 chars | [Used in H1 and title tag] |
| slug | String (auto) | Yes | URL-safe, unique | [Auto-generated from title, editable] |
| excerpt | Text | Yes | Max 300 chars | [Used in cards and meta description] |
| body | Rich Text / Markdown | Yes | — | [Main content] |
| author | Reference → Author | Yes | — | [Linked author record] |
| category | Reference → Category | Yes | — | [Primary category] |
| tags | String[] | No | — | [Additional categorization] |
| featured_image | Image | Yes | Min 1200x630 | [Hero + OG image] |
| publish_date | DateTime | Yes | — | [When to publish] |
| status | Enum | Yes | draft, review, published | [Editorial workflow] |

---

## 3. Editorial Workflow

| Stage | Who | Action | Next Stage |
|---|---|---|---|
| Draft | Author | Creates content | Review |
| Review | Editor | Reviews for quality, SEO, brand voice | Approved / Revisions |
| Revisions | Author | Addresses feedback | Review (re-review) |
| Approved | Editor | Marks as ready to publish | Scheduled / Published |
| Scheduled | System | Auto-publishes at set date/time | Published |
| Published | System | Live on site | — |

### User Roles

| Role | Permissions |
|---|---|
| Admin | Full access — settings, content, users, deployment |
| Editor | Create, edit, publish, delete content. Manage categories/tags. |
| Author | Create and edit own content. Submit for review. Cannot publish. |
| Viewer | Read-only access to content (for stakeholder review) |

---

## 4. CMS Configuration

| Setting | Value |
|---|---|
| API endpoint | [e.g., "https://api.cms.example.com/v1/" or "N/A for git-based"] |
| API key storage | [Environment variable in hosting platform — never in repo] |
| Webhook URL | [e.g., "Triggers rebuild on content publish: https://api.vercel.com/deploy/hook/xxx"] |
| Preview URL | [e.g., "https://preview.example.com/?secret=xxx&slug={slug}"] |
| Media storage | [e.g., "CMS built-in / Cloudinary / S3 bucket"] |
| Localization | [e.g., "Not needed" or "EN + [languages]"] |

---

## 5. Related Documents

- [Hosting Spec](HOSTING_SPEC.md)
- [Blog Spec](../pages/BLOG_SPEC.md)
- [Integrations](INTEGRATIONS.md)
