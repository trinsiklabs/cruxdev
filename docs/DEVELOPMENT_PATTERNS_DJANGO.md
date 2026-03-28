# Development Patterns — Django Stack

Django / Django REST Framework / HTMX / Alpine.js

This document captures stack-specific patterns, conventions, and decisions for Django stack projects (Django/DRF/HTMX/Alpine.js). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure Django models, test with pytest, use HTMX + Alpine.js, build APIs with DRF, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Python | 3.12+ | Minimum 3.12 for `type` statement, f-string improvements |
| Django | 5.1+ | Async views, `GeneratedField`, facet filters in admin |
| Django REST Framework | 3.15+ | Serializers, viewsets, routers, throttling |
| django-htmx | 1.19+ | HTMX middleware and helpers |
| HTMX | 2.0+ | Loaded via CDN or static files |
| Alpine.js | 3.14+ | Lightweight client-side reactivity |
| django-allauth | 65+ | Authentication (password, OAuth, MFA) |
| django-components | 0.120+ | Reusable template components with encapsulated CSS/JS |
| django-cotton | 1.2+ | HTML-like component syntax alternative |
| Celery | 5.4+ | Distributed background task queue |
| Redis | 7+ | Celery broker, caching, Channels layer |
| PostgreSQL | 16+ | Primary database |
| Tailwind CSS | 4.x | Via django-tailwind or standalone CLI |
| WhiteNoise | 6.8+ | Static file serving in production |
| Gunicorn | 23+ | WSGI server for production |
| Uvicorn | 0.34+ | ASGI server for async views and Channels |
| pytest-django | 4.9+ | Test runner |
| factory_boy | 3.3+ | Test factories |
| coverage | 7.6+ | Coverage measurement and enforcement |

### Version Constraint Policy

Use `~=` (compatible release) constraints in `pyproject.toml` pinned to the minor version:

```toml
# Good — allows patch updates, blocks minor/major
django = "~=5.1"
djangorestframework = "~=3.15"
django-allauth = "~=65.0"

# Bad — too loose, allows breaking minor updates
django = ">=5.0"

# Bad — too tight, blocks patch fixes
django = "==5.1.4"
```

Exception: for release candidates or packages with known instability, pin exact.

### Django 5.x Features to Use

| Feature | Version | Use For |
|---|---|---|
| `GeneratedField` | 5.0+ | Computed DB columns (full name, slug, search vectors) |
| Facet filters in admin | 5.0+ | Admin list filter counts |
| Async views | 5.0+ | I/O-bound views (external API calls, file uploads) |
| `Field.db_default` | 5.0+ | Database-level defaults instead of Python-level |
| Form field rendering | 5.0+ | `as_field_group()` for accessible form rendering |
| `{% querystring %}` | 5.1+ | URL query string manipulation in templates |

---

## 2. Project Structure

### Django App Organization

Each Django app is a bounded context. Apps are kept under a project-level package:

```
project_name/
├── manage.py
├── pyproject.toml
├── config/                     # Project configuration (replaces default project package)
│   ├── __init__.py
│   ├── settings/
│   │   ├── __init__.py
│   │   ├── base.py             # Shared settings
│   │   ├── development.py      # Dev overrides
│   │   ├── test.py             # Test overrides
│   │   └── production.py       # Production overrides
│   ├── urls.py                 # Root URL configuration
│   ├── asgi.py                 # ASGI entry point
│   └── wsgi.py                 # WSGI entry point
├── apps/
│   ├── __init__.py
│   ├── accounts/               # Auth domain
│   │   ├── __init__.py
│   │   ├── models.py           # Custom User model
│   │   ├── admin.py
│   │   ├── forms.py
│   │   ├── views.py
│   │   ├── urls.py
│   │   ├── serializers.py      # DRF serializers
│   │   ├── permissions.py      # Custom DRF permissions
│   │   ├── signals.py
│   │   ├── managers.py         # Custom querysets/managers
│   │   ├── tasks.py            # Celery tasks
│   │   ├── components/         # django-components for this app
│   │   │   └── user_card/
│   │   │       ├── user_card.py
│   │   │       ├── user_card.html
│   │   │       └── user_card.css
│   │   └── templates/
│   │       └── accounts/
│   │           ├── profile.html
│   │           └── partials/   # HTMX partial templates
│   │               └── _user_list.html
│   ├── core/                   # Shared models, utilities, base classes
│   │   ├── __init__.py
│   │   ├── models.py           # Abstract base models (TimeStampedModel, etc.)
│   │   ├── middleware.py
│   │   ├── templatetags/
│   │   │   └── core_tags.py
│   │   └── utils.py
│   └── api/                    # API configuration (DRF routers, versioning)
│       ├── __init__.py
│       ├── urls.py             # API URL router
│       ├── versioning.py
│       ├── throttling.py
│       └── pagination.py
├── static/                     # Project-level static files
│   ├── css/
│   ├── js/
│   └── vendor/
├── templates/                  # Project-level templates
│   ├── base.html
│   ├── partials/               # Shared HTMX partials
│   └── components/             # Shared components
├── tests/                      # All tests (mirrors apps/ structure)
│   ├── conftest.py             # Shared fixtures and factories
│   ├── factories.py            # factory_boy definitions
│   ├── accounts/
│   │   ├── test_models.py
│   │   ├── test_views.py
│   │   ├── test_serializers.py
│   │   └── test_tasks.py
│   └── core/
│       └── test_utils.py
└── fixtures/                   # Seed data (JSON/YAML)
    └── initial_data.json
```

**Convention:** One Django app per bounded context. Models are never imported across app boundaries — use ForeignKey references via string labels (`"accounts.User"`) and domain service functions for cross-app logic.

**Convention:** HTMX partial templates go in a `partials/` subdirectory and are prefixed with `_` (e.g., `_user_list.html`). Full page templates do not have the underscore prefix.

**Convention:** Configuration is split into environment-specific settings files. `DJANGO_SETTINGS_MODULE` selects the active file.

### Test Mirror Structure

Tests mirror the `apps/` structure:

```
tests/
├── conftest.py                 # Shared fixtures, factory registration
├── factories.py                # factory_boy model factories
├── accounts/
│   ├── __init__.py
│   ├── test_models.py          # Unit tests (model methods, properties)
│   ├── test_views.py           # View tests (HTMX responses, status codes)
│   ├── test_serializers.py     # Serializer validation tests
│   ├── test_permissions.py     # Permission class tests
│   └── test_tasks.py           # Celery task tests
├── core/
│   ├── __init__.py
│   └── test_utils.py
└── integration/
    ├── __init__.py
    └── test_api_flows.py       # Multi-endpoint API flow tests
```

---

## 3. Django Model Patterns

### Abstract Base Model

Every model inherits from a timestamped base:

```python
# apps/core/models.py
import uuid
from django.db import models


class TimeStampedModel(models.Model):
    """Abstract base model with UUID primary key and timestamps."""

    id = models.UUIDField(primary_key=True, default=uuid.uuid4, editable=False)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    class Meta:
        abstract = True
        ordering = ["-created_at"]
```

### Model Template

Every model follows this structure:

```python
# apps/chapters/models.py
from django.db import models
from django.urls import reverse
from apps.core.models import TimeStampedModel


class Chapter(TimeStampedModel):
    """A BNI chapter with meeting schedule and seat roster."""

    name = models.CharField(max_length=200)
    slug = models.SlugField(unique=True, max_length=200)
    meeting_day = models.IntegerField(
        choices=[(i, day) for i, day in enumerate(
            ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]
        )]
    )
    meeting_time = models.TimeField()
    location = models.CharField(max_length=300)
    is_active = models.BooleanField(default=True)

    # Django 5.x GeneratedField — computed at DB level
    display_name = models.GeneratedField(
        expression=models.functions.Concat(
            models.F("name"), models.Value(" — "), models.F("location")
        ),
        output_field=models.CharField(max_length=500),
        db_persist=True,
    )

    class Meta:
        ordering = ["name"]
        indexes = [
            models.Index(fields=["slug"]),
            models.Index(fields=["is_active", "meeting_day"]),
        ]

    def __str__(self) -> str:
        return self.name

    def get_absolute_url(self) -> str:
        return reverse("chapters:detail", kwargs={"slug": self.slug})
```

**Conventions:**
- Always use `UUIDField` as primary key (via `TimeStampedModel`)
- Always include `__str__` and `get_absolute_url`
- Always add `class Meta` with `ordering` and relevant `indexes`
- Use `GeneratedField` for computed columns that need to be queried
- Use `db_default` for database-level defaults when possible
- Name boolean fields with `is_` or `has_` prefix

### Custom Managers and QuerySets

Use custom managers for reusable query patterns:

```python
class ChapterQuerySet(models.QuerySet):
    def active(self):
        return self.filter(is_active=True)

    def with_seat_counts(self):
        return self.annotate(
            total_seats=models.Count("seats"),
            open_seats=models.Count("seats", filter=models.Q(seats__status="open")),
        )


class Chapter(TimeStampedModel):
    # ... fields ...
    objects = ChapterQuerySet.as_manager()
```

### Migrations

Always generate migrations via Django's migration framework:

```bash
python manage.py makemigrations --name describe_the_change
# Review the generated migration
python manage.py migrate
```

Never edit a migration after it has been committed. Write a new corrective migration instead. For data migrations, use `RunPython` with a reverse function:

```python
from django.db import migrations


def populate_slugs(apps, schema_editor):
    Chapter = apps.get_model("chapters", "Chapter")
    for chapter in Chapter.objects.filter(slug=""):
        chapter.slug = slugify(chapter.name)
        chapter.save(update_fields=["slug"])


def reverse_populate_slugs(apps, schema_editor):
    pass  # No-op reverse — data migration


class Migration(migrations.Migration):
    dependencies = [("chapters", "0002_add_slug")]
    operations = [migrations.RunPython(populate_slugs, reverse_populate_slugs)]
```

---

## 4. Authentication & Authorization

### django-allauth Setup

Authentication is handled entirely by django-allauth — no custom auth code:

```python
# config/settings/base.py

INSTALLED_APPS = [
    # ...
    "allauth",
    "allauth.account",
    "allauth.socialaccount",
    "allauth.socialaccount.providers.google",
    "allauth.mfa",
]

MIDDLEWARE = [
    # ...
    "allauth.account.middleware.AccountMiddleware",
]

# allauth configuration
ACCOUNT_AUTHENTICATION_METHOD = "email"
ACCOUNT_EMAIL_REQUIRED = True
ACCOUNT_USERNAME_REQUIRED = False
ACCOUNT_EMAIL_VERIFICATION = "mandatory"
ACCOUNT_LOGIN_ON_EMAIL_CONFIRMATION = True
ACCOUNT_SIGNUP_FORM_CLASS = "apps.accounts.forms.CustomSignupForm"

# Custom user model
AUTH_USER_MODEL = "accounts.User"

LOGIN_REDIRECT_URL = "/dashboard/"
ACCOUNT_LOGOUT_REDIRECT_URL = "/"
```

### Custom User Model

Always define a custom user model from day one — changing it later is painful:

```python
# apps/accounts/models.py
from django.contrib.auth.models import AbstractUser
from django.db import models
from apps.core.models import TimeStampedModel


class User(AbstractUser):
    """Custom user model with role-based access."""

    class Role(models.TextChoices):
        MEMBER = "member", "Member"
        CHAPTER_ADMIN = "chapter_admin", "Chapter Admin"
        PLATFORM_ADMIN = "platform_admin", "Platform Admin"

    id = models.UUIDField(primary_key=True, default=uuid.uuid4, editable=False)
    email = models.EmailField(unique=True)
    role = models.CharField(max_length=20, choices=Role.choices, default=Role.MEMBER)
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    USERNAME_FIELD = "email"
    REQUIRED_FIELDS = ["username"]

    class Meta:
        ordering = ["-created_at"]

    @property
    def is_chapter_admin(self) -> bool:
        return self.role in (self.Role.CHAPTER_ADMIN, self.Role.PLATFORM_ADMIN)

    @property
    def is_platform_admin(self) -> bool:
        return self.role == self.Role.PLATFORM_ADMIN
```

### Role Model

Four roles, enforced through Django permissions and DRF permission classes:

| Role | Value | Access |
|---|---|---|
| `visitor` | (unauthenticated) | Public pages, registration form |
| `member` | `member` | Own profile, roster, claim seats |
| `chapter_admin` | `chapter_admin` | Chapter management, visitor pipeline, user admin |
| `platform_admin` | `platform_admin` | Everything, all chapters |

### DRF Permission Classes

```python
# apps/accounts/permissions.py
from rest_framework.permissions import BasePermission


class IsChapterAdmin(BasePermission):
    """Allows access to chapter admins and platform admins."""

    def has_permission(self, request, view):
        return (
            request.user.is_authenticated
            and request.user.role in ("chapter_admin", "platform_admin")
        )


class IsOwnerOrAdmin(BasePermission):
    """Object-level: owner or admin can modify."""

    def has_object_permission(self, request, view, obj):
        if request.user.role == "platform_admin":
            return True
        return hasattr(obj, "user") and obj.user == request.user
```

### Django View-Level Authorization

For non-API views, use mixins and decorators:

```python
from django.contrib.auth.mixins import LoginRequiredMixin, UserPassesTestMixin


class ChapterAdminView(LoginRequiredMixin, UserPassesTestMixin):
    """Base view requiring chapter admin or higher."""

    def test_func(self):
        return self.request.user.is_chapter_admin


# Or with decorators for function-based views
from django.contrib.auth.decorators import login_required, user_passes_test


@login_required
@user_passes_test(lambda u: u.is_chapter_admin)
def manage_chapter(request, slug):
    ...
```

---

## 5. Component Library Patterns

### Philosophy

Use a component library for all standard UI. Django offers two main approaches: `django-components` (Python-class based) and `django-cotton` (HTML-like syntax). Pick one per project and stay consistent. Only build custom components for domain-specific UI.

### django-components Setup

```python
# config/settings/base.py
INSTALLED_APPS = [
    # ...
    "django_components",
]

TEMPLATES = [
    {
        "BACKEND": "django.template.backends.django.DjangoTemplates",
        "DIRS": [BASE_DIR / "templates"],
        "OPTIONS": {
            "context_processors": [...],
            "loaders": [
                (
                    "django.template.loaders.cached.Loader",
                    [
                        "django_components.template_loader.Loader",
                        "django.template.loaders.filesystem.Loader",
                        "django.template.loaders.app_directories.Loader",
                    ],
                ),
            ],
        },
    },
]
```

### Component Definition (django-components)

```python
# apps/core/components/card/card.py
from django_components import Component, register


@register("card")
class Card(Component):
    template_name = "card/card.html"

    def get_context_data(self, title, variant="default"):
        return {
            "title": title,
            "variant": variant,
            "css_class": f"card card-{variant}",
        }

    class Media:
        css = "card/card.css"
```

```html
<!-- apps/core/components/card/card.html -->
<div class="{{ css_class }}">
  <div class="card-header">
    <h3>{{ title }}</h3>
  </div>
  <div class="card-body">
    {% slot "body" %}{% endslot %}
  </div>
  {% if "footer" in component.slot_names %}
  <div class="card-footer">
    {% slot "footer" %}{% endslot %}
  </div>
  {% endif %}
</div>
```

### Component Usage in Templates

```html
{% load component_tags %}

{% component "card" title="Chapter Details" variant="primary" %}
  {% fill "body" %}
    <p>Meeting every {{ chapter.get_meeting_day_display }} at {{ chapter.meeting_time }}</p>
  {% endfill %}
  {% fill "footer" %}
    <a href="{% url 'chapters:edit' chapter.slug %}" class="btn btn-sm">Edit</a>
  {% endfill %}
{% endcomponent %}
```

### django-cotton Alternative

For teams preferring HTML-like syntax over Python classes:

```html
<!-- templates/cotton/button.html -->
<c-vars type="primary" size="md" />
<button
  class="btn btn-{{ type }} btn-{{ size }}"
  {{ attrs }}
>
  {{ slot }}
</button>
```

```html
<!-- Usage -->
<c-button type="primary" hx-post="/api/submit/">
  Save Changes
</c-button>
```

### HTMX Partial Components

For HTMX-swappable content, use partial templates that render a fragment:

```html
<!-- templates/chapters/partials/_seat_list.html -->
<div id="seat-list">
  {% for seat in seats %}
    {% component "seat_card" seat=seat %}{% endcomponent %}
  {% empty %}
    <p class="text-muted">No seats found.</p>
  {% endfor %}
</div>
```

### Tailwind CSS Integration

Use `django-tailwind` or the standalone Tailwind CLI:

```python
# config/settings/base.py
INSTALLED_APPS = [
    # ...
    "tailwind",
    "theme",  # Tailwind theme app
]

TAILWIND_APP_NAME = "theme"
```

```css
/* static/css/input.css — Tailwind 4.x */
@import "tailwindcss";
@source "../templates";
@source "../apps/**/templates";
@source "../apps/**/components";
```

---

## 6. Testing Patterns

### Test Pyramid (Django-specific)

```
        /\
       /  \          E2E (Playwright) — deferred to later slices
      /    \
     /------\
    /        \        Feature Tests (Django TestClient + HTMX assertions)
   /          \       Full request/response cycle, template rendering, HTMX partials
  /------------\
 /              \      Integration Tests (pytest-django + DB)
/                \     Model operations through DB, API endpoints, Celery tasks
/------------------\
/                    \   Unit Tests (pytest)
/                      \  Pure functions, model methods, serializer validation
/------------------------\
```

### pytest-django Configuration

```toml
# pyproject.toml
[tool.pytest.ini_options]
DJANGO_SETTINGS_MODULE = "config.settings.test"
python_files = ["test_*.py"]
python_classes = ["Test*"]
python_functions = ["test_*"]
addopts = [
    "--strict-markers",
    "--strict-config",
    "-ra",
    "--tb=short",
    "--cov=apps",
    "--cov-report=term-missing",
    "--cov-fail-under=100",
    "--reuse-db",
]
markers = [
    "slow: marks tests as slow (deselect with '-m \"not slow\"')",
    "integration: marks integration tests",
]
```

### factory_boy Factories

```python
# tests/factories.py
import factory
from factory.django import DjangoModelFactory
from apps.accounts.models import User
from apps.chapters.models import Chapter, Seat


class UserFactory(DjangoModelFactory):
    class Meta:
        model = User
        skip_postgeneration_save = True

    email = factory.LazyAttribute(lambda o: f"user{factory.Faker('random_int').evaluate(o, None, {'locale': None})}@example.com")
    username = factory.LazyAttribute(lambda o: o.email.split("@")[0])
    role = User.Role.MEMBER

    @factory.post_generation
    def password(self, create, extracted, **kwargs):
        password = extracted or "testpass123"
        self.set_password(password)
        if create:
            self.save(update_fields=["password"])


class ChapterFactory(DjangoModelFactory):
    class Meta:
        model = Chapter

    name = factory.Sequence(lambda n: f"Chapter {n}")
    slug = factory.LazyAttribute(lambda o: slugify(o.name))
    meeting_day = 2  # Wednesday
    meeting_time = factory.LazyFunction(lambda: time(7, 0))
    location = "Denver, NC"
    is_active = True


class SeatFactory(DjangoModelFactory):
    class Meta:
        model = Seat

    chapter = factory.SubFactory(ChapterFactory)
    classification = "Plumber"
    category = "Home Services"
    status = "open"
```

### Shared Fixtures (conftest.py)

```python
# tests/conftest.py
import pytest
from tests.factories import UserFactory, ChapterFactory


@pytest.fixture
def user(db):
    return UserFactory()


@pytest.fixture
def admin_user(db):
    return UserFactory(role="platform_admin")


@pytest.fixture
def chapter(db):
    return ChapterFactory()


@pytest.fixture
def authenticated_client(client, user):
    client.force_login(user)
    return client


@pytest.fixture
def admin_client(client, admin_user):
    client.force_login(admin_user)
    return client


@pytest.fixture
def api_client():
    from rest_framework.test import APIClient
    return APIClient()


@pytest.fixture
def authenticated_api_client(api_client, user):
    api_client.force_authenticate(user=user)
    return api_client
```

### View Testing with HTMX

```python
# tests/chapters/test_views.py
import pytest
from django.urls import reverse


class TestChapterListView:
    def test_full_page_load(self, authenticated_client, chapter):
        url = reverse("chapters:list")
        response = authenticated_client.get(url)
        assert response.status_code == 200
        assert "base.html" in [t.name for t in response.templates]

    def test_htmx_partial_response(self, authenticated_client, chapter):
        """HTMX requests get only the partial, not the full page."""
        url = reverse("chapters:list")
        response = authenticated_client.get(
            url,
            HTTP_HX_REQUEST="true",
            HTTP_HX_TARGET="chapter-list",
        )
        assert response.status_code == 200
        # Partial template, not full page
        template_names = [t.name for t in response.templates]
        assert "chapters/partials/_chapter_list.html" in template_names
        assert "base.html" not in template_names


class TestSeatClaimView:
    def test_htmx_swap_response(self, authenticated_client, seat):
        url = reverse("chapters:claim-seat", kwargs={"pk": seat.pk})
        response = authenticated_client.post(
            url,
            HTTP_HX_REQUEST="true",
        )
        assert response.status_code == 200
        assert "HX-Trigger" in response.headers
```

### DRF Serializer Testing

```python
# tests/chapters/test_serializers.py
import pytest
from apps.chapters.serializers import ChapterSerializer


class TestChapterSerializer:
    def test_valid_data(self, chapter):
        serializer = ChapterSerializer(chapter)
        data = serializer.data
        assert data["name"] == chapter.name
        assert data["slug"] == chapter.slug

    def test_validation_rejects_duplicate_slug(self, chapter):
        data = {"name": "New Chapter", "slug": chapter.slug}
        serializer = ChapterSerializer(data=data)
        assert not serializer.is_valid()
        assert "slug" in serializer.errors
```

### Celery Task Testing

```python
# tests/chapters/test_tasks.py
import pytest
from unittest.mock import patch
from apps.chapters.tasks import sync_chapter_to_crm


class TestSyncChapterTask:
    @patch("apps.chapters.tasks.crm_client.create_record")
    def test_creates_crm_record(self, mock_create, chapter):
        mock_create.return_value = {"id": "crm_123"}
        result = sync_chapter_to_crm(str(chapter.id))
        assert result == {"id": "crm_123"}
        mock_create.assert_called_once()

    @patch("apps.chapters.tasks.crm_client.create_record")
    def test_retries_on_connection_error(self, mock_create, chapter):
        mock_create.side_effect = ConnectionError("timeout")
        with pytest.raises(ConnectionError):
            sync_chapter_to_crm(str(chapter.id))
```

### Test Configuration

```python
# config/settings/test.py
from .base import *

# Use in-memory SQLite for speed (or keep Postgres for full compatibility)
# DATABASES = {"default": {"ENGINE": "django.db.backends.sqlite3", "NAME": ":memory:"}}

# Faster password hashing in tests
PASSWORD_HASHERS = ["django.contrib.auth.hashers.MD5PasswordHasher"]

# Celery: run tasks synchronously
CELERY_TASK_ALWAYS_EAGER = True
CELERY_TASK_EAGER_PROPAGATES = True

# Email: capture instead of send
EMAIL_BACKEND = "django.core.mail.backends.locmem.EmailBackend"

# Disable throttling in tests
REST_FRAMEWORK = {
    **REST_FRAMEWORK,
    "DEFAULT_THROTTLE_CLASSES": [],
    "DEFAULT_THROTTLE_RATES": {},
}

# Static files: skip manifest in tests
STATICFILES_STORAGE = "django.contrib.staticfiles.storage.StaticFilesStorage"
```

---

## 7. View Patterns — HTMX + Alpine.js

### HTMX Middleware Setup

```python
# config/settings/base.py
INSTALLED_APPS = [
    # ...
    "django_htmx",
]

MIDDLEWARE = [
    # ...
    "django_htmx.middleware.HtmxMiddleware",
]
```

### View Structure — Dual Response Pattern

Every view that serves HTMX must handle both full-page and partial requests:

```python
# apps/chapters/views.py
from django.shortcuts import render, get_object_or_404
from django.views import View


class ChapterListView(View):
    def get(self, request):
        chapters = Chapter.objects.active().with_seat_counts()

        if request.htmx:
            return render(request, "chapters/partials/_chapter_list.html", {
                "chapters": chapters,
            })

        return render(request, "chapters/chapter_list.html", {
            "chapters": chapters,
        })
```

### HTMX Template Patterns

Base template with HTMX loaded:

```html
<!-- templates/base.html -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{% block title %}{% endblock %} | App</title>
    <link rel="stylesheet" href="{% static 'css/output.css' %}">
    <script src="https://unpkg.com/htmx.org@2.0.4"></script>
    <script defer src="https://unpkg.com/alpinejs@3.14.8/dist/cdn.min.js"></script>
</head>
<body hx-headers='{"X-CSRFToken": "{{ csrf_token }}"}'>
    {% block content %}{% endblock %}
</body>
</html>
```

### HTMX Interaction Patterns

**Search with debounce:**

```html
<input
  type="search"
  name="q"
  hx-get="{% url 'chapters:list' %}"
  hx-trigger="input changed delay:300ms, search"
  hx-target="#chapter-list"
  hx-swap="innerHTML"
  hx-indicator="#search-spinner"
  placeholder="Search chapters..."
>
<span id="search-spinner" class="htmx-indicator">Searching...</span>
<div id="chapter-list">
  {% include "chapters/partials/_chapter_list.html" %}
</div>
```

**Inline editing:**

```html
<!-- Display mode -->
<div id="seat-{{ seat.pk }}" hx-target="this" hx-swap="outerHTML">
  <span>{{ seat.classification }}</span>
  <button hx-get="{% url 'chapters:seat-edit' seat.pk %}" class="btn btn-sm">
    Edit
  </button>
</div>

<!-- Edit mode (returned by the edit view) -->
<form id="seat-{{ seat.pk }}"
      hx-put="{% url 'chapters:seat-update' seat.pk %}"
      hx-target="this"
      hx-swap="outerHTML">
  {% csrf_token %}
  <input name="classification" value="{{ seat.classification }}">
  <button type="submit" class="btn btn-primary btn-sm">Save</button>
  <button hx-get="{% url 'chapters:seat-detail' seat.pk %}"
          class="btn btn-sm">Cancel</button>
</form>
```

**Delete with confirmation:**

```html
<button
  hx-delete="{% url 'chapters:seat-delete' seat.pk %}"
  hx-confirm="Delete this seat? This cannot be undone."
  hx-target="#seat-{{ seat.pk }}"
  hx-swap="outerHTML swap:500ms"
  class="btn btn-danger btn-sm"
>
  Delete
</button>
```

**Infinite scroll:**

```html
<div id="seat-list">
  {% for seat in page_obj %}
    {% include "chapters/partials/_seat_card.html" %}
  {% endfor %}

  {% if page_obj.has_next %}
    <div hx-get="{% url 'chapters:seat-list' %}?page={{ page_obj.next_page_number }}"
         hx-trigger="revealed"
         hx-swap="afterend"
         hx-target="this">
      <span class="htmx-indicator">Loading more...</span>
    </div>
  {% endif %}
</div>
```

### HTMX Response Headers from Views

```python
from django.http import HttpResponse


def claim_seat(request, pk):
    seat = get_object_or_404(Seat, pk=pk)
    seat.claim(request.user)

    response = render(request, "chapters/partials/_seat_card.html", {"seat": seat})
    # Trigger client-side events for Alpine.js or other listeners
    response["HX-Trigger"] = "seatClaimed"
    # Optionally trigger a toast notification
    response["HX-Trigger-After-Swap"] = json.dumps({
        "showToast": {"message": "Seat claimed!", "type": "success"}
    })
    return response
```

### Alpine.js Patterns

**Dropdown with Alpine.js:**

```html
<div x-data="{ open: false }" class="relative">
  <button @click="open = !open" @click.outside="open = false"
          class="btn btn-secondary">
    Actions
    <svg x-bind:class="open && 'rotate-180'" class="ml-1 h-4 w-4 transition-transform">
      <!-- chevron icon -->
    </svg>
  </button>

  <div x-show="open" x-transition
       class="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg z-10">
    <a href="#" class="block px-4 py-2 hover:bg-gray-100">Edit</a>
    <a href="#" class="block px-4 py-2 hover:bg-gray-100">Delete</a>
  </div>
</div>
```

**Alpine.js + HTMX coordination:**

```html
<div x-data="{ count: 0 }"
     @seat-claimed.window="count++"
     @htmx:after-swap.window="$dispatch('refresh-stats')">

  <span x-text="`${count} seats claimed this session`"></span>

  <div hx-get="{% url 'chapters:seat-list' %}"
       hx-trigger="load, seatClaimed from:body"
       hx-target="#seat-list">
    <div id="seat-list">{% include "chapters/partials/_seat_list.html" %}</div>
  </div>
</div>
```

**Tabs with Alpine.js (no server round-trip):**

```html
<div x-data="{ tab: 'overview' }">
  <nav class="flex gap-2 border-b">
    <button @click="tab = 'overview'"
            :class="tab === 'overview' ? 'border-b-2 border-blue-500 font-semibold' : ''"
            class="px-4 py-2">Overview</button>
    <button @click="tab = 'roster'"
            :class="tab === 'roster' ? 'border-b-2 border-blue-500 font-semibold' : ''"
            class="px-4 py-2">Roster</button>
    <button @click="tab = 'visitors'"
            :class="tab === 'visitors' ? 'border-b-2 border-blue-500 font-semibold' : ''"
            class="px-4 py-2">Visitors</button>
  </nav>

  <div x-show="tab === 'overview'" x-cloak>{% include "chapters/partials/_overview.html" %}</div>
  <div x-show="tab === 'roster'" x-cloak>{% include "chapters/partials/_roster.html" %}</div>
  <div x-show="tab === 'visitors'" x-cloak>{% include "chapters/partials/_visitors.html" %}</div>
</div>
```

### Async Views (Django 5.x)

For I/O-bound operations, use async views to avoid blocking the thread pool:

```python
import httpx
from django.http import JsonResponse


async def external_api_proxy(request):
    """Async view for proxying external API calls without blocking workers."""
    async with httpx.AsyncClient() as client:
        response = await client.get("https://api.example.com/data")
        return JsonResponse(response.json())
```

Requires ASGI deployment (Uvicorn/Daphne). The view is callable from both WSGI and ASGI, but only truly async under ASGI.

---

## 8. Django REST Framework Patterns

### DRF Configuration

```python
# config/settings/base.py
REST_FRAMEWORK = {
    "DEFAULT_AUTHENTICATION_CLASSES": [
        "rest_framework.authentication.SessionAuthentication",
        "rest_framework.authentication.TokenAuthentication",
    ],
    "DEFAULT_PERMISSION_CLASSES": [
        "rest_framework.permissions.IsAuthenticated",
    ],
    "DEFAULT_PAGINATION_CLASS": "rest_framework.pagination.PageNumberPagination",
    "PAGE_SIZE": 25,
    "DEFAULT_THROTTLE_CLASSES": [
        "rest_framework.throttling.AnonRateThrottle",
        "rest_framework.throttling.UserRateThrottle",
    ],
    "DEFAULT_THROTTLE_RATES": {
        "anon": "100/hour",
        "user": "1000/hour",
    },
    "DEFAULT_RENDERER_CLASSES": [
        "rest_framework.renderers.JSONRenderer",
    ],
    "DEFAULT_VERSIONING_CLASS": "rest_framework.versioning.URLPathVersioning",
    "DEFAULT_VERSION": "v1",
    "ALLOWED_VERSIONS": ["v1"],
    "TEST_REQUEST_DEFAULT_FORMAT": "json",
}
```

### Serializer Patterns

```python
# apps/chapters/serializers.py
from rest_framework import serializers
from apps.chapters.models import Chapter, Seat


class SeatSerializer(serializers.ModelSerializer):
    class Meta:
        model = Seat
        fields = ["id", "classification", "category", "status", "chapter"]
        read_only_fields = ["id"]


class ChapterListSerializer(serializers.ModelSerializer):
    """Lightweight serializer for list endpoints."""
    open_seat_count = serializers.IntegerField(read_only=True)

    class Meta:
        model = Chapter
        fields = ["id", "name", "slug", "meeting_day", "location", "open_seat_count"]


class ChapterDetailSerializer(serializers.ModelSerializer):
    """Full serializer for detail endpoints."""
    seats = SeatSerializer(many=True, read_only=True)

    class Meta:
        model = Chapter
        fields = [
            "id", "name", "slug", "meeting_day", "meeting_time",
            "location", "is_active", "seats", "created_at",
        ]
        read_only_fields = ["id", "created_at"]
```

**Conventions:**
- Separate list and detail serializers — list endpoints should be lightweight
- Always specify `fields` explicitly — never use `fields = "__all__"`
- Always specify `read_only_fields`
- Use nested serializers for detail views, IDs for list views

### ViewSet Patterns

```python
# apps/chapters/views_api.py
from rest_framework import viewsets, status
from rest_framework.decorators import action
from rest_framework.response import Response
from apps.accounts.permissions import IsChapterAdmin
from apps.chapters.models import Chapter
from apps.chapters.serializers import ChapterListSerializer, ChapterDetailSerializer


class ChapterViewSet(viewsets.ModelViewSet):
    queryset = Chapter.objects.active()
    lookup_field = "slug"

    def get_serializer_class(self):
        if self.action == "list":
            return ChapterListSerializer
        return ChapterDetailSerializer

    def get_permissions(self):
        if self.action in ("list", "retrieve"):
            return []  # Public read
        return [IsChapterAdmin()]

    def get_queryset(self):
        qs = super().get_queryset()
        if self.action == "list":
            qs = qs.with_seat_counts()
        else:
            qs = qs.prefetch_related("seats")
        return qs

    @action(detail=True, methods=["post"], permission_classes=[IsChapterAdmin])
    def activate(self, request, slug=None):
        chapter = self.get_object()
        chapter.is_active = True
        chapter.save(update_fields=["is_active"])
        return Response({"status": "activated"})
```

### Router Configuration

```python
# apps/api/urls.py
from rest_framework.routers import DefaultRouter
from apps.chapters.views_api import ChapterViewSet
from apps.accounts.views_api import UserViewSet

router = DefaultRouter()
router.register("chapters", ChapterViewSet, basename="chapter")
router.register("users", UserViewSet, basename="user")

# config/urls.py
urlpatterns = [
    path("api/v1/", include(("apps.api.urls", "api"), namespace="api-v1")),
    # ...
]
```

### Django Ninja Alternative

For projects preferring type hints and async-first design over DRF:

```python
# apps/api/endpoints.py
from ninja import NinjaAPI, Schema
from ninja.security import django_auth


api = NinjaAPI(version="1.0.0", auth=django_auth)


class ChapterOut(Schema):
    id: str
    name: str
    slug: str
    meeting_day: int
    location: str


@api.get("/chapters", response=list[ChapterOut])
def list_chapters(request, active: bool = True):
    return Chapter.objects.filter(is_active=active)


@api.get("/chapters/{slug}", response=ChapterOut)
def get_chapter(request, slug: str):
    return get_object_or_404(Chapter, slug=slug)


# Async endpoint
@api.get("/chapters/{slug}/external-data")
async def get_external_data(request, slug: str):
    async with httpx.AsyncClient() as client:
        response = await client.get(f"https://api.example.com/chapters/{slug}")
        return response.json()
```

**When to choose Django Ninja over DRF:**
- Greenfield API-only projects
- Async-heavy workloads
- Teams that prefer type hints over serializer classes
- Auto-generated OpenAPI docs are a priority

**When to stick with DRF:**
- Existing DRF codebase
- Need for browsable API during development
- Complex nested serialization
- Large ecosystem of DRF extensions (filters, permissions, etc.)

---

## 9. Background Tasks & Real-Time

### Celery Configuration

```python
# config/celery.py
import os
from celery import Celery

os.environ.setdefault("DJANGO_SETTINGS_MODULE", "config.settings.production")

app = Celery("project_name")
app.config_from_object("django.conf:settings", namespace="CELERY")
app.autodiscover_tasks()
```

```python
# config/settings/base.py
CELERY_BROKER_URL = os.environ.get("REDIS_URL", "redis://localhost:6379/0")
CELERY_RESULT_BACKEND = CELERY_BROKER_URL
CELERY_ACCEPT_CONTENT = ["json"]
CELERY_TASK_SERIALIZER = "json"
CELERY_RESULT_SERIALIZER = "json"
CELERY_TASK_TRACK_STARTED = True
CELERY_TASK_TIME_LIMIT = 300  # 5 minutes hard limit
CELERY_TASK_SOFT_TIME_LIMIT = 240  # 4 minutes soft limit
CELERY_BROKER_CONNECTION_RETRY_ON_STARTUP = True
```

### Task Patterns

```python
# apps/chapters/tasks.py
from celery import shared_task
from celery.utils.log import get_task_logger

logger = get_task_logger(__name__)


@shared_task(
    bind=True,
    max_retries=3,
    default_retry_delay=60,
    autoretry_for=(ConnectionError, TimeoutError),
    retry_backoff=True,
)
def sync_chapter_to_crm(self, chapter_id: str) -> dict:
    """Sync a chapter record to the external CRM."""
    from apps.chapters.models import Chapter

    chapter = Chapter.objects.get(pk=chapter_id)
    logger.info("Syncing chapter %s to CRM", chapter.name)

    result = crm_client.create_or_update(
        external_id=str(chapter.id),
        data={"name": chapter.name, "location": chapter.location},
    )

    chapter.crm_id = result["id"]
    chapter.save(update_fields=["crm_id"])
    return result
```

**Conventions:**
- Always use `shared_task` (not `app.task`) for reusability
- Always pass serializable arguments (IDs, not model instances)
- Always set `max_retries` and `time_limit`
- Always use `autoretry_for` for transient network errors
- Log at INFO level for task start, WARNING for retries, ERROR for final failure

### Dramatiq Alternative

For simpler task queues without Celery's complexity:

```python
# apps/chapters/tasks.py
import dramatiq
from dramatiq.brokers.redis import RedisBroker

broker = RedisBroker(url="redis://localhost:6379/0")
dramatiq.set_broker(broker)


@dramatiq.actor(max_retries=3, min_backoff=1000, max_backoff=60000)
def sync_chapter_to_crm(chapter_id: str):
    from apps.chapters.models import Chapter
    chapter = Chapter.objects.get(pk=chapter_id)
    # ... sync logic
```

### Django Channels for WebSocket

```python
# config/asgi.py
import os
from channels.auth import AuthMiddlewareStack
from channels.routing import ProtocolTypeRouter, URLRouter
from django.core.asgi import get_asgi_application
import apps.notifications.routing

os.environ.setdefault("DJANGO_SETTINGS_MODULE", "config.settings.production")

application = ProtocolTypeRouter({
    "http": get_asgi_application(),
    "websocket": AuthMiddlewareStack(
        URLRouter(apps.notifications.routing.websocket_urlpatterns)
    ),
})
```

```python
# apps/notifications/consumers.py
import json
from channels.generic.websocket import AsyncJsonWebSocketConsumer


class NotificationConsumer(AsyncJsonWebSocketConsumer):
    async def connect(self):
        self.user = self.scope["user"]
        if not self.user.is_authenticated:
            await self.close()
            return

        self.group_name = f"user_{self.user.pk}"
        await self.channel_layer.group_add(self.group_name, self.channel_name)
        await self.accept()

    async def disconnect(self, close_code):
        await self.channel_layer.group_discard(self.group_name, self.channel_name)

    async def notification_message(self, event):
        """Handle notification.message type events."""
        await self.send_json(event["data"])
```

```python
# apps/notifications/routing.py
from django.urls import re_path
from . import consumers

websocket_urlpatterns = [
    re_path(r"ws/notifications/$", consumers.NotificationConsumer.as_asgi()),
]
```

---

## 10. Seed Data & Fixtures

### Fixture Files

Django fixtures live in `fixtures/` and are loaded via `loaddata`:

```json
// fixtures/initial_data.json
[
  {
    "model": "chapters.chapter",
    "pk": "550e8400-e29b-41d4-a716-446655440001",
    "fields": {
      "name": "Westlake Select",
      "slug": "westlake-select",
      "meeting_day": 2,
      "meeting_time": "07:00:00",
      "location": "Denver, NC",
      "is_active": true
    }
  }
]
```

### Management Command for Complex Seeds

For seed data that requires logic (creating relationships, conditional inserts), use a management command:

```python
# apps/core/management/commands/seed.py
from django.core.management.base import BaseCommand
from apps.chapters.models import Chapter, Seat
from apps.accounts.models import User


class Command(BaseCommand):
    help = "Seed database with initial data. Idempotent — safe to run multiple times."

    def handle(self, *args, **options):
        self.stdout.write("Seeding database...")

        # 1. Create platform admin
        admin, created = User.objects.get_or_create(
            email="admin@example.com",
            defaults={
                "username": "admin",
                "role": User.Role.PLATFORM_ADMIN,
                "is_staff": True,
                "is_superuser": True,
            },
        )
        if created:
            admin.set_password("change-me-in-production")
            admin.save()
            self.stdout.write(self.style.SUCCESS("  Created admin user"))

        # 2. Create chapter with seats
        chapter, _ = Chapter.objects.get_or_create(
            slug="westlake-select",
            defaults={
                "name": "Westlake Select",
                "meeting_day": 2,
                "meeting_time": "07:00:00",
                "location": "Denver, NC",
            },
        )

        # 3. Seed seats (idempotent via get_or_create)
        seat_data = [
            {"classification": "Plumber", "category": "Home Services"},
            {"classification": "Electrician", "category": "Home Services"},
            {"classification": "Realtor", "category": "Real Estate"},
            # ... full roster
        ]
        for seat in seat_data:
            Seat.objects.get_or_create(
                chapter=chapter,
                classification=seat["classification"],
                defaults={"category": seat["category"], "status": "open"},
            )

        self.stdout.write(self.style.SUCCESS("Seed complete."))
```

### Factory-Based Seeds for Development

```python
# For local development with realistic volume
# python manage.py shell -c "from tests.factories import *; [ChapterFactory() for _ in range(10)]"
```

Seed data is idempotent (safe to run multiple times). Use `get_or_create` for all seed operations.

---

## 11. Development Workflow

### Feature Development Cycle (Django-specific)

```
1. Write BDD scenarios (docs/scenarios/*.md)
2. Design test levels (unit / integration / feature)
3. Write failing tests (pytest)
4. Write model / migration code
5. Write view + serializer code
6. Write templates (HTMX partials, Alpine.js interactions)
7. Run: pytest
8. Run: ruff check . && ruff format .
9. Refactor while green
10. Run: mypy apps/
```

### Common Commands

```bash
# Development
python manage.py runserver                          # Start dev server
python manage.py runserver 0.0.0.0:8000             # Accessible on LAN
uvicorn config.asgi:application --reload            # ASGI dev server (for async views)
python manage.py tailwind start                     # Watch Tailwind CSS

# Testing
pytest                                              # Run all tests
pytest tests/chapters/                              # Run specific app tests
pytest -x                                           # Stop on first failure
pytest -k "test_chapter_list"                       # Run matching tests
pytest --cov --cov-report=html                      # Coverage with HTML report
pytest -m "not slow"                                # Skip slow tests

# Database
python manage.py makemigrations --name description  # Generate migrations
python manage.py migrate                            # Apply migrations
python manage.py showmigrations                     # Migration status
python manage.py seed                               # Load seed data
python manage.py flush --no-input                   # Reset DB (dev only)

# Quality
ruff check .                                        # Lint (replaces flake8)
ruff format .                                       # Format (replaces black)
ruff check . --fix                                  # Auto-fix lint issues
mypy apps/                                          # Type check

# Shell & Debug
python manage.py shell_plus                         # Enhanced shell (django-extensions)
python manage.py dbshell                            # Database shell

# Celery
celery -A config worker -l info                     # Start worker
celery -A config beat -l info                       # Start scheduler
celery -A config flower                             # Monitoring UI
```

### Makefile / pyproject.toml Scripts

```toml
# pyproject.toml — tool configuration
[tool.ruff]
target-version = "py312"
line-length = 99
src = ["apps", "config", "tests"]

[tool.ruff.lint]
select = ["E", "F", "I", "N", "UP", "B", "A", "C4", "SIM", "TCH"]
ignore = ["E501"]  # Line length handled by formatter

[tool.ruff.lint.isort]
known-first-party = ["apps", "config"]

[tool.mypy]
python_version = "3.12"
plugins = ["mypy_django_plugin.main", "mypy_drf_plugin.main"]
strict = true
warn_unreachable = true
ignore_missing_imports = true

[tool.django-stubs]
django_settings_module = "config.settings.development"
```

```makefile
# Makefile
.PHONY: test lint format quality ci seed

test:
	pytest

lint:
	ruff check .
	mypy apps/

format:
	ruff format .
	ruff check . --fix

quality: lint
	ruff format --check .

ci: quality test

seed:
	python manage.py seed

dev:
	python manage.py runserver & python manage.py tailwind start
```

---

## 12. Deployment

### Gunicorn + Uvicorn (ASGI)

Production deployment uses Gunicorn with Uvicorn workers for ASGI support:

```python
# gunicorn.conf.py
import multiprocessing

bind = "0.0.0.0:8000"
workers = multiprocessing.cpu_count() * 2 + 1
worker_class = "uvicorn.workers.UvicornWorker"  # ASGI support
timeout = 120
keepalive = 5
max_requests = 1000
max_requests_jitter = 50
accesslog = "-"
errorlog = "-"
```

```bash
# Start command
gunicorn config.asgi:application -c gunicorn.conf.py
```

### WhiteNoise for Static Files

```python
# config/settings/base.py
MIDDLEWARE = [
    "django.middleware.security.SecurityMiddleware",
    "whitenoise.middleware.WhiteNoiseMiddleware",  # After SecurityMiddleware
    # ...
]

STORAGES = {
    "staticfiles": {
        "BACKEND": "whitenoise.storage.CompressedManifestStaticFilesStorage",
    },
}
```

WhiteNoise serves static files efficiently without a CDN. For high-traffic sites, put Cloudflare or another CDN in front.

### Docker Deployment

```dockerfile
# Dockerfile
FROM python:3.12-slim AS base

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1 \
    PIP_NO_CACHE_DIR=1

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev gcc && \
    rm -rf /var/lib/apt/lists/*

# Install Python dependencies
COPY pyproject.toml .
RUN pip install --no-cache-dir .

# Copy application code
COPY . .

# Collect static files
RUN python manage.py collectstatic --noinput

# Run migrations and start server
EXPOSE 8000
CMD ["sh", "-c", "python manage.py migrate --noinput && gunicorn config.asgi:application -c gunicorn.conf.py"]
```

### Platform-Specific Deployment

**Fly.io:**

```toml
# fly.toml
app = "my-django-app"
primary_region = "iad"

[deploy]
  release_command = "python manage.py migrate --noinput"

[http_service]
  internal_port = 8000
  force_https = true
  min_machines_running = 1

[env]
  DJANGO_SETTINGS_MODULE = "config.settings.production"

[[vm]]
  size = "shared-cpu-1x"
  memory = "512mb"
```

**Railway / Render / DigitalOcean App Platform:**

All use similar patterns: Dockerfile or buildpack, environment variables for secrets, managed Postgres addon, release command for migrations.

### CI/CD Pipeline (GitHub Actions)

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_DB: test_db
          POSTGRES_USER: test_user
          POSTGRES_PASSWORD: test_pass
        ports: ["5432:5432"]
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
      redis:
        image: redis:7
        ports: ["6379:6379"]

    env:
      DJANGO_SETTINGS_MODULE: config.settings.test
      DATABASE_URL: postgres://test_user:test_pass@localhost:5432/test_db

    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - run: pip install -e ".[test]"
      - run: ruff check .
      - run: ruff format --check .
      - run: mypy apps/
      - run: pytest --cov --cov-fail-under=100

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: superfly/flyctl-actions/setup-flyctl@master
      - run: flyctl deploy --remote-only
        env:
          FLY_API_TOKEN: ${{ secrets.FLY_API_TOKEN }}
```

---

## 13. Security Headers

Django has built-in security middleware. Configure it in settings and add custom headers:

```python
# config/settings/production.py

# --- Django built-in security ---
SECURE_SSL_REDIRECT = True
SECURE_HSTS_SECONDS = 31536000  # 1 year
SECURE_HSTS_INCLUDE_SUBDOMAINS = True
SECURE_HSTS_PRELOAD = True
SECURE_CONTENT_TYPE_NOSNIFF = True  # X-Content-Type-Options: nosniff
SECURE_BROWSER_XSS_FILTER = False   # Deprecated — use CSP instead
SESSION_COOKIE_SECURE = True
CSRF_COOKIE_SECURE = True
SESSION_COOKIE_HTTPONLY = True
CSRF_COOKIE_HTTPONLY = True

# X-Frame-Options (clickjacking protection)
X_FRAME_OPTIONS = "DENY"

# --- Custom CSP middleware ---
# Use django-csp for Content-Security-Policy
CSP_DEFAULT_SRC = ("'self'",)
CSP_SCRIPT_SRC = ("'self'", "'unsafe-inline'", "https://unpkg.com")
CSP_STYLE_SRC = ("'self'", "'unsafe-inline'")
CSP_IMG_SRC = ("'self'", "data:")
CSP_FONT_SRC = ("'self'",)
CSP_CONNECT_SRC = ("'self'", "wss:")
CSP_FRAME_ANCESTORS = ("'self'",)
```

### Django Built-in Security Features

| Feature | Protection | Status |
|---|---|---|
| CSRF middleware | Cross-Site Request Forgery | Always on (CsrfViewMiddleware) |
| XSS auto-escaping | Cross-Site Scripting | Template engine auto-escapes by default |
| SQL injection | SQL Injection | ORM parameterizes all queries |
| Clickjacking | Clickjacking | X-Frame-Options middleware |
| Host header validation | Host header attacks | ALLOWED_HOSTS setting |
| Session security | Session hijacking | Secure, HttpOnly cookies |
| Password hashing | Credential theft | PBKDF2 (default), Argon2 (recommended) |

**Password Hasher Upgrade:**

```python
# config/settings/base.py
PASSWORD_HASHERS = [
    "django.contrib.auth.hashers.Argon2PasswordHasher",  # Preferred
    "django.contrib.auth.hashers.PBKDF2PasswordHasher",  # Fallback for existing hashes
    "django.contrib.auth.hashers.PBKDF2SHA1PasswordHasher",
]
```

Requires `pip install argon2-cffi`. Existing passwords auto-upgrade to Argon2 on next login.

**HTMX CSRF handling:**

HTMX requires the CSRF token on every POST/PUT/DELETE. Set it globally via `hx-headers` on the `<body>` tag:

```html
<body hx-headers='{"X-CSRFToken": "{{ csrf_token }}"}'>
```

Or per-request with `hx-vals`:

```html
<button hx-post="/api/action/" hx-vals='{"csrfmiddlewaretoken": "{{ csrf_token }}"}'>
  Do Thing
</button>
```

---

## 14. Coverage Enforcement

Test coverage is enforced via `coverage` + `pytest-cov`:

```toml
# pyproject.toml
[tool.coverage.run]
source = ["apps"]
branch = true
omit = [
    "*/migrations/*",
    "*/tests/*",
    "*/management/commands/*",
    "*/admin.py",
    "config/*",
    "manage.py",
]

[tool.coverage.report]
fail_under = 100
show_missing = true
skip_covered = true
precision = 2
exclude_lines = [
    "pragma: no cover",
    "def __repr__",
    "if TYPE_CHECKING:",
    "if settings.DEBUG:",
    "raise NotImplementedError",
    "pass",
    "\\.\\.\\.",
]
```

**Commands:**

```bash
pytest --cov --cov-report=term-missing         # Coverage report to terminal
pytest --cov --cov-report=html                  # HTML report in htmlcov/
pytest --cov --cov-fail-under=100               # Fail below threshold
```

Target is 100% (per CLAUDE.md core rules). The `fail_under` in pyproject.toml is the hard gate — CI fails below this threshold. Verify with `--cov-report=term-missing`, not by assertion.

---

## 15. Form Compliance

All forms must pass the 9-dimension audit from `FORM_PATTERNS.md`:

| Dimension | Key Requirements |
|-----------|-----------------|
| **layout** | Single column, logical grouping with `<fieldset>` + `<legend>` |
| **labels** | Top-aligned, visible `<label>`, optional fields marked "(optional)" |
| **validation** | Submit-only for short forms (<7 fields), reward-early-punish-late otherwise |
| **errors** | Inline + error summary, multi-cue (icon + text + border), focus management |
| **accessibility** | `novalidate` on form, `autocomplete` attributes, `aria-live` on error summary |
| **mobile** | `type="tel"` / `type="email"`, min 48px touch targets, `autocomplete` |
| **cta** | Outcome-focused text ("Reserve My Free Visit" not "Submit"), loading state |
| **trust** | Minimal fields, "(optional)" markers, post-submit clarity |
| **performance** | HTMX validation on blur for long forms, debounce search inputs |

### Django Form Pattern

```python
# apps/visitors/forms.py
from django import forms


class VisitorRegistrationForm(forms.ModelForm):
    class Meta:
        model = Visitor
        fields = ["name", "email", "phone"]
        widgets = {
            "name": forms.TextInput(attrs={
                "autocomplete": "name",
                "placeholder": "Full Name",
            }),
            "email": forms.EmailInput(attrs={
                "autocomplete": "email",
                "placeholder": "you@example.com",
            }),
            "phone": forms.TextInput(attrs={
                "type": "tel",
                "autocomplete": "tel",
                "placeholder": "(555) 123-4567",
            }),
        }
```

### Django Template Form Pattern

```html
<form method="post"
      action="{% url 'visitors:register' %}"
      novalidate
      hx-post="{% url 'visitors:register' %}"
      hx-target="#registration-form"
      hx-swap="outerHTML">
  {% csrf_token %}

  {% if form.errors %}
  <div role="alert" aria-live="assertive" class="bg-red-50 border border-red-200 rounded p-4 mb-4">
    <h3 class="text-red-800 font-semibold">Please fix the following errors:</h3>
    <ul class="list-disc pl-5 text-red-700">
      {% for field, errors in form.errors.items %}
        {% for error in errors %}
          <li>{{ error }}</li>
        {% endfor %}
      {% endfor %}
    </ul>
  </div>
  {% endif %}

  <fieldset>
    <legend class="text-sm font-semibold mb-4">Your Information</legend>

    {% for field in form %}
    <div class="mb-4">
      <label for="{{ field.id_for_label }}" class="block text-sm font-medium mb-1">
        {{ field.label }}
        {% if not field.field.required %}<span class="text-gray-500">(optional)</span>{% endif %}
      </label>
      {{ field }}
      {% if field.errors %}
      <p class="text-red-600 text-sm mt-1" role="alert">
        <span aria-hidden="true">&#9888;</span> {{ field.errors.0 }}
      </p>
      {% endif %}
    </div>
    {% endfor %}
  </fieldset>

  <button type="submit"
          class="btn btn-primary w-full h-12"
          hx-disabled-elt="this"
          hx-indicator="#submit-spinner">
    Reserve My Free Visit
    <span id="submit-spinner" class="htmx-indicator ml-2">
      <svg class="animate-spin h-4 w-4 inline" viewBox="0 0 24 24">...</svg>
    </span>
  </button>
</form>
```

### HTMX Field-Level Validation (Long Forms)

For forms with 7+ fields, validate on blur:

```html
<input
  type="email"
  name="email"
  id="id_email"
  autocomplete="email"
  hx-post="{% url 'visitors:validate-field' %}"
  hx-trigger="blur"
  hx-target="#email-errors"
  hx-swap="innerHTML"
  hx-include="[name='email']"
>
<div id="email-errors"></div>
```

```python
# apps/visitors/views.py
def validate_field(request):
    """HTMX endpoint for field-level validation."""
    form = VisitorRegistrationForm(request.POST)
    form.is_valid()  # Run validation

    field_name = list(request.POST.keys() - {"csrfmiddlewaretoken"})[0]
    errors = form.errors.get(field_name, [])

    if errors:
        return HttpResponse(
            f'<p class="text-red-600 text-sm" role="alert">'
            f'<span aria-hidden="true">&#9888;</span> {errors[0]}</p>'
        )
    return HttpResponse(
        '<p class="text-green-600 text-sm">&#10003;</p>'
    )
```

---

## 16. Anti-Patterns (Django-specific)

| Anti-Pattern | Do This Instead |
|---|---|
| Using the default `User` model | Always define a custom `User` model from day one |
| `fields = "__all__"` in serializers | Always list fields explicitly |
| Raw SQL queries | Use the ORM — it parameterizes queries and prevents SQL injection |
| `objects.all()` in views without pagination | Always paginate querysets |
| N+1 queries (accessing related objects in loops) | Use `select_related()` and `prefetch_related()` |
| Fat views with business logic | Move logic to model methods, managers, or service functions |
| Importing models across app boundaries | Use string references (`"accounts.User"`) and service functions |
| Custom authentication code | Use django-allauth — it handles everything |
| Testing with `Model.objects.create()` | Use factory_boy factories for consistent test data |
| Inline styles in templates | Tailwind utility classes only |
| `<form>` without `novalidate` | Always add `novalidate` — HTML5 native validation is unreliable across assistive technologies |
| `<form>` without `autocomplete` attributes | Always add `autocomplete="name"`, `autocomplete="email"`, `autocomplete="tel"`, etc. |
| "Submit" button text | Use outcome-focused CTA: "Reserve My Free Visit", "Log In", "Create Account" |
| Deploying without test gate | CI must run tests + lint + format before deploy |
| Missing security headers | Configure all Django security settings in production |
| `<label>` without `for` attribute | Always bind `<label for="id">` to `<input id="id">` — WCAG 1.3.1 requirement |
| Secrets in settings files | Use environment variables via `os.environ.get()` or `django-environ` |
| `settings.py` for all environments | Split into `base.py`, `development.py`, `test.py`, `production.py` |
| Synchronous external API calls in views | Use async views or Celery tasks for I/O-bound operations |
| HTMX without CSRF token | Set `hx-headers` on `<body>` with the CSRF token globally |
| `print()` for debugging | Use `logging` module with proper levels |
| Mutable default arguments in tasks | Pass IDs, not model instances, to Celery tasks |
| Missing `select_related`/`prefetch_related` | Profile queries with `django-debug-toolbar` and optimize |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a Django patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:django&title=[Django]%20)**

Use the `patterns:django` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
