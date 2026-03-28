# Form Design Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** 40+ sources including NNg, Baymard Institute, GOV.UK Design System, USWDS, W3C WAI, WCAG 2.2
**Last updated:** 2026-03-27

## Why This Matters

67% of cart abandonments trace to form issues. Forms following usability guidelines achieve 78% first-try submission vs. 42% for non-compliant forms (Seckler et al., CHI study). Expedia gained $12M/year from removing one optional field. Form design is a conversion lever, not a cosmetic concern.

---

## 1. Core Principles

1. **Every field must justify its existence.** Apply the NNg EAS framework: Eliminate unnecessary fields, Automate what you can (autofill, geolocation), Simplify the rest.
2. **Progressive disclosure.** Show only what's needed now. Complex forms break into steps.
3. **Forgiving format.** Accept messy input — ignore stray spaces, hyphens, punctuation. Reject only what cannot be correct or is too ambiguous to use (GOV.UK).
4. **Immediate, contextual feedback.** But not too immediate — see validation section.

---

## 2. Layout

**Single-column is the default.** Multi-column forms take 15.4 seconds longer to complete (CXL research). Users misinterpret field relationships and skip fields in multi-column layouts (Baymard).

**Exceptions:**
- Logically grouped short fields on one row: City / State / Zip
- First name / Last name (when both required)

**Never:** Arbitrary side-by-side layout for aesthetic reasons.

---

## 3. Labels

**Top-aligned labels are fastest.** Eye-tracking research (Penzo, UXmatters) shows top-aligned labels require half as many visual fixations and produce the fastest completion times. Left-aligned labels have the slowest completion times.

**Rules:**
- Labels above inputs, left-aligned text
- Use `<label>` element with explicit `for` attribute — never rely on placeholder text as the label
- Placeholders are hints, not labels — they disappear on focus and fail screen readers
- Mark optional fields "(optional)" — don't mark required fields with asterisks (assume required by default)

**Floating labels:** Popular but problematic. Shrunk text fails low-vision users. Animation triggers motion sensitivity. If used, ensure the floating state meets WCAG minimum contrast and text size.

---

## 4. Validation

**The orthodox "inline validation" advice is oversimplified.** Two controlled studies (n=77 and n=90) found users make significantly more errors with instant inline validation due to mode-switching between filling and fixing (Vitaly Friedman, Smart Interface Design Patterns).

**Recommended pattern — Reward early, punish late:**

| Scenario | When to validate |
|----------|-----------------|
| Field was previously flagged as error | Immediately on each keystroke (reward the correction) |
| Field was valid, user leaves it | On blur (punish late — only after they've finished) |
| Empty required field | Only on submit (don't validate untouched fields) |
| Short form (< 5 fields) | On submit only |

**GOV.UK approach (strongest research backing):**
- Validate only when user clicks continue/submit
- Disable HTML5 native validation (`novalidate` attribute)
- Native browser validation is unreliable across assistive technologies

**Server-side validation is always primary.** Client-side is a convenience, not a guarantee.

---

## 5. Error Display

Errors must be **inline, multi-cue, specific, and constructive:**

1. **Position:** Directly beneath the corresponding input — not in a summary block alone
2. **Multi-cue:** Red text + icon + heavier border (color alone fails 8% of males with color vision deficiency)
3. **Specific:** "Enter an email address in the format name@example.com" — not "Invalid input"
4. **Constructive:** Tell users how to fix it, not just what's wrong
5. **Preserve input:** Never clear the field on error — let users correct, not retype
6. **Error summary:** On submit, show a linked summary at the top of the form AND inline errors. Set keyboard focus to the summary.
7. **Page title:** Prepend "Error: " to the page title so screen readers announce it

**Two error types require different handling:**
- **Slips** (right intention, wrong execution): "Did you mean name@gmail.com?"
- **Mistakes** (wrong mental model): Explain the requirement, don't just reject

---

## 6. Multi-Step Forms

**Use when:** 6+ fields, distinct topic groups, complex data collection, unfamiliar tasks performed rarely.
**Don't use when:** 2-5 fields, single topic, users want full transparency upfront.

**The "300% conversion increase" claim has selection bias** — multi-step forms are typically used in high-intent scenarios. The Intertop case shows moving FROM multi-page TO single-page increased conversions for simple flows.

**Rules:**
- Maximum 10 steps (NNg, PatternFly)
- Show progress — but design progress indicators to start fast and end slow (PMC research). Showing slow early progress increases abandonment.
- Save state between steps — poor persistence reduces conversion by up to 10%
- Allow backward navigation without data loss
- Show a review/confirmation step before final submit

---

## 7. Accessibility (WCAG 2.2)

Non-negotiable requirements:

| Requirement | WCAG | Level |
|-------------|------|-------|
| Every input has a programmatic `<label>` | 1.3.1 | A |
| Errors identified in text, not color alone | 3.3.1 | A |
| Labels or instructions provided | 3.3.2 | A |
| Error suggestions when detected | 3.3.3 | AA |
| Error prevention for legal/financial data | 3.3.4 | AA |
| **Redundant Entry:** Auto-populate previously entered data | 3.3.7 (new in 2.2) | A |
| **Accessible Auth:** No cognitive puzzles required | 3.3.8 (new in 2.2) | AA |
| **Target Size:** Touch targets >= 24x24 CSS px | 2.5.8 (new in 2.2) | AA |

**Additional requirements:**
- Group related controls with `<fieldset>` and `<legend>`
- Use semantic HTML5 form elements
- Support keyboard navigation (Tab, Shift+Tab, Enter, Space)
- Focus management on errors — move focus to error summary or first error field
- Announce dynamic changes via ARIA live regions

---

## 8. Mobile

- **Touch targets:** 48px minimum (Material), 44pt minimum (Apple HIG). WCAG 2.2 legal minimum is 24x24 CSS px — target the platform guidelines, not the legal minimum.
- **Spacing between targets:** 8-12px minimum, 16-24px preferred
- **Input types:** Use `type="email"`, `type="tel"`, `type="number"` — reduces input time by 30% and typing errors by 50%
- **Layout:** Single-column mandatory
- **Keyboard:** The soft keyboard covers ~50% of the screen. Ensure the active field scrolls into view.
- **Autofill:** Support `autocomplete` attributes — browsers can fill name, email, address, payment automatically

---

## 9. CTAs (Call-to-Action Buttons)

- **Describe the outcome, not the action.** "Create Account" not "Submit". "Download Free Guide" not "Submit Form".
- **1-3 words**, active voice, present tense
- Specific CTA text improves conversion up to 161% vs generic "Submit" (14% conversion rate)
- **Never include Reset/Clear buttons** — accidental deletion risk far outweighs the use case (NNg)
- **Loading state:** Show spinner or disable button after click. Prevent double-submission.
- **Visual weight:** Primary CTA should be the most prominent element. Secondary actions (Cancel, Back) should be visually subordinate.

---

## 10. Trust & Conversion

- **Field count:** Each additional field decreases conversion by ~4.1% on average (HubSpot). But the real rule is relevance — users abandon irrelevant fields, not long forms. Optimal B2B lead gen: 3-5 fields (Forrester).
- **Guest checkout:** 63% abandon if guest checkout unavailable (Baymard)
- **Address autocomplete:** Reduces fields to 3-5 characters. Use Google Places or similar.
- **Social proof:** Display trust badges, customer count, security indicators near payment fields
- **Post-submit clarity:** Tell users exactly what happens next — confirmation email, response time, next step

---

## 11. Date Inputs

**Date pickers are wrong for known dates.** Both GOV.UK and USWDS independently recommend separate text fields for dates users already know (birthdays, card expiry). Calendar pickers are appropriate only for scheduling/booking unknown dates.

**Pattern:**
- Memorable dates (DOB, expiry): Month select + Day text + Year text
- Scheduling dates (appointment, booking): Calendar date picker
- Date ranges: Two date pickers with validation

---

## 12. Dark Patterns to Avoid

Regulatory enforcement now carries billion-dollar consequences (Amazon Prime: $2.5B FTC settlement, 2025). Pre-checked consent boxes are illegal under GDPR, CCPA, and multiple US state laws.

**Never:**
- Pre-check opt-in boxes
- Use confusing double-negative button text
- Make decline/cancel visually inferior to accept
- Hide terms in tiny fonts
- Require cognitive puzzles for authentication (WCAG 2.2 3.3.8)
- Make cancellation harder than sign-up ("roach motel")

---

## 13. Anti-Patterns Checklist

| Anti-Pattern | Fix |
|-------------|-----|
| Placeholder text as only label | Add visible `<label>` above field |
| Vague errors ("Invalid input") | Specific, constructive message with format example |
| Validating on every keystroke | Reward early, punish late pattern |
| All fields marked with * | Mark optional fields "(optional)" instead |
| Date picker for birthdays | Separate text fields (month/day/year) |
| No loading state on submit | Disable button + show spinner |
| Optional fields that confuse (Expedia's "Company") | Remove or clearly explain purpose |
| Binary gender/title options | Include inclusive options or make optional |
| Reset/Clear button | Remove entirely |
| Color-only error indicators | Add icon + text + border weight |

---

## 14. Audit Dimensions

For convergence engine integration — audit forms against these 9 dimensions:

1. **layout** — Single column default, logical grouping, visual hierarchy
2. **labels** — Top-aligned, visible, not placeholder-only, optional marked
3. **validation** — Reward-early-punish-late or submit-only, server-side primary
4. **errors** — Inline + summary, multi-cue, specific, constructive, preserves input
5. **accessibility** — WCAG 2.2 AA, semantic HTML, keyboard nav, ARIA, focus management
6. **mobile** — Touch targets >= 48px, proper input types, single column, autofill
7. **cta** — Outcome-focused text, loading state, no reset button, visual hierarchy
8. **trust** — Minimal fields, guest checkout option, post-submit clarity, no dark patterns
9. **performance** — Fast load, no layout shift, debounced validation, optimistic submission

---

## 15. Form Layout

**Single-column is the only safe default.** Multi-column forms increase completion time by 15.4 seconds on average (CXL) and cause field-skipping errors (Baymard). Users process a single visual flow; multi-column layouts force lateral scanning that breaks reading order.

**Rules:**

| Principle | Guidance |
|-----------|----------|
| Column count | Single column. No exceptions for aesthetic reasons. |
| Field alignment | Labels and inputs left-aligned to a single vertical edge |
| Field width | Match the expected input length — width is an affordance (see Section 18) |
| Vertical spacing | Consistent 16-24px between field groups; 8-12px between label and input |
| Mobile stacking | All fields stack vertically; no side-by-side on viewports < 768px |
| Logical grouping exceptions | City / State / Zip on one row; First / Last name on one row — only when all are short, related, and required |

**Field alignment detail:** Left-align everything to a single vertical axis. Center-aligned forms look "designed" but increase cognitive load — the eye has to re-find the start of each row. Right-aligned labels (beside inputs) create the worst completion times of any layout (Wroblewski).

---

## 16. Label Positioning

**Top-aligned labels produce the fastest completion times.** This is the single most-replicated finding in form usability research:

- **Penzo (UXmatters, 2006):** Eye-tracking study found top-aligned labels require ~50% fewer visual fixations than left-aligned labels.
- **Wroblewski (Web Form Design, 2008):** Confirmed top-aligned labels yield the fastest completion times across all label positions tested.
- **Matteo Penzo replication (2014):** Results held across mobile and desktop.

**The hierarchy (fastest to slowest):**

1. **Top-aligned** — label directly above input, left-aligned text
2. **Right-aligned (beside input)** — compact but harder to scan
3. **Left-aligned (beside input)** — worst: large gutter between label and field causes misassociation

**Rules:**
- Labels ABOVE inputs, left-aligned text — this is the default, always
- Never use floating/animated labels as the primary label strategy. Floating labels shrink below WCAG minimum text size (12px), trigger motion sensitivity, and disappear the hint at the moment the user needs it most
- Never rely on placeholder text as a label — it vanishes on focus and fails screen readers (WCAG 1.3.1, 3.3.2)
- Maintain clear visual hierarchy: label (smaller, muted) above input (larger, bordered)

---

## 17. Required vs. Optional Field Indicators

**The core principle: minimize noise, maximize clarity.**

The correct strategy depends on the ratio of required to optional fields:

| Form composition | Strategy | Rationale |
|-----------------|----------|-----------|
| Most fields required (>75%) | Mark only optional fields with "(optional)" text after the label | Reduces visual clutter; users infer unmarked = required |
| Most fields optional (>75%) | Mark only required fields with asterisk (*) after the label | Same principle, inverted |
| Mixed or ambiguous | Mark BOTH required (*) and optional "(optional)" | Baymard research: 32% of users failed to complete required fields when only optional was marked |

**Asterisk rules:**
- Place the asterisk (*) immediately after the label text, not before
- Use `aria-required="true"` or the `required` attribute on the input for programmatic access — the visual asterisk alone is insufficient for screen readers
- Include a legend at the top of the form: "Fields marked with * are required"
- Red asterisks are conventional but ensure sufficient contrast (4.5:1 minimum)

**What NOT to do:**
- Do not mark every field with an asterisk when all are required — it creates visual noise with zero information gain
- Do not use "(Required)" text on every field — same problem
- Do not rely solely on color to indicate required status (WCAG 1.4.1)

**Baymard checkout finding:** Only 14% of e-commerce sites mark both required and optional fields explicitly. Sites that do see measurably fewer validation errors at checkout.

---

## 18. Input Sizing

**Field width is an affordance.** The width of an input communicates the expected length of the content. A zip code field the width of an email field creates ambiguity. A full-width field for a 2-digit number suggests something is wrong. Users interpret field size as a constraint — respect that.

**Recommended sizing:**

| Field type | Width | Rationale |
|-----------|-------|-----------|
| Email | Full width (100%) | Addresses vary wildly in length |
| Full name | Full width (100%) | Names can be long |
| First name / Last name | ~50% each (side by side) | Short, paired, related |
| Phone | ~200px / ~50% | 10-15 characters typical |
| Zip/Postal code | ~100-120px / ~25% | 5-10 characters |
| State/Province | ~200px or dropdown | Fixed set of values |
| Street address | Full width (100%) | Addresses are long |
| City | ~60-70% width | Medium-length content |
| Credit card number | ~250px / ~60% | 16-19 characters |
| CVV/CVC | ~80px | 3-4 characters |
| Age / Quantity | ~80-100px | 1-3 digits |
| URL | Full width (100%) | URLs can be very long |
| Description / Notes | Full width textarea | Multi-line content (see Section 19) |

**Implementation notes:**
- Use `max-width` to prevent fields from stretching beyond readable width on ultrawide screens
- On mobile (< 768px), all fields go full-width — narrow fields are unusable on touch keyboards
- The CSS `field-sizing: content` property (Chromium 123+) auto-sizes fields to content, but lacks Safari/Firefox support as of 2026 — use with fallback
- For fixed-format fields (zip, phone, CVV), set `maxlength` and `size` attributes to match

---

## 19. Textarea Patterns

**Use a textarea whenever the expected input is more than a single line.** The `<input type="text">` element is for single-line data — names, emails, phone numbers. The `<textarea>` element is for multi-line content.

**When to use textarea (not input):**

- Descriptions, details, explanations
- Notes, comments, feedback
- Messages (contact forms, support tickets)
- Bio, about, summary fields
- Addresses (when not structured as separate fields)
- Any "tell us more" or "additional information" prompt
- Any field where copy-paste of multi-line content is expected

**Textarea rules:**

| Property | Recommendation |
|----------|---------------|
| Minimum height | 120px (~4-5 visible lines) — gives users space to think |
| Resize behavior | `resize: vertical` — prevent horizontal layout breakage while allowing expansion |
| Auto-resize | Grow with content using JS (`el.style.height = el.scrollHeight + 'px'`) or CSS `field-sizing: content` (Chromium only) |
| Max height | Cap at ~400px, then scroll — prevent page-length textareas |
| Character count | Show when a limit exists. Display remaining characters, update live. GOV.UK pattern: show count below the textarea, announce to screen readers via `aria-live="polite"` |
| Character limit UX | Allow typing past the limit, show warning — do NOT hard-block input, which breaks paste workflows |
| Placeholder | Brief example of expected content: "Describe the issue you're experiencing..." |
| Whitespace | `white-space: pre-wrap` to preserve user formatting |

**Anti-pattern:** Using `<input type="text">` for a "Message" or "Description" field. This forces users to compose multi-line content in a single-line box, hiding most of what they've typed. Always use `<textarea>`.

---

## 20. Field Grouping

**Group related fields with `<fieldset>` and `<legend>`.** This is both a usability pattern and an accessibility requirement (WCAG 1.3.1). Screen readers announce the legend before each field in the group, giving users context.

**When to group:**

| Field group | Example fields |
|------------|----------------|
| Personal information | First name, Last name, Email, Phone |
| Mailing address | Street, Apt/Suite, City, State, Zip, Country |
| Billing address | Same structure, separate fieldset |
| Payment details | Card number, Expiry, CVV, Cardholder name |
| Date of birth | Month, Day, Year |
| Account credentials | Username/Email, Password, Confirm password |
| Preferences | Checkboxes or radios sharing a common question |

**Rules:**
- The `<legend>` must be the first child of `<fieldset>` — this is an HTML requirement, not a suggestion
- The legend text should be the group's heading: "Shipping Address", "Payment Information", "Your Details"
- Visual separation between groups: 24-32px vertical gap or a subtle horizontal rule
- Consistent internal padding within groups: 16px
- On multi-step forms, each step typically maps to one fieldset group
- Nest fieldsets only when semantically necessary (rare) — deeply nested fieldsets confuse screen readers

**Visual design:**
- Default browser fieldset styling (beveled border) is ugly — reset with CSS: `border: none; padding: 0; margin: 0;`
- Use spacing, subtle background color, or light borders to visually distinguish groups
- The legend can be styled as a heading (e.g., `font-weight: 600; font-size: 1.125rem`)

---

## 21. Error Display (Extended)

This section extends Section 5 with implementation-specific patterns.

**Error positioning hierarchy:**

1. **Error summary at top of form** — on submit, display a linked list of all errors at the top. Set `tabindex="-1"` and move keyboard focus to it. Each error links to its field via anchor.
2. **Inline error below the field** — immediately beneath the input, above the next label. Never above the input (it shifts the field the user is trying to fix).
3. **Field-level visual cues** — red/error-color border on the input, error icon inside or beside the field.

**`aria-describedby` linking (mandatory):**
```html
<label for="email">Email address</label>
<input type="email" id="email" aria-describedby="email-error" aria-invalid="true">
<p id="email-error" class="error-message">Enter a valid email address, like name@example.com</p>
```

**Focus management on submit:**
1. Run client-side validation
2. If errors exist, generate error summary, prepend to form
3. Move focus to error summary (`summary.focus()`)
4. User clicks error link, focus moves to the errored field
5. Inline error is announced by screen reader via `aria-describedby`

**Error message tone:**
- Specific: "Enter your email address" not "This field is required"
- Constructive: "Enter a date after today" not "Invalid date"
- Human: "We couldn't find that zip code — check for typos" not "Validation error: postal_code"
- Never blame the user: "Enter a valid phone number" not "You entered an invalid phone number"

**Page title on error:** Prepend "Error: " to `<title>` so screen readers announce it on page load (GOV.UK pattern).

---

## 22. Submit Button Patterns

**The submit button is the form's call to action. Its text should describe the outcome, not the mechanism.**

**Outcome-focused text examples:**

| Bad (mechanism) | Good (outcome) |
|----------------|----------------|
| Submit | Reserve My Visit |
| Send | Send Message |
| Submit Form | Create Account |
| Submit | Download Free Guide |
| Process | Complete Purchase |
| Save | Save Changes |
| Go | Search Flights |

**Rules:**

| Property | Guidance |
|----------|----------|
| Text | 1-4 words, active voice, describes what happens next |
| Width (desktop) | Match content width with comfortable padding (min 200px) |
| Width (mobile) | Full width (100%) — easy tap target, no precision required |
| Position | Directly after the last form field — never floating, never in a fixed bar (unless checkout) |
| Alignment | Left-aligned on desktop (consistent with field alignment), full-width on mobile |
| Loading state | On click: disable button, replace text with spinner + "Processing..." or similar. Prevent double-submission |
| Disabled state | Only disable AFTER click (during processing). Never pre-disable — users can't discover why it won't work |
| Visual weight | Highest visual prominence in the form. Primary color, solid fill, not outline |
| Secondary actions | "Cancel", "Back", "Clear" — visually subordinate (text link or ghost button, never same prominence as primary CTA) |
| Reset button | Do not include. Ever. Accidental data loss risk vastly outweighs the edge case (NNg) |

**Loading state implementation:**
```
[Reserve My Visit] → click → [⟳ Reserving...] (disabled) → success → redirect/confirmation
```
- Disable the button to prevent double-submission
- Show a spinner or progress indicator
- Keep the button visible (don't hide it)
- If the request fails, re-enable the button and show an error message

---

## 23. Progressive Disclosure

**Show fields only when they become relevant.** Progressive disclosure reduces cognitive load by hiding complexity until the user's context demands it.

**Common patterns:**

| Trigger | Revealed fields |
|---------|----------------|
| "Billing address is different from shipping" checkbox | Billing address fieldset |
| "Other" selected in a dropdown/radio | Free-text input for custom value |
| "Yes" to "Do you have a referral code?" | Referral code input |
| Business account selected (vs. personal) | Company name, VAT number, department |
| "Add a gift message" link | Textarea for message |
| Country selection | Country-specific fields (state/province, postal code format) |

**Rules:**
- Revealed fields must animate in smoothly (opacity + height transition, ~200ms) — abrupt layout shifts are disorienting
- When fields are hidden, their values must be cleared and excluded from submission — do not submit invisible field data
- Hidden fields must not be validated — only validate what the user can see
- Use `aria-expanded` on the trigger and `aria-controls` pointing to the revealed section
- Announce changes to screen readers via `aria-live="polite"` on the container
- On page load, only show fields relevant to default selections
- Never use progressive disclosure to hide required fields behind an interaction that isn't obviously necessary

**Anti-patterns:**
- Hiding essential fields behind "Advanced" toggles that most users need
- Requiring a specific sequence of interactions to reveal a required field
- Progressive disclosure that causes significant layout shift (push content below the fold)

---

## 24. Input Types and Mobile Keyboard Optimization

**The correct `type`, `inputmode`, and `autocomplete` attributes form a team.** Together, they trigger the right virtual keyboard, enable browser autofill, and reduce typing effort by 30-50% on mobile (Baymard).

**Input type reference:**

| Field | `type` | `inputmode` | `autocomplete` | Keyboard shown |
|-------|--------|-------------|-----------------|----------------|
| Email | `email` | — | `email` | @ and .com keys |
| Phone | `tel` | — | `tel` | Numeric dialpad |
| URL | `url` | — | `url` | / and .com keys |
| Credit card | `text` | `numeric` | `cc-number` | Number pad |
| CVV | `text` | `numeric` | `cc-csc` | Number pad |
| Zip code | `text` | `numeric` | `postal-code` | Number pad |
| Quantity / Age | `number` | — | — | Number pad with +/- |
| One-time code | `text` | `numeric` | `one-time-code` | Number pad + auto-fill from SMS |
| Search | `search` | — | — | Search-optimized with "Go" key |
| Password | `password` | — | `current-password` or `new-password` | Standard with show/hide toggle |
| Full name | `text` | — | `name` | Standard with auto-capitalize |
| Street address | `text` | — | `street-address` | Standard |
| City | `text` | — | `address-level2` | Standard |
| State | `text` | — | `address-level1` | Standard (or use `<select>`) |
| Country | `text` | — | `country-name` | Standard (or use `<select>`) |

**Why `inputmode` matters separately from `type`:**
- `type="number"` adds increment spinners and rejects non-numeric input — wrong for credit cards, zip codes, phone numbers
- `inputmode="numeric"` shows the number pad WITHOUT the `type="number"` side effects
- Use `type="text"` + `inputmode="numeric"` for numeric-looking data that isn't a mathematical number

**`autocomplete` rules:**
- Always set `autocomplete` on name, email, phone, and address fields — browser autofill reduces mobile completion time dramatically
- Use `autocomplete="new-password"` on registration forms so password managers offer to generate
- Use `autocomplete="current-password"` on login forms so password managers offer to fill
- Use `autocomplete="one-time-code"` for SMS/email verification codes — iOS and Android will auto-suggest the code
- Never set `autocomplete="off"` on login or checkout forms — it breaks password managers and annoys users

---

## References

### Tier 1 (Primary/Authoritative)
- W3C WAI Forms Tutorial — w3.org/WAI/tutorials/forms/
- WCAG 2.2 — w3.org/TR/WCAG22/
- GOV.UK Design System — design-system.service.gov.uk/patterns/
- USWDS — designsystem.digital.gov/components/
- Apple HIG — developer.apple.com/design/human-interface-guidelines/
- Google Material Design 3 — m3.material.io/components/text-fields/

### Tier 2 (Research-Backed)
- Baymard Institute — baymard.com/research/checkout-usability
- NNg — nngroup.com/articles/web-form-design/
- NNg EAS Framework — nngroup.com/articles/eas-framework-simplify-forms/
- Seckler et al. CHI Study (78% vs 42% first-try submission)
- UXmatters (Penzo eye-tracking) — uxmatters.com/mt/archives/2006/07/label-placement-in-forms.php
- CXL single vs multi-column — cxl.com/research-study/form-field-usability/
- Smart Interface Design Patterns (Friedman) — inline validation studies

### Tier 3 (Practitioner)
- Smashing Magazine — inline validation, multi-step forms, mobile form design
- Venture Harbour — multi-step form studies
- HubSpot — field count conversion data
- Wroblewski — Web Form Design (Rosenfeld Media, 2008)
- Andrew Coyle — "Design Better Input Fields" (field width as affordance)
- CSS-Tricks — auto-growing inputs and textareas, `field-sizing` property
- MDN — inputmode global attribute, textarea element, autocomplete attribute
- Baymard Institute Touch Keyboard Types Cheat Sheet — baymard.com/labs/touch-keyboard-types
- Jakob Nielsen — "Required Fields in Forms: Best Design Practices" (Substack, 2024)
