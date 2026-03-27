# Dual-License Framework

**Template Version:** 1.0
**Last Updated:** 2026-03-24
**Legal Review Required:** Yes

---

## DUAL-LICENSE FRAMEWORK

This document provides a framework for offering software under a dual-license model: an open source license for community use and a commercial license for proprietary use.

---

### 1. OVERVIEW

Dual licensing lets you offer the same software under two different licenses:

1. **Open Source License** — Free for anyone who complies with its terms (typically copyleft, requiring derivative works to also be open source).
2. **Commercial License** — Paid license for users who want to use the software in proprietary products without copyleft obligations.

This model works when you are the sole copyright holder (or have CLA agreements from all contributors granting you the necessary rights).

### 2. WHEN TO USE DUAL LICENSING

Dual licensing is appropriate when:
- You own or control all copyrights in the software
- The software has commercial value as a component or library
- Some users need to embed it in proprietary products
- You want to support open source while generating revenue
- You have (or can implement) a CLA process for external contributions

Dual licensing is NOT appropriate when:
- You have contributions from third parties without CLAs
- The software is purely an end-user application (no embedding use case)
- You cannot enforce the copyleft license effectively

### 3. RECOMMENDED LICENSE COMBINATIONS

| Open Source License | Commercial License | Best For |
|--------------------|-------------------|----------|
| AGPLv3 | Commercial (see template) | SaaS-relevant libraries |
| GPLv3 | Commercial (see template) | Desktop/embedded libraries |
| MPL 2.0 | Commercial (see template) | File-level copyleft with flexibility |

**Recommendation:** AGPLv3 + Commercial is the strongest dual-license combination because AGPL's network copyleft clause means even SaaS users must either open-source their code or purchase a commercial license.

### 4. PROJECT SETUP

#### 4.1 Repository Structure
```
project/
├── LICENSE                  # Open source license (full text)
├── LICENSE-COMMERCIAL.md    # Commercial license summary + how to purchase
├── NOTICE                   # Attribution notices
├── CLA.md                   # Contributor License Agreement
├── README.md                # Includes licensing section
└── src/
    └── *.ext                # Source files with dual-license headers
```

#### 4.2 Source File Header

Add this header to every source file:

```
// {{SOFTWARE_NAME}}
// Copyright (c) {{YEAR}} {{COPYRIGHT_HOLDER}}
//
// This software is dual-licensed:
//
// 1. Open Source: {{OPEN_SOURCE_LICENSE}} — see LICENSE file
//    You may use, modify, and distribute this software under the terms
//    of the {{OPEN_SOURCE_LICENSE}}.
//
// 2. Commercial: For use in proprietary software without {{OPEN_SOURCE_LICENSE}}
//    obligations, purchase a commercial license at {{COMMERCIAL_URL}}
//    or contact {{SALES_EMAIL}}.
//
// See LICENSE and LICENSE-COMMERCIAL.md for full terms.
```

#### 4.3 README Licensing Section

```markdown
## License

{{SOFTWARE_NAME}} is dual-licensed:

- **Open Source:** [{{OPEN_SOURCE_LICENSE}}](LICENSE) — free for open source projects
  that comply with {{OPEN_SOURCE_LICENSE}} terms.
- **Commercial:** [Commercial License](LICENSE-COMMERCIAL.md) — for proprietary use
  without copyleft obligations. [Contact sales](mailto:{{SALES_EMAIL}}) or
  visit [{{COMMERCIAL_URL}}]({{COMMERCIAL_URL}}).

### Which license do I need?

| Your Use Case | License Needed |
|--------------|---------------|
| Open source project ({{OPEN_SOURCE_LICENSE}}-compatible) | Open Source (free) |
| Internal tools (not distributed) | Open Source (free)* |
| Proprietary product (distributed) | Commercial |
| SaaS using this as a component | Commercial** |
| Evaluation / testing | Open Source (free) |

*Internal use without distribution is generally permitted under copyleft licenses.
**Required if using AGPL; may not be required under GPL alone (consult counsel).
```

### 5. COMMERCIAL LICENSE TERMS TEMPLATE

Use the Commercial License Agreement template (`license-commercial.md`) with these dual-license-specific additions:

**Additional Grant:** The commercial license grants Licensee the right to use, modify, and distribute the Software (and derivative works) in proprietary products without obligation to disclose source code or apply the {{OPEN_SOURCE_LICENSE}} to such products.

**Pricing Tiers:**

| Tier | Description | Annual Fee |
|------|-------------|-----------|
| Startup | Up to {{STARTUP_REVENUE_CAP}} annual revenue, {{STARTUP_DEV_LIMIT}} developers | ${{STARTUP_PRICE}} |
| Business | Up to {{BUSINESS_REVENUE_CAP}} annual revenue, {{BUSINESS_DEV_LIMIT}} developers | ${{BUSINESS_PRICE}} |
| Enterprise | Unlimited revenue and developers | ${{ENTERPRISE_PRICE}} |
| OEM / Redistribution | Embedding in products sold to third parties | Custom pricing |

### 6. CONTRIBUTOR LICENSE AGREEMENT

A CLA is essential for dual licensing. Without it, external contributors retain copyright and you cannot offer their contributions under the commercial license.

Key CLA provisions:
1. Contributor grants you a perpetual, worldwide, non-exclusive, royalty-free license to use, reproduce, modify, display, perform, sublicense, and distribute contributions.
2. Contributor represents they have the right to make the contribution.
3. Contributor understands the contribution will be available under both the open source and commercial licenses.

Options:
- **Full CLA** — Maximum protection; some contributors may resist
- **DCO + License Grant** — Lighter touch; `Signed-off-by` plus license grant in PR template

### 7. ENFORCEMENT STRATEGY

7.1 **Monitoring:** Periodically check for use of your software in proprietary products (code scanning, GitHub search, dependency tracking).

7.2 **Compliance Path:**
1. Contact the user privately, explain the licensing
2. Offer a grace period to either purchase a commercial license or comply with the open source license
3. If unresolved, formal cease-and-desist
4. If still unresolved, legal action (rare but necessary for deterrence)

7.3 **Revenue Optimization:**
- Make the commercial license easy to purchase (self-serve if possible)
- Price it reasonably relative to the value delivered
- Offer annual subscriptions with maintenance included
- Provide volume discounts for large organizations

### 8. CHECKLIST

- [ ] All source code has dual-license header
- [ ] LICENSE file contains open source license text
- [ ] LICENSE-COMMERCIAL.md explains commercial option
- [ ] CLA process in place for external contributors
- [ ] All existing contributors have signed CLA (or contributed under CLA-equivalent terms)
- [ ] README includes licensing section
- [ ] Commercial license purchase process is documented
- [ ] Pricing tiers defined
- [ ] Sales contact established ({{SALES_EMAIL}})
- [ ] Legal counsel has reviewed both licenses
- [ ] Website/landing page explains dual licensing
