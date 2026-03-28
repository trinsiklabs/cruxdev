# Internationalization (i18n) & Localization (l10n) Patterns

**Research method:** 5-pass iterative deepening per RESEARCH_PATTERNS.md
**Sources:** MDN, Astro docs, Next.js docs, Smashing Magazine, web.dev, W3C i18n, Google Search Central, Crowdin docs, Lokalise docs, Phrase docs, ICU/CLDR specs, Hugo docs, Phoenix/Elixir docs, Django docs, Rails docs, FormatJS docs, i18next docs, inlang/Paraglide docs, Adrian Roselli, Ben Myers, RTL Styling 101
**Last updated:** 2026-03-28

---

## Table of Contents

1. [i18n Architecture](#1-i18n-architecture)
2. [Framework-Specific i18n](#2-framework-specific-i18n)
3. [Translation Workflow](#3-translation-workflow)
4. [URL Strategy](#4-url-strategy)
5. [Content Strategy](#5-content-strategy)
6. [SEO for Multilingual Sites](#6-seo-for-multilingual-sites)
7. [Static Site i18n](#7-static-site-i18n)
8. [Dynamic App i18n](#8-dynamic-app-i18n)
9. [Date/Time/Number Formatting](#9-datetimenumber-formatting)
10. [Accessibility in i18n](#10-accessibility-in-i18n)
11. [Testing i18n](#11-testing-i18n)
12. [Anti-Patterns](#12-anti-patterns)
13. [Audit Dimensions](#13-audit-dimensions)

---

## 1. i18n Architecture

### 1.1 Core Concepts

Internationalization (i18n) and localization (l10n) are distinct activities:

| Term | Definition | Who Does It | When |
|------|-----------|-------------|------|
| **Internationalization (i18n)** | Making software capable of supporting multiple languages and regions | Developers | Once, at architecture time |
| **Localization (l10n)** | Adapting content for a specific language/region | Translators | Per locale, ongoing |

The number abbreviations come from the character count between the first and last letters: i-nternationalizatio-n (18 letters), l-ocalizatio-n (10 letters).

**Key principle:** i18n is infrastructure. l10n is content. If you architect i18n correctly, adding new locales is a content task, not an engineering task.

### 1.2 String Extraction

All user-facing strings must be externalized from source code into translation files. The extraction workflow:

1. **Mark strings** in source code with a translation function (`t()`, `_()`, `gettext()`, `useTranslations()`)
2. **Extract strings** automatically using tooling (babel plugin, gettext xgettext, i18next-parser, astro recipes)
3. **Generate template files** (.pot for gettext, JSON for JS frameworks, .xliff for enterprise)
4. **Send to translators** via TMS or file handoff
5. **Receive translations** and merge back into locale files
6. **Build/deploy** with translations bundled or loaded at runtime

**What gets extracted:**
- UI labels, buttons, navigation
- Error messages, validation text
- Placeholder text, tooltips, ARIA labels
- Email templates, notification text
- Date/time format strings (but prefer Intl API)

**What does NOT get extracted:**
- Code identifiers, variable names
- API field names, database column names
- Log messages (unless user-facing)
- Technical documentation (initially)

### 1.3 Translation File Formats

#### JSON (Web Standard)

The dominant format for JavaScript/TypeScript projects. No single standard exists — variants include flat key-value, nested/namespaced, i18next format, and FormatJS/react-intl (ICU MessageFormat in values).

**Flat key-value (simplest):**
```json
{
  "nav.home": "Home",
  "nav.about": "About",
  "greeting": "Hello, {name}!",
  "items.count": "{count, plural, one {# item} other {# items}}"
}
```

**Nested/namespaced (better organization at scale):**
```json
{
  "nav": {
    "home": "Home",
    "about": "About"
  },
  "greeting": "Hello, {name}!",
  "items": {
    "count": "{count, plural, one {# item} other {# items}}"
  }
}
```

**Pros:** Native to JavaScript, human-readable, diff-friendly, works with version control, no tooling needed to read.
**Cons:** No standard schema, no metadata (translator comments, context), no plural category enforcement.

**When to use:** All JavaScript/TypeScript web projects. The default choice for 2025-2026.

#### PO/Gettext

The GNU gettext system uses `.po` (Portable Object) files for human-editable translations and `.mo` (Machine Object) binary files for runtime. The standard for C, Python, PHP, Elixir/Phoenix, and many server-side frameworks.

```po
#: src/components/Header.vue:15
#. Translator note: main navigation home link
msgid "Home"
msgstr "Accueil"

#: src/components/Cart.vue:42
msgid "You have %d item in your cart."
msgid_plural "You have %d items in your cart."
msgstr[0] "Vous avez %d article dans votre panier."
msgstr[1] "Vous avez %d articles dans votre panier."
```

**Pros:** Built-in metadata (file references, translator comments, context via msgctxt), native plural support, mature tooling (Poedit, gettext CLI), standard across many languages.
**Cons:** Verbose, XML-unfriendly diffs, binary .mo compilation step, less natural for JavaScript projects.

**When to use:** Phoenix/Elixir, Django, Rails (via gettext adapter), any project with existing gettext infrastructure.

#### XLIFF (XML Localisation Interchange File Format)

The OASIS standard for translation interchange. Used primarily in enterprise translation workflows and CAT (Computer-Assisted Translation) tools.

```xml
<xliff version="2.0" xmlns="urn:oasis:names:tc:xliff:document:2.0"
       srcLang="en" trgLang="fr">
  <file id="f1" original="src/components/Header">
    <unit id="nav.home">
      <segment>
        <source>Home</source>
        <target>Accueil</target>
      </segment>
    </unit>
  </file>
</xliff>
```

**Pros:** Industry standard for enterprise translation, rich metadata, supports translation states (new, translated, reviewed, final), interoperable with all major TMS platforms.
**Cons:** Verbose XML, poor diff readability, requires tooling to edit, overkill for small projects.

**When to use:** Enterprise projects with professional translation workflows, CAT tool integration, or regulatory requirements for translation audit trails.

#### ICU MessageFormat

Not a file format but a syntax standard for expressing complex translations. Defined by the International Components for Unicode (ICU) project and backed by the Common Locale Data Repository (CLDR). Used within JSON, PO, or XLIFF files as message values.

```
{count, plural,
  =0 {No items}
  one {# item}
  other {# items}
}

{gender, select,
  male {He}
  female {She}
  other {They}
} sent you a message.

{count, plural,
  =0 {No files}
  one {# file}
  other {# files}
} {count, plural,
  =0 {were}
  one {was}
  other {were}
} uploaded.
```

**CLDR plural categories by language:**

| Language | Categories | Forms |
|----------|-----------|-------|
| English | one, other | 2 |
| French | one, many, other | 3 |
| Russian | one, few, many, other | 4 |
| Arabic | zero, one, two, few, many, other | 6 |
| Japanese | other | 1 |
| Polish | one, few, many, other | 4 |

**Key rule:** Never assume all languages have the same plural forms as English. Always use ICU plural syntax or equivalent — never `count === 1 ? "item" : "items"`.

### 1.4 Namespace Organization

For projects with more than a few dozen strings, organize translations by feature or domain:

```
locales/
  en/
    common.json      # Shared: nav, footer, buttons, errors
    auth.json         # Login, signup, password reset
    dashboard.json    # Dashboard-specific strings
    billing.json      # Plans, pricing, invoices
    blog.json         # Blog-specific strings
  fr/
    common.json
    auth.json
    dashboard.json
    billing.json
    blog.json
```

**Benefits:**
- Lazy-load namespaces per route (only load billing strings on billing pages)
- Smaller translation files for translators to work with
- Parallel translation work across teams
- Clear ownership boundaries

**Naming conventions:**
- Use feature names, not component names (features are stable, components change)
- Keep `common` namespace small — strings used on 3+ pages
- Never put route-specific strings in `common`

### 1.5 Pluralization Rules

Pluralization is the single most-broken feature in i18n implementations. The failure modes:

1. **Binary pluralization** — assuming all languages have `singular` and `plural` like English
2. **Hardcoded rules** — writing `if (count === 1)` instead of using CLDR rules
3. **Missing categories** — providing `one` and `other` but not `few`, `many`, `zero` where required

**Correct approach:** Use ICU MessageFormat or a library that implements CLDR plural rules (react-intl, i18next with icu plugin, vue-i18n). Let the CLDR database define which categories exist for each language.

```typescript
// WRONG — breaks in Russian, Arabic, Polish, etc.
const message = count === 1 ? t('item_singular') : t('item_plural');

// RIGHT — ICU MessageFormat handles all plural categories
const message = t('items', { count });
// where items = "{count, plural, one {# item} other {# items}}"
```

### 1.6 RTL (Right-to-Left) Support

RTL languages include Arabic, Hebrew, Persian (Farsi), and Urdu. Supporting RTL requires changes at every layer:

**HTML layer:**
```html
<html lang="ar" dir="rtl">
```

**CSS layer — use logical properties:**
```css
/* WRONG — physical properties break in RTL */
.sidebar {
  margin-left: 20px;
  padding-right: 16px;
  text-align: left;
  border-left: 2px solid #ccc;
}

/* RIGHT — logical properties auto-flip */
.sidebar {
  margin-inline-start: 20px;
  padding-inline-end: 16px;
  text-align: start;
  border-inline-start: 2px solid #ccc;
}
```

**Logical property mapping:**

| Physical (LTR-only) | Logical (direction-aware) |
|---------------------|--------------------------|
| `margin-left` | `margin-inline-start` |
| `margin-right` | `margin-inline-end` |
| `padding-left` | `padding-inline-start` |
| `padding-right` | `padding-inline-end` |
| `border-left` | `border-inline-start` |
| `text-align: left` | `text-align: start` |
| `float: left` | `float: inline-start` |
| `left: 0` | `inset-inline-start: 0` |
| `width` | `inline-size` |
| `height` | `block-size` |

**Browser support (2026):** Excellent. All major browsers support CSS logical properties. Use them from day one — they work correctly in LTR too.

**Tailwind CSS RTL support:**
```html
<!-- Tailwind uses rtl: and ltr: variants -->
<div class="ltr:ml-4 rtl:mr-4">
  <!-- Or better: use logical utilities directly -->
</div>

<!-- Tailwind v4 supports logical properties natively -->
<div class="ms-4 ps-3 text-start border-s-2">
  <!-- ms = margin-start, ps = padding-start, border-s = border-start -->
</div>
```

**What needs manual RTL handling:**
- Icons with directional meaning (arrows, progress indicators)
- Horizontal scroll carousels
- CSS transforms with X-axis translation
- Background position with pixel offsets
- Fixed/absolute positioning with left/right

---

## 2. Framework-Specific i18n

### 2.1 Astro (Recommended for cruxdev.dev)

Astro v4.0+ has built-in i18n routing support. This is the recommended stack for static content sites.

#### Configuration

```javascript
// astro.config.mjs
import { defineConfig } from "astro/config";

export default defineConfig({
  i18n: {
    locales: ["en", "es", "fr", "de", "ja"],
    defaultLocale: "en",
    routing: {
      prefixDefaultLocale: false, // /about for English, /fr/about for French
    },
    fallback: {
      de: "en",  // Missing German pages fall back to English
      ja: "en",
    },
  },
});
```

With `prefixDefaultLocale: false`:
- English: `example.com/about/`
- French: `example.com/fr/about/`
- Spanish: `example.com/es/about/`

With `prefixDefaultLocale: true`:
- English: `example.com/en/about/`
- French: `example.com/fr/about/`
- Requires a root `src/pages/index.astro` for redirect

#### UI Translation System

**Define translations (`src/i18n/ui.ts`):**
```typescript
export const languages = {
  en: "English",
  es: "Espanol",
  fr: "Francais",
} as const;

export const defaultLang = "en";

export const ui = {
  en: {
    "nav.home": "Home",
    "nav.about": "About",
    "nav.blog": "Blog",
    "nav.pricing": "Pricing",
    "cta.getStarted": "Get Started",
    "cta.learnMore": "Learn More",
    "footer.copyright": "All rights reserved.",
    "blog.readMore": "Read more",
    "blog.minRead": "{minutes} min read",
    "blog.publishedOn": "Published on {date}",
  },
  es: {
    "nav.home": "Inicio",
    "nav.about": "Acerca de",
    "nav.blog": "Blog",
    "nav.pricing": "Precios",
    "cta.getStarted": "Comenzar",
    "cta.learnMore": "Saber mas",
    "footer.copyright": "Todos los derechos reservados.",
    "blog.readMore": "Leer mas",
    "blog.minRead": "{minutes} min de lectura",
    "blog.publishedOn": "Publicado el {date}",
  },
  fr: {
    "nav.home": "Accueil",
    "nav.about": "A propos",
    "nav.blog": "Blog",
    "nav.pricing": "Tarifs",
    "cta.getStarted": "Commencer",
    "cta.learnMore": "En savoir plus",
    "footer.copyright": "Tous droits reserves.",
    "blog.readMore": "Lire la suite",
    "blog.minRead": "{minutes} min de lecture",
    "blog.publishedOn": "Publie le {date}",
  },
} as const;

// Route translations (optional — for translating URL slugs)
export const routes = {
  es: {
    about: "acerca-de",
    pricing: "precios",
    blog: "blog",
  },
  fr: {
    about: "a-propos",
    pricing: "tarifs",
    blog: "blog",
  },
} as const;
```

**Helper functions (`src/i18n/utils.ts`):**
```typescript
import { ui, defaultLang, routes } from "./ui";

export type Lang = keyof typeof ui;

export function getLangFromUrl(url: URL): Lang {
  const [, lang] = url.pathname.split("/");
  if (lang in ui) return lang as Lang;
  return defaultLang;
}

export function useTranslations(lang: Lang) {
  return function t(
    key: keyof (typeof ui)[typeof defaultLang],
    params?: Record<string, string | number>
  ): string {
    let str = (ui[lang]?.[key] || ui[defaultLang][key]) as string;
    if (params) {
      Object.entries(params).forEach(([k, v]) => {
        str = str.replace(`{${k}}`, String(v));
      });
    }
    return str;
  };
}

export function useTranslatedPath(lang: Lang) {
  return function translatePath(path: string, targetLang: Lang = lang): string {
    const pathName = path.replace(/^\/|\/$/g, "");
    const hasTranslation =
      targetLang !== defaultLang &&
      routes[targetLang as keyof typeof routes]?.[
        pathName as keyof (typeof routes)[keyof typeof routes]
      ];
    const translatedPath = hasTranslation
      ? `/${routes[targetLang as keyof typeof routes][pathName as keyof (typeof routes)[keyof typeof routes]]}`
      : `/${pathName}`;
    return targetLang === defaultLang
      ? translatedPath
      : `/${targetLang}${translatedPath}`;
  };
}

export function getLocaleFromPath(pathname: string): Lang {
  const segments = pathname.split("/").filter(Boolean);
  if (segments[0] in ui) return segments[0] as Lang;
  return defaultLang;
}
```

#### Content Collections Per Locale

**Directory structure:**
```
src/content/
  blog/
    en/
      getting-started.md
      advanced-features.md
    es/
      getting-started.md
      advanced-features.md
    fr/
      getting-started.md
```

**Content config (`src/content.config.ts`):**
```typescript
import { defineCollection, z } from "astro:content";

const blogCollection = defineCollection({
  schema: z.object({
    title: z.string(),
    description: z.string(),
    date: z.date(),
    author: z.string(),
    tags: z.array(z.string()).optional(),
    draft: z.boolean().default(false),
  }),
});

export const collections = {
  blog: blogCollection,
};
```

**Dynamic route (`src/pages/[lang]/blog/[...slug].astro`):**
```astro
---
import { getCollection, render } from "astro:content";
import BaseLayout from "../../layouts/BaseLayout.astro";

export async function getStaticPaths() {
  const posts = await getCollection("blog");
  return posts.map((post) => {
    const [lang, ...slugParts] = post.id.split("/");
    return {
      params: { lang, slug: slugParts.join("/") || undefined },
      props: { post },
    };
  });
}

const { lang, slug } = Astro.params;
const { post } = Astro.props;
const { Content } = await render(post);
const formattedDate = post.data.date.toLocaleDateString(lang, {
  year: "numeric",
  month: "long",
  day: "numeric",
});
---

<BaseLayout lang={lang} title={post.data.title}>
  <article>
    <h1>{post.data.title}</h1>
    <time datetime={post.data.date.toISOString()}>{formattedDate}</time>
    <Content />
  </article>
</BaseLayout>
```

**Blog index with locale filtering (`src/pages/[lang]/blog/index.astro`):**
```astro
---
import { getCollection } from "astro:content";
import BaseLayout from "../../../layouts/BaseLayout.astro";
import { useTranslations, type Lang } from "../../../i18n/utils";

export function getStaticPaths() {
  return [
    { params: { lang: "en" } },
    { params: { lang: "es" } },
    { params: { lang: "fr" } },
  ];
}

const { lang } = Astro.params;
const t = useTranslations(lang as Lang);

const allPosts = await getCollection("blog");
const localePosts = allPosts
  .filter((post) => post.id.startsWith(`${lang}/`))
  .filter((post) => !post.data.draft)
  .sort((a, b) => b.data.date.valueOf() - a.data.date.valueOf());
---

<BaseLayout lang={lang} title={t("nav.blog")}>
  <h1>{t("nav.blog")}</h1>
  <ul>
    {localePosts.map((post) => {
      const slug = post.id.split("/").slice(1).join("/");
      return (
        <li>
          <a href={`/${lang}/blog/${slug}`}>
            <h2>{post.data.title}</h2>
            <time>{post.data.date.toLocaleDateString(lang)}</time>
          </a>
        </li>
      );
    })}
  </ul>
</BaseLayout>
```

#### Language Picker Component

```astro
---
// src/components/LanguagePicker.astro
import { languages } from "../i18n/ui";
import { getLangFromUrl, useTranslatedPath } from "../i18n/utils";

const currentLang = getLangFromUrl(Astro.url);
const currentPath = Astro.url.pathname;

// Extract the path without locale prefix
const pathWithoutLocale = currentPath.replace(
  new RegExp(`^/(${Object.keys(languages).join("|")})/`),
  "/"
);
---

<nav aria-label="Language selector">
  <ul>
    {Object.entries(languages).map(([lang, label]) => {
      const translatePath = useTranslatedPath(lang);
      const isActive = lang === currentLang;
      return (
        <li>
          <a
            href={translatePath(pathWithoutLocale)}
            hreflang={lang}
            aria-current={isActive ? "true" : undefined}
            class:list={[{ active: isActive }]}
          >
            {label}
          </a>
        </li>
      );
    })}
  </ul>
</nav>
```

#### Base Layout with lang Attribute

```astro
---
// src/layouts/BaseLayout.astro
import { getLangFromUrl } from "../i18n/utils";
import LanguagePicker from "../components/LanguagePicker.astro";
import { getRelativeLocaleUrl } from "astro:i18n";

interface Props {
  title: string;
  description?: string;
  lang?: string;
}

const { title, description, lang: propLang } = Astro.props;
const lang = propLang || getLangFromUrl(Astro.url);
const dir = ["ar", "he", "fa", "ur"].includes(lang) ? "rtl" : "ltr";
---

<!doctype html>
<html lang={lang} dir={dir}>
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>{title}</title>
    {description && <meta name="description" content={description} />}

    <!-- hreflang tags for all supported locales -->
    <link rel="alternate" hreflang="en" href={getRelativeLocaleUrl("en", Astro.url.pathname)} />
    <link rel="alternate" hreflang="es" href={getRelativeLocaleUrl("es", Astro.url.pathname)} />
    <link rel="alternate" hreflang="fr" href={getRelativeLocaleUrl("fr", Astro.url.pathname)} />
    <link rel="alternate" hreflang="x-default" href={getRelativeLocaleUrl("en", Astro.url.pathname)} />
  </head>
  <body>
    <header>
      <LanguagePicker />
    </header>
    <main>
      <slot />
    </main>
  </body>
</html>
```

#### Astro i18n with Paraglide

For projects that need type-safe, tree-shakable translations with compile-time checking:

```bash
npx @inlang/paraglide-js init
npm install @inlang/paraglide-astro
```

```javascript
// astro.config.mjs
import { defineConfig } from "astro/config";
import paraglide from "@inlang/paraglide-astro";

export default defineConfig({
  output: "static",
  integrations: [
    paraglide({
      project: "./project.inlang",
      outdir: "./src/paraglide",
    }),
  ],
});
```

```astro
---
// In any Astro component
import * as m from "../paraglide/messages";
---

<h1>{m.greeting({ name: "World" })}</h1>
<p>{m.itemCount({ count: 5 })}</p>
```

**Paraglide advantages over manual JSON approach:**
- Full TypeScript autocompletion for message keys and parameters
- Compile-time error if you reference a non-existent key
- Tree-shaking: only translations used in client-side islands ship to the browser
- Up to 70% smaller i18n bundle vs. runtime-based libraries

### 2.2 Next.js

Two dominant libraries: **next-intl** (recommended for App Router) and **next-i18next** (Pages Router legacy).

#### next-intl (App Router, recommended)

```bash
npm install next-intl
```

```typescript
// next.config.ts
import createNextIntlPlugin from "next-intl/plugin";
const withNextIntl = createNextIntlPlugin();

export default withNextIntl({
  // other Next.js config
});
```

```typescript
// src/i18n/request.ts
import { getRequestConfig } from "next-intl/server";

export default getRequestConfig(async ({ locale }) => ({
  messages: (await import(`../messages/${locale}.json`)).default,
}));
```

```typescript
// src/app/[locale]/layout.tsx
import { NextIntlClientProvider } from "next-intl";
import { getMessages } from "next-intl/server";

export default async function LocaleLayout({
  children,
  params: { locale },
}: {
  children: React.ReactNode;
  params: { locale: string };
}) {
  const messages = await getMessages();

  return (
    <html lang={locale}>
      <body>
        <NextIntlClientProvider messages={messages}>
          {children}
        </NextIntlClientProvider>
      </body>
    </html>
  );
}
```

```typescript
// Server Component — zero JS shipped to client
import { getTranslations } from "next-intl/server";

export default async function HomePage() {
  const t = await getTranslations("home");
  return <h1>{t("title")}</h1>;
}

// Client Component — hooks-based API
"use client";
import { useTranslations } from "next-intl";

export default function Counter() {
  const t = useTranslations("counter");
  return <p>{t("items", { count: 5 })}</p>;
}
```

**next-intl features:** ~2KB client bundle, Server Component support (zero JS for server-rendered translations), ICU MessageFormat, TypeScript autocomplete for message keys, built-in date/number formatting, middleware for locale detection and routing.

#### next-i18next (Pages Router)

Legacy choice. If your project uses Next.js Pages Router, next-i18next wraps i18next and react-i18next. For new App Router projects, use next-intl instead.

### 2.3 React (react-intl vs. react-i18next)

| Feature | react-intl (FormatJS) | react-i18next |
|---------|----------------------|---------------|
| Message syntax | ICU MessageFormat | i18next format (or ICU via plugin) |
| Bundle size | ~18KB gzipped | ~18KB gzipped |
| TypeScript | Strong typing | Strong typing (with plugin) |
| Pluralization | ICU plural rules (CLDR) | Built-in + ICU plugin |
| Namespaces | Via separate message files | First-class support |
| Lazy loading | Manual | Built-in with Suspense |
| Ecosystem | FormatJS CLI, formatjs/intl | i18next plugins (100+) |
| TMS integration | Good | Excellent (locize, Crowdin, etc.) |
| Best for | Enterprise, ICU-standard compliance | Flexibility, large plugin ecosystem |

**react-intl example:**
```tsx
import { FormattedMessage, useIntl } from "react-intl";

function ProductCard({ product }) {
  const intl = useIntl();

  return (
    <div>
      <h2>
        <FormattedMessage id="product.title" values={{ name: product.name }} />
      </h2>
      <p>
        <FormattedMessage
          id="product.price"
          values={{
            price: intl.formatNumber(product.price, {
              style: "currency",
              currency: product.currency,
            }),
          }}
        />
      </p>
      <p>
        <FormattedMessage
          id="product.reviews"
          values={{ count: product.reviewCount }}
        />
      </p>
    </div>
  );
}
```

**react-i18next example:**
```tsx
import { useTranslation, Trans } from "react-i18next";

function ProductCard({ product }) {
  const { t } = useTranslation("products");

  return (
    <div>
      <h2>{t("title", { name: product.name })}</h2>
      <p>{t("price", { price: product.price })}</p>
      <p>{t("reviews", { count: product.reviewCount })}</p>
      <Trans i18nKey="terms">
        By purchasing you agree to our <a href="/terms">terms</a>.
      </Trans>
    </div>
  );
}
```

**Selection guidance (2025-2026):**
- Building a new Next.js App Router project: **next-intl**
- Enterprise React with strict ICU compliance: **react-intl**
- React with complex namespace/lazy-loading needs: **react-i18next**
- Performance-critical with minimal bundle: **LinguiJS** (compile-time extraction)
- Migrating mid-project is extremely painful. Choose once, choose carefully.

### 2.4 Vue (vue-i18n)

```typescript
// i18n.ts
import { createI18n } from "vue-i18n";
import en from "./locales/en.json";
import fr from "./locales/fr.json";

export const i18n = createI18n({
  legacy: false, // Composition API mode
  locale: "en",
  fallbackLocale: "en",
  messages: { en, fr },
});
```

```vue
<template>
  <h1>{{ t("welcome") }}</h1>
  <p>{{ t("items", { count: itemCount }) }}</p>
  <p>{{ d(new Date(), "long") }}</p>
  <p>{{ n(1234.56, "currency") }}</p>
</template>

<script setup>
import { useI18n } from "vue-i18n";
const { t, d, n } = useI18n();
</script>
```

**vue-i18n features:** Composition API and Options API support, ICU MessageFormat compatible, datetime and number formatting, per-component translations, lazy loading via dynamic imports, PO file support via loader.

### 2.5 SvelteKit (Paraglide)

Paraglide JS is SvelteKit's officially recommended i18n library, succeeding the now-unmaintained typesafe-i18n.

```svelte
<script>
  import * as m from "$lib/paraglide/messages";
  import { languageTag } from "$lib/paraglide/runtime";
</script>

<h1>{m.greeting({ name: "World" })}</h1>
<p>Current language: {languageTag()}</p>
```

**Paraglide advantages:** Compiler-based (not runtime), tree-shakable per-message, full TypeScript support, ~0KB runtime overhead for server-rendered pages, works with Astro, SvelteKit, Next.js, and vanilla TS.

### 2.6 Phoenix (Elixir — Gettext)

Phoenix ships with Gettext support out of the box. No additional dependencies needed.

```elixir
# In a Phoenix template or LiveView
<h1><%= gettext("Welcome to our app") %></h1>
<p><%= ngettext("1 file", "%{count} files", @count) %></p>
<p><%= pgettext("menu", "Home") %></p>  # Context-aware translation
```

```elixir
# Set locale in a plug
defmodule MyAppWeb.Plugs.SetLocale do
  import Plug.Conn

  def init(opts), do: opts

  def call(conn, _opts) do
    locale = conn.params["locale"] || get_session(conn, :locale) || "en"
    Gettext.put_locale(MyAppWeb.Gettext, locale)
    assign(conn, :locale, locale)
  end
end
```

**Workflow:**
1. Mark strings with `gettext()`, `ngettext()`, `pgettext()`
2. Run `mix gettext.extract` to generate .pot template files
3. Run `mix gettext.merge priv/gettext` to create/update .po files per locale
4. Translate .po files
5. Compile — .po files are compiled into .mo at build time

### 2.7 Rails (I18n)

Rails uses YAML-based translation files with a built-in I18n framework.

```yaml
# config/locales/en.yml
en:
  nav:
    home: "Home"
    about: "About"
  greeting: "Hello, %{name}!"
  items:
    zero: "No items"
    one: "1 item"
    other: "%{count} items"
```

```erb
<h1><%= t('greeting', name: current_user.name) %></h1>
<p><%= t('items', count: @cart.items.count) %></p>
```

**Rails i18n best practices:**
- Store translations in `config/locales/` organized by feature (`en/nav.yml`, `en/auth.yml`)
- Set locale from URL (`/:locale/...`) via a `before_action`
- Use `I18n.default_locale` and `I18n.available_locales` in config
- Use `rails-i18n` gem for CLDR plural rules and locale data

### 2.8 Django (Gettext)

```python
# views.py
from django.utils.translation import gettext as _
from django.utils.translation import ngettext

def product_view(request, product_id):
    product = get_object_or_404(Product, pk=product_id)
    review_msg = ngettext(
        "%(count)d review",
        "%(count)d reviews",
        product.review_count
    ) % {"count": product.review_count}
    return render(request, "product.html", {
        "product": product,
        "review_msg": review_msg,
    })
```

```python
# settings.py
USE_I18N = True
USE_L10N = True
LANGUAGE_CODE = "en"
LANGUAGES = [
    ("en", "English"),
    ("es", "Spanish"),
    ("fr", "French"),
]
LOCALE_PATHS = [BASE_DIR / "locale"]
```

**Workflow:**
1. Mark strings with `_()` (gettext) or `ngettext()` (plurals)
2. Run `django-admin makemessages -l es` to extract into .po files
3. Translate .po files
4. Run `django-admin compilemessages` to generate .mo binaries
5. Set locale via `LocaleMiddleware` (reads Accept-Language, URL prefix, session, cookie)

---

## 3. Translation Workflow

### 3.1 AI-Assisted Translation

The translation landscape shifted dramatically in 2024-2025. AI translation now produces near-professional quality for most language pairs.

#### Translation Quality Ranking (2025-2026)

Based on blind evaluations by professional translators:

| Provider | Strengths | Weaknesses | Cost (per 1M chars) |
|----------|-----------|------------|---------------------|
| **DeepL** | Highest raw quality for European languages, purpose-built translation LLM, 2-3x fewer edits than competitors | Fewer language pairs historically (now expanding), no general LLM capabilities | ~$20 (API Pro) |
| **Claude 3.5+** | Best "good" rating in Lokalise blind study, excellent context understanding, handles ambiguity well | Not purpose-built for translation, requires prompting for consistency | ~$15 (API) |
| **GPT-4** | Good quality, wide language support, flexible prompting | Higher edit rates than DeepL for European languages, inconsistent with technical terms | ~$15 (API) |
| **Google Cloud Translation** | Widest language coverage (130+ languages), Neural MT for common pairs, lowest cost for high volume | Lower quality for nuanced content, less context awareness | ~$20 (base), $80 (advanced) |

#### AI Translation Workflow

The production-grade workflow uses AI as a first pass with human review:

```
Source strings (en)
  |
  v
[AI Translation] — DeepL API or LLM
  |
  v
[Translation Memory Check] — skip strings already in TM
  |
  v
[Automated QA] — placeholder integrity, length, forbidden terms
  |
  v
[Human Review] — native speaker review, context correction
  |
  v
[TM Update] — approved translations stored in translation memory
  |
  v
Translated strings (fr, es, de, etc.)
```

**DeepL API integration example:**
```typescript
// translate.ts
async function translateStrings(
  strings: Record<string, string>,
  targetLang: string
): Promise<Record<string, string>> {
  const results: Record<string, string> = {};

  // Batch strings to minimize API calls
  const keys = Object.keys(strings);
  const values = Object.values(strings);

  const response = await fetch("https://api-free.deepl.com/v2/translate", {
    method: "POST",
    headers: {
      Authorization: `DeepL-Auth-Key ${process.env.DEEPL_API_KEY}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      text: values,
      target_lang: targetLang.toUpperCase(),
      preserve_formatting: true,
      tag_handling: "html", // Preserves HTML tags in translations
    }),
  });

  const data = await response.json();
  data.translations.forEach((t: { text: string }, i: number) => {
    results[keys[i]] = t.text;
  });

  return results;
}
```

### 3.2 Translation Memory (TM)

Translation Memory stores previously approved translations as source-target pairs. When a new string matches or fuzzy-matches an existing entry, the TM suggests the stored translation.

**Benefits:**
- Consistency across the project (same term always translated the same way)
- Cost reduction (no re-translation of repeated strings)
- Speed (instant suggestions for exact or fuzzy matches)

**TM match types:**
- **100% match** — exact string already translated
- **Context match (101%)** — exact string in the same context (file, position)
- **Fuzzy match (75-99%)** — similar string, needs review
- **Machine translation** — no TM match, AI-translated

All major TMS platforms (Crowdin, Lokalise, Phrase) maintain TM automatically.

### 3.3 Continuous Localization

Continuous localization integrates translation into the CI/CD pipeline. Translations flow as continuously as code.

**The workflow:**

```
Developer pushes code with new/changed strings
  |
  v
[CI] Extracts strings, pushes to TMS (Crowdin/Lokalise/Phrase)
  |
  v
[TMS] Applies TM matches, triggers AI pre-translation
  |
  v
[Translators] Review and approve in TMS UI
  |
  v
[TMS] Pushes approved translations as PR back to repo
  |
  v
[CI] Merges translation PR, deploys
```

#### Platform Comparison (2025-2026)

| Feature | Crowdin | Lokalise | Phrase | Transifex |
|---------|---------|----------|--------|-----------|
| GitHub/GitLab integration | Excellent | Good | Good | Good |
| AI pre-translation | Built-in (DeepL, GPT, etc.) | Built-in | Built-in | Built-in |
| Translation Memory | Yes | Yes | Yes | Yes |
| In-context editing | Yes (screenshots + live) | Yes (screenshots) | Yes (screenshots) | Limited |
| CLI tool | crowdin-cli | lokalise2 | phrase-cli | tx-cli |
| File format support | 50+ formats | 40+ formats | 50+ formats | 30+ formats |
| Free tier | Open source projects | None | None | Open source projects |
| Starting price | $30/mo | $120/mo | Custom | $99/mo |
| Best for | Open source, developer-first | Product teams, design integration | Enterprise, complex workflows | Community translation |

**Crowdin GitHub integration example (`.crowdin.yml`):**
```yaml
project_id_env: CROWDIN_PROJECT_ID
api_token_env: CROWDIN_API_TOKEN

files:
  - source: /src/i18n/locales/en/*.json
    translation: /src/i18n/locales/%locale%/%original_file_name%
    type: json

  - source: /src/content/blog/en/**/*.md
    translation: /src/content/blog/%locale%/**/%original_file_name%
    type: md
```

---

## 4. URL Strategy

### 4.1 URL Structure Options

| Strategy | Example | SEO Signal | Management Complexity | Best For |
|----------|---------|-----------|----------------------|----------|
| **Subdirectory** | `example.com/fr/about` | Strong (consolidated domain authority) | Low (single domain, single hosting) | Most projects |
| **Subdomain** | `fr.example.com/about` | Moderate (treated as separate site by Google) | Medium (DNS config, separate Search Console properties) | Different platforms per market |
| **ccTLD** | `example.fr/about` | Strongest geo-signal | High (separate domains, separate hosting, separate SEO) | Large enterprises targeting specific countries |
| **Query parameter** | `example.com/about?lang=fr` | None (Google ignores query params for lang) | Low | Never recommended for SEO |

**Recommendation for most projects: Subdirectory.**

Subdirectories concentrate all link equity on a single domain. You get one Google Search Console property, one analytics setup, and one CDN configuration. The SEO benefit of a single strong domain outweighs the weak geo-signal penalty, which hreflang tags solve.

### 4.2 URL Design Decisions

**Should you translate URL slugs?**

| Approach | URL | Pros | Cons |
|----------|-----|------|------|
| Keep English slugs | `/fr/about` | Simpler, no redirect management, consistent across locales | Less natural for users |
| Translate slugs | `/fr/a-propos` | More natural, potential slight SEO benefit for localized keywords | Requires redirect mapping, increases maintenance, complicates language switching |

**Recommendation:** Keep English slugs unless SEO research shows significant keyword volume in translated slugs. The maintenance cost of translated slugs rarely justifies the benefit.

**Should the default locale have a prefix?**

| Approach | English URL | French URL | Tradeoffs |
|----------|------------|-----------|-----------|
| No prefix for default | `/about` | `/fr/about` | Cleaner URLs for primary audience, but asymmetric structure |
| Prefix for all | `/en/about` | `/fr/about` | Consistent structure, clearer for crawlers, requires root redirect |

**Recommendation:** No prefix for default locale for content sites (cleaner URLs). Prefix for all in web apps (consistent routing logic).

### 4.3 hreflang Implementation

hreflang tells search engines which language version of a page to show to which users. It is the single most important SEO signal for multilingual sites — and 75% of implementations across the web have errors.

**Three non-negotiable rules:**
1. Every page must include a **self-referencing** hreflang tag
2. All annotations must be **symmetric** (if page A references page B, page B must reference page A)
3. Language codes must be **valid ISO 639-1** (with optional ISO 3166-1 country codes)

**Implementation via HTML head (recommended for static sites):**
```html
<!-- On /about (English page) -->
<link rel="alternate" hreflang="en" href="https://example.com/about" />
<link rel="alternate" hreflang="fr" href="https://example.com/fr/about" />
<link rel="alternate" hreflang="es" href="https://example.com/es/about" />
<link rel="alternate" hreflang="x-default" href="https://example.com/about" />

<!-- x-default = fallback for users whose language isn't matched -->
```

**Implementation via XML sitemap (recommended for large sites):**
```xml
<url>
  <loc>https://example.com/about</loc>
  <xhtml:link rel="alternate" hreflang="en" href="https://example.com/about" />
  <xhtml:link rel="alternate" hreflang="fr" href="https://example.com/fr/about" />
  <xhtml:link rel="alternate" hreflang="es" href="https://example.com/es/about" />
  <xhtml:link rel="alternate" hreflang="x-default" href="https://example.com/about" />
</url>
```

**Implementation via HTTP header (for non-HTML resources like PDFs):**
```
Link: <https://example.com/about>; rel="alternate"; hreflang="en",
      <https://example.com/fr/about>; rel="alternate"; hreflang="fr",
      <https://example.com/es/about>; rel="alternate"; hreflang="es"
```

**Common hreflang mistakes:**
- Missing self-referencing tag (most common error)
- Asymmetric annotations (page A points to B, but B doesn't point back to A)
- Using relative URLs instead of absolute URLs
- Using invalid language codes (`en-UK` instead of `en-GB`)
- Pointing hreflang to redirected URLs (must point to final canonical URL)
- Missing `x-default` tag

**Astro hreflang generation:**
```astro
---
// src/components/HreflangTags.astro
import { getAbsoluteLocaleUrl } from "astro:i18n";

interface Props {
  path: string;
  locales: string[];
  defaultLocale: string;
  siteUrl: string;
}

const { path, locales, defaultLocale, siteUrl } = Astro.props;
---

{locales.map((locale) => (
  <link
    rel="alternate"
    hreflang={locale}
    href={new URL(getAbsoluteLocaleUrl(locale, path), siteUrl).href}
  />
))}
<link
  rel="alternate"
  hreflang="x-default"
  href={new URL(getAbsoluteLocaleUrl(defaultLocale, path), siteUrl).href}
/>
```

---

## 5. Content Strategy

### 5.1 What to Translate (Priority Order)

Not all content needs translation simultaneously. Prioritize by business impact:

**Tier 1 — Translate immediately (blocks revenue/conversion):**
- Navigation and UI chrome
- Landing pages and hero sections
- Pricing pages
- Signup/login flows
- Error messages and form validation
- Legal pages (privacy policy, terms of service) — often legally required
- Transactional emails (welcome, password reset, receipts)

**Tier 2 — Translate soon (improves retention):**
- Product/feature pages
- Help center / FAQ
- Onboarding flows
- Settings and account pages
- Marketing emails

**Tier 3 — Translate when ready (improves SEO and reach):**
- Blog posts (start with top-traffic posts)
- Documentation
- Case studies and testimonials
- Social proof elements

**Tier 4 — Translate selectively or never:**
- API documentation (English is standard for developer docs)
- Internal tools
- Debug/log messages
- Changelogs (unless user-facing)
- Code comments
- Technical specs

### 5.2 Content That Should Not Be Translated

- **Brand names and product names** — "CruxDev" stays "CruxDev" in all locales
- **Code samples and code blocks** — code is code
- **URLs and API endpoints** — never translate technical paths
- **Proper nouns** — names of people, companies, technologies
- **Standardized abbreviations** — HTML, CSS, API, JSON, URL

### 5.3 Locale-Specific Content (Not Just Translation)

Some content needs localization beyond translation:

- **Currency display** — show prices in local currency, not just translated
- **Date formats** — `March 28, 2026` vs. `28 mars 2026` vs. `2026年3月28日`
- **Phone number formats** — country code, spacing conventions
- **Address formats** — order of fields varies by country
- **Legal compliance** — GDPR for EU, CCPA for California, LGPD for Brazil
- **Payment methods** — iDEAL in Netherlands, Boleto in Brazil, Alipay in China
- **Social proof** — testimonials from the target market resonate more
- **Images and screenshots** — localize UI screenshots, avoid text in images

---

## 6. SEO for Multilingual Sites

### 6.1 Technical SEO Checklist

| Element | Implementation | Priority |
|---------|---------------|----------|
| hreflang tags | On every page, symmetric, self-referencing | Critical |
| `<html lang>` attribute | Set per page, valid BCP 47 | Critical |
| Canonical URLs | One canonical per locale (not cross-locale) | Critical |
| Sitemap per locale | Separate sitemap or hreflang in sitemap | High |
| x-default hreflang | Points to default/fallback page | High |
| Absolute URLs in hreflang | Never relative URLs | High |
| Meta descriptions per locale | Translated, not just copied | Medium |
| Structured data per locale | Translated names, descriptions | Medium |
| Open Graph tags per locale | Translated titles, descriptions | Medium |
| Image alt text per locale | Translated alt attributes | Medium |

### 6.2 Canonical URLs

**Rule:** Each locale version of a page is its own canonical. Do NOT point French pages to English canonicals.

```html
<!-- On /fr/about — CORRECT -->
<link rel="canonical" href="https://example.com/fr/about" />

<!-- On /fr/about — WRONG (points to English) -->
<link rel="canonical" href="https://example.com/about" />
```

Cross-locale canonicals tell Google that the translated page is a duplicate of the English page, which defeats the purpose of i18n.

### 6.3 Sitemaps

**Option A: hreflang in sitemap (recommended)**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"
        xmlns:xhtml="http://www.w3.org/1999/xhtml">
  <url>
    <loc>https://example.com/about</loc>
    <xhtml:link rel="alternate" hreflang="en" href="https://example.com/about"/>
    <xhtml:link rel="alternate" hreflang="fr" href="https://example.com/fr/about"/>
    <xhtml:link rel="alternate" hreflang="x-default" href="https://example.com/about"/>
  </url>
  <url>
    <loc>https://example.com/fr/about</loc>
    <xhtml:link rel="alternate" hreflang="en" href="https://example.com/about"/>
    <xhtml:link rel="alternate" hreflang="fr" href="https://example.com/fr/about"/>
    <xhtml:link rel="alternate" hreflang="x-default" href="https://example.com/about"/>
  </url>
</urlset>
```

**Option B: Separate sitemap per locale**
```xml
<!-- sitemap-index.xml -->
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <sitemap><loc>https://example.com/sitemap-en.xml</loc></sitemap>
  <sitemap><loc>https://example.com/sitemap-fr.xml</loc></sitemap>
  <sitemap><loc>https://example.com/sitemap-es.xml</loc></sitemap>
</sitemapindex>
```

### 6.4 Google Search Console

- **Subdirectory setup:** One property (`example.com`) with URL prefix. Use the "International Targeting" report to monitor hreflang errors.
- **Subdomain setup:** Separate properties (`fr.example.com`, `es.example.com`) plus a domain-level property.
- **Check hreflang errors regularly.** Google reports missing return tags, unsupported language codes, and other issues.

### 6.5 Structured Data per Locale

```json
{
  "@context": "https://schema.org",
  "@type": "WebPage",
  "name": "A propos de CruxDev",
  "description": "CruxDev est un framework de convergence autonome pour le developpement pilote par l'IA.",
  "inLanguage": "fr",
  "url": "https://cruxdev.dev/fr/about",
  "isPartOf": {
    "@type": "WebSite",
    "name": "CruxDev",
    "url": "https://cruxdev.dev/fr/"
  }
}
```

Every structured data block must use the locale's language for `name`, `description`, and other user-facing fields. The `inLanguage` property signals the content language to search engines.

---

## 7. Static Site i18n

### 7.1 Astro Content Collections Per Locale

The recommended pattern for Astro sites (and what cruxdev.dev should use):

**Directory structure:**
```
src/
  content/
    blog/
      en/
        getting-started.md
        convergence-patterns.md
        tdd-for-ai.md
      es/
        getting-started.md
        convergence-patterns.md
      fr/
        getting-started.md
    docs/
      en/
        installation.md
        configuration.md
      es/
        installation.md
  pages/
    index.astro           # Root redirect or default locale page
    about.astro           # English (default locale, no prefix)
    blog/
      index.astro         # English blog index
      [...slug].astro     # English blog posts
    [lang]/
      about.astro         # Translated about pages
      blog/
        index.astro       # Translated blog index
        [...slug].astro   # Translated blog posts
```

**Key pattern:** Content collections use `[lang]/slug` as the entry ID. Filter by language prefix when querying:

```typescript
// Get all posts for a specific locale, with fallback to default
async function getPostsForLocale(locale: string, defaultLocale = "en") {
  const allPosts = await getCollection("blog");
  const localePosts = allPosts.filter((p) => p.id.startsWith(`${locale}/`));

  // If a post exists in the target locale, use it; otherwise fall back
  const defaultPosts = allPosts.filter((p) => p.id.startsWith(`${defaultLocale}/`));
  const postSlugs = new Set(defaultPosts.map((p) => p.id.replace(`${defaultLocale}/`, "")));

  const result = [];
  for (const slug of postSlugs) {
    const localized = localePosts.find((p) => p.id === `${locale}/${slug}`);
    const fallback = defaultPosts.find((p) => p.id === `${defaultLocale}/${slug}`);
    result.push(localized || fallback);
  }

  return result.filter(Boolean).sort((a, b) => b.data.date.valueOf() - a.data.date.valueOf());
}
```

### 7.2 Hugo Multilingual

Hugo has built-in multilingual support with two content organization modes:

**Mode 1: File suffix (recommended for small sites):**
```
content/
  posts/
    my-post.en.md
    my-post.fr.md
    my-post.es.md
```

**Mode 2: Directory per language (recommended for large sites):**
```
content/
  en/
    posts/
      my-post.md
  fr/
    posts/
      my-post.md
```

**Hugo config (`hugo.yaml`):**
```yaml
defaultContentLanguage: en
defaultContentLanguageInSubdir: false  # /about for English, /fr/about for French

languages:
  en:
    languageName: English
    weight: 1
    title: My Site
  fr:
    languageName: Francais
    weight: 2
    title: Mon Site
  es:
    languageName: Espanol
    weight: 3
    title: Mi Sitio
```

**Hugo translation strings (`i18n/en.yaml`):**
```yaml
home:
  other: "Home"
readMore:
  other: "Read more"
items:
  one: "{{ .Count }} item"
  other: "{{ .Count }} items"
```

**Usage in templates:**
```go
{{ i18n "home" }}
{{ i18n "items" (dict "Count" 5) }}
```

### 7.3 Build-Time Page Generation

For static sites, every locale combination generates a separate HTML file at build time:

```
dist/
  index.html                 # English home
  about/index.html           # English about
  blog/index.html            # English blog index
  blog/getting-started/index.html
  fr/
    index.html               # French home
    about/index.html         # French about (or a-propos/index.html)
    blog/index.html          # French blog index
    blog/getting-started/index.html
  es/
    index.html
    about/index.html
    blog/index.html
    blog/getting-started/index.html
```

**Build time considerations:**
- Each locale multiplies page count. 100 pages x 5 locales = 500 pages.
- Hugo handles this trivially (sub-second builds at 10K+ pages).
- Astro handles it well (minutes for thousands of pages).
- Next.js SSG can be slow — consider ISR (Incremental Static Regeneration) for large multilingual sites.

---

## 8. Dynamic App i18n

### 8.1 Locale Detection

Detect the user's preferred locale using this priority chain:

```
1. Explicit user preference (stored in DB/profile)
       |
       v (not set)
2. URL locale prefix (/fr/... → French)
       |
       v (not present)
3. Cookie/localStorage preference (previous visit)
       |
       v (not set)
4. Accept-Language header (browser preference)
       |
       v (no match with supported locales)
5. Geo-IP (country → default locale for that country)
       |
       v (no match)
6. Default locale (en)
```

**Critical rules:**
- **Never override an explicit user choice** with automatic detection
- **Never use geo-IP for language** — a French speaker in Germany should not get German
- **Geo-IP is acceptable for regional defaults** — currency, date format, country selection
- **Always show a visible language switcher** — automated detection will be wrong for some users

#### Server-Side Detection (Next.js Middleware Example)

```typescript
// middleware.ts
import { NextRequest, NextResponse } from "next/server";
import { match } from "@formatjs/intl-localematcher";
import Negotiator from "negotiator";

const locales = ["en", "fr", "es", "de"];
const defaultLocale = "en";

function getLocale(request: NextRequest): string {
  // 1. Check URL
  const pathname = request.nextUrl.pathname;
  const urlLocale = locales.find(
    (locale) => pathname.startsWith(`/${locale}/`) || pathname === `/${locale}`
  );
  if (urlLocale) return urlLocale;

  // 2. Check cookie
  const cookieLocale = request.cookies.get("NEXT_LOCALE")?.value;
  if (cookieLocale && locales.includes(cookieLocale)) return cookieLocale;

  // 3. Check Accept-Language header
  const headers = { "accept-language": request.headers.get("accept-language") || "" };
  const languages = new Negotiator({ headers }).languages();
  try {
    return match(languages, locales, defaultLocale);
  } catch {
    return defaultLocale;
  }
}

export function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;

  // Skip if URL already has a locale
  const hasLocale = locales.some(
    (locale) => pathname.startsWith(`/${locale}/`) || pathname === `/${locale}`
  );
  if (hasLocale) return;

  // Redirect to detected locale
  const locale = getLocale(request);
  return NextResponse.redirect(new URL(`/${locale}${pathname}`, request.url));
}

export const config = {
  matcher: ["/((?!api|_next|.*\\..*).*)"],
};
```

### 8.2 Locale Switching

When a user switches locale, preserve their current page position:

```typescript
// Language switcher logic
function switchLocale(currentPath: string, currentLocale: string, newLocale: string): string {
  // Remove current locale prefix
  const pathWithoutLocale = currentPath.replace(new RegExp(`^/${currentLocale}`), "");

  // Add new locale prefix (skip for default locale if not using prefix)
  return newLocale === defaultLocale
    ? pathWithoutLocale || "/"
    : `/${newLocale}${pathWithoutLocale || "/"}`;
}
```

### 8.3 Persisting Preference

```typescript
// After user explicitly selects a language
function setLocalePreference(locale: string) {
  // 1. Set cookie (server-readable on next request)
  document.cookie = `PREFERRED_LOCALE=${locale};path=/;max-age=31536000;SameSite=Lax`;

  // 2. Set localStorage (client-readable for SPA hydration)
  localStorage.setItem("preferred-locale", locale);

  // 3. If user is logged in, save to profile (persists across devices)
  if (isAuthenticated()) {
    updateUserProfile({ preferredLocale: locale });
  }
}
```

---

## 9. Date/Time/Number Formatting

### 9.1 The Intl API

The JavaScript `Intl` API provides locale-aware formatting for dates, numbers, currencies, and relative time — with zero dependencies. Use it instead of libraries like moment.js, date-fns/locale, or numeral.js.

#### Date and Time Formatting

```typescript
// Basic date formatting
const date = new Date("2026-03-28T14:30:00Z");

new Intl.DateTimeFormat("en-US").format(date);
// "3/28/2026"

new Intl.DateTimeFormat("de-DE").format(date);
// "28.3.2026"

new Intl.DateTimeFormat("ja-JP").format(date);
// "2026/3/28"

// dateStyle + timeStyle (modern, simplified)
new Intl.DateTimeFormat("en-US", {
  dateStyle: "full",
  timeStyle: "short",
}).format(date);
// "Saturday, March 28, 2026 at 2:30 PM"

new Intl.DateTimeFormat("fr-FR", {
  dateStyle: "long",
  timeStyle: "short",
}).format(date);
// "28 mars 2026 a 14:30"

// Custom format
new Intl.DateTimeFormat("en-US", {
  weekday: "short",
  year: "numeric",
  month: "short",
  day: "numeric",
  hour: "2-digit",
  minute: "2-digit",
  timeZone: "America/New_York",
  timeZoneName: "short",
}).format(date);
// "Sat, Mar 28, 2026, 10:30 AM EDT"
```

#### Relative Time Formatting

```typescript
const rtf = new Intl.RelativeTimeFormat("en", { numeric: "auto" });

rtf.format(-1, "day");    // "yesterday"
rtf.format(2, "hour");    // "in 2 hours"
rtf.format(-3, "month");  // "3 months ago"

const rtfFr = new Intl.RelativeTimeFormat("fr", { numeric: "auto" });
rtfFr.format(-1, "day");  // "hier"
rtfFr.format(2, "hour");  // "dans 2 heures"
```

#### Number Formatting

```typescript
// Basic number formatting
new Intl.NumberFormat("en-US").format(1234567.89);
// "1,234,567.89"

new Intl.NumberFormat("de-DE").format(1234567.89);
// "1.234.567,89"

new Intl.NumberFormat("fr-FR").format(1234567.89);
// "1 234 567,89"

// Currency
new Intl.NumberFormat("en-US", {
  style: "currency",
  currency: "USD",
}).format(42.5);
// "$42.50"

new Intl.NumberFormat("ja-JP", {
  style: "currency",
  currency: "JPY",
}).format(4250);
// "¥4,250"

new Intl.NumberFormat("de-DE", {
  style: "currency",
  currency: "EUR",
}).format(42.5);
// "42,50 €"

// Percentage
new Intl.NumberFormat("en-US", {
  style: "percent",
  minimumFractionDigits: 1,
}).format(0.875);
// "87.5%"

// Compact notation
new Intl.NumberFormat("en-US", {
  notation: "compact",
  compactDisplay: "short",
}).format(1500000);
// "1.5M"
```

#### List Formatting

```typescript
const list = ["apples", "oranges", "bananas"];

new Intl.ListFormat("en", { style: "long", type: "conjunction" }).format(list);
// "apples, oranges, and bananas"

new Intl.ListFormat("fr", { style: "long", type: "conjunction" }).format(
  ["pommes", "oranges", "bananes"]
);
// "pommes, oranges et bananes"

new Intl.ListFormat("ja", { style: "long", type: "conjunction" }).format(
  ["りんご", "オレンジ", "バナナ"]
);
// "りんご、オレンジ、バナナ"
```

### 9.2 Timezone Handling

**Key principle:** Store dates in UTC. Display in the user's timezone.

```typescript
// Store in UTC (always)
const utcDate = new Date().toISOString();
// "2026-03-28T14:30:00.000Z"

// Display in user's timezone
const userTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
// "America/New_York"

new Intl.DateTimeFormat("en-US", {
  dateStyle: "medium",
  timeStyle: "short",
  timeZone: userTimezone,
}).format(new Date(utcDate));
// "Mar 28, 2026, 10:30 AM"
```

**Astro-specific timezone pattern (client-side display):**
```astro
---
// Server renders UTC, client script updates to local time
const publishDate = post.data.date.toISOString();
---

<time datetime={publishDate} data-localize-date>
  {post.data.date.toLocaleDateString("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
  })}
</time>

<script>
  document.querySelectorAll("[data-localize-date]").forEach((el) => {
    const utc = el.getAttribute("datetime");
    if (utc) {
      el.textContent = new Date(utc).toLocaleDateString(undefined, {
        year: "numeric",
        month: "long",
        day: "numeric",
      });
    }
  });
</script>
```

### 9.3 Currency Display

**Never hardcode currency symbols.** Use `Intl.NumberFormat` with the correct currency code:

```typescript
interface Price {
  amount: number;
  currency: string; // ISO 4217: "USD", "EUR", "JPY", etc.
}

function formatPrice(price: Price, locale: string): string {
  return new Intl.NumberFormat(locale, {
    style: "currency",
    currency: price.currency,
    // JPY has 0 fraction digits, USD has 2 — Intl handles this automatically
  }).format(price.amount);
}

formatPrice({ amount: 49.99, currency: "USD" }, "en-US");  // "$49.99"
formatPrice({ amount: 49.99, currency: "EUR" }, "de-DE");  // "49,99 €"
formatPrice({ amount: 5000, currency: "JPY" }, "ja-JP");   // "¥5,000"
```

---

## 10. Accessibility in i18n

### 10.1 The lang Attribute (WCAG 3.1.1 and 3.1.2)

**WCAG 3.1.1 — Language of Page (Level A):** Every page must declare its primary language on the `<html>` element.

```html
<html lang="fr">
```

**WCAG 3.1.2 — Language of Parts (Level AA):** Inline content in a different language must be marked with its own `lang` attribute.

```html
<p>
  The French word <span lang="fr">bonjour</span> means "hello."
</p>
```

**Why this matters:** Screen readers use the `lang` attribute to select the correct pronunciation engine. Without it, a French screen reader will try to pronounce English text with French phonetics, making content unintelligible.

**Use valid BCP 47 language tags:**
- `en` (English), `fr` (French), `de` (German), `ja` (Japanese)
- `en-US` (American English), `en-GB` (British English), `pt-BR` (Brazilian Portuguese)
- NOT `en-UK` (invalid — use `en-GB`), NOT `jp` (invalid — use `ja`)

### 10.2 Text Direction

```html
<!-- Set direction on the root element -->
<html lang="ar" dir="rtl">

<!-- Override direction for embedded LTR content -->
<p dir="rtl">
  هذا النص بالعربية مع <span dir="ltr">English text</span> في الوسط.
</p>
```

**Use the `dir` attribute, not CSS `direction` property.** The HTML `dir` attribute affects the bidi algorithm; CSS `direction` only affects visual rendering and can create accessibility issues when the two disagree.

### 10.3 ARIA Labels in Translation

All ARIA attributes that contain user-facing text must be translated:

```html
<!-- English -->
<button aria-label="Close dialog">X</button>
<nav aria-label="Main navigation">...</nav>
<input aria-describedby="email-help" />

<!-- French -->
<button aria-label="Fermer la boite de dialogue">X</button>
<nav aria-label="Navigation principale">...</nav>
<input aria-describedby="email-help" />
```

**Untranslated ARIA labels are worse than missing ARIA labels** — they create a confusing bilingual experience for screen reader users.

### 10.4 Font Considerations

| Language | Consideration |
|----------|--------------|
| Chinese, Japanese, Korean (CJK) | Need dedicated CJK fonts; Latin fonts lack these glyphs. Font files are large (2-15MB) — use font subsetting or dynamic subsetting services (Google Fonts handles this automatically). |
| Arabic, Hebrew | Need fonts that support right-to-left cursive joining. Not all fonts handle this correctly. |
| Thai, Burmese | Complex script shaping — test with real text, not lorem ipsum. |
| All languages | Set `line-height` generously (1.5+) — CJK and Arabic text often needs more vertical space than Latin. |

```css
/* Recommended font stack with CJK fallbacks */
body {
  font-family:
    "Inter",                    /* Primary Latin font */
    "Noto Sans JP",             /* Japanese fallback */
    "Noto Sans SC",             /* Simplified Chinese fallback */
    "Noto Sans KR",             /* Korean fallback */
    "Noto Sans Arabic",         /* Arabic fallback */
    system-ui,
    sans-serif;
  line-height: 1.6;            /* Generous for CJK and Arabic */
}
```

### 10.5 Screen Reader Language Switching

Modern screen readers (NVDA, JAWS, VoiceOver) automatically switch voice/pronunciation based on the `lang` attribute. This means:

1. Correct `<html lang>` enables correct page-level pronunciation
2. Inline `lang` attributes switch pronunciation mid-sentence
3. Missing or incorrect `lang` values cause mispronunciation
4. Screen readers that support multiple languages switch "fairly seamlessly" when markup is correct

**Testing tip:** Use NVDA (free, Windows) or VoiceOver (built-in, macOS) to test language switching on your multilingual pages.

---

## 11. Testing i18n

### 11.1 Pseudo-Localization

Pseudo-localization transforms English text into a modified version that simulates translation characteristics while remaining readable. It is the single most effective i18n testing technique.

**What pseudo-localization looks like:**
```
English:  "Welcome to our app"
Pseudo:   "[Ŵëľčöɱë ţö öüŕ àƥƥ XXXXXXXXX]"
```

**What it catches:**
- **Hardcoded strings** — untranslated text is immediately visible (no accented characters)
- **Text expansion** — added padding characters simulate 30-40% growth
- **Encoding issues** — accented characters break if UTF-8 is misconfigured
- **Placeholder corruption** — `{name}` and `{{count}}` must survive transformation
- **Truncation** — UI elements that can't handle longer text become obvious
- **Concatenation bugs** — broken sentences from string concatenation are visible

**i18next pseudo-localization:**
```typescript
import i18next from "i18next";
import i18nextPseudo from "i18next-pseudo";

i18next.use(i18nextPseudo).init({
  lng: "en",
  postProcess: ["pseudo"],
  // ... other config
});
```

**Phoenix pseudo-localization:**
```elixir
# mix.exs
{:gettext_pseudolocalize, "~> 0.1", only: [:dev]}
```

### 11.2 Text Expansion Testing

Average text expansion by language (relative to English):

| Language | Expansion | Example |
|----------|----------|---------|
| German | +30-35% | "Settings" → "Einstellungen" |
| Finnish | +30-40% | "Settings" → "Asetukset" |
| French | +15-20% | "Settings" → "Parametres" |
| Spanish | +20-25% | "Settings" → "Configuracion" |
| Italian | +15-20% | "Settings" → "Impostazioni" |
| Russian | +15-25% | "Settings" → "Настройки" |
| Japanese | -10-30% | "Settings" → "設定" (shorter but taller) |
| Chinese | -20-40% | "Settings" → "设置" (shorter but taller) |
| Arabic | +20-25% | RTL, different metrics |

**Design rules to survive expansion:**
- Never use fixed-width containers for translated text
- Use `min-width` and `max-width` instead of `width`
- Test with German or Finnish as worst-case expansion
- Allow buttons and labels to wrap or expand
- Use CSS `text-overflow: ellipsis` as a safety net, not as a design choice

### 11.3 RTL Visual Testing

RTL testing checklist:

- [ ] Layout mirrors correctly (sidebar, navigation, content flow)
- [ ] Icons with directional meaning are mirrored (arrows, progress bars)
- [ ] Text alignment switches to right-aligned
- [ ] Form fields maintain correct order
- [ ] Horizontal scroll direction reverses
- [ ] Bidirectional text renders correctly (Arabic with embedded English)
- [ ] CSS animations with X-axis transforms reverse
- [ ] Images with embedded text are replaced with RTL versions

**Automated RTL testing:**
```typescript
// Playwright test for RTL layout
import { test, expect } from "@playwright/test";

test("RTL layout mirrors correctly", async ({ page }) => {
  await page.goto("/ar/");
  const html = await page.locator("html");
  await expect(html).toHaveAttribute("dir", "rtl");
  await expect(html).toHaveAttribute("lang", "ar");

  // Verify sidebar is on the right
  const sidebar = page.locator("[data-testid='sidebar']");
  const box = await sidebar.boundingBox();
  const viewport = page.viewportSize();
  expect(box.x).toBeGreaterThan(viewport.width / 2);
});
```

### 11.4 Missing Translation Detection

**Build-time detection (recommended):**
```typescript
// scripts/check-translations.ts
import { readdirSync, readFileSync } from "fs";
import { join } from "path";

const localesDir = "src/i18n/locales";
const defaultLocale = "en";

const defaultStrings = JSON.parse(
  readFileSync(join(localesDir, defaultLocale, "common.json"), "utf-8")
);
const defaultKeys = new Set(Object.keys(flattenObject(defaultStrings)));

const locales = readdirSync(localesDir).filter((d) => d !== defaultLocale);
let hasErrors = false;

for (const locale of locales) {
  const localeStrings = JSON.parse(
    readFileSync(join(localesDir, locale, "common.json"), "utf-8")
  );
  const localeKeys = new Set(Object.keys(flattenObject(localeStrings)));

  // Check for missing translations
  for (const key of defaultKeys) {
    if (!localeKeys.has(key)) {
      console.error(`MISSING: ${locale}/common.json is missing key "${key}"`);
      hasErrors = true;
    }
  }

  // Check for orphaned translations (in locale but not in default)
  for (const key of localeKeys) {
    if (!defaultKeys.has(key)) {
      console.warn(`ORPHAN: ${locale}/common.json has extra key "${key}"`);
    }
  }
}

if (hasErrors) process.exit(1);

function flattenObject(obj: any, prefix = ""): Record<string, string> {
  const result: Record<string, string> = {};
  for (const [key, value] of Object.entries(obj)) {
    const fullKey = prefix ? `${prefix}.${key}` : key;
    if (typeof value === "object" && value !== null) {
      Object.assign(result, flattenObject(value, fullKey));
    } else {
      result[fullKey] = String(value);
    }
  }
  return result;
}
```

**Runtime detection (development mode):**
```typescript
// Log missing translations in development
function t(key: string, locale: string): string {
  const translation = messages[locale]?.[key];
  if (!translation) {
    if (process.env.NODE_ENV === "development") {
      console.warn(`[i18n] Missing translation: ${locale}/${key}`);
    }
    return messages[defaultLocale]?.[key] || key;
  }
  return translation;
}
```

### 11.5 i18n CI Pipeline

```yaml
# .github/workflows/i18n-check.yml
name: i18n Checks
on: [push, pull_request]

jobs:
  i18n:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Check for missing translations
        run: npx tsx scripts/check-translations.ts

      - name: Validate hreflang tags
        run: npx tsx scripts/validate-hreflang.ts

      - name: Check placeholder integrity
        run: npx tsx scripts/check-placeholders.ts

      - name: Build all locales
        run: npm run build

      - name: Verify all locale pages generated
        run: |
          for locale in en fr es de; do
            if [ ! -f "dist/${locale}/index.html" ] && [ "$locale" != "en" ]; then
              echo "ERROR: Missing ${locale}/index.html"
              exit 1
            fi
          done
```

---

## 12. Anti-Patterns

### 12.1 Hardcoded Strings

**Problem:** Strings embedded directly in source code cannot be extracted for translation.

```tsx
// WRONG
<button>Submit</button>
<p>Welcome back, {user.name}!</p>
<span>Error: Invalid email address</span>

// RIGHT
<button>{t("form.submit")}</button>
<p>{t("greeting.welcome", { name: user.name })}</p>
<span>{t("errors.invalidEmail")}</span>
```

**Detection:** Pseudo-localization makes hardcoded strings immediately visible — they lack accented characters while translated strings have them.

### 12.2 Concatenated Translations

**Problem:** Building sentences by joining string fragments breaks in languages with different word order.

```typescript
// WRONG — word order varies across languages
const msg = t("you_have") + " " + count + " " + t("new_messages");
// English: "You have 5 new messages" ✓
// German:  "Sie haben 5 neue Nachrichten" ✓ (happens to work)
// Japanese: "5件の新しいメッセージがあります" ✗ (number goes at start)
// Arabic:  "لديك 5 رسائل جديدة" ✗ (completely different structure)

// RIGHT — use interpolation
const msg = t("new_messages", { count: 5 });
// messages: { new_messages: "You have {count} new messages" }
// messages_ja: { new_messages: "{count}件の新しいメッセージがあります" }
```

**Rule:** Never concatenate translated strings. Always use interpolation with the full sentence as a single translation key.

### 12.3 Locale in Global State

**Problem:** Storing locale in a global variable or React Context that triggers full-page re-renders on change.

```typescript
// WRONG — global mutable state
let currentLocale = "en";
function setLocale(locale: string) {
  currentLocale = locale; // How do components know to re-render?
}

// RIGHT (React) — use the i18n library's built-in locale management
// react-intl uses IntlProvider, i18next uses I18nextProvider
// Both handle re-rendering efficiently
```

**For Astro/static sites:** Locale is part of the URL, not state. Switching locale is a navigation, not a state change. This is the correct architectural pattern.

### 12.4 Translating Slugs Without Redirects

**Problem:** Translating URL slugs without setting up redirects from old URLs breaks bookmarks and SEO.

```
# If you change /fr/about to /fr/a-propos:
# - Old bookmarks to /fr/about break (404)
# - Google's index still points to /fr/about
# - Inbound links break
```

**Solution:** If you translate slugs, always implement 301 redirects from old slugs to new ones. Better yet, don't translate slugs unless SEO research shows significant benefit.

### 12.5 Ignoring Pluralization

**Problem:** Using binary singular/plural logic that breaks in languages with multiple plural forms.

```typescript
// WRONG — assumes all languages have 2 plural forms
const msg = count === 1 ? t("one_item") : t("many_items");

// WRONG — even with "zero" it's not enough for Arabic (6 forms)
const msg = count === 0 ? t("no_items")
          : count === 1 ? t("one_item")
          : t("many_items");

// RIGHT — use ICU plural syntax
// "{count, plural, =0 {No items} one {# item} other {# items}}"
const msg = t("items", { count });
```

### 12.6 Assuming Text Renders the Same Size

**Problem:** Designing fixed-size UI elements that break when text expands.

```css
/* WRONG — fixed width button */
.cta-button {
  width: 120px;
}

/* RIGHT — flexible button */
.cta-button {
  min-width: 120px;
  padding-inline: 24px;
  white-space: nowrap;
}
```

### 12.7 Date/Number Format Assumptions

**Problem:** Hardcoding date or number formats instead of using locale-aware formatting.

```typescript
// WRONG — US-centric date format
const dateStr = `${date.getMonth() + 1}/${date.getDate()}/${date.getFullYear()}`;

// WRONG — hardcoded decimal separator
const priceStr = `$${price.toFixed(2)}`;

// RIGHT — Intl API
const dateStr = new Intl.DateTimeFormat(locale).format(date);
const priceStr = new Intl.NumberFormat(locale, {
  style: "currency",
  currency: currencyCode,
}).format(price);
```

### 12.8 Images with Embedded Text

**Problem:** Text baked into images cannot be translated.

**Solution:** Use CSS text overlays on images, or generate locale-specific image variants. For screenshots of your own UI, regenerate screenshots for each locale.

### 12.9 Using Google Translate as Your Translation Strategy

**Problem:** Raw machine translation without human review leads to embarrassing errors, especially for marketing copy, legal text, and brand messaging.

**Solution:** AI-assisted translation (DeepL, Claude, GPT-4) as a first pass, followed by professional human review for Tier 1 content. Raw MT is acceptable only for user-generated content or internal tools.

---

## 13. Audit Dimensions

Use this checklist to audit any project's i18n implementation. Each dimension has specific, testable criteria.

### 13.1 Translation Coverage

| Check | How to Verify | Pass Criteria |
|-------|--------------|---------------|
| All UI strings externalized | Run pseudo-localization, scan for unaccented text | Zero hardcoded strings visible |
| All supported locales have translation files | Compare file count across locale directories | Same file count per locale |
| Missing key percentage per locale | Run translation checker script | < 5% missing for launched locales, 0% for Tier 1 strings |
| Pluralization uses ICU/CLDR | Grep for ternary-based plural logic | Zero instances of `count === 1 ?` pattern |
| ARIA labels translated | Check all `aria-label`, `aria-describedby` values | All ARIA text in translation files |

### 13.2 String Extraction Completeness

| Check | How to Verify | Pass Criteria |
|-------|--------------|---------------|
| Extraction tool configured | Check build config for i18n extract step | Extraction runs in CI |
| No orphaned keys | Compare locale keys against source code usage | Zero unused keys |
| No dynamic key construction | Grep for `t(\`...\`)` or `t("prefix" + var)` | Zero dynamic key patterns |
| Context provided for ambiguous strings | Check translation files for comments/context | Ambiguous strings have context |

### 13.3 hreflang Correctness

| Check | How to Verify | Pass Criteria |
|-------|--------------|---------------|
| Self-referencing tags present | Parse HTML for hreflang pointing to own URL | Every page self-references |
| Symmetric annotations | For each hreflang pair, verify bidirectional reference | 100% symmetric |
| Valid language codes | Validate against ISO 639-1 / BCP 47 | Zero invalid codes |
| Absolute URLs used | Check hreflang href values | All absolute, no relative |
| x-default present | Check for hreflang="x-default" | Present on every page |
| Points to canonical URLs | Verify hreflang URLs are not redirected | Zero redirected hreflang targets |

### 13.4 Locale Detection

| Check | How to Verify | Pass Criteria |
|-------|--------------|---------------|
| Priority chain implemented | Test with various header/cookie/URL combinations | Correct priority: explicit > URL > cookie > Accept-Language > default |
| User preference persisted | Switch language, close browser, return | Preference remembered |
| Language switcher visible | Visual inspection on every page | Always visible |
| Geo-IP not used for language | Check server-side detection code | No language-from-IP logic |
| Default locale fallback works | Request with unsupported Accept-Language | Falls back to default cleanly |

### 13.5 Date/Number Formatting

| Check | How to Verify | Pass Criteria |
|-------|--------------|---------------|
| Dates use Intl.DateTimeFormat | Grep for manual date formatting | Zero manual date concatenation |
| Numbers use Intl.NumberFormat | Grep for `.toFixed()`, manual formatting | Zero manual number formatting |
| Currency uses correct codes | Check currency display in multiple locales | Correct symbols and decimal handling |
| Timezone handling correct | Check dates stored as UTC, displayed in local TZ | UTC storage, local display |
| Relative time localized | Check "3 days ago" type strings | Uses Intl.RelativeTimeFormat |

### 13.6 RTL Support

| Check | How to Verify | Pass Criteria |
|-------|--------------|---------------|
| `dir="rtl"` set on html | Inspect HTML in RTL locale | Correct `dir` attribute |
| CSS logical properties used | Grep for `margin-left`, `padding-right`, etc. | Zero physical direction properties (or acceptable exceptions) |
| Layout mirrors correctly | Visual inspection in RTL locale | Sidebar, nav, content flow reversed |
| Icons mirrored where needed | Check directional icons (arrows, etc.) | Directional icons reversed |
| Bidirectional text renders | Test mixed LTR/RTL content | Correct rendering per bidi algorithm |

### 13.7 Accessibility

| Check | How to Verify | Pass Criteria |
|-------|--------------|---------------|
| `<html lang>` set correctly | Inspect HTML source per locale | Correct BCP 47 code |
| Inline `lang` for mixed content | Check for foreign-language spans | `lang` attribute on non-primary text |
| ARIA labels translated | Compare ARIA attributes across locales | All ARIA text localized |
| Screen reader tested | Test with NVDA/VoiceOver | Correct pronunciation switching |
| Font support for all scripts | Test CJK, Arabic, Thai rendering | No missing glyphs (tofu boxes) |

### 13.8 Performance

| Check | How to Verify | Pass Criteria |
|-------|--------------|---------------|
| Translation bundle size | Check JS bundle for translation data | < 50KB per locale (gzipped) |
| Lazy loading per namespace | Check network tab during navigation | Only active namespace loaded |
| No full-page re-render on locale switch | Profile rendering during locale change | Targeted re-renders only (or page navigation for static sites) |
| Font loading optimized | Check font loading strategy | `font-display: swap`, preload for primary fonts |
| Build time acceptable | Measure build time with all locales | < 5 minutes for full build |

---

## Appendix A: Quick-Start Decision Matrix

| Question | If Yes... | If No... |
|----------|-----------|----------|
| Static site (Astro, Hugo)? | Use built-in i18n routing + content collections per locale | See dynamic app patterns |
| Next.js App Router? | Use next-intl | Use next-i18next (Pages Router) or migrate to App Router |
| React (no Next.js)? | react-i18next for flexibility, react-intl for ICU compliance | — |
| Vue? | vue-i18n | — |
| SvelteKit? | Paraglide JS | — |
| Phoenix/Elixir? | Gettext (built-in) | — |
| Rails? | Rails I18n (built-in) | — |
| Django? | Django gettext (built-in) | — |
| Need type-safe translations? | Paraglide JS (any framework) or next-intl (Next.js) | Any runtime library |
| Enterprise TMS integration? | Crowdin or Phrase with XLIFF support | JSON + Git-based workflow |
| > 10 locales? | Invest in TMS, translation memory, continuous localization | Manual JSON files may suffice |
| RTL languages needed? | CSS logical properties from day one, test with real Arabic/Hebrew content | Still use logical properties (future-proofing) |
| SEO critical? | Subdirectory URL strategy + hreflang + per-locale sitemaps | URL strategy matters less |

## Appendix B: Astro i18n Implementation Checklist

For cruxdev.dev specifically:

- [ ] Configure `i18n` in `astro.config.mjs` with locales and defaultLocale
- [ ] Create `src/i18n/ui.ts` with all UI strings per locale
- [ ] Create `src/i18n/utils.ts` with `getLangFromUrl`, `useTranslations`, `useTranslatedPath`
- [ ] Set `<html lang={lang} dir={dir}>` in base layout
- [ ] Add hreflang tags to `<head>` for all supported locales
- [ ] Add `x-default` hreflang pointing to English version
- [ ] Organize content collections with `[locale]/` prefix directories
- [ ] Create language picker component with `hreflang` attributes on links
- [ ] Add `aria-label` translations for all interactive elements
- [ ] Generate sitemap with hreflang annotations
- [ ] Set up missing translation detection in CI
- [ ] Test pseudo-localization to find hardcoded strings
- [ ] Test German locale for text expansion issues
- [ ] Add structured data with `inLanguage` per locale
- [ ] Add Open Graph tags per locale
- [ ] Configure separate meta descriptions per locale
- [ ] Set up Crowdin or equivalent for continuous localization
- [ ] Verify all dates use `Intl.DateTimeFormat` or `toLocaleDateString(lang)`

## Appendix C: Language Code Reference

Common language codes used in i18n:

| Code | Language | Plural Forms | Script Direction |
|------|----------|-------------|-----------------|
| `en` | English | 2 (one, other) | LTR |
| `es` | Spanish | 2 (one, other) | LTR |
| `fr` | French | 3 (one, many, other) | LTR |
| `de` | German | 2 (one, other) | LTR |
| `pt-BR` | Brazilian Portuguese | 2 (one, other) | LTR |
| `ja` | Japanese | 1 (other) | LTR |
| `zh-CN` | Simplified Chinese | 1 (other) | LTR |
| `zh-TW` | Traditional Chinese | 1 (other) | LTR |
| `ko` | Korean | 1 (other) | LTR |
| `ar` | Arabic | 6 (zero, one, two, few, many, other) | RTL |
| `he` | Hebrew | 4 (one, two, many, other) | RTL |
| `fa` | Persian (Farsi) | 2 (one, other) | RTL |
| `ru` | Russian | 4 (one, few, many, other) | LTR |
| `pl` | Polish | 4 (one, few, many, other) | LTR |
| `tr` | Turkish | 2 (one, other) | LTR |
| `hi` | Hindi | 2 (one, other) | LTR |
| `th` | Thai | 1 (other) | LTR |
| `vi` | Vietnamese | 1 (other) | LTR |
| `nl` | Dutch | 2 (one, other) | LTR |
| `it` | Italian | 3 (one, many, other) | LTR |
| `sv` | Swedish | 2 (one, other) | LTR |
