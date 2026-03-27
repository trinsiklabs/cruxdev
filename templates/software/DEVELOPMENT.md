---
title: Development Guide
last_updated: [YYYY-MM-DD]
last_audit_plan: [PLAN-XXXX]
audit_status: current
---

# Development Guide

> Everything you need to set up, build, test, and contribute to [project name].

## Prerequisites

| Tool | Version | Purpose | Install |
|---|---|---|---|
| [Language runtime] | >= [version] | Application runtime | [install link/command] |
| [Package manager] | >= [version] | Dependency management | [install link/command] |
| [Database] | >= [version] | Data storage | [install link/command] |
| [Docker] | >= [version] | Container builds (optional) | [install link/command] |
| [Other tool] | >= [version] | [Purpose] | [install link/command] |

## Initial Setup

```bash
# 1. Clone the repository
git clone [repo-url]
cd [project-name]

# 2. Install dependencies
[install command]

# 3. Set up local configuration
cp .env.example .env
# Edit .env — see Configuration section below

# 4. Set up the database
[database setup command]
[migration command]
[optional seed command]

# 5. Verify setup
[verification command, e.g.: make check or ./scripts/verify-setup.sh]
```

### Expected Output After Setup

```
[What the developer should see when setup is complete, e.g.:]
✓ Dependencies installed
✓ Database created and migrated
✓ Configuration valid
✓ All checks pass — ready to develop
```

## Local Configuration

[Configuration needed for local development. For full reference, see [CONFIGURATION.md](CONFIGURATION.md).]

| Variable | Local Default | Description |
|---|---|---|
| `DATABASE_URL` | `postgres://localhost:5432/project_dev` | Local database |
| `LOG_LEVEL` | `debug` | Verbose logging for development |
| `PORT` | `8080` | Local HTTP port |
| [Other vars] | [defaults] | [descriptions] |

## Running the Application

### Development Server

```bash
# Start the development server with hot reload
[dev server command]

# The application is now available at http://localhost:[port]
```

### Background Services

[If the application has background workers, schedulers, etc.]

```bash
# Start background workers
[worker command]

# Start the scheduler (if applicable)
[scheduler command]
```

### Running Everything

```bash
# Start all services (app + workers + dependencies)
[docker-compose up / make dev / etc.]
```

## Project Structure

```
[project-name]/
├── src/                       # Application source code
│   ├── [module-a]/            # [Description of module A]
│   │   ├── [handler/controller] # Request handlers
│   │   ├── [service/logic]    # Business logic
│   │   └── [model/entity]     # Data models
│   ├── [module-b]/            # [Description of module B]
│   ├── [shared/common]/       # Shared utilities and types
│   └── [main entry point]     # Application entry point
├── tests/                     # Test suite
│   ├── unit/                  # Unit tests
│   ├── integration/           # Integration tests
│   └── [e2e/fixtures/etc.]   # Other test infrastructure
├── scripts/                   # Build, deploy, utility scripts
├── config/                    # Configuration files
├── migrations/                # Database migrations
├── docs/                      # Documentation
└── [other dirs]               # [Descriptions]
```

### Key Files

| File | Purpose |
|---|---|
| `[Makefile / package.json / Cargo.toml]` | Build configuration and task runner |
| `[main entry]` | Application entry point |
| `[config file]` | Application configuration |
| `.env.example` | Template for local environment variables |
| `[CI config]` | CI/CD pipeline definition |

## Build

```bash
# Development build (fast, with debug info)
[dev build command]

# Production build (optimized)
[prod build command]

# Clean build artifacts
[clean command]
```

## Testing

[For full testing strategy, see [TESTING.md](TESTING.md).]

### Running Tests

```bash
# Run all tests
[test command]

# Run unit tests only
[unit test command]

# Run integration tests only
[integration test command]

# Run a specific test file
[specific test command]

# Run tests matching a pattern
[pattern test command]
```

### Test Coverage

```bash
# Generate coverage report
[coverage command]

# View coverage report
[view coverage command, e.g.: open coverage/index.html]
```

### Writing Tests

- Place unit tests in `tests/unit/` (or alongside source files per project convention)
- Place integration tests in `tests/integration/`
- Name test files: `[module]_test.[ext]` or `test_[module].[ext]`
- Each test should be independent and not rely on execution order
- Use descriptive test names that explain the behavior being tested

## Code Style and Linting

### Linter

```bash
# Run the linter
[lint command]

# Auto-fix linting issues
[lint fix command]
```

### Formatter

```bash
# Format all code
[format command]

# Check formatting without changing files
[format check command]
```

### Style Rules

[Brief summary of project-specific style rules, or link to CODING_STANDARDS.md if it exists.]

- [Key rule 1, e.g.: Maximum line length is 100 characters]
- [Key rule 2, e.g.: Use snake_case for functions, PascalCase for types]
- [Key rule 3, e.g.: All public functions must have doc comments]

## Git Workflow

### Branch Naming

| Branch Type | Pattern | Example |
|---|---|---|
| Feature | `feature/[description]` | `feature/add-user-auth` |
| Bug fix | `fix/[description]` | `fix/login-timeout` |
| Hotfix | `hotfix/[description]` | `hotfix/critical-crash` |
| Release | `release/[version]` | `release/2.1.0` |

### Commit Messages

[Commit message convention. Example: Conventional Commits.]

```
<type>(<scope>): <subject>

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `perf`

### Pull Request Process

1. Create a branch from `[main branch]`
2. Make changes, add tests
3. Run `[lint and test commands]` locally
4. Push and create a pull request
5. Ensure CI passes
6. Get [N] approvals from [reviewers]
7. Squash and merge (or per project merge strategy)

## Debugging

### Logging

```bash
# Enable verbose logging
[environment variable or flag to increase log verbosity]

# View logs
[command to view logs]
```

### Common Development Issues

| Problem | Cause | Solution |
|---|---|---|
| [Common error message] | [Why it happens] | [How to fix it] |
| [Common error message] | [Why it happens] | [How to fix it] |
| Database connection refused | Database not running | `[start database command]` |
| Port already in use | Another process on the port | `lsof -i :[port]` to find and kill it |

## Database Development

### Creating Migrations

```bash
# Generate a new migration
[migration create command]
```

### Running Migrations Locally

```bash
# Apply all pending migrations
[migration up command]

# Roll back the last migration
[migration down command]

# Reset database (drop, create, migrate, seed)
[reset command]
```

## Dependency Management

### Adding Dependencies

```bash
# Add a runtime dependency
[add dependency command]

# Add a development dependency
[add dev dependency command]
```

### Updating Dependencies

```bash
# Check for outdated dependencies
[outdated check command]

# Update all dependencies
[update command]

# Update a specific dependency
[specific update command]
```

## IDE Setup

### Recommended Editor

[Editor name] with the following extensions/plugins:

| Extension | Purpose |
|---|---|
| [Extension name] | [What it does] |
| [Extension name] | [What it does] |
| [Extension name] | [What it does] |

### Editor Configuration

The project includes editor configuration files:

- `.editorconfig` — Indent style, line endings, etc.
- `[.vscode/settings.json]` — VS Code specific settings
- `[other config]` — [Purpose]

---

## Related Documents

- [Architecture](ARCHITECTURE.md) — System design overview
- [Testing](TESTING.md) — Full testing strategy
- [Configuration](CONFIGURATION.md) — Full configuration reference
- [Contributing](../CONTRIBUTING.md) — Contribution guidelines
