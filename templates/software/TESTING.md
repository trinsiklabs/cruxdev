---
title: Testing Strategy
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Testing Strategy

> How [project name] is tested: categories, tools, coverage, and conventions.

## Testing Philosophy

[Describe the project's testing philosophy in 2-3 sentences. Example: "We prioritize integration tests that exercise real dependencies over mocked unit tests. Every user-facing feature has at least one end-to-end test. Tests are a first-class deliverable — untested code is not shippable."]

## Test Categories

### Unit Tests

- **Scope:** Individual functions, methods, or classes in isolation
- **Dependencies:** Mocked or stubbed
- **Speed:** Fast (< 1 second per test)
- **Location:** `tests/unit/` [or alongside source files]
- **Naming:** `[module]_test.[ext]` or `test_[module].[ext]`
- **Run command:** `[unit test command]`

### Integration Tests

- **Scope:** Multiple components working together; real database, real file system
- **Dependencies:** Real (local instances or test containers)
- **Speed:** Moderate (seconds per test)
- **Location:** `tests/integration/`
- **Naming:** `[feature]_integration_test.[ext]`
- **Run command:** `[integration test command]`
- **Setup:** [Requires running database / docker-compose / etc.]

### End-to-End Tests

- **Scope:** Full system from external interface to database and back
- **Dependencies:** Full running system
- **Speed:** Slow (seconds to minutes per test)
- **Location:** `tests/e2e/`
- **Naming:** `[scenario]_e2e_test.[ext]`
- **Run command:** `[e2e test command]`
- **Setup:** [Requires full system running / test environment / etc.]

### [Smoke Tests / Contract Tests / Performance Tests / etc.]

- **Scope:** [What they test]
- **Location:** `tests/[type]/`
- **Run command:** `[command]`
- **When run:** [On every commit / before deploy / nightly / etc.]

## Test Pyramid

```
        /  E2E   \          Few, slow, high confidence
       / ──────── \
      / Integration \       Moderate count, moderate speed
     / ──────────── \
    /    Unit Tests   \     Many, fast, focused
   / ──────────────── \
```

| Category | Approximate Count | Target Coverage | Run Frequency |
|---|---|---|---|
| Unit | [count] | [X]% of business logic | Every commit |
| Integration | [count] | [X]% of component interactions | Every commit / PR |
| E2E | [count] | Critical user paths | Pre-deploy / nightly |

## Test Infrastructure

### Framework and Tools

| Tool | Purpose | Version |
|---|---|---|
| [Test framework] | Test runner and assertions | [version] |
| [Mock library] | Mocking and stubbing | [version] |
| [Coverage tool] | Code coverage measurement | [version] |
| [Test containers] | Database/service containers for tests | [version] |
| [Fixture library] | Test data generation | [version] |
| [HTTP testing] | API endpoint testing | [version] |

### Test Database

- **Setup:** [How the test database is created and seeded]
- **Isolation:** [How tests are isolated: transactions, truncation, separate databases]
- **Teardown:** [How test data is cleaned up]

### Test Fixtures / Factories

[How test data is created.]

```
[Example of a fixture/factory definition, e.g.:]

# factories/user.py
def create_user(overrides={}):
    defaults = {
        "name": "Test User",
        "email": f"test-{uuid4()}@example.com",
        "status": "active",
    }
    return User.create(**{**defaults, **overrides})
```

## Running Tests

### Quick Reference

| Command | What It Runs | When To Use |
|---|---|---|
| `[test all]` | All tests | Before pushing |
| `[test unit]` | Unit tests only | During development |
| `[test integration]` | Integration tests | After changing data layer |
| `[test e2e]` | End-to-end tests | After changing user flows |
| `[test specific]` | Single test file/pattern | Debugging a specific test |
| `[test coverage]` | All tests + coverage report | Before PR review |

### Running in CI

[How tests run in CI. What triggers them. How to read CI results.]

```yaml
# Example CI test step
[CI configuration snippet showing how tests are run]
```

## Coverage

### Targets

| Metric | Target | Current | Notes |
|---|---|---|---|
| Line coverage | [X]% | [Y]% | [Notes] |
| Branch coverage | [X]% | [Y]% | [Notes] |
| Function coverage | [X]% | [Y]% | [Notes] |

### Coverage Report

```bash
# Generate and view coverage
[coverage command]
```

### What Counts Toward Coverage

- Business logic: YES (must meet target)
- Configuration/boilerplate: NO (excluded from coverage)
- Generated code: NO (excluded from coverage)
- Test utilities: NO (excluded from coverage)

### Coverage Exclusions

[Files or patterns excluded from coverage measurement and why.]

```
[coverage config snippet showing exclusions]
```

## Test Conventions

### Naming

Tests should read as specifications:

```
# Good
test_user_creation_sends_welcome_email
test_expired_token_returns_401
test_admin_can_delete_any_resource

# Bad
test_1
test_user
test_api
```

### Structure (Arrange-Act-Assert)

```
[Example in the project's language showing the AAA pattern]

def test_deactivated_user_cannot_login():
    # Arrange
    user = create_user(status="suspended")

    # Act
    response = client.post("/login", json={
        "email": user.email,
        "password": "valid-password"
    })

    # Assert
    assert response.status_code == 403
    assert response.json()["error"]["code"] == "ACCOUNT_SUSPENDED"
```

### Test Independence

- Each test must be independent; no test relies on another test's side effects
- Tests must not depend on execution order
- Shared state (database, files) is reset between tests

### What to Test

| Should Test | Should NOT Test |
|---|---|
| Business logic and rules | Framework internals |
| Edge cases and error paths | Third-party library behavior |
| API contracts (input/output) | Private implementation details |
| Security boundaries | Getter/setter boilerplate |
| Data validation | Trivial constructors |
| State transitions | UI layout (unless critical) |

## Flaky Test Policy

- Flaky tests are treated as bugs
- A test that fails intermittently is marked with `[flaky marker]` and a tracking issue is created
- Flaky tests must be fixed within [X days] or deleted
- A flaky test NEVER blocks CI permanently — it is quarantined, not ignored

## Test Data Management

### Seed Data

[How seed data works for tests: fixtures, factories, SQL scripts, etc.]

### External Service Mocking

| Service | Mock Strategy | Tool |
|---|---|---|
| [External API] | [Record/replay / manual mock / fake server] | [Tool name] |
| [Payment provider] | [Sandbox environment / mock] | [Tool name] |
| [Email service] | [In-memory capture / mock] | [Tool name] |

---

## Related Documents

- [Development Guide](DEVELOPMENT.md) — How to run tests during development
- [API Reference](API.md) — API contracts that tests verify
- [Architecture](ARCHITECTURE.md) — System boundaries that integration tests cover
- [CI/CD](DEPLOYMENT.md) — How tests run in the pipeline
