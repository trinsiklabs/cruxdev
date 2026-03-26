---
title: "CruxDev Deployment"
last_updated: "2026-03-26"
---

# CruxDev Deployment

## Website Deployment

**Site:** https://cruxdev.dev
**Repo:** `/Users/user/personal/cruxdev-dev`
**Framework:** Astro (static site generator)

### Build

```bash
cd /Users/user/personal/cruxdev-dev
npm install
npm run build
```

Output: `./dist/` directory with static files.

### Deploy

Production deployment is not yet configured. Options under consideration:
- Cloudflare Pages (recommended — free, fast, global CDN)
- Vercel
- Netlify

### Pre-Deployment Checklist

- [ ] Metrics on homepage match current codebase (tests, tools, coverage)
- [ ] Comparison pages generated from latest COMPETITORS.md
- [ ] All internal links verified (no 404s)
- [ ] `robots.txt` and `llms.txt` current
- [ ] Favicon present
- [ ] Meta descriptions on all pages
- [ ] OG images for social sharing
- [ ] Structured data (FAQPage on comparison pages)

### Metric Update Process

When the codebase changes (new tests, new MCP tools), the homepage metrics must be updated:

1. Get current counts from the codebase:
   ```bash
   cd /Users/user/personal/cruxdev
   python3 -m pytest tests/ -q 2>&1 | tail -1  # test count
   grep -c "@mcp.tool()" src/mcp_server.py       # MCP tool count
   ```
2. Update `src/pages/index.astro` in the cruxdev-dev repo
3. Rebuild and redeploy

The WEBSITE_CONVERGENCE phase in the convergence engine auto-triggers this check.

## PyPI Deployment

CruxDev is not yet published to PyPI. When ready:

```bash
python3 -m build
python3 -m twine upload dist/*
```

## GitHub

**Repo:** https://github.com/trinsiklabs/cruxdev
**CI:** `.github/workflows/evolve.yml` (daily evolution at 6 AM UTC)
