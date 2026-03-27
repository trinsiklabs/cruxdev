# Open Source License Selection Guide

**Template Version:** 1.0
**Last Updated:** 2026-03-24
**Legal Review Required:** Recommended before first use

---

## OPEN SOURCE LICENSE SELECTION GUIDE

This guide helps you select the appropriate open source license for your project. Each license has different implications for how others can use, modify, and distribute your code.

---

### 1. DECISION FRAMEWORK

Answer these questions to narrow your choice:

| Question | If Yes | If No |
|----------|--------|-------|
| Must derivative works remain open source? | Copyleft (GPL, AGPL) | Permissive (MIT, Apache, BSD) |
| Is this a web service (SaaS)? | AGPL if copyleft desired | GPL sufficient for copyleft |
| Do you want patent protection? | Apache 2.0 | MIT or BSD |
| Must attribution be preserved? | Any OSS license | Public domain (CC0, Unlicense) |
| Will this be used in commercial products? | Permissive recommended | Any |
| Do you want maximum adoption? | MIT or Apache 2.0 | Depends on goals |

### 2. COMMON LICENSES

#### 2.1 MIT License (Permissive)
**Best for:** Maximum adoption, minimal restrictions
**Allows:** Commercial use, modification, distribution, private use
**Requires:** License and copyright notice
**Forbids:** Liability claims against author

```
MIT License

Copyright (c) {{YEAR}} {{COPYRIGHT_HOLDER}}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

#### 2.2 Apache License 2.0 (Permissive + Patent Grant)
**Best for:** Projects where patent protection matters
**Allows:** Commercial use, modification, distribution, patent use, private use
**Requires:** License and copyright notice, state changes, NOTICE file
**Forbids:** Trademark use, liability claims

*Full text at: https://www.apache.org/licenses/LICENSE-2.0*

Key differentiators from MIT:
- Explicit patent grant from contributors
- Patent retaliation clause (patent license terminates if you sue for patent infringement)
- Requires documenting changes to the code
- Requires preserving NOTICE file

#### 2.3 BSD 2-Clause (Permissive)
**Best for:** Similar to MIT, preferred in BSD ecosystem
**Allows:** Commercial use, modification, distribution
**Requires:** License and copyright notice

```
BSD 2-Clause License

Copyright (c) {{YEAR}}, {{COPYRIGHT_HOLDER}}

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

#### 2.4 GNU General Public License v3 (Strong Copyleft)
**Best for:** Ensuring all derivative works remain open source
**Allows:** Commercial use, modification, distribution, patent use
**Requires:** Source code disclosure, license and copyright notice, state changes, same license for derivatives
**Forbids:** Sublicensing, liability claims

*Full text at: https://www.gnu.org/licenses/gpl-3.0.en.html*

Key characteristics:
- Derivative works must use GPL
- Must provide source code with binary distributions
- "Tivoization" protections (anti-DRM)
- Compatible with Apache 2.0 (one-way: Apache code can be included in GPL projects)

#### 2.5 GNU Affero General Public License v3 (Network Copyleft)
**Best for:** SaaS/web applications where you want copyleft to apply
**Same as GPLv3 plus:** Users who interact with the software over a network must be able to receive the source code

*Full text at: https://www.gnu.org/licenses/agpl-3.0.en.html*

#### 2.6 Mozilla Public License 2.0 (Weak Copyleft)
**Best for:** File-level copyleft — modified files must stay open, but can be combined with proprietary code
**Allows:** Commercial use, modification, distribution, patent use
**Requires:** Source for modified files, license and copyright notice

*Full text at: https://www.mozilla.org/en-US/MPL/2.0/*

### 3. LICENSE COMPATIBILITY MATRIX

| License | MIT | Apache 2.0 | GPLv3 | AGPLv3 | MPL 2.0 |
|---------|-----|-----------|-------|--------|---------|
| MIT | Yes | Yes | Yes | Yes | Yes |
| Apache 2.0 | No* | Yes | Yes | Yes | Yes |
| GPLv3 | No | No | Yes | Yes | No** |
| AGPLv3 | No | No | No | Yes | No** |
| MPL 2.0 | No* | No* | Yes | Yes | Yes |

*Can include the more permissive code in the more restrictive project, but not vice versa.*
**GPLv3 Section 13 allows combining with AGPL.*

### 4. APPLYING A LICENSE

1. Create a `LICENSE` or `LICENSE.md` file in the project root with the full license text
2. Add a copyright notice to the top of each source file (recommended):
   ```
   // Copyright (c) {{YEAR}} {{COPYRIGHT_HOLDER}}
   // Licensed under the {{LICENSE_NAME}}. See LICENSE file for details.
   ```
3. Add license badge and reference to README
4. If using Apache 2.0, create a `NOTICE` file for third-party attributions
5. Document any third-party dependencies and their licenses
6. Run a license compatibility check before release

### 5. CONTRIBUTOR LICENSE AGREEMENT (CLA)

If accepting external contributions, consider requiring a CLA to ensure:
- Contributors have the right to contribute their code
- You receive the necessary rights to distribute contributions under your chosen license
- Contributors cannot later revoke their contribution

Options:
- **Individual CLA** — For individual contributors
- **Corporate CLA** — For contributions made by employees of a company
- **Developer Certificate of Origin (DCO)** — Lighter alternative, `Signed-off-by` line in commits

### 6. CHECKLIST BEFORE RELEASING

- [ ] License file in project root
- [ ] Copyright notices in source files
- [ ] README references the license
- [ ] All third-party dependencies have compatible licenses
- [ ] NOTICE file (if Apache 2.0)
- [ ] CLA or DCO process in place (if accepting contributions)
- [ ] License compatibility verified for all dependencies
- [ ] Legal counsel has reviewed (recommended for first release)
