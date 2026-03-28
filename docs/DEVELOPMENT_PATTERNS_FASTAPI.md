# Development Patterns — FastAPI Stack

FastAPI / Pydantic v2 / SQLAlchemy 2.0+ / SQLModel

This document captures stack-specific patterns, conventions, and decisions for FastAPI stack projects (FastAPI/Pydantic/SQLAlchemy/SQLModel). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure FastAPI projects, define Pydantic schemas, use SQLAlchemy async, test with pytest + httpx, deploy with Docker + Uvicorn, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Python | 3.12+ | Minimum 3.12 for `type` statement, f-string improvements |
| FastAPI | 0.115+ | Async-first, OpenAPI 3.1, Pydantic v2 native |
| Pydantic | 2.10+ | V2 engine (Rust-based), `model_validator`, `field_validator` |
| Pydantic Settings | 2.7+ | Environment-based configuration with type coercion |
| SQLAlchemy | 2.0+ | Async engine, mapped_column, type-annotated ORM |
| SQLModel | 0.0.22+ | SQLAlchemy + Pydantic hybrid models |
| Alembic | 1.14+ | Database migrations |
| Uvicorn | 0.34+ | ASGI server, HTTP/1.1 and HTTP/2 |
| httpx | 0.28+ | Async HTTP client + test client |
| pytest | 8.3+ | Test runner |
| pytest-asyncio | 0.25+ | Async test support |
| factory_boy | 3.3+ | Test factories |
| coverage | 7.6+ | Coverage measurement and enforcement |
| python-jose | 3.3+ | JWT encoding/decoding (or PyJWT 2.9+) |
| passlib | 1.7+ | Password hashing (bcrypt) |
| python-multipart | 0.0.20+ | Form data parsing |
| Redis | 7+ | Caching, rate limiting, task broker |
| PostgreSQL | 16+ | Primary database |
| Docker | 27+ | Containerized deployment |
| Ruff | 0.9+ | Linter and formatter (replaces black + isort + flake8) |

### Version Constraint Policy

Use `~=` (compatible release) constraints in `pyproject.toml` pinned to the minor version:

```toml
# Good — allows patch updates, blocks minor/major
fastapi = "~=0.115"
pydantic = "~=2.10"
sqlalchemy = "~=2.0"

# Bad — too loose, allows breaking minor updates
fastapi = ">=0.100"

# Bad — too tight, blocks patch fixes
fastapi = "==0.115.6"
```

Exception: for release candidates or packages with known instability, pin exact.

### FastAPI 0.115+ Features to Use

| Feature | Version | Use For |
|---|---|---|
| Pydantic v2 native | 0.100+ | All schema validation, serialization |
| `Annotated` dependencies | 0.95+ | Clean dependency injection signatures |
| Lifespan context manager | 0.93+ | Startup/shutdown (replaces `on_event`) |
| OpenAPI 3.1.0 | 0.99+ | Full JSON Schema compliance |
| `model_config` in response models | 0.100+ | Response serialization control |
| WebSocket improvements | 0.110+ | Real-time features |
| `generate_unique_id_function` | 0.99+ | Custom operation IDs for client generation |

---

## 2. Project Structure

### Application Organization

Each module is a bounded context. The project uses a `src/` layout with the application package inside:

```
project_name/
├── pyproject.toml
├── alembic.ini
├── Dockerfile
├── docker-compose.yml
├── src/
│   └── app/
│       ├── __init__.py
│       ├── main.py                 # FastAPI application factory
│       ├── config.py               # Pydantic Settings configuration
│       ├── database.py             # SQLAlchemy engine, session factory
│       ├── dependencies.py         # Shared dependencies (get_db, get_current_user)
│       ├── middleware.py           # Custom middleware (timing, correlation ID)
│       ├── exceptions.py          # Custom exception handlers
│       ├── accounts/              # Auth domain
│       │   ├── __init__.py
│       │   ├── models.py          # SQLAlchemy/SQLModel models
│       │   ├── schemas.py         # Pydantic request/response schemas
│       │   ├── router.py          # APIRouter with endpoints
│       │   ├── service.py         # Business logic layer
│       │   ├── repository.py      # Database access layer
│       │   └── dependencies.py    # Module-specific dependencies
│       ├── chapters/              # Chapter management domain
│       │   ├── __init__.py
│       │   ├── models.py
│       │   ├── schemas.py
│       │   ├── router.py
│       │   ├── service.py
│       │   └── repository.py
│       ├── visitors/              # Visitor pipeline domain
│       │   ├── __init__.py
│       │   ├── models.py
│       │   ├── schemas.py
│       │   ├── router.py
│       │   ├── service.py
│       │   └── repository.py
│       └── core/                  # Shared utilities
│           ├── __init__.py
│           ├── security.py        # JWT, password hashing, OAuth2
│           ├── pagination.py      # Pagination helpers
│           └── tasks.py           # Background task utilities
├── alembic/
│   ├── env.py
│   ├── script.py.mako
│   └── versions/                  # Migration files
├── tests/
│   ├── __init__.py
│   ├── conftest.py                # Shared fixtures (engine, session, client)
│   ├── factories.py               # factory_boy factories
│   ├── accounts/
│   │   ├── __init__.py
│   │   ├── test_router.py
│   │   ├── test_service.py
│   │   └── test_schemas.py
│   ├── chapters/
│   │   ├── __init__.py
│   │   ├── test_router.py
│   │   ├── test_service.py
│   │   └── test_schemas.py
│   └── visitors/
│       ├── __init__.py
│       ├── test_router.py
│       ├── test_service.py
│       └── test_schemas.py
└── scripts/
    ├── seed.py                    # Database seeding
    └── healthcheck.py             # Container health check
```

**Convention:** One module per bounded context. Each module has its own models, schemas, router, service, and repository. Cross-module communication goes through service functions, never direct model imports.

**Convention:** The `src/` layout prevents accidentally importing from the project root. Install in editable mode: `pip install -e ".[dev]"`.

### Test Mirror Structure

Tests mirror the `src/app/` structure:

```
tests/
├── conftest.py              # Engine, session, client, auth fixtures
├── factories.py             # All factory_boy factories
├── accounts/
│   ├── test_router.py       # HTTP endpoint tests
│   ├── test_service.py      # Business logic tests
│   └── test_schemas.py      # Pydantic validation tests
├── chapters/
│   └── ...
└── visitors/
    └── ...
```

---

## 3. SQLAlchemy / SQLModel Patterns

### Database Configuration (Async)

```python
# src/app/database.py
from collections.abc import AsyncGenerator

from sqlalchemy.ext.asyncio import (
    AsyncSession,
    async_sessionmaker,
    create_async_engine,
)
from sqlmodel import SQLModel

from app.config import settings

engine = create_async_engine(
    settings.database_url,
    echo=settings.debug,
    pool_size=settings.db_pool_size,
    max_overflow=settings.db_max_overflow,
    pool_pre_ping=True,
)

async_session_factory = async_sessionmaker(
    engine,
    class_=AsyncSession,
    expire_on_commit=False,
)


async def get_db() -> AsyncGenerator[AsyncSession, None]:
    async with async_session_factory() as session:
        try:
            yield session
            await session.commit()
        except Exception:
            await session.rollback()
            raise
```

**Conventions:**
- Always use `expire_on_commit=False` for async sessions — accessing attributes after commit would trigger lazy loads that fail in async context
- Always use `pool_pre_ping=True` to handle stale connections
- Always commit in the dependency; rollback on exception
- Use `create_async_engine` with `asyncpg` driver: `postgresql+asyncpg://...`

### SQLModel Model Template

Use SQLModel for models that double as Pydantic schemas. Use plain SQLAlchemy `DeclarativeBase` when you need full ORM features that SQLModel does not support (e.g., complex polymorphic inheritance, composite keys).

```python
# src/app/chapters/models.py
import uuid
from datetime import datetime

from sqlmodel import Field, Relationship, SQLModel


class Chapter(SQLModel, table=True):
    __tablename__ = "chapters"

    id: uuid.UUID = Field(default_factory=uuid.uuid4, primary_key=True)
    name: str = Field(max_length=255, index=True)
    slug: str = Field(max_length=255, unique=True, index=True)
    meeting_day: int = Field(ge=0, le=6)  # 0=Monday, 6=Sunday
    meeting_time: str = Field(max_length=8)  # HH:MM:SS
    location: str = Field(max_length=500)
    is_active: bool = Field(default=True)
    created_at: datetime = Field(default_factory=datetime.utcnow)
    updated_at: datetime = Field(default_factory=datetime.utcnow)

    # Relationships
    seats: list["Seat"] = Relationship(back_populates="chapter")


class Seat(SQLModel, table=True):
    __tablename__ = "seats"

    id: uuid.UUID = Field(default_factory=uuid.uuid4, primary_key=True)
    chapter_id: uuid.UUID = Field(foreign_key="chapters.id", index=True)
    classification: str = Field(max_length=255)
    category: str = Field(max_length=255)
    status: str = Field(default="open", max_length=50)
    created_at: datetime = Field(default_factory=datetime.utcnow)

    # Relationships
    chapter: Chapter | None = Relationship(back_populates="seats")
```

### Plain SQLAlchemy Model Template (When Needed)

For models that need features beyond SQLModel:

```python
# src/app/accounts/models.py
import uuid
from datetime import datetime

from sqlalchemy import String, func
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column, relationship


class Base(DeclarativeBase):
    pass


class User(Base):
    __tablename__ = "users"

    id: Mapped[uuid.UUID] = mapped_column(
        primary_key=True, default=uuid.uuid4
    )
    email: Mapped[str] = mapped_column(
        String(255), unique=True, index=True
    )
    hashed_password: Mapped[str] = mapped_column(String(255))
    role: Mapped[str] = mapped_column(String(50), default="member")
    is_active: Mapped[bool] = mapped_column(default=True)
    created_at: Mapped[datetime] = mapped_column(
        server_default=func.now()
    )
    updated_at: Mapped[datetime] = mapped_column(
        server_default=func.now(), onupdate=func.now()
    )
```

**Conventions:**
- Always use `uuid.UUID` for primary keys
- Always include `created_at` and `updated_at` timestamps
- Always use `Mapped[]` type annotations (SQLAlchemy 2.0 style)
- Always set `index=True` on columns used in WHERE clauses and foreign keys
- Always set `max_length` on string columns — never unbounded text for structured fields

### Alembic Migrations

```bash
# Generate migration from model changes
alembic revision --autogenerate -m "describe the change"

# Apply migrations
alembic upgrade head

# Rollback one step (dev only, never production)
alembic downgrade -1
```

Configure Alembic for async:

```python
# alembic/env.py
import asyncio
from logging.config import fileConfig

from alembic import context
from sqlalchemy.ext.asyncio import create_async_engine

from app.config import settings
from app.database import SQLModel  # or Base if using plain SQLAlchemy

config = context.config
if config.config_file_name is not None:
    fileConfig(config.config_file_name)

target_metadata = SQLModel.metadata


def run_migrations_offline() -> None:
    url = settings.database_url
    context.configure(
        url=url,
        target_metadata=target_metadata,
        literal_binds=True,
        dialect_opts={"paramstyle": "named"},
    )
    with context.begin_transaction():
        context.run_migrations()


def do_run_migrations(connection):
    context.configure(connection=connection, target_metadata=target_metadata)
    with context.begin_transaction():
        context.run_migrations()


async def run_async_migrations() -> None:
    connectable = create_async_engine(settings.database_url)
    async with connectable.connect() as connection:
        await connection.run_sync(do_run_migrations)
    await connectable.dispose()


def run_migrations_online() -> None:
    asyncio.run(run_async_migrations())


if context.is_offline_mode():
    run_migrations_offline()
else:
    run_migrations_online()
```

Never edit a migration after it has been committed. Write a new corrective migration instead.

---

## 4. Authentication & Authorization

### OAuth2 + JWT Setup

Authentication uses FastAPI's built-in OAuth2 with JWT tokens:

```python
# src/app/core/security.py
from datetime import datetime, timedelta, timezone

from fastapi import Depends, HTTPException, status
from fastapi.security import OAuth2PasswordBearer
from jose import JWTError, jwt
from passlib.context import CryptContext
from typing import Annotated

from app.config import settings

pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")
oauth2_scheme = OAuth2PasswordBearer(tokenUrl="/api/v1/auth/token")


def verify_password(plain_password: str, hashed_password: str) -> bool:
    return pwd_context.verify(plain_password, hashed_password)


def get_password_hash(password: str) -> str:
    return pwd_context.hash(password)


def create_access_token(
    data: dict,
    expires_delta: timedelta | None = None,
) -> str:
    to_encode = data.copy()
    expire = datetime.now(timezone.utc) + (
        expires_delta or timedelta(minutes=settings.access_token_expire_minutes)
    )
    to_encode.update({"exp": expire})
    return jwt.encode(to_encode, settings.secret_key, algorithm=settings.algorithm)


def create_refresh_token(data: dict) -> str:
    to_encode = data.copy()
    expire = datetime.now(timezone.utc) + timedelta(
        days=settings.refresh_token_expire_days
    )
    to_encode.update({"exp": expire, "type": "refresh"})
    return jwt.encode(to_encode, settings.secret_key, algorithm=settings.algorithm)
```

### Current User Dependency

```python
# src/app/dependencies.py
from typing import Annotated

from fastapi import Depends, HTTPException, status
from jose import JWTError, jwt
from sqlalchemy import select
from sqlalchemy.ext.asyncio import AsyncSession

from app.accounts.models import User
from app.config import settings
from app.core.security import oauth2_scheme
from app.database import get_db


async def get_current_user(
    token: Annotated[str, Depends(oauth2_scheme)],
    db: Annotated[AsyncSession, Depends(get_db)],
) -> User:
    credentials_exception = HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="Could not validate credentials",
        headers={"WWW-Authenticate": "Bearer"},
    )
    try:
        payload = jwt.decode(
            token, settings.secret_key, algorithms=[settings.algorithm]
        )
        user_id: str | None = payload.get("sub")
        if user_id is None:
            raise credentials_exception
    except JWTError:
        raise credentials_exception

    result = await db.execute(select(User).where(User.id == user_id))
    user = result.scalar_one_or_none()
    if user is None or not user.is_active:
        raise credentials_exception
    return user


async def get_current_active_user(
    current_user: Annotated[User, Depends(get_current_user)],
) -> User:
    if not current_user.is_active:
        raise HTTPException(status_code=400, detail="Inactive user")
    return current_user


def require_role(required_role: str):
    """Dependency factory for role-based access control."""

    async def role_checker(
        current_user: Annotated[User, Depends(get_current_user)],
    ) -> User:
        if current_user.role != required_role and current_user.role != "platform_admin":
            raise HTTPException(
                status_code=status.HTTP_403_FORBIDDEN,
                detail="Insufficient permissions",
            )
        return current_user

    return role_checker
```

### Role Model

Four roles, enforced through dependency injection:

| Role | Value | Access |
|---|---|---|
| `visitor` | (unauthenticated) | Public endpoints, registration |
| `member` | `"member"` | Own profile, roster, claim seats |
| `chapter_admin` | `"chapter_admin"` | Chapter management, visitor pipeline, user admin |
| `platform_admin` | `"platform_admin"` | Everything, all chapters |

```python
# Usage in routers
from typing import Annotated
from app.dependencies import get_current_user, require_role

CurrentUser = Annotated[User, Depends(get_current_user)]
AdminUser = Annotated[User, Depends(require_role("chapter_admin"))]
PlatformAdmin = Annotated[User, Depends(require_role("platform_admin"))]


@router.get("/chapters/{chapter_id}/admin")
async def chapter_admin_panel(
    chapter_id: uuid.UUID,
    current_user: AdminUser,
    db: Annotated[AsyncSession, Depends(get_db)],
):
    ...
```

### Auth Router

```python
# src/app/accounts/router.py
from datetime import timedelta
from typing import Annotated

from fastapi import APIRouter, Depends, HTTPException, status
from fastapi.security import OAuth2PasswordRequestForm
from sqlalchemy.ext.asyncio import AsyncSession

from app.accounts.schemas import Token, UserCreate, UserRead
from app.accounts.service import authenticate_user, create_user
from app.config import settings
from app.core.security import create_access_token, create_refresh_token
from app.database import get_db

router = APIRouter(prefix="/api/v1/auth", tags=["authentication"])


@router.post("/token", response_model=Token)
async def login_for_access_token(
    form_data: Annotated[OAuth2PasswordRequestForm, Depends()],
    db: Annotated[AsyncSession, Depends(get_db)],
):
    user = await authenticate_user(db, form_data.username, form_data.password)
    if not user:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Incorrect email or password",
            headers={"WWW-Authenticate": "Bearer"},
        )
    access_token = create_access_token(
        data={"sub": str(user.id)},
        expires_delta=timedelta(minutes=settings.access_token_expire_minutes),
    )
    refresh_token = create_refresh_token(data={"sub": str(user.id)})
    return Token(
        access_token=access_token,
        refresh_token=refresh_token,
        token_type="bearer",
    )


@router.post("/register", response_model=UserRead, status_code=201)
async def register(
    user_in: UserCreate,
    db: Annotated[AsyncSession, Depends(get_db)],
):
    return await create_user(db, user_in)
```

---

## 5. Pydantic v2 Schema Patterns

### Schema Design Philosophy

Separate schemas for each operation. Never use the database model as a request/response schema.

```python
# src/app/chapters/schemas.py
import uuid
from datetime import datetime

from pydantic import BaseModel, ConfigDict, Field, field_validator


class ChapterCreate(BaseModel):
    """Schema for creating a new chapter."""

    name: str = Field(min_length=1, max_length=255)
    slug: str = Field(min_length=1, max_length=255, pattern=r"^[a-z0-9-]+$")
    meeting_day: int = Field(ge=0, le=6)
    meeting_time: str = Field(pattern=r"^\d{2}:\d{2}(:\d{2})?$")
    location: str = Field(min_length=1, max_length=500)

    @field_validator("slug")
    @classmethod
    def slug_must_be_lowercase(cls, v: str) -> str:
        return v.lower()


class ChapterUpdate(BaseModel):
    """Schema for updating a chapter. All fields optional."""

    name: str | None = Field(default=None, min_length=1, max_length=255)
    meeting_day: int | None = Field(default=None, ge=0, le=6)
    meeting_time: str | None = Field(
        default=None, pattern=r"^\d{2}:\d{2}(:\d{2})?$"
    )
    location: str | None = Field(default=None, min_length=1, max_length=500)
    is_active: bool | None = None


class ChapterRead(BaseModel):
    """Schema for reading a chapter. Maps from ORM model."""

    model_config = ConfigDict(from_attributes=True)

    id: uuid.UUID
    name: str
    slug: str
    meeting_day: int
    meeting_time: str
    location: str
    is_active: bool
    created_at: datetime
    updated_at: datetime


class ChapterList(BaseModel):
    """Paginated list of chapters."""

    items: list[ChapterRead]
    total: int
    page: int
    page_size: int
    pages: int
```

**Conventions:**
- `*Create` — input schema for creation, required fields only
- `*Update` — input schema for updates, all fields `Optional`
- `*Read` — output schema with `model_config = ConfigDict(from_attributes=True)`
- `*List` — paginated response wrapper
- Always use `Field()` with constraints (`min_length`, `max_length`, `ge`, `le`, `pattern`)
- Always use `field_validator` for complex validation, not `@validator` (v1 syntax)
- Never expose `hashed_password` or internal fields in `*Read` schemas

### Model Validators for Cross-Field Validation

```python
from pydantic import BaseModel, model_validator


class DateRangeFilter(BaseModel):
    start_date: datetime | None = None
    end_date: datetime | None = None

    @model_validator(mode="after")
    def validate_date_range(self) -> "DateRangeFilter":
        if self.start_date and self.end_date and self.start_date > self.end_date:
            raise ValueError("start_date must be before end_date")
        return self
```

### Pydantic Settings for Configuration

```python
# src/app/config.py
from pydantic import Field, PostgresDsn
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        case_sensitive=False,
    )

    # Application
    app_name: str = "FastAPI App"
    debug: bool = False
    environment: str = "production"

    # Database
    database_url: str = "postgresql+asyncpg://localhost/app"
    db_pool_size: int = 5
    db_max_overflow: int = 10

    # Auth
    secret_key: str
    algorithm: str = "HS256"
    access_token_expire_minutes: int = 30
    refresh_token_expire_days: int = 7

    # CORS
    cors_origins: list[str] = ["http://localhost:3000"]

    # Redis
    redis_url: str = "redis://localhost:6379/0"


settings = Settings()
```

**Never hardcode secrets.** All sensitive values come from environment variables or `.env` files. The `.env` file is in `.gitignore` — never committed.

---

## 6. Testing Patterns

### Test Pyramid (FastAPI-specific)

```
        /\
       /  \          E2E (Playwright) — deferred to later slices
      /    \
     /------\
    /        \        Integration Tests (httpx AsyncClient + real DB)
   /          \       Full request/response cycle, auth flows, middleware
  /------------\
 /              \      Service Tests (pytest-asyncio + DB)
/                \     Business logic through DB, external service mocks
/------------------\
/                    \   Unit Tests (pytest)
/                      \  Pydantic schemas, pure functions, validators
/------------------------\
```

### pytest + pytest-asyncio Configuration

```toml
# pyproject.toml
[tool.pytest.ini_options]
asyncio_mode = "auto"
python_files = ["test_*.py"]
python_classes = ["Test*"]
python_functions = ["test_*"]
addopts = [
    "--strict-markers",
    "--strict-config",
    "-ra",
    "--tb=short",
    "--cov=src/app",
    "--cov-report=term-missing",
    "--cov-fail-under=100",
]
markers = [
    "slow: marks tests as slow (deselect with '-m \"not slow\"')",
    "integration: marks integration tests",
]
```

### Core Test Fixtures (conftest.py)

```python
# tests/conftest.py
import asyncio
from collections.abc import AsyncGenerator
from typing import Annotated

import pytest
from httpx import ASGITransport, AsyncClient
from sqlalchemy.ext.asyncio import (
    AsyncSession,
    async_sessionmaker,
    create_async_engine,
)
from sqlmodel import SQLModel

from app.config import settings
from app.database import get_db
from app.main import create_app

# Use a separate test database
TEST_DATABASE_URL = settings.database_url + "_test"


@pytest.fixture(scope="session")
def event_loop():
    """Create a session-scoped event loop."""
    loop = asyncio.new_event_loop()
    yield loop
    loop.close()


@pytest.fixture(scope="session")
async def engine():
    """Create a session-scoped async engine."""
    engine = create_async_engine(TEST_DATABASE_URL, echo=False)
    async with engine.begin() as conn:
        await conn.run_sync(SQLModel.metadata.create_all)
    yield engine
    async with engine.begin() as conn:
        await conn.run_sync(SQLModel.metadata.drop_all)
    await engine.dispose()


@pytest.fixture
async def db_session(engine) -> AsyncGenerator[AsyncSession, None]:
    """Create a per-test async session with transaction rollback."""
    async_session = async_sessionmaker(
        engine, class_=AsyncSession, expire_on_commit=False
    )
    async with async_session() as session:
        async with session.begin():
            yield session
            await session.rollback()


@pytest.fixture
async def client(db_session: AsyncSession) -> AsyncGenerator[AsyncClient, None]:
    """Create an httpx AsyncClient with dependency overrides."""
    app = create_app()

    async def override_get_db():
        yield db_session

    app.dependency_overrides[get_db] = override_get_db

    async with AsyncClient(
        transport=ASGITransport(app=app),
        base_url="http://test",
    ) as client:
        yield client

    app.dependency_overrides.clear()


@pytest.fixture
async def authenticated_client(
    client: AsyncClient,
    user_factory,
) -> AsyncClient:
    """Client with a valid auth token."""
    from app.core.security import create_access_token

    user = await user_factory()
    token = create_access_token(data={"sub": str(user.id)})
    client.headers["Authorization"] = f"Bearer {token}"
    return client


@pytest.fixture
async def admin_client(
    client: AsyncClient,
    admin_factory,
) -> AsyncClient:
    """Client with a platform_admin auth token."""
    from app.core.security import create_access_token

    admin = await admin_factory()
    token = create_access_token(data={"sub": str(admin.id)})
    client.headers["Authorization"] = f"Bearer {token}"
    return client
```

### factory_boy Factories

```python
# tests/factories.py
import uuid
from datetime import datetime

import factory
from sqlalchemy.ext.asyncio import AsyncSession

from app.accounts.models import User
from app.chapters.models import Chapter, Seat
from app.core.security import get_password_hash


class UserFactory(factory.Factory):
    class Meta:
        model = User
        exclude = ["_session"]

    id = factory.LazyFunction(uuid.uuid4)
    email = factory.Sequence(lambda n: f"user{n}@example.com")
    hashed_password = factory.LazyFunction(
        lambda: get_password_hash("testpass123")
    )
    role = "member"
    is_active = True
    created_at = factory.LazyFunction(datetime.utcnow)
    updated_at = factory.LazyFunction(datetime.utcnow)


class ChapterFactory(factory.Factory):
    class Meta:
        model = Chapter

    id = factory.LazyFunction(uuid.uuid4)
    name = factory.Sequence(lambda n: f"Chapter {n}")
    slug = factory.Sequence(lambda n: f"chapter-{n}")
    meeting_day = 2  # Wednesday
    meeting_time = "07:00:00"
    location = "Denver, NC"
    is_active = True
    created_at = factory.LazyFunction(datetime.utcnow)
    updated_at = factory.LazyFunction(datetime.utcnow)


class SeatFactory(factory.Factory):
    class Meta:
        model = Seat

    id = factory.LazyFunction(uuid.uuid4)
    classification = "Plumber"
    category = "Home Services"
    status = "open"
    created_at = factory.LazyFunction(datetime.utcnow)
```

Use conftest fixtures to wire factories to the test database session:

```python
# tests/conftest.py (additional fixtures)

@pytest.fixture
def user_factory(db_session: AsyncSession):
    async def _create(**kwargs):
        user = UserFactory(**kwargs)
        db_session.add(user)
        await db_session.flush()
        return user
    return _create


@pytest.fixture
def chapter_factory(db_session: AsyncSession):
    async def _create(**kwargs):
        chapter = ChapterFactory(**kwargs)
        db_session.add(chapter)
        await db_session.flush()
        return chapter
    return _create
```

### Router Testing with httpx

```python
# tests/chapters/test_router.py
import pytest
from httpx import AsyncClient


class TestChapterEndpoints:
    async def test_list_chapters(
        self, authenticated_client: AsyncClient, chapter_factory
    ):
        chapter = await chapter_factory(name="Test Chapter")
        response = await authenticated_client.get("/api/v1/chapters/")
        assert response.status_code == 200
        data = response.json()
        assert data["total"] >= 1
        assert any(c["name"] == "Test Chapter" for c in data["items"])

    async def test_create_chapter_requires_admin(
        self, authenticated_client: AsyncClient
    ):
        response = await authenticated_client.post(
            "/api/v1/chapters/",
            json={
                "name": "New Chapter",
                "slug": "new-chapter",
                "meeting_day": 2,
                "meeting_time": "07:00",
                "location": "Charlotte, NC",
            },
        )
        assert response.status_code == 403

    async def test_create_chapter_as_admin(
        self, admin_client: AsyncClient
    ):
        response = await admin_client.post(
            "/api/v1/chapters/",
            json={
                "name": "New Chapter",
                "slug": "new-chapter",
                "meeting_day": 2,
                "meeting_time": "07:00",
                "location": "Charlotte, NC",
            },
        )
        assert response.status_code == 201
        data = response.json()
        assert data["name"] == "New Chapter"
        assert data["slug"] == "new-chapter"

    async def test_get_chapter_not_found(
        self, authenticated_client: AsyncClient
    ):
        response = await authenticated_client.get(
            "/api/v1/chapters/00000000-0000-0000-0000-000000000000"
        )
        assert response.status_code == 404
```

### Service Testing

```python
# tests/chapters/test_service.py
import pytest
from sqlalchemy.ext.asyncio import AsyncSession

from app.chapters.schemas import ChapterCreate
from app.chapters.service import create_chapter, get_chapter_by_slug


class TestChapterService:
    async def test_create_chapter(self, db_session: AsyncSession):
        data = ChapterCreate(
            name="Test Chapter",
            slug="test-chapter",
            meeting_day=2,
            meeting_time="07:00",
            location="Denver, NC",
        )
        chapter = await create_chapter(db_session, data)
        assert chapter.name == "Test Chapter"
        assert chapter.slug == "test-chapter"

    async def test_get_chapter_by_slug(
        self, db_session: AsyncSession, chapter_factory
    ):
        await chapter_factory(slug="test-slug")
        chapter = await get_chapter_by_slug(db_session, "test-slug")
        assert chapter is not None
        assert chapter.slug == "test-slug"

    async def test_get_chapter_by_slug_not_found(
        self, db_session: AsyncSession
    ):
        chapter = await get_chapter_by_slug(db_session, "nonexistent")
        assert chapter is None
```

### Pydantic Schema Testing

```python
# tests/chapters/test_schemas.py
import pytest
from pydantic import ValidationError

from app.chapters.schemas import ChapterCreate, ChapterUpdate


class TestChapterCreate:
    def test_valid_data(self):
        data = ChapterCreate(
            name="Test",
            slug="test-chapter",
            meeting_day=2,
            meeting_time="07:00",
            location="Denver, NC",
        )
        assert data.name == "Test"

    def test_slug_must_be_lowercase(self):
        data = ChapterCreate(
            name="Test",
            slug="Test-Chapter",
            meeting_day=2,
            meeting_time="07:00",
            location="Denver, NC",
        )
        assert data.slug == "test-chapter"

    def test_invalid_meeting_day(self):
        with pytest.raises(ValidationError) as exc_info:
            ChapterCreate(
                name="Test",
                slug="test",
                meeting_day=7,
                meeting_time="07:00",
                location="Denver, NC",
            )
        assert "meeting_day" in str(exc_info.value)

    def test_slug_rejects_special_characters(self):
        with pytest.raises(ValidationError):
            ChapterCreate(
                name="Test",
                slug="test chapter!",
                meeting_day=2,
                meeting_time="07:00",
                location="Denver, NC",
            )


class TestChapterUpdate:
    def test_all_fields_optional(self):
        data = ChapterUpdate()
        assert data.name is None
        assert data.meeting_day is None

    def test_partial_update(self):
        data = ChapterUpdate(name="Updated Name")
        assert data.name == "Updated Name"
        assert data.location is None
```

### External Service Mocking

Use `unittest.mock.AsyncMock` for async external services:

```python
# tests/visitors/test_service.py
from unittest.mock import AsyncMock, patch

import pytest

from app.visitors.service import register_visitor


class TestRegisterVisitor:
    @patch("app.visitors.service.crm_client.create_contact", new_callable=AsyncMock)
    async def test_creates_crm_contact(self, mock_create, db_session, chapter_factory):
        mock_create.return_value = {"id": "crm_123"}
        chapter = await chapter_factory()

        visitor = await register_visitor(
            db_session,
            name="Jane Smith",
            email="jane@example.com",
            chapter_id=chapter.id,
        )

        assert visitor.crm_contact_id == "crm_123"
        mock_create.assert_called_once()

    @patch("app.visitors.service.crm_client.create_contact", new_callable=AsyncMock)
    async def test_handles_crm_failure_gracefully(
        self, mock_create, db_session, chapter_factory
    ):
        mock_create.side_effect = ConnectionError("CRM down")
        chapter = await chapter_factory()

        # Visitor is still created, CRM sync is retried via background task
        visitor = await register_visitor(
            db_session,
            name="Jane Smith",
            email="jane@example.com",
            chapter_id=chapter.id,
        )
        assert visitor is not None
        assert visitor.crm_contact_id is None
```

### Test Configuration

```python
# .env.test
DATABASE_URL=postgresql+asyncpg://localhost/app_test
SECRET_KEY=test-secret-key-not-for-production
DEBUG=false
ENVIRONMENT=test
REDIS_URL=redis://localhost:6379/1
```

---

## 7. Dependency Injection Patterns

### Philosophy

FastAPI's dependency injection is the primary mechanism for composing behavior. Dependencies replace middleware for request-scoped logic, replace global state for configuration, and replace decorators for access control.

### Annotated Type Aliases

Define reusable type aliases for common dependencies:

```python
# src/app/dependencies.py
from typing import Annotated

from fastapi import Depends
from sqlalchemy.ext.asyncio import AsyncSession

from app.accounts.models import User
from app.database import get_db

# Database session
DbSession = Annotated[AsyncSession, Depends(get_db)]

# Auth
CurrentUser = Annotated[User, Depends(get_current_user)]
ActiveUser = Annotated[User, Depends(get_current_active_user)]
AdminUser = Annotated[User, Depends(require_role("chapter_admin"))]
PlatformAdmin = Annotated[User, Depends(require_role("platform_admin"))]
```

Usage in routers becomes clean:

```python
@router.get("/chapters/")
async def list_chapters(db: DbSession, user: CurrentUser):
    ...

@router.post("/chapters/")
async def create_chapter(db: DbSession, user: AdminUser, data: ChapterCreate):
    ...
```

### Dependency Factories (Parameterized Dependencies)

```python
# src/app/core/pagination.py
from dataclasses import dataclass
from typing import Annotated

from fastapi import Query


@dataclass
class PaginationParams:
    page: int = 1
    page_size: int = 20

    @property
    def offset(self) -> int:
        return (self.page - 1) * self.page_size


def pagination(
    max_page_size: int = 100,
):
    """Dependency factory for pagination with configurable max page size."""

    async def _pagination(
        page: Annotated[int, Query(ge=1)] = 1,
        page_size: Annotated[int, Query(ge=1, le=max_page_size)] = 20,
    ) -> PaginationParams:
        return PaginationParams(page=page, page_size=page_size)

    return _pagination


# Usage: different max sizes per router
Pagination = Annotated[PaginationParams, Depends(pagination(max_page_size=100))]
SmallPagination = Annotated[PaginationParams, Depends(pagination(max_page_size=20))]
```

### Repository Dependencies

```python
# src/app/chapters/repository.py
from typing import Annotated

from fastapi import Depends
from sqlalchemy import func, select
from sqlalchemy.ext.asyncio import AsyncSession

from app.chapters.models import Chapter
from app.database import get_db


class ChapterRepository:
    def __init__(self, session: AsyncSession):
        self.session = session

    async def get_by_id(self, chapter_id: uuid.UUID) -> Chapter | None:
        result = await self.session.execute(
            select(Chapter).where(Chapter.id == chapter_id)
        )
        return result.scalar_one_or_none()

    async def get_by_slug(self, slug: str) -> Chapter | None:
        result = await self.session.execute(
            select(Chapter).where(Chapter.slug == slug)
        )
        return result.scalar_one_or_none()

    async def list(
        self, *, offset: int = 0, limit: int = 20, active_only: bool = True
    ) -> tuple[list[Chapter], int]:
        query = select(Chapter)
        count_query = select(func.count()).select_from(Chapter)

        if active_only:
            query = query.where(Chapter.is_active.is_(True))
            count_query = count_query.where(Chapter.is_active.is_(True))

        total = (await self.session.execute(count_query)).scalar_one()
        result = await self.session.execute(
            query.offset(offset).limit(limit).order_by(Chapter.name)
        )
        return list(result.scalars().all()), total

    async def create(self, chapter: Chapter) -> Chapter:
        self.session.add(chapter)
        await self.session.flush()
        await self.session.refresh(chapter)
        return chapter

    async def update(self, chapter: Chapter, data: dict) -> Chapter:
        for key, value in data.items():
            if value is not None:
                setattr(chapter, key, value)
        await self.session.flush()
        await self.session.refresh(chapter)
        return chapter


async def get_chapter_repository(
    db: Annotated[AsyncSession, Depends(get_db)],
) -> ChapterRepository:
    return ChapterRepository(db)


ChapterRepo = Annotated[ChapterRepository, Depends(get_chapter_repository)]
```

### Service Layer Dependencies

```python
# src/app/chapters/service.py
from typing import Annotated

from fastapi import Depends

from app.chapters.models import Chapter
from app.chapters.repository import ChapterRepo, ChapterRepository
from app.chapters.schemas import ChapterCreate, ChapterUpdate


class ChapterService:
    def __init__(self, repo: ChapterRepository):
        self.repo = repo

    async def create(self, data: ChapterCreate) -> Chapter:
        chapter = Chapter(**data.model_dump())
        return await self.repo.create(chapter)

    async def update(self, chapter_id: uuid.UUID, data: ChapterUpdate) -> Chapter:
        chapter = await self.repo.get_by_id(chapter_id)
        if chapter is None:
            raise ValueError("Chapter not found")
        update_data = data.model_dump(exclude_unset=True)
        return await self.repo.update(chapter, update_data)

    async def list(
        self, *, page: int = 1, page_size: int = 20
    ) -> tuple[list[Chapter], int]:
        offset = (page - 1) * page_size
        return await self.repo.list(offset=offset, limit=page_size)


async def get_chapter_service(repo: ChapterRepo) -> ChapterService:
    return ChapterService(repo)


ChapterSvc = Annotated[ChapterService, Depends(get_chapter_service)]
```

---

## 8. Router & Endpoint Patterns

### Router Organization

```python
# src/app/main.py
from contextlib import asynccontextmanager

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.accounts.router import router as accounts_router
from app.chapters.router import router as chapters_router
from app.visitors.router import router as visitors_router
from app.config import settings
from app.middleware import CorrelationIdMiddleware, TimingMiddleware


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Startup and shutdown events."""
    # Startup: initialize connections, warm caches
    yield
    # Shutdown: close connections, flush buffers


def create_app() -> FastAPI:
    app = FastAPI(
        title=settings.app_name,
        version="1.0.0",
        lifespan=lifespan,
        docs_url="/api/docs" if settings.debug else None,
        redoc_url="/api/redoc" if settings.debug else None,
    )

    # Middleware (order matters — last added runs first)
    app.add_middleware(TimingMiddleware)
    app.add_middleware(CorrelationIdMiddleware)
    app.add_middleware(
        CORSMiddleware,
        allow_origins=settings.cors_origins,
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )

    # Routers
    app.include_router(accounts_router)
    app.include_router(chapters_router)
    app.include_router(visitors_router)

    return app
```

**Conventions:**
- Use `lifespan` context manager, not `@app.on_event("startup")` (deprecated)
- Disable docs in production (`docs_url=None`)
- Always use `create_app()` factory for testability
- CORS origins come from settings, never hardcoded

### Endpoint Patterns

```python
# src/app/chapters/router.py
import uuid
from typing import Annotated

from fastapi import APIRouter, Depends, HTTPException, status

from app.chapters.schemas import ChapterCreate, ChapterList, ChapterRead, ChapterUpdate
from app.chapters.service import ChapterSvc
from app.core.pagination import Pagination
from app.dependencies import AdminUser, CurrentUser

router = APIRouter(prefix="/api/v1/chapters", tags=["chapters"])


@router.get("/", response_model=ChapterList)
async def list_chapters(
    service: ChapterSvc,
    user: CurrentUser,
    pagination: Pagination,
):
    items, total = await service.list(
        page=pagination.page, page_size=pagination.page_size
    )
    return ChapterList(
        items=items,
        total=total,
        page=pagination.page,
        page_size=pagination.page_size,
        pages=(total + pagination.page_size - 1) // pagination.page_size,
    )


@router.get("/{chapter_id}", response_model=ChapterRead)
async def get_chapter(
    chapter_id: uuid.UUID,
    service: ChapterSvc,
    user: CurrentUser,
):
    chapter = await service.repo.get_by_id(chapter_id)
    if chapter is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Chapter not found",
        )
    return chapter


@router.post("/", response_model=ChapterRead, status_code=status.HTTP_201_CREATED)
async def create_chapter(
    data: ChapterCreate,
    service: ChapterSvc,
    user: AdminUser,
):
    return await service.create(data)


@router.patch("/{chapter_id}", response_model=ChapterRead)
async def update_chapter(
    chapter_id: uuid.UUID,
    data: ChapterUpdate,
    service: ChapterSvc,
    user: AdminUser,
):
    try:
        return await service.update(chapter_id, data)
    except ValueError:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Chapter not found",
        )


@router.delete("/{chapter_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_chapter(
    chapter_id: uuid.UUID,
    service: ChapterSvc,
    user: AdminUser,
):
    chapter = await service.repo.get_by_id(chapter_id)
    if chapter is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail="Chapter not found",
        )
    await service.repo.delete(chapter)
```

### Custom Exception Handlers

```python
# src/app/exceptions.py
from fastapi import FastAPI, Request, status
from fastapi.responses import JSONResponse
from pydantic import ValidationError
from sqlalchemy.exc import IntegrityError


def register_exception_handlers(app: FastAPI) -> None:
    @app.exception_handler(IntegrityError)
    async def integrity_error_handler(
        request: Request, exc: IntegrityError
    ) -> JSONResponse:
        return JSONResponse(
            status_code=status.HTTP_409_CONFLICT,
            content={"detail": "A record with this data already exists."},
        )

    @app.exception_handler(ValueError)
    async def value_error_handler(
        request: Request, exc: ValueError
    ) -> JSONResponse:
        return JSONResponse(
            status_code=status.HTTP_422_UNPROCESSABLE_ENTITY,
            content={"detail": str(exc)},
        )
```

---

## 9. Background Tasks & Async Patterns

### FastAPI Background Tasks (Simple)

For lightweight, fire-and-forget tasks that do not need retries or persistence:

```python
from fastapi import BackgroundTasks


async def send_welcome_email(email: str, name: str) -> None:
    """Send a welcome email. Runs after the response is sent."""
    # ... email sending logic
    pass


@router.post("/api/v1/visitors/", response_model=VisitorRead, status_code=201)
async def register_visitor(
    data: VisitorCreate,
    service: VisitorSvc,
    background_tasks: BackgroundTasks,
):
    visitor = await service.register(data)
    background_tasks.add_task(send_welcome_email, visitor.email, visitor.name)
    return visitor
```

**When to use `BackgroundTasks`:**
- Email notifications
- Logging/analytics events
- Cache warming
- Non-critical side effects

**When NOT to use `BackgroundTasks`:**
- Tasks that must survive server restarts
- Tasks that need retries on failure
- Tasks that take more than a few seconds
- Tasks that need monitoring/observability

### Celery / ARQ for Persistent Tasks

For tasks that need persistence, retries, and monitoring, use ARQ (async-native) or Celery:

```python
# src/app/core/tasks.py
from arq import create_pool
from arq.connections import RedisSettings

from app.config import settings


async def get_task_pool():
    return await create_pool(
        RedisSettings.from_dsn(settings.redis_url)
    )


# Task definitions
async def sync_visitor_to_crm(ctx, visitor_id: str) -> dict:
    """Sync a visitor record to the external CRM."""
    from app.database import async_session_factory
    from app.visitors.models import Visitor

    async with async_session_factory() as session:
        result = await session.execute(
            select(Visitor).where(Visitor.id == visitor_id)
        )
        visitor = result.scalar_one()

        # ... CRM sync logic
        return {"status": "synced", "visitor_id": visitor_id}


# Worker configuration
class WorkerSettings:
    functions = [sync_visitor_to_crm]
    redis_settings = RedisSettings.from_dsn(settings.redis_url)
    max_jobs = 10
    job_timeout = 300  # 5 minutes
    retry_jobs = True
    max_tries = 3
```

### Enqueuing Tasks

```python
# In service layer
from app.core.tasks import get_task_pool


async def register_visitor(db: AsyncSession, data: VisitorCreate) -> Visitor:
    visitor = Visitor(**data.model_dump())
    db.add(visitor)
    await db.flush()

    # Enqueue CRM sync
    pool = await get_task_pool()
    await pool.enqueue_job("sync_visitor_to_crm", str(visitor.id))

    return visitor
```

### Async Patterns

**Never block the event loop:**

```python
# BAD — blocks the event loop
import time
time.sleep(5)

# GOOD — async sleep
import asyncio
await asyncio.sleep(5)

# BAD — synchronous file I/O
with open("file.txt") as f:
    data = f.read()

# GOOD — use asyncio.to_thread for CPU/IO-bound sync work
import asyncio
data = await asyncio.to_thread(read_large_file, "file.txt")

# BAD — synchronous HTTP call
import requests
response = requests.get("https://api.example.com")

# GOOD — async HTTP client
import httpx
async with httpx.AsyncClient() as client:
    response = await client.get("https://api.example.com")
```

### WebSocket Pattern

```python
# src/app/notifications/router.py
from fastapi import APIRouter, WebSocket, WebSocketDisconnect, Depends

from app.dependencies import get_current_user_ws

router = APIRouter()


class ConnectionManager:
    def __init__(self):
        self.active_connections: dict[str, list[WebSocket]] = {}

    async def connect(self, websocket: WebSocket, user_id: str):
        await websocket.accept()
        if user_id not in self.active_connections:
            self.active_connections[user_id] = []
        self.active_connections[user_id].append(websocket)

    def disconnect(self, websocket: WebSocket, user_id: str):
        self.active_connections[user_id].remove(websocket)
        if not self.active_connections[user_id]:
            del self.active_connections[user_id]

    async def send_to_user(self, user_id: str, message: dict):
        if user_id in self.active_connections:
            for connection in self.active_connections[user_id]:
                await connection.send_json(message)


manager = ConnectionManager()


@router.websocket("/ws/notifications")
async def websocket_endpoint(
    websocket: WebSocket,
    token: str,
):
    user = await get_current_user_ws(token)
    if user is None:
        await websocket.close(code=4001)
        return

    await manager.connect(websocket, str(user.id))
    try:
        while True:
            data = await websocket.receive_json()
            # Handle incoming messages
    except WebSocketDisconnect:
        manager.disconnect(websocket, str(user.id))
```

---

## 10. Middleware Patterns

### Custom Middleware

```python
# src/app/middleware.py
import time
import uuid
from collections.abc import Callable

from starlette.middleware.base import BaseHTTPMiddleware
from starlette.requests import Request
from starlette.responses import Response


class TimingMiddleware(BaseHTTPMiddleware):
    """Add X-Process-Time header to every response."""

    async def dispatch(self, request: Request, call_next: Callable) -> Response:
        start_time = time.perf_counter()
        response = await call_next(request)
        process_time = time.perf_counter() - start_time
        response.headers["X-Process-Time"] = f"{process_time:.4f}"
        return response


class CorrelationIdMiddleware(BaseHTTPMiddleware):
    """Assign a correlation ID to every request for tracing."""

    async def dispatch(self, request: Request, call_next: Callable) -> Response:
        correlation_id = request.headers.get(
            "X-Correlation-ID", str(uuid.uuid4())
        )
        request.state.correlation_id = correlation_id
        response = await call_next(request)
        response.headers["X-Correlation-ID"] = correlation_id
        return response


class SecurityHeadersMiddleware(BaseHTTPMiddleware):
    """Add security headers to every response."""

    async def dispatch(self, request: Request, call_next: Callable) -> Response:
        response = await call_next(request)
        response.headers["Strict-Transport-Security"] = (
            "max-age=31536000; includeSubDomains"
        )
        response.headers["X-Content-Type-Options"] = "nosniff"
        response.headers["X-Frame-Options"] = "DENY"
        response.headers["Referrer-Policy"] = "strict-origin-when-cross-origin"
        response.headers["Content-Security-Policy"] = (
            "default-src 'self'; "
            "script-src 'self'; "
            "style-src 'self' 'unsafe-inline'; "
            "img-src 'self' data:; "
            "font-src 'self'; "
            "frame-ancestors 'none'"
        )
        response.headers["Permissions-Policy"] = (
            "camera=(), microphone=(), geolocation=()"
        )
        return response
```

### Pure ASGI Middleware (Higher Performance)

For performance-critical paths, use pure ASGI middleware instead of `BaseHTTPMiddleware` (which buffers the entire response body):

```python
# src/app/middleware.py
from starlette.types import ASGIApp, Receive, Scope, Send


class PureTimingMiddleware:
    """ASGI middleware — no response body buffering."""

    def __init__(self, app: ASGIApp):
        self.app = app

    async def __call__(self, scope: Scope, receive: Receive, send: Send):
        if scope["type"] != "http":
            await self.app(scope, receive, send)
            return

        start = time.perf_counter()

        async def send_with_timing(message):
            if message["type"] == "http.response.start":
                headers = dict(message.get("headers", []))
                process_time = time.perf_counter() - start
                headers[b"x-process-time"] = f"{process_time:.4f}".encode()
                message["headers"] = list(headers.items())
            await send(message)

        await self.app(scope, receive, send_with_timing)
```

### Rate Limiting

```python
# src/app/middleware.py
from slowapi import Limiter, _rate_limit_exceeded_handler
from slowapi.errors import RateLimitExceeded
from slowapi.util import get_remote_address

limiter = Limiter(key_func=get_remote_address)


# In create_app():
def create_app() -> FastAPI:
    app = FastAPI(...)
    app.state.limiter = limiter
    app.add_exception_handler(RateLimitExceeded, _rate_limit_exceeded_handler)
    ...

# In routers:
@router.post("/api/v1/auth/token")
@limiter.limit("5/minute")
async def login(request: Request, ...):
    ...
```

---

## 11. Development Workflow

### Feature Development Cycle (FastAPI-specific)

```
1. Design schemas (Pydantic models in schemas.py)
2. Write failing tests (pytest + httpx)
3. Write models (SQLAlchemy/SQLModel)
4. Generate migration (alembic revision --autogenerate)
5. Write repository (database access layer)
6. Write service (business logic layer)
7. Write router (HTTP endpoints)
8. Run: pytest
9. Run: ruff check . && ruff format .
10. Refactor while green
```

### Common Commands

```bash
# Development
uvicorn app.main:app --reload --factory      # Start dev server (uses create_app)
uvicorn app.main:app --reload --port 8000    # Explicit port

# Testing
pytest                                        # Run all tests
pytest tests/chapters/                        # Run specific module tests
pytest -k "test_create"                       # Run tests matching pattern
pytest --cov --cov-report=html                # Coverage with HTML report
pytest -x                                     # Stop on first failure

# Database
alembic revision --autogenerate -m "desc"     # Generate migration
alembic upgrade head                          # Apply all migrations
alembic downgrade -1                          # Rollback one (dev only)
alembic history                               # Show migration history

# Quality
ruff check .                                  # Lint
ruff check . --fix                            # Lint with auto-fix
ruff format .                                 # Format
ruff format . --check                         # Format check (CI)
mypy src/                                     # Type checking

# Dependencies
pip install -e ".[dev]"                       # Install with dev deps
pip-compile pyproject.toml                    # Lock dependencies
```

### pyproject.toml Configuration

```toml
[project]
name = "app"
version = "1.0.0"
requires-python = ">=3.12"
dependencies = [
    "fastapi~=0.115.0",
    "uvicorn[standard]~=0.34.0",
    "pydantic~=2.10.0",
    "pydantic-settings~=2.7.0",
    "sqlalchemy[asyncio]~=2.0.0",
    "sqlmodel~=0.0.22",
    "asyncpg~=0.30.0",
    "alembic~=1.14.0",
    "python-jose[cryptography]~=3.3.0",
    "passlib[bcrypt]~=1.7.0",
    "python-multipart~=0.0.20",
    "httpx~=0.28.0",
]

[project.optional-dependencies]
dev = [
    "pytest~=8.3.0",
    "pytest-asyncio~=0.25.0",
    "pytest-cov~=6.0.0",
    "factory-boy~=3.3.0",
    "ruff~=0.9.0",
    "mypy~=1.14.0",
]

[tool.ruff]
target-version = "py312"
line-length = 88

[tool.ruff.lint]
select = [
    "E",    # pycodestyle errors
    "W",    # pycodestyle warnings
    "F",    # pyflakes
    "I",    # isort
    "UP",   # pyupgrade
    "B",    # flake8-bugbear
    "SIM",  # flake8-simplify
    "RUF",  # ruff-specific
]
ignore = ["E501"]  # line length handled by formatter

[tool.ruff.lint.isort]
known-first-party = ["app"]
```

---

## 12. Deployment

### Docker + Uvicorn

Production deployment uses a multi-stage Docker build with Uvicorn:

```dockerfile
# Dockerfile
FROM python:3.12-slim AS builder

WORKDIR /build
COPY pyproject.toml .
RUN pip install --no-cache-dir --prefix=/install .

FROM python:3.12-slim AS runtime

# Security: non-root user
RUN useradd --create-home --shell /bin/bash appuser

WORKDIR /app
COPY --from=builder /install /usr/local
COPY src/ ./src/
COPY alembic/ ./alembic/
COPY alembic.ini .
COPY scripts/ ./scripts/

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD python scripts/healthcheck.py

USER appuser
EXPOSE 8000

CMD ["uvicorn", "app.main:create_app", \
     "--host", "0.0.0.0", \
     "--port", "8000", \
     "--factory", \
     "--workers", "4", \
     "--proxy-headers", \
     "--forwarded-allow-ips", "*", \
     "--access-log"]
```

### docker-compose.yml

```yaml
# docker-compose.yml
services:
  app:
    build: .
    ports:
      - "8000:8000"
    environment:
      - DATABASE_URL=postgresql+asyncpg://postgres:postgres@db:5432/app
      - SECRET_KEY=${SECRET_KEY}
      - REDIS_URL=redis://redis:6379/0
      - ENVIRONMENT=production
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_healthy

  db:
    image: postgres:16-alpine
    environment:
      POSTGRES_DB: app
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 5s
      timeout: 5s
      retries: 5

  worker:
    build: .
    command: arq app.core.tasks.WorkerSettings
    environment:
      - DATABASE_URL=postgresql+asyncpg://postgres:postgres@db:5432/app
      - REDIS_URL=redis://redis:6379/0
    depends_on:
      - db
      - redis

volumes:
  postgres_data:
```

### Health Check Script

```python
# scripts/healthcheck.py
import sys
import httpx

try:
    response = httpx.get("http://localhost:8000/health", timeout=5.0)
    sys.exit(0 if response.status_code == 200 else 1)
except Exception:
    sys.exit(1)
```

### Health Check Endpoint

```python
# In main.py or a dedicated router
@app.get("/health")
async def health_check():
    return {"status": "healthy"}


@app.get("/health/ready")
async def readiness_check(db: DbSession):
    """Check database connectivity."""
    try:
        await db.execute(text("SELECT 1"))
        return {"status": "ready"}
    except Exception:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Database not available",
        )
```

### CI/CD Pipeline (GitHub Actions)

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16-alpine
        env:
          POSTGRES_DB: app_test
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - run: pip install -e ".[dev]"
      - run: ruff check .
      - run: ruff format . --check
      - run: mypy src/
      - run: pytest --cov --cov-fail-under=100
        env:
          DATABASE_URL: postgresql+asyncpg://postgres:postgres@localhost:5432/app_test
          SECRET_KEY: ci-test-secret
          ENVIRONMENT: test

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      # Deploy steps (fly.io, AWS ECS, GCP Cloud Run, etc.)
```

### Uvicorn Production Configuration

| Setting | Value | Rationale |
|---|---|---|
| `--workers` | `2 * CPU + 1` | Uvicorn workers (each runs its own event loop) |
| `--proxy-headers` | enabled | Trust X-Forwarded-For from reverse proxy |
| `--forwarded-allow-ips` | `*` or proxy IP | Which proxies to trust |
| `--access-log` | enabled | Request logging in production |
| `--limit-concurrency` | 100-1000 | Prevent overload |
| `--timeout-keep-alive` | 5 | Close idle keep-alive connections |

For high-traffic deployments, run Gunicorn with Uvicorn workers:

```bash
gunicorn app.main:create_app \
    --factory \
    --worker-class uvicorn.workers.UvicornWorker \
    --workers 4 \
    --bind 0.0.0.0:8000 \
    --access-logfile - \
    --error-logfile - \
    --timeout 120
```

---

## 13. Security

### Security Headers

Applied via middleware (see Section 10). Every response includes:

| Header | Value | Purpose |
|---|---|---|
| `Strict-Transport-Security` | `max-age=31536000; includeSubDomains` | Force HTTPS for 1 year |
| `X-Content-Type-Options` | `nosniff` | Prevent MIME type sniffing |
| `X-Frame-Options` | `DENY` | Prevent clickjacking |
| `Referrer-Policy` | `strict-origin-when-cross-origin` | Control referrer information |
| `Content-Security-Policy` | `default-src 'self'; ...` | Prevent XSS, data injection |
| `Permissions-Policy` | `camera=(), microphone=(), geolocation=()` | Restrict browser features |

### CORS Configuration

```python
from fastapi.middleware.cors import CORSMiddleware

app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.cors_origins,  # Explicit list, never ["*"] in production
    allow_credentials=True,
    allow_methods=["GET", "POST", "PUT", "PATCH", "DELETE"],
    allow_headers=["*"],
    expose_headers=["X-Correlation-ID", "X-Process-Time"],
)
```

**Never use `allow_origins=["*"]` in production.** Explicit origins only.

### Input Validation Security

Pydantic v2 handles input validation, but additional security measures:

```python
# Prevent mass assignment — always use explicit schemas
class UserUpdate(BaseModel):
    name: str | None = None
    email: str | None = None
    # role is NOT here — cannot be updated via this endpoint


# SQL injection prevention — always use parameterized queries
# BAD
result = await session.execute(text(f"SELECT * FROM users WHERE email = '{email}'"))

# GOOD
result = await session.execute(
    select(User).where(User.email == email)
)
```

### Password and Token Security

```python
# Password requirements
class UserCreate(BaseModel):
    email: str = Field(max_length=255)
    password: str = Field(min_length=8, max_length=128)

    @field_validator("password")
    @classmethod
    def password_strength(cls, v: str) -> str:
        if not any(c.isupper() for c in v):
            raise ValueError("Password must contain at least one uppercase letter")
        if not any(c.isdigit() for c in v):
            raise ValueError("Password must contain at least one digit")
        return v


# Token configuration
ACCESS_TOKEN_EXPIRE_MINUTES = 30      # Short-lived access tokens
REFRESH_TOKEN_EXPIRE_DAYS = 7         # Longer-lived refresh tokens
ALGORITHM = "HS256"                    # Or RS256 for asymmetric
```

### Rate Limiting for Auth Endpoints

Authentication endpoints must be rate-limited to prevent brute force:

```python
@router.post("/api/v1/auth/token")
@limiter.limit("5/minute")
async def login(request: Request, ...):
    ...

@router.post("/api/v1/auth/register")
@limiter.limit("3/minute")
async def register(request: Request, ...):
    ...

@router.post("/api/v1/auth/forgot-password")
@limiter.limit("3/hour")
async def forgot_password(request: Request, ...):
    ...
```

### Dependency Security Scanning

```bash
# In CI pipeline
pip-audit                              # Check for known vulnerabilities
safety check                           # Alternative vulnerability scanner
```

---

## 14. Coverage Enforcement

Test coverage is enforced via `pytest-cov`:

```toml
# pyproject.toml
[tool.coverage.run]
source = ["src/app"]
branch = true
omit = [
    "*/migrations/*",
    "*/tests/*",
    "src/app/main.py",
    "alembic/*",
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
    "if settings.debug:",
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

For FastAPI projects that serve HTML forms (via Jinja2 templates or a frontend framework), all forms must pass the 9-dimension audit from `FORM_PATTERNS.md`:

| Dimension | Key Requirements |
|-----------|-----------------|
| **layout** | Single column, logical grouping with `<fieldset>` + `<legend>` |
| **labels** | Top-aligned, visible `<label>`, optional fields marked "(optional)" |
| **validation** | Server-side via Pydantic, client-side validation as progressive enhancement only |
| **errors** | Structured error responses with field-level detail via Pydantic's `ValidationError` |
| **accessibility** | `novalidate` on form, `autocomplete` attributes, `aria-live` on error summary |
| **mobile** | `type="tel"` / `type="email"`, min 48px touch targets, `autocomplete` |
| **cta** | Outcome-focused text ("Reserve My Free Visit" not "Submit"), loading states |
| **trust** | Minimal fields, "(optional)" markers, post-submit clarity |
| **performance** | Debounce search/autocomplete inputs, pagination on list endpoints |

### API Error Response Pattern

FastAPI returns structured validation errors by default. Extend for consistency:

```python
# Pydantic validation errors return:
{
    "detail": [
        {
            "type": "string_too_short",
            "loc": ["body", "name"],
            "msg": "String should have at least 1 character",
            "input": "",
            "ctx": {"min_length": 1}
        }
    ]
}

# Custom business logic errors should follow the same pattern:
from fastapi import HTTPException

raise HTTPException(
    status_code=422,
    detail=[
        {
            "type": "duplicate_slug",
            "loc": ["body", "slug"],
            "msg": "A chapter with this slug already exists",
        }
    ],
)
```

### Jinja2 Template Form Pattern (When Serving HTML)

```python
# src/app/main.py
from fastapi.templating import Jinja2Templates

templates = Jinja2Templates(directory="templates")
```

```html
<!-- templates/visitors/register.html -->
<form method="post"
      action="/visitors/register"
      novalidate>
  {% if errors %}
  <div role="alert" aria-live="assertive" class="bg-red-50 border border-red-200 rounded p-4 mb-4">
    <h3 class="text-red-800 font-semibold">Please fix the following errors:</h3>
    <ul class="list-disc pl-5 text-red-700">
      {% for error in errors %}
        <li>{{ error.msg }}</li>
      {% endfor %}
    </ul>
  </div>
  {% endif %}

  <fieldset>
    <legend class="text-sm font-semibold mb-4">Your Information</legend>

    <div class="mb-4">
      <label for="name" class="block text-sm font-medium mb-1">Full Name</label>
      <input type="text" id="name" name="name" required
             autocomplete="name" class="input input-bordered w-full" />
    </div>

    <div class="mb-4">
      <label for="email" class="block text-sm font-medium mb-1">Email</label>
      <input type="email" id="email" name="email" required
             autocomplete="email" class="input input-bordered w-full" />
    </div>

    <div class="mb-4">
      <label for="phone" class="block text-sm font-medium mb-1">Phone</label>
      <input type="tel" id="phone" name="phone" required
             autocomplete="tel" class="input input-bordered w-full" />
    </div>
  </fieldset>

  <button type="submit" class="btn btn-primary h-12">
    Reserve My Free Visit
  </button>
</form>
```

---

## 16. Anti-Patterns (FastAPI-specific)

| Anti-Pattern | Do This Instead |
|---|---|
| Using the database model as request/response schema | Separate `*Create`, `*Update`, `*Read` Pydantic schemas |
| `response_model=None` or returning raw dicts | Always declare `response_model` with a Pydantic schema |
| Synchronous database calls in async endpoints | Use `async` SQLAlchemy with `asyncpg` driver |
| `@app.on_event("startup")` / `@app.on_event("shutdown")` | Use `lifespan` context manager |
| Blocking the event loop with `time.sleep()` or `requests.get()` | Use `asyncio.sleep()`, `httpx.AsyncClient`, or `asyncio.to_thread()` |
| Business logic in router functions | Move to service layer; routers handle HTTP, services handle logic |
| `select(Model).all()` without pagination | Always paginate with `offset`/`limit` |
| N+1 queries (accessing relationships in loops) | Use `selectinload()`, `joinedload()` eager loading options |
| Hardcoding secrets in config files | Use `pydantic-settings` with environment variables |
| `allow_origins=["*"]` in CORS | Explicit origin list in production |
| Global mutable state (e.g., `connections = []`) | Use dependency injection or `app.state` |
| Missing `expire_on_commit=False` on async sessions | Always set `expire_on_commit=False` to avoid lazy-load failures |
| `Depends()` without `Annotated[]` | Use `Annotated[Type, Depends(dep)]` for clean signatures |
| Catching bare `Exception` in endpoints | Catch specific exceptions; let FastAPI handle the rest |
| Using `@validator` (Pydantic v1 syntax) | Use `@field_validator` or `@model_validator` (Pydantic v2) |
| Missing `from_attributes=True` on response models | Always set `model_config = ConfigDict(from_attributes=True)` for ORM models |
| Editing committed Alembic migrations | Write a new corrective migration instead |
| Using `BaseHTTPMiddleware` for performance-critical paths | Use pure ASGI middleware to avoid response body buffering |
| `print()` for debugging/logging | Use Python `logging` module with `structlog` or standard formatters |
| Returning 200 for creation endpoints | Return `status_code=201` for resource creation |
| Missing rate limiting on auth endpoints | Rate-limit `/token`, `/register`, `/forgot-password` |
| `pytest` without `asyncio_mode = "auto"` | Set `asyncio_mode = "auto"` in pyproject.toml for seamless async tests |
| Exposing `/docs` and `/redoc` in production | Set `docs_url=None, redoc_url=None` in production |
| Mutable default arguments in background tasks | Pass IDs, not model instances, to background/worker tasks |
| Missing health check endpoint | Always expose `/health` and `/health/ready` for container orchestration |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a FastAPI patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:fastapi&title=[FastAPI]%20)**

Use the `patterns:fastapi` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
