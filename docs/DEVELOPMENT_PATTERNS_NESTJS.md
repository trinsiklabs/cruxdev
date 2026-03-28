# Development Patterns — NestJS Stack

NestJS / TypeScript / TypeORM / Prisma / Drizzle / Passport.js / Jest / Bull / Swagger

This document captures stack-specific patterns, conventions, and decisions for NestJS stack projects (NestJS/TypeScript/TypeORM or Prisma or Drizzle/Passport.js/Jest/Bull/Swagger). It complements `DEVELOPMENT_PATTERNS.md` (methodology, planning, audit cycles) with the **how** of building in this specific stack.

**Relationship to other files:**
- **DEVELOPMENT_PATTERNS.md** — the methodology authority. Planning cycles, audit patterns, the user's prompt toolkit, anti-patterns. Stack-agnostic.
- **DEVELOPMENT_PATTERNS_CRUXDEV.md** — the autonomous convergence methodology. Lights-out execution model.
- **FORM_PATTERNS.md** — form design standards. All forms must pass the 9-dimension audit.
- **WEBSITE_PLANNING.md** — website standards. SEO, accessibility, performance, security.
- **This file** — stack-specific patterns. How we structure NestJS modules, test with Jest, handle authentication with Passport.js, manage data with TypeORM/Prisma/Drizzle, build microservices, process queues with Bull, document APIs with Swagger, etc.
- **Build plan files** (`BUILD_PLAN_NNN_*.md`) — per-slice actionable plans with checkboxes.

---

## 1. Stack & Versions

Pinned to what's installed on the development machine. These are the versions we build and test against.

| Component | Version | Notes |
|---|---|---|
| Node.js | 22+ LTS | Minimum 22 for native fetch, Web Crypto, test runner |
| NestJS | 11+ | Modules, controllers, providers, guards, pipes, interceptors |
| TypeScript | 5.6+ | `satisfies`, `const` type parameters, `NoInfer`, decorators |
| Jest | 30+ | Unit + integration testing, `@nestjs/testing` utilities |
| TypeORM | 0.3+ | Decorator-based ORM, migrations, repository pattern |
| Prisma | 6+ | Type-safe ORM alternative, Prisma Client, migrations |
| Drizzle ORM | 0.38+ | SQL-like type-safe ORM alternative, zero abstraction overhead |
| Passport.js | 0.7+ | Authentication strategies via `@nestjs/passport` |
| JWT | — | `@nestjs/jwt` for token signing/verification |
| Bull | 5+ | Redis-backed job queues via `@nestjs/bullmq` |
| Swagger | — | `@nestjs/swagger` for OpenAPI 3.1 spec generation |
| class-validator | 0.14+ | DTO validation via decorators |
| class-transformer | 0.5+ | DTO transformation, serialization |
| PostgreSQL | 15+ | Primary database, running locally |
| Redis | 7+ | Session store, Bull queue backend, caching |
| Docker | 24+ | Container builds, multi-stage Dockerfiles |

### Version Constraint Policy

Use caret (`^`) constraints in `package.json` pinned to the minor version:

```json
{
  "dependencies": {
    "@nestjs/core": "^11.0.0",
    "@nestjs/common": "^11.0.0",
    "@nestjs/platform-express": "^11.0.0",
    "typeorm": "^0.3.20",
    "@nestjs/swagger": "^8.1.0",
    "class-validator": "^0.14.1",
    "class-transformer": "^0.5.1"
  }
}
```

```json
// Bad — too loose, allows any major
"@nestjs/core": "*"

// Bad — too tight, blocks patch fixes
"@nestjs/core": "11.0.3"

// Good — allows patch and minor updates within major
"@nestjs/core": "^11.0.0"
```

Exception: for pre-release packages or packages with known instability, pin exact.

### TypeScript Configuration

```json
// tsconfig.json
{
  "compilerOptions": {
    "module": "commonjs",
    "target": "ES2022",
    "lib": ["ES2022"],
    "declaration": true,
    "removeComments": true,
    "emitDecoratorMetadata": true,
    "experimentalDecorators": true,
    "allowSyntheticDefaultImports": true,
    "sourceMap": true,
    "outDir": "./dist",
    "baseUrl": "./",
    "incremental": true,
    "skipLibCheck": true,
    "strictNullChecks": true,
    "noImplicitAny": true,
    "strictBindCallApply": true,
    "forceConsistentCasingInFileNames": true,
    "noFallthroughCasesInSwitch": true,
    "strict": true,
    "paths": {
      "@app/*": ["src/*"],
      "@config/*": ["src/config/*"],
      "@modules/*": ["src/modules/*"],
      "@common/*": ["src/common/*"]
    }
  }
}
```

**Critical:** `emitDecoratorMetadata` and `experimentalDecorators` must be `true` — NestJS relies on reflection metadata for dependency injection. Without these, `@Injectable()`, `@Controller()`, and all other decorators silently fail.

---

## 2. Project Structure

### Module-Based Organization

NestJS enforces modular architecture. Each feature is a self-contained module with its own controllers, services, entities, DTOs, and tests:

```
src/
├── app.module.ts              # Root module — imports all feature modules
├── main.ts                    # Bootstrap, global pipes, Swagger setup
├── config/                    # Configuration
│   ├── app.config.ts          # Typed config using @nestjs/config
│   ├── database.config.ts     # TypeORM/Prisma/Drizzle connection config
│   ├── auth.config.ts         # JWT secrets, strategy options
│   ├── queue.config.ts        # Bull/Redis connection config
│   └── swagger.config.ts      # OpenAPI metadata
├── common/                    # Shared code (no feature logic)
│   ├── decorators/            # Custom decorators (@CurrentUser, @Roles, @Public)
│   │   ├── current-user.decorator.ts
│   │   ├── roles.decorator.ts
│   │   └── public.decorator.ts
│   ├── filters/               # Exception filters
│   │   ├── http-exception.filter.ts
│   │   └── all-exceptions.filter.ts
│   ├── guards/                # Auth/role guards
│   │   ├── jwt-auth.guard.ts
│   │   └── roles.guard.ts
│   ├── interceptors/          # Logging, transform, timeout
│   │   ├── logging.interceptor.ts
│   │   ├── transform.interceptor.ts
│   │   └── timeout.interceptor.ts
│   ├── pipes/                 # Validation, parse pipes
│   │   └── validation.pipe.ts
│   ├── middleware/             # Request middleware
│   │   ├── correlation-id.middleware.ts
│   │   └── request-logger.middleware.ts
│   ├── interfaces/            # Shared TypeScript interfaces
│   │   └── paginated-result.interface.ts
│   └── dto/                   # Shared DTOs
│       ├── pagination.dto.ts
│       └── api-response.dto.ts
├── modules/                   # Feature modules
│   ├── auth/
│   │   ├── auth.module.ts
│   │   ├── auth.controller.ts
│   │   ├── auth.service.ts
│   │   ├── auth.controller.spec.ts
│   │   ├── auth.service.spec.ts
│   │   ├── strategies/
│   │   │   ├── jwt.strategy.ts
│   │   │   ├── local.strategy.ts
│   │   │   └── refresh-token.strategy.ts
│   │   ├── guards/
│   │   │   └── jwt-refresh.guard.ts
│   │   └── dto/
│   │       ├── login.dto.ts
│   │       ├── register.dto.ts
│   │       └── token-response.dto.ts
│   ├── users/
│   │   ├── users.module.ts
│   │   ├── users.controller.ts
│   │   ├── users.service.ts
│   │   ├── users.controller.spec.ts
│   │   ├── users.service.spec.ts
│   │   ├── entities/
│   │   │   └── user.entity.ts
│   │   └── dto/
│   │       ├── create-user.dto.ts
│   │       ├── update-user.dto.ts
│   │       └── user-response.dto.ts
│   ├── orders/
│   │   ├── orders.module.ts
│   │   ├── orders.controller.ts
│   │   ├── orders.service.ts
│   │   ├── orders.controller.spec.ts
│   │   ├── orders.service.spec.ts
│   │   ├── entities/
│   │   │   ├── order.entity.ts
│   │   │   └── order-item.entity.ts
│   │   ├── dto/
│   │   │   ├── create-order.dto.ts
│   │   │   └── order-response.dto.ts
│   │   ├── processors/
│   │   │   └── order-fulfillment.processor.ts
│   │   └── events/
│   │       └── order-created.event.ts
│   └── notifications/
│       ├── notifications.module.ts
│       ├── notifications.service.ts
│       ├── notifications.gateway.ts       # WebSocket gateway
│       └── dto/
│           └── notification.dto.ts
├── database/                  # Database configuration and migrations
│   ├── migrations/            # TypeORM migrations
│   ├── seeds/                 # Seed data scripts
│   └── data-source.ts         # TypeORM DataSource for CLI
└── prisma/                    # If using Prisma instead
    ├── schema.prisma
    ├── migrations/
    └── seed.ts
```

**Convention:** One module per bounded context. The module file imports and exports everything the feature needs. Other modules interact via the exported service, never by importing internal components directly.

**Convention:** Co-locate tests with source files. `users.service.spec.ts` lives next to `users.service.ts`. Integration/e2e tests go in a top-level `test/` directory.

### Test Structure

```
test/                          # E2E / integration tests
├── app.e2e-spec.ts            # Full app integration tests
├── auth.e2e-spec.ts           # Auth flow e2e tests
├── users.e2e-spec.ts          # Users CRUD e2e tests
├── orders.e2e-spec.ts         # Orders e2e tests
└── jest-e2e.json              # Jest config for e2e tests

src/modules/users/             # Unit tests co-located
├── users.service.spec.ts      # Service unit tests
├── users.controller.spec.ts   # Controller unit tests
```

---

## 3. Modules, Controllers & Providers

### Module Template

Every feature module follows this structure:

```typescript
import { Module } from '@nestjs/common';
import { TypeOrmModule } from '@nestjs/typeorm';
import { UsersController } from './users.controller';
import { UsersService } from './users.service';
import { User } from './entities/user.entity';

@Module({
  imports: [TypeOrmModule.forFeature([User])],
  controllers: [UsersController],
  providers: [UsersService],
  exports: [UsersService], // Only export what other modules need
})
export class UsersModule {}
```

**Conventions:**
- Every feature module explicitly declares its imports, controllers, providers, and exports.
- Only export services that other modules need. Keep internal providers private.
- Use `forFeature()` to register entities/repositories scoped to the module.
- Never import a module's internal service directly — always import the module itself.

### Dynamic Module Pattern

For configurable modules (database connections, third-party integrations):

```typescript
import { Module, DynamicModule } from '@nestjs/common';

@Module({})
export class MailModule {
  static forRoot(options: MailModuleOptions): DynamicModule {
    return {
      module: MailModule,
      global: true, // Available everywhere without re-importing
      providers: [
        {
          provide: MAIL_OPTIONS,
          useValue: options,
        },
        MailService,
      ],
      exports: [MailService],
    };
  }

  static forRootAsync(options: MailModuleAsyncOptions): DynamicModule {
    return {
      module: MailModule,
      global: true,
      imports: options.imports || [],
      providers: [
        {
          provide: MAIL_OPTIONS,
          useFactory: options.useFactory,
          inject: options.inject || [],
        },
        MailService,
      ],
      exports: [MailService],
    };
  }
}
```

### Controller Template

```typescript
import {
  Controller,
  Get,
  Post,
  Put,
  Delete,
  Body,
  Param,
  Query,
  ParseUUIDPipe,
  HttpCode,
  HttpStatus,
} from '@nestjs/common';
import { ApiTags, ApiOperation, ApiResponse, ApiBearerAuth } from '@nestjs/swagger';
import { UsersService } from './users.service';
import { CreateUserDto } from './dto/create-user.dto';
import { UpdateUserDto } from './dto/update-user.dto';
import { UserResponseDto } from './dto/user-response.dto';
import { PaginationDto } from '@common/dto/pagination.dto';
import { Roles } from '@common/decorators/roles.decorator';
import { Role } from '@common/enums/role.enum';

@ApiTags('users')
@ApiBearerAuth()
@Controller('users')
export class UsersController {
  constructor(private readonly usersService: UsersService) {}

  @Post()
  @Roles(Role.Admin)
  @ApiOperation({ summary: 'Create a new user' })
  @ApiResponse({ status: 201, type: UserResponseDto })
  async create(@Body() createUserDto: CreateUserDto): Promise<UserResponseDto> {
    return this.usersService.create(createUserDto);
  }

  @Get()
  @ApiOperation({ summary: 'List all users with pagination' })
  @ApiResponse({ status: 200, type: [UserResponseDto] })
  async findAll(@Query() pagination: PaginationDto): Promise<UserResponseDto[]> {
    return this.usersService.findAll(pagination);
  }

  @Get(':id')
  @ApiOperation({ summary: 'Get a user by ID' })
  @ApiResponse({ status: 200, type: UserResponseDto })
  @ApiResponse({ status: 404, description: 'User not found' })
  async findOne(@Param('id', ParseUUIDPipe) id: string): Promise<UserResponseDto> {
    return this.usersService.findOne(id);
  }

  @Put(':id')
  @ApiOperation({ summary: 'Update a user' })
  @ApiResponse({ status: 200, type: UserResponseDto })
  async update(
    @Param('id', ParseUUIDPipe) id: string,
    @Body() updateUserDto: UpdateUserDto,
  ): Promise<UserResponseDto> {
    return this.usersService.update(id, updateUserDto);
  }

  @Delete(':id')
  @Roles(Role.Admin)
  @HttpCode(HttpStatus.NO_CONTENT)
  @ApiOperation({ summary: 'Delete a user' })
  @ApiResponse({ status: 204, description: 'User deleted' })
  async remove(@Param('id', ParseUUIDPipe) id: string): Promise<void> {
    return this.usersService.remove(id);
  }
}
```

**Conventions:**
- Controllers are thin — they validate input (via DTOs + pipes), delegate to the service, and return the response. No business logic in controllers.
- Always use `ParseUUIDPipe` for UUID path parameters.
- Always decorate with `@ApiTags`, `@ApiOperation`, and `@ApiResponse` for Swagger documentation.
- Use `@HttpCode()` to override default status codes where appropriate (e.g., 204 for DELETE).
- Return DTO response types, not raw entities. Use `class-transformer` `@Exclude()` / `plainToInstance()` to strip internal fields.

### Service (Provider) Template

```typescript
import { Injectable, NotFoundException, ConflictException } from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { User } from './entities/user.entity';
import { CreateUserDto } from './dto/create-user.dto';
import { UpdateUserDto } from './dto/update-user.dto';
import { PaginationDto } from '@common/dto/pagination.dto';

@Injectable()
export class UsersService {
  constructor(
    @InjectRepository(User)
    private readonly usersRepository: Repository<User>,
  ) {}

  async create(createUserDto: CreateUserDto): Promise<User> {
    const existing = await this.usersRepository.findOne({
      where: { email: createUserDto.email },
    });
    if (existing) {
      throw new ConflictException('Email already registered');
    }

    const user = this.usersRepository.create(createUserDto);
    return this.usersRepository.save(user);
  }

  async findAll(pagination: PaginationDto): Promise<User[]> {
    const { page = 1, limit = 20 } = pagination;
    return this.usersRepository.find({
      skip: (page - 1) * limit,
      take: limit,
      order: { createdAt: 'DESC' },
    });
  }

  async findOne(id: string): Promise<User> {
    const user = await this.usersRepository.findOne({ where: { id } });
    if (!user) {
      throw new NotFoundException(`User with ID "${id}" not found`);
    }
    return user;
  }

  async update(id: string, updateUserDto: UpdateUserDto): Promise<User> {
    const user = await this.findOne(id);
    Object.assign(user, updateUserDto);
    return this.usersRepository.save(user);
  }

  async remove(id: string): Promise<void> {
    const user = await this.findOne(id);
    await this.usersRepository.remove(user);
  }
}
```

**Conventions:**
- Services contain all business logic. They are the single source of truth for a domain's operations.
- Throw NestJS built-in exceptions (`NotFoundException`, `ConflictException`, `BadRequestException`) — the global exception filter handles HTTP response formatting.
- Inject repositories via `@InjectRepository()` for TypeORM. For Prisma, inject `PrismaService`. For Drizzle, inject the Drizzle client.
- Never return raw database errors to the client. Catch and translate to appropriate HTTP exceptions.

### Custom Providers (Factory, Value, Class)

```typescript
// Factory provider — runtime configuration
{
  provide: 'CACHE_MANAGER',
  useFactory: (configService: ConfigService) => {
    return new CacheManager({
      ttl: configService.get<number>('CACHE_TTL'),
      store: configService.get<string>('CACHE_STORE'),
    });
  },
  inject: [ConfigService],
}

// Value provider — constants
{
  provide: 'API_VERSION',
  useValue: 'v1',
}

// Class provider — swap implementations
{
  provide: PaymentService,
  useClass:
    process.env.NODE_ENV === 'test'
      ? MockPaymentService
      : StripePaymentService,
}
```

---

## 4. Guards, Pipes & Interceptors

### Request Lifecycle

Understanding the NestJS request lifecycle is essential for placing logic in the correct layer:

```
Incoming Request
  → Middleware (express/fastify level)
  → Guards (authorization — can this user access this route?)
  → Interceptors (pre-controller — logging, transform, cache)
  → Pipes (validation + transformation of input)
  → Controller method
  → Interceptors (post-controller — response transform, timing)
  → Exception Filters (if error thrown at any stage)
  → Response
```

### Guards

Guards determine whether a request should be handled by the route handler. They run after middleware but before pipes and interceptors.

#### JWT Auth Guard (Global)

```typescript
import { Injectable, ExecutionContext } from '@nestjs/common';
import { AuthGuard } from '@nestjs/passport';
import { Reflector } from '@nestjs/core';
import { IS_PUBLIC_KEY } from '@common/decorators/public.decorator';

@Injectable()
export class JwtAuthGuard extends AuthGuard('jwt') {
  constructor(private reflector: Reflector) {
    super();
  }

  canActivate(context: ExecutionContext) {
    const isPublic = this.reflector.getAllAndOverride<boolean>(IS_PUBLIC_KEY, [
      context.getHandler(),
      context.getClass(),
    ]);
    if (isPublic) {
      return true;
    }
    return super.canActivate(context);
  }
}
```

#### Roles Guard

```typescript
import { Injectable, CanActivate, ExecutionContext } from '@nestjs/common';
import { Reflector } from '@nestjs/core';
import { ROLES_KEY } from '@common/decorators/roles.decorator';
import { Role } from '@common/enums/role.enum';

@Injectable()
export class RolesGuard implements CanActivate {
  constructor(private reflector: Reflector) {}

  canActivate(context: ExecutionContext): boolean {
    const requiredRoles = this.reflector.getAllAndOverride<Role[]>(ROLES_KEY, [
      context.getHandler(),
      context.getClass(),
    ]);
    if (!requiredRoles) {
      return true; // No roles required — allow access
    }
    const { user } = context.switchToHttp().getRequest();
    return requiredRoles.some((role) => user.roles?.includes(role));
  }
}
```

#### Custom Decorators for Guards

```typescript
// @Public() — skip JWT auth
import { SetMetadata } from '@nestjs/common';
export const IS_PUBLIC_KEY = 'isPublic';
export const Public = () => SetMetadata(IS_PUBLIC_KEY, true);

// @Roles() — require specific roles
import { SetMetadata } from '@nestjs/common';
export const ROLES_KEY = 'roles';
export const Roles = (...roles: Role[]) => SetMetadata(ROLES_KEY, roles);

// @CurrentUser() — extract user from request
import { createParamDecorator, ExecutionContext } from '@nestjs/common';
export const CurrentUser = createParamDecorator(
  (data: string | undefined, ctx: ExecutionContext) => {
    const request = ctx.switchToHttp().getRequest();
    if (data) {
      return request.user?.[data];
    }
    return request.user;
  },
);
```

### Pipes

Pipes transform or validate input before it reaches the controller.

#### Global Validation Pipe

```typescript
// main.ts
app.useGlobalPipes(
  new ValidationPipe({
    whitelist: true,           // Strip properties not in DTO
    forbidNonWhitelisted: true, // Throw on unexpected properties
    transform: true,            // Auto-transform to DTO class instances
    transformOptions: {
      enableImplicitConversion: true, // Convert query string params to types
    },
  }),
);
```

**Critical:** `whitelist: true` prevents mass-assignment vulnerabilities. Without it, a client can send `{ "role": "admin" }` and it passes through if your DTO does not explicitly block it. With `forbidNonWhitelisted: true`, sending unknown properties returns a 400 error.

#### Custom Pipe

```typescript
import { PipeTransform, Injectable, BadRequestException } from '@nestjs/common';

@Injectable()
export class ParseDatePipe implements PipeTransform<string, Date> {
  transform(value: string): Date {
    const date = new Date(value);
    if (isNaN(date.getTime())) {
      throw new BadRequestException(`"${value}" is not a valid date`);
    }
    return date;
  }
}

// Usage in controller
@Get('report')
async getReport(@Query('from', ParseDatePipe) from: Date): Promise<Report> {
  return this.reportService.generate(from);
}
```

### Interceptors

Interceptors wrap the route handler execution. They can transform the result, add timing, handle caching, or modify the response.

#### Response Transform Interceptor

```typescript
import {
  Injectable,
  NestInterceptor,
  ExecutionContext,
  CallHandler,
} from '@nestjs/common';
import { Observable } from 'rxjs';
import { map } from 'rxjs/operators';

export interface ApiResponse<T> {
  data: T;
  meta: {
    timestamp: string;
    path: string;
  };
}

@Injectable()
export class TransformInterceptor<T>
  implements NestInterceptor<T, ApiResponse<T>>
{
  intercept(
    context: ExecutionContext,
    next: CallHandler,
  ): Observable<ApiResponse<T>> {
    const request = context.switchToHttp().getRequest();
    return next.handle().pipe(
      map((data) => ({
        data,
        meta: {
          timestamp: new Date().toISOString(),
          path: request.url,
        },
      })),
    );
  }
}
```

#### Logging Interceptor

```typescript
import {
  Injectable,
  NestInterceptor,
  ExecutionContext,
  CallHandler,
  Logger,
} from '@nestjs/common';
import { Observable } from 'rxjs';
import { tap } from 'rxjs/operators';

@Injectable()
export class LoggingInterceptor implements NestInterceptor {
  private readonly logger = new Logger(LoggingInterceptor.name);

  intercept(context: ExecutionContext, next: CallHandler): Observable<unknown> {
    const request = context.switchToHttp().getRequest();
    const { method, url } = request;
    const now = Date.now();

    return next
      .handle()
      .pipe(
        tap(() =>
          this.logger.log(`${method} ${url} — ${Date.now() - now}ms`),
        ),
      );
  }
}
```

#### Timeout Interceptor

```typescript
import {
  Injectable,
  NestInterceptor,
  ExecutionContext,
  CallHandler,
  RequestTimeoutException,
} from '@nestjs/common';
import { Observable, throwError, TimeoutError } from 'rxjs';
import { catchError, timeout } from 'rxjs/operators';

@Injectable()
export class TimeoutInterceptor implements NestInterceptor {
  constructor(private readonly timeoutMs: number = 15000) {}

  intercept(context: ExecutionContext, next: CallHandler): Observable<unknown> {
    return next.handle().pipe(
      timeout(this.timeoutMs),
      catchError((err) => {
        if (err instanceof TimeoutError) {
          return throwError(() => new RequestTimeoutException());
        }
        return throwError(() => err);
      }),
    );
  }
}
```

### Exception Filters

```typescript
import {
  ExceptionFilter,
  Catch,
  ArgumentsHost,
  HttpException,
  HttpStatus,
  Logger,
} from '@nestjs/common';
import { Request, Response } from 'express';

@Catch()
export class AllExceptionsFilter implements ExceptionFilter {
  private readonly logger = new Logger(AllExceptionsFilter.name);

  catch(exception: unknown, host: ArgumentsHost): void {
    const ctx = host.switchToHttp();
    const response = ctx.getResponse<Response>();
    const request = ctx.getRequest<Request>();

    const status =
      exception instanceof HttpException
        ? exception.getStatus()
        : HttpStatus.INTERNAL_SERVER_ERROR;

    const message =
      exception instanceof HttpException
        ? exception.getResponse()
        : 'Internal server error';

    // Log the full error for debugging, return sanitized error to client
    this.logger.error(
      `${request.method} ${request.url} — ${status}`,
      exception instanceof Error ? exception.stack : String(exception),
    );

    response.status(status).json({
      statusCode: status,
      message: typeof message === 'string' ? message : (message as Record<string, unknown>).message,
      timestamp: new Date().toISOString(),
      path: request.url,
    });
  }
}
```

### Registering Globally

```typescript
// main.ts
import { NestFactory, Reflector } from '@nestjs/core';
import { AppModule } from './app.module';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  // Global prefix
  app.setGlobalPrefix('api/v1');

  // Global pipes
  app.useGlobalPipes(
    new ValidationPipe({
      whitelist: true,
      forbidNonWhitelisted: true,
      transform: true,
    }),
  );

  // Global filters
  app.useGlobalFilters(new AllExceptionsFilter());

  // Global interceptors
  app.useGlobalInterceptors(
    new LoggingInterceptor(),
    new TransformInterceptor(),
  );

  // Global guards (via module for DI support)
  // Register in AppModule providers instead:
  // { provide: APP_GUARD, useClass: JwtAuthGuard }
  // { provide: APP_GUARD, useClass: RolesGuard }

  await app.listen(3000);
}
bootstrap();
```

**Important:** When a guard or interceptor needs dependency injection (e.g., `Reflector`, `ConfigService`), register it via `APP_GUARD` / `APP_INTERCEPTOR` in a module's `providers` array instead of `app.useGlobalGuards()`. The latter does not support DI.

```typescript
// app.module.ts
import { APP_GUARD, APP_INTERCEPTOR } from '@nestjs/core';

@Module({
  providers: [
    { provide: APP_GUARD, useClass: JwtAuthGuard },
    { provide: APP_GUARD, useClass: RolesGuard },
    { provide: APP_INTERCEPTOR, useClass: LoggingInterceptor },
  ],
})
export class AppModule {}
```

---

## 5. Authentication & Authorization

### Passport.js + JWT Strategy

Authentication is handled by `@nestjs/passport` with JWT tokens. The auth module manages login, registration, token refresh, and logout.

#### Auth Module

```typescript
import { Module } from '@nestjs/common';
import { JwtModule } from '@nestjs/jwt';
import { PassportModule } from '@nestjs/passport';
import { ConfigModule, ConfigService } from '@nestjs/config';
import { AuthService } from './auth.service';
import { AuthController } from './auth.controller';
import { JwtStrategy } from './strategies/jwt.strategy';
import { LocalStrategy } from './strategies/local.strategy';
import { RefreshTokenStrategy } from './strategies/refresh-token.strategy';
import { UsersModule } from '@modules/users/users.module';

@Module({
  imports: [
    UsersModule,
    PassportModule,
    JwtModule.registerAsync({
      imports: [ConfigModule],
      useFactory: (configService: ConfigService) => ({
        secret: configService.get<string>('JWT_ACCESS_SECRET'),
        signOptions: {
          expiresIn: configService.get<string>('JWT_ACCESS_EXPIRY', '15m'),
        },
      }),
      inject: [ConfigService],
    }),
  ],
  controllers: [AuthController],
  providers: [AuthService, LocalStrategy, JwtStrategy, RefreshTokenStrategy],
  exports: [AuthService],
})
export class AuthModule {}
```

#### JWT Strategy

```typescript
import { Injectable, UnauthorizedException } from '@nestjs/common';
import { PassportStrategy } from '@nestjs/passport';
import { ExtractJwt, Strategy } from 'passport-jwt';
import { ConfigService } from '@nestjs/config';
import { UsersService } from '@modules/users/users.service';

interface JwtPayload {
  sub: string;
  email: string;
  roles: string[];
}

@Injectable()
export class JwtStrategy extends PassportStrategy(Strategy, 'jwt') {
  constructor(
    configService: ConfigService,
    private readonly usersService: UsersService,
  ) {
    super({
      jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
      ignoreExpiration: false,
      secretOrKey: configService.get<string>('JWT_ACCESS_SECRET'),
    });
  }

  async validate(payload: JwtPayload) {
    const user = await this.usersService.findOne(payload.sub);
    if (!user || !user.isActive) {
      throw new UnauthorizedException('User not found or inactive');
    }
    return user;
  }
}
```

#### Local Strategy (Username/Password)

```typescript
import { Injectable, UnauthorizedException } from '@nestjs/common';
import { PassportStrategy } from '@nestjs/passport';
import { Strategy } from 'passport-local';
import { AuthService } from '../auth.service';

@Injectable()
export class LocalStrategy extends PassportStrategy(Strategy) {
  constructor(private readonly authService: AuthService) {
    super({
      usernameField: 'email', // Use email instead of default 'username'
    });
  }

  async validate(email: string, password: string) {
    const user = await this.authService.validateUser(email, password);
    if (!user) {
      throw new UnauthorizedException('Invalid credentials');
    }
    return user;
  }
}
```

#### Refresh Token Strategy

```typescript
import { Injectable } from '@nestjs/common';
import { PassportStrategy } from '@nestjs/passport';
import { ExtractJwt, Strategy } from 'passport-jwt';
import { ConfigService } from '@nestjs/config';
import { Request } from 'express';

@Injectable()
export class RefreshTokenStrategy extends PassportStrategy(
  Strategy,
  'jwt-refresh',
) {
  constructor(configService: ConfigService) {
    super({
      jwtFromRequest: ExtractJwt.fromAuthHeaderAsBearerToken(),
      secretOrKey: configService.get<string>('JWT_REFRESH_SECRET'),
      passReqToCallback: true,
    });
  }

  validate(req: Request, payload: { sub: string; email: string }) {
    const refreshToken = req.get('Authorization')?.replace('Bearer ', '').trim();
    return { ...payload, refreshToken };
  }
}
```

#### Auth Service

```typescript
import { Injectable, UnauthorizedException, ConflictException } from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';
import { ConfigService } from '@nestjs/config';
import * as bcrypt from 'bcrypt';
import { UsersService } from '@modules/users/users.service';
import { RegisterDto } from './dto/register.dto';

@Injectable()
export class AuthService {
  constructor(
    private readonly usersService: UsersService,
    private readonly jwtService: JwtService,
    private readonly configService: ConfigService,
  ) {}

  async validateUser(email: string, password: string) {
    const user = await this.usersService.findByEmail(email);
    if (user && (await bcrypt.compare(password, user.password))) {
      return user;
    }
    return null;
  }

  async register(registerDto: RegisterDto) {
    const existing = await this.usersService.findByEmail(registerDto.email);
    if (existing) {
      throw new ConflictException('Email already registered');
    }

    const hashedPassword = await bcrypt.hash(registerDto.password, 12);
    const user = await this.usersService.create({
      ...registerDto,
      password: hashedPassword,
    });

    return this.generateTokens(user.id, user.email, user.roles);
  }

  async login(user: { id: string; email: string; roles: string[] }) {
    return this.generateTokens(user.id, user.email, user.roles);
  }

  async refreshTokens(userId: string, refreshToken: string) {
    const user = await this.usersService.findOne(userId);
    if (!user || !user.hashedRefreshToken) {
      throw new UnauthorizedException('Access denied');
    }

    const isValid = await bcrypt.compare(refreshToken, user.hashedRefreshToken);
    if (!isValid) {
      throw new UnauthorizedException('Invalid refresh token');
    }

    const tokens = await this.generateTokens(user.id, user.email, user.roles);
    await this.updateRefreshToken(user.id, tokens.refreshToken);
    return tokens;
  }

  async logout(userId: string) {
    await this.usersService.update(userId, { hashedRefreshToken: null });
  }

  private async generateTokens(
    userId: string,
    email: string,
    roles: string[],
  ) {
    const payload = { sub: userId, email, roles };

    const [accessToken, refreshToken] = await Promise.all([
      this.jwtService.signAsync(payload, {
        secret: this.configService.get<string>('JWT_ACCESS_SECRET'),
        expiresIn: this.configService.get<string>('JWT_ACCESS_EXPIRY', '15m'),
      }),
      this.jwtService.signAsync(payload, {
        secret: this.configService.get<string>('JWT_REFRESH_SECRET'),
        expiresIn: this.configService.get<string>('JWT_REFRESH_EXPIRY', '7d'),
      }),
    ]);

    return { accessToken, refreshToken };
  }

  private async updateRefreshToken(userId: string, refreshToken: string) {
    const hashed = await bcrypt.hash(refreshToken, 12);
    await this.usersService.update(userId, { hashedRefreshToken: hashed });
  }
}
```

### Role Model

| Role | Value | Access |
|---|---|---|
| `user` | `Role.User` | Own profile, own resources |
| `moderator` | `Role.Moderator` | User management, content moderation |
| `admin` | `Role.Admin` | Full system access, user CRUD, configuration |
| `super_admin` | `Role.SuperAdmin` | Everything, including admin management |

```typescript
// common/enums/role.enum.ts
export enum Role {
  User = 'user',
  Moderator = 'moderator',
  Admin = 'admin',
  SuperAdmin = 'super_admin',
}
```

Roles are stored as an array on the user entity and checked by the `RolesGuard`:

```typescript
@Entity()
export class User {
  @Column('simple-array', { default: Role.User })
  roles: Role[];
}
```

---

## 6. DTOs & Validation

### DTO Design with class-validator

Every request body, query parameter, and response uses a typed DTO. DTOs define the contract between client and server.

#### Request DTOs

```typescript
import {
  IsString,
  IsEmail,
  IsNotEmpty,
  MinLength,
  MaxLength,
  IsOptional,
  IsEnum,
  IsInt,
  Min,
  Max,
  IsArray,
  ValidateNested,
  IsUUID,
  Matches,
} from 'class-validator';
import { Type } from 'class-transformer';
import { ApiProperty, ApiPropertyOptional } from '@nestjs/swagger';

export class CreateUserDto {
  @ApiProperty({ example: 'jane@example.com' })
  @IsEmail()
  @IsNotEmpty()
  email: string;

  @ApiProperty({ example: 'Jane Smith' })
  @IsString()
  @IsNotEmpty()
  @MinLength(2)
  @MaxLength(100)
  name: string;

  @ApiProperty({ example: 'SecurePass123!', minLength: 8 })
  @IsString()
  @MinLength(8)
  @MaxLength(128)
  @Matches(/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/, {
    message: 'Password must contain at least one uppercase letter, one lowercase letter, and one digit',
  })
  password: string;

  @ApiPropertyOptional({ enum: Role, isArray: true })
  @IsOptional()
  @IsArray()
  @IsEnum(Role, { each: true })
  roles?: Role[];
}
```

#### Partial Update DTOs

Use `PartialType` and `PickType` from `@nestjs/swagger` to derive update DTOs from create DTOs:

```typescript
import { PartialType, OmitType } from '@nestjs/swagger';
import { CreateUserDto } from './create-user.dto';

// All fields from CreateUserDto, all optional, except password is removed
export class UpdateUserDto extends PartialType(
  OmitType(CreateUserDto, ['password'] as const),
) {}
```

**Important:** Import `PartialType`, `OmitType`, `PickType`, and `IntersectionType` from `@nestjs/swagger`, not from `@nestjs/mapped-types`. The `@nestjs/swagger` versions carry Swagger metadata; the mapped-types versions do not.

#### Pagination DTO

```typescript
import { IsOptional, IsInt, Min, Max } from 'class-validator';
import { Type } from 'class-transformer';
import { ApiPropertyOptional } from '@nestjs/swagger';

export class PaginationDto {
  @ApiPropertyOptional({ default: 1, minimum: 1 })
  @IsOptional()
  @Type(() => Number)
  @IsInt()
  @Min(1)
  page?: number = 1;

  @ApiPropertyOptional({ default: 20, minimum: 1, maximum: 100 })
  @IsOptional()
  @Type(() => Number)
  @IsInt()
  @Min(1)
  @Max(100)
  limit?: number = 20;
}
```

**Critical:** `@Type(() => Number)` is required for query parameters. Express delivers query strings as strings; without `@Type`, `class-validator` sees `"1"` (a string) and `@IsInt()` fails. The `transform: true` option on `ValidationPipe` uses `class-transformer` decorators to convert types before validation.

#### Response DTOs

```typescript
import { Exclude, Expose } from 'class-transformer';
import { ApiProperty } from '@nestjs/swagger';

export class UserResponseDto {
  @ApiProperty()
  @Expose()
  id: string;

  @ApiProperty()
  @Expose()
  email: string;

  @ApiProperty()
  @Expose()
  name: string;

  @ApiProperty({ enum: Role, isArray: true })
  @Expose()
  roles: Role[];

  @ApiProperty()
  @Expose()
  createdAt: Date;

  // These are excluded from the response
  @Exclude()
  password: string;

  @Exclude()
  hashedRefreshToken: string;

  constructor(partial: Partial<UserResponseDto>) {
    Object.assign(this, partial);
  }
}
```

Use `ClassSerializerInterceptor` globally or per-controller to automatically apply `@Exclude()` / `@Expose()` decorators:

```typescript
// main.ts or per-controller
app.useGlobalInterceptors(new ClassSerializerInterceptor(app.get(Reflector)));
```

---

## 7. Testing Patterns

### Test Pyramid (NestJS-specific)

```
        /\
       /  \          E2E Tests (supertest — full HTTP cycle)
      /    \         Full app bootstrap, real DB, auth flows
     /------\
    /        \        Integration Tests (@nestjs/testing)
   /          \       Module bootstrap, service + repo, DB transactions
  /------------\
 /              \      Unit Tests (Jest mocks)
/                \     Services, guards, pipes, interceptors in isolation
/------------------\
```

### Unit Testing Services

```typescript
import { Test, TestingModule } from '@nestjs/testing';
import { getRepositoryToken } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { UsersService } from './users.service';
import { User } from './entities/user.entity';
import { NotFoundException, ConflictException } from '@nestjs/common';

describe('UsersService', () => {
  let service: UsersService;
  let repository: jest.Mocked<Repository<User>>;

  const mockUser: User = {
    id: '550e8400-e29b-41d4-a716-446655440000',
    email: 'jane@example.com',
    name: 'Jane Smith',
    password: 'hashed',
    roles: ['user'],
    isActive: true,
    createdAt: new Date(),
    updatedAt: new Date(),
  } as User;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [
        UsersService,
        {
          provide: getRepositoryToken(User),
          useValue: {
            find: jest.fn(),
            findOne: jest.fn(),
            create: jest.fn(),
            save: jest.fn(),
            remove: jest.fn(),
          },
        },
      ],
    }).compile();

    service = module.get<UsersService>(UsersService);
    repository = module.get(getRepositoryToken(User));
  });

  describe('findOne', () => {
    it('should return a user when found', async () => {
      repository.findOne.mockResolvedValue(mockUser);

      const result = await service.findOne(mockUser.id);

      expect(result).toEqual(mockUser);
      expect(repository.findOne).toHaveBeenCalledWith({
        where: { id: mockUser.id },
      });
    });

    it('should throw NotFoundException when user not found', async () => {
      repository.findOne.mockResolvedValue(null);

      await expect(service.findOne('nonexistent')).rejects.toThrow(
        NotFoundException,
      );
    });
  });

  describe('create', () => {
    it('should create and return a new user', async () => {
      const dto = { email: 'new@example.com', name: 'New User', password: 'hashed' };
      repository.findOne.mockResolvedValue(null); // No existing user
      repository.create.mockReturnValue(mockUser);
      repository.save.mockResolvedValue(mockUser);

      const result = await service.create(dto as any);

      expect(result).toEqual(mockUser);
      expect(repository.create).toHaveBeenCalledWith(dto);
    });

    it('should throw ConflictException for duplicate email', async () => {
      repository.findOne.mockResolvedValue(mockUser);

      await expect(
        service.create({ email: 'jane@example.com' } as any),
      ).rejects.toThrow(ConflictException);
    });
  });
});
```

### Unit Testing Controllers

```typescript
import { Test, TestingModule } from '@nestjs/testing';
import { UsersController } from './users.controller';
import { UsersService } from './users.service';

describe('UsersController', () => {
  let controller: UsersController;
  let service: jest.Mocked<UsersService>;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      controllers: [UsersController],
      providers: [
        {
          provide: UsersService,
          useValue: {
            create: jest.fn(),
            findAll: jest.fn(),
            findOne: jest.fn(),
            update: jest.fn(),
            remove: jest.fn(),
          },
        },
      ],
    }).compile();

    controller = module.get<UsersController>(UsersController);
    service = module.get(UsersService);
  });

  it('should return all users', async () => {
    const users = [{ id: '1', name: 'Jane' }];
    service.findAll.mockResolvedValue(users as any);

    const result = await controller.findAll({ page: 1, limit: 20 });

    expect(result).toEqual(users);
    expect(service.findAll).toHaveBeenCalledWith({ page: 1, limit: 20 });
  });
});
```

### Unit Testing Guards

```typescript
import { Reflector } from '@nestjs/core';
import { ExecutionContext } from '@nestjs/common';
import { RolesGuard } from './roles.guard';
import { Role } from '@common/enums/role.enum';

describe('RolesGuard', () => {
  let guard: RolesGuard;
  let reflector: Reflector;

  beforeEach(() => {
    reflector = new Reflector();
    guard = new RolesGuard(reflector);
  });

  it('should allow access when no roles are required', () => {
    jest.spyOn(reflector, 'getAllAndOverride').mockReturnValue(undefined);

    const context = createMockContext({ roles: [Role.User] });
    expect(guard.canActivate(context)).toBe(true);
  });

  it('should allow access when user has required role', () => {
    jest.spyOn(reflector, 'getAllAndOverride').mockReturnValue([Role.Admin]);

    const context = createMockContext({ roles: [Role.Admin] });
    expect(guard.canActivate(context)).toBe(true);
  });

  it('should deny access when user lacks required role', () => {
    jest.spyOn(reflector, 'getAllAndOverride').mockReturnValue([Role.Admin]);

    const context = createMockContext({ roles: [Role.User] });
    expect(guard.canActivate(context)).toBe(false);
  });
});

function createMockContext(user: { roles: Role[] }): ExecutionContext {
  return {
    switchToHttp: () => ({
      getRequest: () => ({ user }),
    }),
    getHandler: () => jest.fn(),
    getClass: () => jest.fn(),
  } as unknown as ExecutionContext;
}
```

### E2E Testing

```typescript
import { Test, TestingModule } from '@nestjs/testing';
import { INestApplication, ValidationPipe } from '@nestjs/common';
import * as request from 'supertest';
import { AppModule } from '../src/app.module';
import { DataSource } from 'typeorm';

describe('UsersController (e2e)', () => {
  let app: INestApplication;
  let dataSource: DataSource;
  let accessToken: string;

  beforeAll(async () => {
    const moduleFixture: TestingModule = await Test.createTestingModule({
      imports: [AppModule],
    }).compile();

    app = moduleFixture.createNestApplication();
    app.useGlobalPipes(
      new ValidationPipe({
        whitelist: true,
        forbidNonWhitelisted: true,
        transform: true,
      }),
    );
    await app.init();

    dataSource = moduleFixture.get<DataSource>(DataSource);

    // Authenticate to get a token for protected routes
    const loginResponse = await request(app.getHttpServer())
      .post('/api/v1/auth/login')
      .send({ email: 'admin@test.com', password: 'TestPassword123!' })
      .expect(200);

    accessToken = loginResponse.body.data.accessToken;
  });

  afterAll(async () => {
    await dataSource.destroy();
    await app.close();
  });

  describe('POST /api/v1/users', () => {
    it('should create a user with valid data', () => {
      return request(app.getHttpServer())
        .post('/api/v1/users')
        .set('Authorization', `Bearer ${accessToken}`)
        .send({
          email: 'new@example.com',
          name: 'New User',
          password: 'SecurePass123!',
        })
        .expect(201)
        .expect((res) => {
          expect(res.body.data).toHaveProperty('id');
          expect(res.body.data.email).toBe('new@example.com');
          expect(res.body.data).not.toHaveProperty('password');
        });
    });

    it('should reject invalid email', () => {
      return request(app.getHttpServer())
        .post('/api/v1/users')
        .set('Authorization', `Bearer ${accessToken}`)
        .send({
          email: 'not-an-email',
          name: 'Test',
          password: 'SecurePass123!',
        })
        .expect(400);
    });

    it('should reject unauthenticated requests', () => {
      return request(app.getHttpServer())
        .post('/api/v1/users')
        .send({
          email: 'new@example.com',
          name: 'New User',
          password: 'SecurePass123!',
        })
        .expect(401);
    });
  });
});
```

### Test Configuration

```json
// package.json — jest config
{
  "jest": {
    "moduleFileExtensions": ["js", "json", "ts"],
    "rootDir": "src",
    "testRegex": ".*\\.spec\\.ts$",
    "transform": {
      "^.+\\.(t|j)s$": "ts-jest"
    },
    "collectCoverageFrom": [
      "**/*.(t|j)s",
      "!**/*.spec.ts",
      "!**/*.e2e-spec.ts",
      "!**/node_modules/**",
      "!**/dist/**",
      "!main.ts"
    ],
    "coverageDirectory": "../coverage",
    "testEnvironment": "node",
    "moduleNameMapper": {
      "^@app/(.*)$": "<rootDir>/$1",
      "^@common/(.*)$": "<rootDir>/common/$1",
      "^@modules/(.*)$": "<rootDir>/modules/$1",
      "^@config/(.*)$": "<rootDir>/config/$1"
    }
  }
}
```

```json
// jest-e2e.json
{
  "moduleFileExtensions": ["js", "json", "ts"],
  "rootDir": ".",
  "testEnvironment": "node",
  "testRegex": ".e2e-spec.ts$",
  "transform": {
    "^.+\\.(t|j)s$": "ts-jest"
  },
  "moduleNameMapper": {
    "^@app/(.*)$": "<rootDir>/../src/$1",
    "^@common/(.*)$": "<rootDir>/../src/common/$1",
    "^@modules/(.*)$": "<rootDir>/../src/modules/$1",
    "^@config/(.*)$": "<rootDir>/../src/config/$1"
  }
}
```

### Test Database Strategy

Use a separate test database with transaction rollback per test:

```typescript
// test/setup.ts
import { DataSource } from 'typeorm';

let dataSource: DataSource;

beforeAll(async () => {
  dataSource = new DataSource({
    type: 'postgres',
    host: process.env.TEST_DB_HOST || 'localhost',
    port: parseInt(process.env.TEST_DB_PORT || '5432'),
    username: process.env.TEST_DB_USER || 'test',
    password: process.env.TEST_DB_PASSWORD || 'test',
    database: process.env.TEST_DB_NAME || 'app_test',
    entities: ['src/**/*.entity.ts'],
    synchronize: true, // Only for test DB — never production
    dropSchema: true,  // Clean slate each run
  });
  await dataSource.initialize();
});

afterAll(async () => {
  await dataSource.destroy();
});
```

---

## 8. Data Layer

### Option A: TypeORM

TypeORM is the default ORM for NestJS projects. It uses decorators for entity definitions and provides the repository pattern.

#### Entity Template

```typescript
import {
  Entity,
  PrimaryGeneratedColumn,
  Column,
  CreateDateColumn,
  UpdateDateColumn,
  ManyToOne,
  OneToMany,
  JoinColumn,
  Index,
} from 'typeorm';

@Entity('users')
export class User {
  @PrimaryGeneratedColumn('uuid')
  id: string;

  @Column({ unique: true })
  @Index()
  email: string;

  @Column()
  name: string;

  @Column({ select: false }) // Excluded from default queries
  password: string;

  @Column('simple-array', { default: 'user' })
  roles: string[];

  @Column({ default: true })
  isActive: boolean;

  @Column({ nullable: true, select: false })
  hashedRefreshToken: string;

  @OneToMany(() => Order, (order) => order.user)
  orders: Order[];

  @CreateDateColumn()
  createdAt: Date;

  @UpdateDateColumn()
  updatedAt: Date;
}
```

#### TypeORM Module Registration

```typescript
// app.module.ts
import { TypeOrmModule } from '@nestjs/typeorm';

@Module({
  imports: [
    TypeOrmModule.forRootAsync({
      imports: [ConfigModule],
      useFactory: (configService: ConfigService) => ({
        type: 'postgres',
        host: configService.get('DB_HOST'),
        port: configService.get<number>('DB_PORT'),
        username: configService.get('DB_USERNAME'),
        password: configService.get('DB_PASSWORD'),
        database: configService.get('DB_DATABASE'),
        entities: [__dirname + '/**/*.entity{.ts,.js}'],
        synchronize: false, // NEVER true in production
        migrations: [__dirname + '/database/migrations/*{.ts,.js}'],
        migrationsRun: true,
        logging: configService.get('NODE_ENV') === 'development',
      }),
      inject: [ConfigService],
    }),
  ],
})
export class AppModule {}
```

#### Migrations

```bash
# Generate migration from entity changes
npx typeorm migration:generate -d src/database/data-source.ts src/database/migrations/AddUserRoles

# Create empty migration for custom SQL
npx typeorm migration:create src/database/migrations/SeedAdminUser

# Run pending migrations
npx typeorm migration:run -d src/database/data-source.ts

# Revert last migration (development only)
npx typeorm migration:revert -d src/database/data-source.ts
```

**Convention:** Never edit a migration after it has been committed. Write a new corrective migration instead.

#### Query Builder for Complex Queries

```typescript
async findActiveUsersWithOrders(): Promise<User[]> {
  return this.usersRepository
    .createQueryBuilder('user')
    .leftJoinAndSelect('user.orders', 'order')
    .where('user.isActive = :isActive', { isActive: true })
    .andWhere('order.createdAt >= :date', {
      date: new Date(Date.now() - 30 * 24 * 60 * 60 * 1000),
    })
    .orderBy('user.createdAt', 'DESC')
    .getMany();
}
```

### Option B: Prisma

Prisma provides a type-safe query builder generated from a schema file. It is an alternative to TypeORM with stronger type safety and a different mental model (schema-first vs. decorator-first).

#### Prisma Schema

```prisma
// prisma/schema.prisma
generator client {
  provider = "prisma-client-js"
}

datasource db {
  provider = "postgresql"
  url      = env("DATABASE_URL")
}

model User {
  id                 String   @id @default(uuid())
  email              String   @unique
  name               String
  password           String
  roles              String[] @default(["user"])
  isActive           Boolean  @default(true)
  hashedRefreshToken String?
  orders             Order[]
  createdAt          DateTime @default(now()) @map("created_at")
  updatedAt          DateTime @updatedAt @map("updated_at")

  @@map("users")
}

model Order {
  id        String      @id @default(uuid())
  userId    String      @map("user_id")
  user      User        @relation(fields: [userId], references: [id])
  status    OrderStatus @default(PENDING)
  total     Decimal     @db.Decimal(10, 2)
  items     OrderItem[]
  createdAt DateTime    @default(now()) @map("created_at")
  updatedAt DateTime    @updatedAt @map("updated_at")

  @@map("orders")
}

enum OrderStatus {
  PENDING
  PROCESSING
  SHIPPED
  DELIVERED
  CANCELLED
}
```

#### Prisma Service

```typescript
import { Injectable, OnModuleInit, OnModuleDestroy } from '@nestjs/common';
import { PrismaClient } from '@prisma/client';

@Injectable()
export class PrismaService extends PrismaClient implements OnModuleInit, OnModuleDestroy {
  async onModuleInit() {
    await this.$connect();
  }

  async onModuleDestroy() {
    await this.$disconnect();
  }
}
```

#### Prisma Usage in Services

```typescript
@Injectable()
export class UsersService {
  constructor(private readonly prisma: PrismaService) {}

  async findAll(pagination: PaginationDto) {
    const { page = 1, limit = 20 } = pagination;
    return this.prisma.user.findMany({
      skip: (page - 1) * limit,
      take: limit,
      orderBy: { createdAt: 'desc' },
      select: {
        id: true,
        email: true,
        name: true,
        roles: true,
        createdAt: true,
        // password excluded automatically by not selecting it
      },
    });
  }

  async findOne(id: string) {
    const user = await this.prisma.user.findUnique({ where: { id } });
    if (!user) {
      throw new NotFoundException(`User with ID "${id}" not found`);
    }
    return user;
  }
}
```

#### Prisma Migrations

```bash
# Create migration from schema changes
npx prisma migrate dev --name add_user_roles

# Apply pending migrations (production)
npx prisma migrate deploy

# Generate Prisma Client (after schema changes)
npx prisma generate

# Reset database (development only)
npx prisma migrate reset
```

### Option C: Drizzle ORM

Drizzle is a SQL-like, type-safe ORM with zero abstraction overhead. It maps directly to SQL semantics.

#### Drizzle Schema

```typescript
// src/database/schema.ts
import {
  pgTable,
  uuid,
  varchar,
  boolean,
  timestamp,
  text,
  decimal,
  pgEnum,
} from 'drizzle-orm/pg-core';
import { relations } from 'drizzle-orm';

export const users = pgTable('users', {
  id: uuid('id').primaryKey().defaultRandom(),
  email: varchar('email', { length: 255 }).unique().notNull(),
  name: varchar('name', { length: 255 }).notNull(),
  password: text('password').notNull(),
  roles: text('roles').array().default(['user']),
  isActive: boolean('is_active').default(true),
  hashedRefreshToken: text('hashed_refresh_token'),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});

export const usersRelations = relations(users, ({ many }) => ({
  orders: many(orders),
}));

export const orderStatusEnum = pgEnum('order_status', [
  'pending',
  'processing',
  'shipped',
  'delivered',
  'cancelled',
]);

export const orders = pgTable('orders', {
  id: uuid('id').primaryKey().defaultRandom(),
  userId: uuid('user_id')
    .references(() => users.id)
    .notNull(),
  status: orderStatusEnum('status').default('pending'),
  total: decimal('total', { precision: 10, scale: 2 }).notNull(),
  createdAt: timestamp('created_at').defaultNow(),
  updatedAt: timestamp('updated_at').defaultNow(),
});
```

#### Drizzle Usage in Services

```typescript
import { Inject, Injectable } from '@nestjs/common';
import { eq, desc } from 'drizzle-orm';
import { NodePgDatabase } from 'drizzle-orm/node-postgres';
import * as schema from '@app/database/schema';

@Injectable()
export class UsersService {
  constructor(
    @Inject('DRIZZLE') private readonly db: NodePgDatabase<typeof schema>,
  ) {}

  async findAll(pagination: PaginationDto) {
    const { page = 1, limit = 20 } = pagination;
    return this.db
      .select({
        id: schema.users.id,
        email: schema.users.email,
        name: schema.users.name,
        roles: schema.users.roles,
        createdAt: schema.users.createdAt,
      })
      .from(schema.users)
      .orderBy(desc(schema.users.createdAt))
      .limit(limit)
      .offset((page - 1) * limit);
  }
}
```

### ORM Selection Guide

| Criteria | TypeORM | Prisma | Drizzle |
|---|---|---|---|
| **Type safety** | Moderate (decorators) | Strong (generated client) | Strong (schema inference) |
| **Mental model** | Active Record / Repository | Schema-first, generated | SQL-like, explicit |
| **Migration** | Code-gen from entities | Code-gen from schema | Code-gen from schema |
| **NestJS integration** | First-class (`@nestjs/typeorm`) | Community module | Community module |
| **Raw SQL access** | Query builder | `$queryRaw` | Native SQL support |
| **Best for** | Large NestJS codebases | Type safety priority | SQL purists |

---

## 9. Microservices

### Transport Layers

NestJS supports multiple transport layers for microservice communication:

```typescript
// main.ts — hybrid application (HTTP + microservice)
import { NestFactory } from '@nestjs/core';
import { Transport, MicroserviceOptions } from '@nestjs/microservices';
import { AppModule } from './app.module';

async function bootstrap() {
  // HTTP server
  const app = await NestFactory.create(AppModule);

  // Connect microservice transport
  app.connectMicroservice<MicroserviceOptions>({
    transport: Transport.REDIS,
    options: {
      host: process.env.REDIS_HOST || 'localhost',
      port: parseInt(process.env.REDIS_PORT || '6379'),
    },
  });

  // TCP transport alternative
  app.connectMicroservice<MicroserviceOptions>({
    transport: Transport.TCP,
    options: {
      host: '0.0.0.0',
      port: 3001,
    },
  });

  // gRPC transport alternative
  app.connectMicroservice<MicroserviceOptions>({
    transport: Transport.GRPC,
    options: {
      package: 'orders',
      protoPath: join(__dirname, '../proto/orders.proto'),
      url: '0.0.0.0:5000',
    },
  });

  await app.startAllMicroservices();
  await app.listen(3000);
}
bootstrap();
```

### Message Patterns (Request/Response)

```typescript
// orders.controller.ts — microservice handler
import { Controller } from '@nestjs/common';
import { MessagePattern, Payload } from '@nestjs/microservices';

@Controller()
export class OrdersController {
  constructor(private readonly ordersService: OrdersService) {}

  @MessagePattern('orders.create')
  async handleCreateOrder(@Payload() data: CreateOrderDto) {
    return this.ordersService.create(data);
  }

  @MessagePattern('orders.findOne')
  async handleFindOrder(@Payload() data: { id: string }) {
    return this.ordersService.findOne(data.id);
  }
}
```

```typescript
// Client — sending messages to the microservice
import { Inject, Injectable } from '@nestjs/common';
import { ClientProxy } from '@nestjs/microservices';
import { firstValueFrom } from 'rxjs';

@Injectable()
export class OrdersClientService {
  constructor(
    @Inject('ORDERS_SERVICE') private readonly client: ClientProxy,
  ) {}

  async createOrder(dto: CreateOrderDto) {
    return firstValueFrom(
      this.client.send('orders.create', dto),
    );
  }
}
```

### Event Patterns (Fire and Forget)

```typescript
// Publisher
@Injectable()
export class OrdersService {
  constructor(
    @Inject('NOTIFICATIONS_SERVICE') private readonly client: ClientProxy,
  ) {}

  async create(dto: CreateOrderDto) {
    const order = await this.ordersRepository.save(dto);

    // Fire and forget — no response expected
    this.client.emit('order.created', {
      orderId: order.id,
      userId: order.userId,
      total: order.total,
    });

    return order;
  }
}

// Subscriber
@Controller()
export class NotificationsController {
  @EventPattern('order.created')
  async handleOrderCreated(@Payload() data: OrderCreatedEvent) {
    await this.notificationsService.sendOrderConfirmation(data);
  }
}
```

### Client Registration

```typescript
// orders-client.module.ts
import { Module } from '@nestjs/common';
import { ClientsModule, Transport } from '@nestjs/microservices';

@Module({
  imports: [
    ClientsModule.registerAsync([
      {
        name: 'ORDERS_SERVICE',
        useFactory: (configService: ConfigService) => ({
          transport: Transport.REDIS,
          options: {
            host: configService.get('REDIS_HOST'),
            port: configService.get<number>('REDIS_PORT'),
          },
        }),
        inject: [ConfigService],
      },
    ]),
  ],
  exports: [ClientsModule],
})
export class OrdersClientModule {}
```

---

## 10. WebSockets

### Gateway Setup

NestJS WebSocket gateways use the `@WebSocketGateway()` decorator. They are providers that can inject services and emit events to connected clients.

```typescript
import {
  WebSocketGateway,
  WebSocketServer,
  SubscribeMessage,
  OnGatewayInit,
  OnGatewayConnection,
  OnGatewayDisconnect,
  MessageBody,
  ConnectedSocket,
} from '@nestjs/websockets';
import { Server, Socket } from 'socket.io';
import { Logger, UseGuards } from '@nestjs/common';
import { WsJwtGuard } from './guards/ws-jwt.guard';

@WebSocketGateway({
  cors: {
    origin: process.env.CORS_ORIGIN || 'http://localhost:3000',
    credentials: true,
  },
  namespace: '/notifications',
})
export class NotificationsGateway
  implements OnGatewayInit, OnGatewayConnection, OnGatewayDisconnect
{
  @WebSocketServer()
  server: Server;

  private readonly logger = new Logger(NotificationsGateway.name);

  afterInit(server: Server) {
    this.logger.log('WebSocket Gateway initialized');
  }

  handleConnection(client: Socket) {
    this.logger.log(`Client connected: ${client.id}`);
  }

  handleDisconnect(client: Socket) {
    this.logger.log(`Client disconnected: ${client.id}`);
  }

  @UseGuards(WsJwtGuard)
  @SubscribeMessage('joinRoom')
  handleJoinRoom(
    @ConnectedSocket() client: Socket,
    @MessageBody() data: { room: string },
  ) {
    client.join(data.room);
    this.logger.log(`Client ${client.id} joined room: ${data.room}`);
    return { event: 'joinedRoom', data: { room: data.room } };
  }

  @SubscribeMessage('leaveRoom')
  handleLeaveRoom(
    @ConnectedSocket() client: Socket,
    @MessageBody() data: { room: string },
  ) {
    client.leave(data.room);
    return { event: 'leftRoom', data: { room: data.room } };
  }

  // Emit to specific user (call from any service)
  sendToUser(userId: string, event: string, payload: unknown) {
    this.server.to(`user:${userId}`).emit(event, payload);
  }

  // Emit to all connected clients
  broadcast(event: string, payload: unknown) {
    this.server.emit(event, payload);
  }
}
```

### WebSocket Authentication Guard

```typescript
import { CanActivate, ExecutionContext, Injectable } from '@nestjs/common';
import { JwtService } from '@nestjs/jwt';
import { WsException } from '@nestjs/websockets';
import { Socket } from 'socket.io';

@Injectable()
export class WsJwtGuard implements CanActivate {
  constructor(private readonly jwtService: JwtService) {}

  canActivate(context: ExecutionContext): boolean {
    const client: Socket = context.switchToWs().getClient();
    const token =
      client.handshake.auth?.token ||
      client.handshake.headers?.authorization?.split(' ')[1];

    if (!token) {
      throw new WsException('Missing authentication token');
    }

    try {
      const payload = this.jwtService.verify(token);
      client.data.user = payload; // Attach user to socket
      return true;
    } catch {
      throw new WsException('Invalid authentication token');
    }
  }
}
```

### WebSocket Module

```typescript
import { Module } from '@nestjs/common';
import { JwtModule } from '@nestjs/jwt';
import { NotificationsGateway } from './notifications.gateway';
import { WsJwtGuard } from './guards/ws-jwt.guard';

@Module({
  imports: [JwtModule],
  providers: [NotificationsGateway, WsJwtGuard],
  exports: [NotificationsGateway],
})
export class NotificationsModule {}
```

### Testing WebSocket Gateways

```typescript
import { Test, TestingModule } from '@nestjs/testing';
import { INestApplication } from '@nestjs/common';
import { io, Socket as ClientSocket } from 'socket.io-client';
import { NotificationsGateway } from './notifications.gateway';

describe('NotificationsGateway', () => {
  let app: INestApplication;
  let clientSocket: ClientSocket;

  beforeAll(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [NotificationsGateway],
    }).compile();

    app = module.createNestApplication();
    await app.listen(0); // Random port
    const port = app.getHttpServer().address().port;

    clientSocket = io(`http://localhost:${port}/notifications`, {
      auth: { token: 'valid-test-token' },
    });

    await new Promise<void>((resolve) => {
      clientSocket.on('connect', resolve);
    });
  });

  afterAll(async () => {
    clientSocket.disconnect();
    await app.close();
  });

  it('should join a room', (done) => {
    clientSocket.emit('joinRoom', { room: 'test-room' }, (response: any) => {
      expect(response.event).toBe('joinedRoom');
      expect(response.data.room).toBe('test-room');
      done();
    });
  });
});
```

---

## 11. Bull Queues

### Queue Setup

Bull (via `@nestjs/bullmq`) provides Redis-backed job queues for background processing, scheduled tasks, and rate-limited operations.

#### Module Registration

```typescript
import { Module } from '@nestjs/common';
import { BullModule } from '@nestjs/bullmq';
import { ConfigModule, ConfigService } from '@nestjs/config';

@Module({
  imports: [
    BullModule.forRootAsync({
      imports: [ConfigModule],
      useFactory: (configService: ConfigService) => ({
        connection: {
          host: configService.get('REDIS_HOST'),
          port: configService.get<number>('REDIS_PORT'),
          password: configService.get('REDIS_PASSWORD'),
        },
        defaultJobOptions: {
          attempts: 3,
          backoff: {
            type: 'exponential',
            delay: 1000,
          },
          removeOnComplete: 1000, // Keep last 1000 completed jobs
          removeOnFail: 5000,      // Keep last 5000 failed jobs
        },
      }),
      inject: [ConfigService],
    }),
  ],
})
export class AppModule {}
```

#### Queue Registration Per Module

```typescript
import { Module } from '@nestjs/common';
import { BullModule } from '@nestjs/bullmq';
import { OrderFulfillmentProcessor } from './processors/order-fulfillment.processor';
import { OrdersService } from './orders.service';

@Module({
  imports: [
    BullModule.registerQueue({
      name: 'order-fulfillment',
    }),
    BullModule.registerQueue({
      name: 'order-notifications',
    }),
  ],
  providers: [OrdersService, OrderFulfillmentProcessor],
  exports: [OrdersService],
})
export class OrdersModule {}
```

#### Processor (Worker)

```typescript
import { Processor, WorkerHost, OnWorkerEvent } from '@nestjs/bullmq';
import { Logger } from '@nestjs/common';
import { Job } from 'bullmq';

interface OrderFulfillmentJob {
  orderId: string;
  userId: string;
  items: Array<{ productId: string; quantity: number }>;
}

@Processor('order-fulfillment')
export class OrderFulfillmentProcessor extends WorkerHost {
  private readonly logger = new Logger(OrderFulfillmentProcessor.name);

  async process(job: Job<OrderFulfillmentJob>): Promise<void> {
    this.logger.log(`Processing order fulfillment: ${job.data.orderId}`);

    // Step 1: Reserve inventory
    await this.reserveInventory(job.data.items);
    await job.updateProgress(33);

    // Step 2: Create shipping label
    const trackingNumber = await this.createShippingLabel(job.data.orderId);
    await job.updateProgress(66);

    // Step 3: Update order status
    await this.updateOrderStatus(job.data.orderId, 'shipped', trackingNumber);
    await job.updateProgress(100);

    this.logger.log(
      `Order ${job.data.orderId} fulfilled — tracking: ${trackingNumber}`,
    );
  }

  @OnWorkerEvent('failed')
  onFailed(job: Job, error: Error) {
    this.logger.error(
      `Job ${job.id} failed after ${job.attemptsMade} attempts: ${error.message}`,
      error.stack,
    );
  }

  @OnWorkerEvent('completed')
  onCompleted(job: Job) {
    this.logger.log(`Job ${job.id} completed`);
  }

  private async reserveInventory(
    items: Array<{ productId: string; quantity: number }>,
  ): Promise<void> {
    // Inventory reservation logic
  }

  private async createShippingLabel(orderId: string): Promise<string> {
    // Shipping label creation logic
    return `TRACK-${orderId}`;
  }

  private async updateOrderStatus(
    orderId: string,
    status: string,
    trackingNumber: string,
  ): Promise<void> {
    // Order status update logic
  }
}
```

#### Enqueueing Jobs

```typescript
import { Injectable } from '@nestjs/common';
import { InjectQueue } from '@nestjs/bullmq';
import { Queue } from 'bullmq';

@Injectable()
export class OrdersService {
  constructor(
    @InjectQueue('order-fulfillment')
    private readonly fulfillmentQueue: Queue,
    @InjectQueue('order-notifications')
    private readonly notificationsQueue: Queue,
  ) {}

  async create(dto: CreateOrderDto) {
    const order = await this.ordersRepository.save(dto);

    // Enqueue fulfillment job
    await this.fulfillmentQueue.add('fulfill', {
      orderId: order.id,
      userId: order.userId,
      items: order.items,
    });

    // Enqueue notification with delay
    await this.notificationsQueue.add(
      'confirmation',
      { orderId: order.id, email: order.user.email },
      { delay: 5000 }, // Send 5 seconds after order
    );

    // Scheduled/recurring job
    await this.fulfillmentQueue.add(
      'check-stale-orders',
      {},
      {
        repeat: {
          pattern: '0 */6 * * *', // Every 6 hours
        },
      },
    );

    return order;
  }
}
```

#### Testing Processors

```typescript
import { Test, TestingModule } from '@nestjs/testing';
import { getQueueToken } from '@nestjs/bullmq';
import { OrderFulfillmentProcessor } from './order-fulfillment.processor';
import { Job } from 'bullmq';

describe('OrderFulfillmentProcessor', () => {
  let processor: OrderFulfillmentProcessor;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [OrderFulfillmentProcessor],
    }).compile();

    processor = module.get(OrderFulfillmentProcessor);
  });

  it('should process order fulfillment', async () => {
    const mockJob = {
      data: {
        orderId: 'order-1',
        userId: 'user-1',
        items: [{ productId: 'product-1', quantity: 2 }],
      },
      updateProgress: jest.fn(),
    } as unknown as Job;

    await processor.process(mockJob);

    expect(mockJob.updateProgress).toHaveBeenCalledWith(100);
  });
});
```

#### Testing Queue Enqueueing

```typescript
describe('OrdersService', () => {
  let service: OrdersService;
  let fulfillmentQueue: jest.Mocked<Queue>;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [
        OrdersService,
        {
          provide: getQueueToken('order-fulfillment'),
          useValue: { add: jest.fn() },
        },
        {
          provide: getQueueToken('order-notifications'),
          useValue: { add: jest.fn() },
        },
      ],
    }).compile();

    service = module.get(OrdersService);
    fulfillmentQueue = module.get(getQueueToken('order-fulfillment'));
  });

  it('should enqueue fulfillment job on order creation', async () => {
    await service.create(mockOrderDto);

    expect(fulfillmentQueue.add).toHaveBeenCalledWith(
      'fulfill',
      expect.objectContaining({ orderId: expect.any(String) }),
    );
  });
});
```

---

## 12. Swagger / OpenAPI

### Setup

```typescript
// main.ts
import { DocumentBuilder, SwaggerModule } from '@nestjs/swagger';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  const config = new DocumentBuilder()
    .setTitle('My API')
    .setDescription('API documentation')
    .setVersion('1.0')
    .addBearerAuth(
      {
        type: 'http',
        scheme: 'bearer',
        bearerFormat: 'JWT',
        name: 'Authorization',
        description: 'Enter JWT access token',
        in: 'header',
      },
      'access-token', // Security scheme name
    )
    .addTag('auth', 'Authentication endpoints')
    .addTag('users', 'User management')
    .addTag('orders', 'Order management')
    .build();

  const document = SwaggerModule.createDocument(app, config);
  SwaggerModule.setup('docs', app, document, {
    swaggerOptions: {
      persistAuthorization: true,
      tagsSorter: 'alpha',
      operationsSorter: 'method',
    },
  });

  await app.listen(3000);
}
```

### DTO Documentation

Swagger decorators on DTOs generate the schema section of the OpenAPI spec:

```typescript
import { ApiProperty, ApiPropertyOptional } from '@nestjs/swagger';

export class CreateOrderDto {
  @ApiProperty({
    description: 'Array of order items',
    type: [OrderItemDto],
    example: [{ productId: 'prod-1', quantity: 2 }],
  })
  @ValidateNested({ each: true })
  @Type(() => OrderItemDto)
  items: OrderItemDto[];

  @ApiPropertyOptional({
    description: 'Optional shipping notes',
    example: 'Leave at front door',
    maxLength: 500,
  })
  @IsOptional()
  @IsString()
  @MaxLength(500)
  notes?: string;

  @ApiProperty({
    description: 'Shipping address',
    type: AddressDto,
  })
  @ValidateNested()
  @Type(() => AddressDto)
  shippingAddress: AddressDto;
}
```

### Controller Documentation

```typescript
@ApiTags('orders')
@ApiBearerAuth('access-token')
@Controller('orders')
export class OrdersController {
  @Post()
  @ApiOperation({
    summary: 'Create a new order',
    description: 'Creates an order and enqueues fulfillment processing',
  })
  @ApiResponse({
    status: 201,
    description: 'Order created successfully',
    type: OrderResponseDto,
  })
  @ApiResponse({ status: 400, description: 'Validation error' })
  @ApiResponse({ status: 401, description: 'Unauthorized' })
  @ApiResponse({ status: 409, description: 'Insufficient inventory' })
  async create(@Body() dto: CreateOrderDto): Promise<OrderResponseDto> {
    return this.ordersService.create(dto);
  }

  @Get()
  @ApiOperation({ summary: 'List orders with pagination' })
  @ApiQuery({ name: 'page', required: false, type: Number })
  @ApiQuery({ name: 'limit', required: false, type: Number })
  @ApiQuery({
    name: 'status',
    required: false,
    enum: OrderStatus,
    description: 'Filter by order status',
  })
  async findAll(
    @Query() pagination: PaginationDto,
    @Query('status') status?: OrderStatus,
  ): Promise<OrderResponseDto[]> {
    return this.ordersService.findAll(pagination, status);
  }
}
```

### Generating OpenAPI JSON at Build Time

```typescript
// scripts/generate-openapi.ts
import { NestFactory } from '@nestjs/core';
import { DocumentBuilder, SwaggerModule } from '@nestjs/swagger';
import { AppModule } from '../src/app.module';
import * as fs from 'fs';

async function generate() {
  const app = await NestFactory.create(AppModule, { logger: false });
  const config = new DocumentBuilder()
    .setTitle('My API')
    .setVersion('1.0')
    .build();
  const document = SwaggerModule.createDocument(app, config);
  fs.writeFileSync('openapi.json', JSON.stringify(document, null, 2));
  await app.close();
}
generate();
```

**Convention:** Generate the OpenAPI spec in CI and publish it as an artifact. This ensures the spec stays in sync with the code. Clients can use the generated spec for code generation (e.g., `openapi-typescript`, `openapi-generator`).

---

## 13. Security

### Security Headers

```typescript
// main.ts
import helmet from 'helmet';

async function bootstrap() {
  const app = await NestFactory.create(AppModule);

  // Helmet sets security headers
  app.use(
    helmet({
      contentSecurityPolicy: {
        directives: {
          defaultSrc: ["'self'"],
          scriptSrc: ["'self'"],
          styleSrc: ["'self'", "'unsafe-inline'"],
          imgSrc: ["'self'", 'data:', 'https:'],
          connectSrc: ["'self'"],
          fontSrc: ["'self'"],
          frameSrc: ["'none'"],
          frameAncestors: ["'self'"],
        },
      },
      hsts: {
        maxAge: 31536000,
        includeSubDomains: true,
        preload: true,
      },
    }),
  );

  // CORS
  app.enableCors({
    origin: process.env.CORS_ORIGIN?.split(',') || ['http://localhost:3000'],
    methods: ['GET', 'POST', 'PUT', 'PATCH', 'DELETE'],
    allowedHeaders: ['Content-Type', 'Authorization'],
    credentials: true,
    maxAge: 86400,
  });

  // Rate limiting
  // Use @nestjs/throttler for global rate limiting
}
```

### Rate Limiting

```typescript
import { Module } from '@nestjs/common';
import { ThrottlerModule, ThrottlerGuard } from '@nestjs/throttler';
import { APP_GUARD } from '@nestjs/core';

@Module({
  imports: [
    ThrottlerModule.forRoot([
      {
        name: 'short',
        ttl: 1000,   // 1 second
        limit: 3,     // 3 requests per second
      },
      {
        name: 'medium',
        ttl: 10000,  // 10 seconds
        limit: 20,    // 20 requests per 10 seconds
      },
      {
        name: 'long',
        ttl: 60000,  // 1 minute
        limit: 100,   // 100 requests per minute
      },
    ]),
  ],
  providers: [
    {
      provide: APP_GUARD,
      useClass: ThrottlerGuard,
    },
  ],
})
export class AppModule {}
```

Override rate limits per route:

```typescript
import { SkipThrottle, Throttle } from '@nestjs/throttler';

@Controller('auth')
export class AuthController {
  @Post('login')
  @Throttle({ short: { limit: 5, ttl: 60000 } }) // Stricter: 5 per minute
  async login(@Body() dto: LoginDto) {
    return this.authService.login(dto);
  }

  @Get('profile')
  @SkipThrottle() // No rate limit for authenticated profile reads
  async getProfile(@CurrentUser() user: User) {
    return user;
  }
}
```

### Input Sanitization

```typescript
// common/pipes/sanitize.pipe.ts
import { PipeTransform, Injectable } from '@nestjs/common';
import * as sanitizeHtml from 'sanitize-html';

@Injectable()
export class SanitizeHtmlPipe implements PipeTransform {
  transform(value: unknown): unknown {
    if (typeof value === 'string') {
      return sanitizeHtml(value, {
        allowedTags: [],
        allowedAttributes: {},
      });
    }
    if (typeof value === 'object' && value !== null) {
      return this.sanitizeObject(value);
    }
    return value;
  }

  private sanitizeObject(obj: Record<string, unknown>): Record<string, unknown> {
    const sanitized: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(obj)) {
      sanitized[key] = this.transform(value);
    }
    return sanitized;
  }
}
```

### Environment Configuration

```typescript
// config/app.config.ts
import { registerAs } from '@nestjs/config';
import * as Joi from 'joi';

export const appConfig = registerAs('app', () => ({
  port: parseInt(process.env.PORT || '3000', 10),
  environment: process.env.NODE_ENV || 'development',
  apiPrefix: process.env.API_PREFIX || 'api/v1',
}));

// config/validation.schema.ts — env validation
export const envValidationSchema = Joi.object({
  NODE_ENV: Joi.string()
    .valid('development', 'test', 'production')
    .default('development'),
  PORT: Joi.number().default(3000),
  DB_HOST: Joi.string().required(),
  DB_PORT: Joi.number().default(5432),
  DB_USERNAME: Joi.string().required(),
  DB_PASSWORD: Joi.string().required(),
  DB_DATABASE: Joi.string().required(),
  JWT_ACCESS_SECRET: Joi.string().min(32).required(),
  JWT_REFRESH_SECRET: Joi.string().min(32).required(),
  JWT_ACCESS_EXPIRY: Joi.string().default('15m'),
  JWT_REFRESH_EXPIRY: Joi.string().default('7d'),
  REDIS_HOST: Joi.string().default('localhost'),
  REDIS_PORT: Joi.number().default(6379),
  CORS_ORIGIN: Joi.string().default('http://localhost:3000'),
});

// app.module.ts
import { ConfigModule } from '@nestjs/config';
import { envValidationSchema } from '@config/validation.schema';

@Module({
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      validationSchema: envValidationSchema,
      validationOptions: {
        abortEarly: true, // Fail fast on first env error
      },
    }),
  ],
})
export class AppModule {}
```

**Critical:** Never put secrets in code or commit `.env` files to version control. Use `ConfigModule` with Joi validation to fail fast when required environment variables are missing.

---

## 14. Coverage Enforcement

Test coverage is enforced via Jest's built-in coverage:

```json
// package.json — coverage config within jest section
{
  "jest": {
    "coverageThreshold": {
      "global": {
        "branches": 80,
        "functions": 80,
        "lines": 80,
        "statements": 80
      }
    },
    "collectCoverageFrom": [
      "src/**/*.ts",
      "!src/**/*.spec.ts",
      "!src/**/*.e2e-spec.ts",
      "!src/main.ts",
      "!src/**/*.module.ts",
      "!src/**/*.dto.ts",
      "!src/**/*.entity.ts",
      "!src/**/*.interface.ts",
      "!src/**/*.enum.ts",
      "!src/database/migrations/**"
    ]
  }
}
```

**Commands:**

```bash
# Run tests with coverage
npm run test -- --coverage

# Run tests with detailed missing-line report
npm run test -- --coverage --verbose

# Run e2e tests with coverage
npm run test:e2e -- --coverage

# Generate HTML coverage report
npm run test -- --coverage --coverageReporters=html
# Opens in coverage/lcov-report/index.html
```

Target is 100% (per CLAUDE.md core rules). The `coverageThreshold` in `package.json` is the hard gate — CI fails below this threshold.

**Convention:** Exclude declaration-only files (DTOs, entities, interfaces, enums, module files) from coverage. These are structural and tested implicitly through integration and e2e tests. Focus coverage on services, guards, pipes, interceptors, and processors.

---

## 15. Docker Deployment

### Multi-Stage Dockerfile

```dockerfile
# Stage 1: Build
FROM node:22-alpine AS builder

WORKDIR /app

# Install dependencies first (layer caching)
COPY package.json package-lock.json ./
RUN npm ci --ignore-scripts

# Copy source and build
COPY tsconfig*.json nest-cli.json ./
COPY src/ ./src/
COPY prisma/ ./prisma/
RUN npx prisma generate
RUN npm run build

# Remove dev dependencies
RUN npm ci --omit=dev --ignore-scripts

# Stage 2: Production
FROM node:22-alpine AS production

# Security: run as non-root
RUN addgroup -g 1001 -S nestjs && \
    adduser -S nestjs -u 1001

WORKDIR /app

# Copy only production artifacts
COPY --from=builder --chown=nestjs:nestjs /app/dist ./dist
COPY --from=builder --chown=nestjs:nestjs /app/node_modules ./node_modules
COPY --from=builder --chown=nestjs:nestjs /app/package.json ./
COPY --from=builder --chown=nestjs:nestjs /app/prisma ./prisma

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
  CMD wget -qO- http://localhost:3000/api/v1/health || exit 1

USER nestjs

EXPOSE 3000

CMD ["node", "dist/main.js"]
```

### Docker Compose (Development)

```yaml
# docker-compose.yml
version: '3.9'

services:
  app:
    build:
      context: .
      target: builder
    command: npm run start:dev
    ports:
      - '3000:3000'
      - '9229:9229' # Debugger
    volumes:
      - .:/app
      - /app/node_modules
    environment:
      - NODE_ENV=development
      - DB_HOST=postgres
      - DB_PORT=5432
      - DB_USERNAME=app
      - DB_PASSWORD=app
      - DB_DATABASE=app_dev
      - REDIS_HOST=redis
      - REDIS_PORT=6379
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy

  postgres:
    image: postgres:15-alpine
    environment:
      POSTGRES_USER: app
      POSTGRES_PASSWORD: app
      POSTGRES_DB: app_dev
    ports:
      - '5432:5432'
    volumes:
      - postgres_data:/var/lib/postgresql/data
    healthcheck:
      test: ['CMD-SHELL', 'pg_isready -U app']
      interval: 5s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    ports:
      - '6379:6379'
    healthcheck:
      test: ['CMD', 'redis-cli', 'ping']
      interval: 5s
      timeout: 5s
      retries: 5

  redis-commander:
    image: rediscommander/redis-commander:latest
    environment:
      - REDIS_HOSTS=local:redis:6379
    ports:
      - '8081:8081'
    depends_on:
      - redis

volumes:
  postgres_data:
```

### Health Check Endpoint

```typescript
// modules/health/health.controller.ts
import { Controller, Get } from '@nestjs/common';
import {
  HealthCheck,
  HealthCheckService,
  TypeOrmHealthIndicator,
  MemoryHealthIndicator,
  DiskHealthIndicator,
} from '@nestjs/terminus';
import { Public } from '@common/decorators/public.decorator';
import { ApiTags, ApiOperation } from '@nestjs/swagger';

@ApiTags('health')
@Controller('health')
export class HealthController {
  constructor(
    private readonly health: HealthCheckService,
    private readonly db: TypeOrmHealthIndicator,
    private readonly memory: MemoryHealthIndicator,
    private readonly disk: DiskHealthIndicator,
  ) {}

  @Get()
  @Public()
  @HealthCheck()
  @ApiOperation({ summary: 'Application health check' })
  check() {
    return this.health.check([
      () => this.db.pingCheck('database'),
      () => this.memory.checkHeap('memory_heap', 200 * 1024 * 1024), // 200MB
      () =>
        this.disk.checkStorage('disk', {
          path: '/',
          thresholdPercent: 0.9, // 90%
        }),
    ]);
  }
}
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
        image: postgres:15
        env:
          POSTGRES_USER: test
          POSTGRES_PASSWORD: test
          POSTGRES_DB: app_test
        ports:
          - 5432:5432
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

      redis:
        image: redis:7
        ports:
          - 6379:6379
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5

    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-node@v4
        with:
          node-version: 22
          cache: 'npm'

      - run: npm ci

      - name: Lint
        run: npm run lint

      - name: Type check
        run: npx tsc --noEmit

      - name: Unit tests with coverage
        run: npm run test -- --coverage
        env:
          NODE_ENV: test

      - name: E2E tests
        run: npm run test:e2e
        env:
          NODE_ENV: test
          DB_HOST: localhost
          DB_PORT: 5432
          DB_USERNAME: test
          DB_PASSWORD: test
          DB_DATABASE: app_test
          REDIS_HOST: localhost
          REDIS_PORT: 6379
          JWT_ACCESS_SECRET: test-access-secret-at-least-32-chars!!
          JWT_REFRESH_SECRET: test-refresh-secret-at-least-32-chars!!

      - name: Build
        run: npm run build

  deploy:
    needs: test
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build and push Docker image
        run: |
          docker build -t ${{ secrets.REGISTRY }}/${{ github.repository }}:${{ github.sha }} .
          docker push ${{ secrets.REGISTRY }}/${{ github.repository }}:${{ github.sha }}

      # Deployment steps (Fly.io, AWS ECS, Kubernetes, etc.)
```

### Common Commands

```bash
# Development
npm run start:dev                  # Start with hot reload
npm run start:debug                # Start with debugger
npm run build                      # Compile TypeScript

# Testing
npm run test                       # Unit tests
npm run test:watch                 # Unit tests in watch mode
npm run test:cov                   # Unit tests with coverage
npm run test:e2e                   # E2E tests

# Linting
npm run lint                       # ESLint check
npm run lint -- --fix              # ESLint auto-fix
npm run format                     # Prettier format

# Database (TypeORM)
npx typeorm migration:generate -d src/database/data-source.ts src/database/migrations/Name
npx typeorm migration:run -d src/database/data-source.ts

# Database (Prisma)
npx prisma migrate dev --name name
npx prisma migrate deploy
npx prisma generate
npx prisma studio                  # Visual database browser

# Docker
docker compose up                  # Start all services
docker compose up -d               # Start detached
docker compose down                # Stop all services
docker compose logs -f app         # Tail app logs
```

---

## 16. Anti-Patterns (NestJS-specific)

| Anti-Pattern | Do This Instead |
|---|---|
| Business logic in controllers | Controllers are thin — delegate everything to services. Controllers validate input and return output. |
| Injecting repositories directly into controllers | Inject the service, which owns the repository. Controllers never talk to the database. |
| Using `any` type | Use proper TypeScript types. Infer from DTOs, entities, and Prisma generated types. |
| Missing `ValidationPipe` with `whitelist: true` | Always enable `whitelist` and `forbidNonWhitelisted` globally. Without it, clients can inject any property. |
| Importing services from another module directly | Import the module, not the service. Use `exports` to make services available. |
| Circular module dependencies | Extract shared logic into a new module. Use `forwardRef()` only as a last resort. |
| Using `synchronize: true` in production | Never. Use migrations. `synchronize` drops data without warning on schema changes. |
| Hardcoding secrets in code or config files | Use `@nestjs/config` with `ConfigService`. Validate required env vars with Joi at startup. |
| Missing `@Injectable()` on services | Every provider (service, guard, pipe, interceptor, strategy) needs `@Injectable()`. Without it, DI silently fails. |
| Registering guards/interceptors with `app.useGlobal*()` when they need DI | Use `APP_GUARD` / `APP_INTERCEPTOR` tokens in module providers. `useGlobal*()` runs outside the DI container. |
| Not using `ParseUUIDPipe` for UUID params | Always validate path parameters. A malformed UUID hitting your DB query wastes resources and leaks implementation. |
| Catching exceptions in controllers manually | Let NestJS exception filters handle it. Throw typed exceptions (`NotFoundException`, `ConflictException`) from services. |
| N+1 queries with TypeORM `find()` | Use `relations` option or `QueryBuilder` with `leftJoinAndSelect()`. Each lazy-loaded relation fires a separate query. |
| Raw SQL strings in services | Use QueryBuilder (TypeORM), Prisma Client, or Drizzle schema queries. Raw SQL bypasses type safety and is vulnerable to injection if not parameterized. |
| Missing `@ApiTags` and `@ApiOperation` on controllers | Always document every endpoint. Swagger is the API contract — undocumented endpoints are invisible to consumers. |
| Deploying without health check endpoint | Always include `@nestjs/terminus` health checks. Load balancers, orchestrators, and Docker need them. |
| Using `console.log` for logging | Use NestJS `Logger` service. It supports log levels, structured output, and can be swapped to production loggers (Winston, Pino). |
| Giant monolithic modules | Split by bounded context. One module per feature domain. A module with 20+ providers is a smell. |
| Missing error handling in Bull processors | Always implement `@OnWorkerEvent('failed')`. Unhandled job failures are invisible and silently dropped. |
| Using `setTimeout`/`setInterval` for scheduled tasks | Use `@nestjs/schedule` with `@Cron()` or Bull repeatable jobs. Native timers do not survive restarts or scale across instances. |
| Returning entity instances directly from controllers | Use response DTOs with `@Exclude()` to strip internal fields (passwords, tokens, soft-delete flags). |
| Missing CORS configuration | Always configure CORS explicitly. Default NestJS has no CORS — browsers block cross-origin requests silently. |
| Not validating query parameters | Apply `@Type(() => Number)` and validation decorators to query DTOs. Express delivers all query params as strings. |
| Storing auth tokens in localStorage | Use HTTP-only cookies for refresh tokens. Tokens in localStorage are vulnerable to XSS. |
| Not using transactions for multi-step mutations | Use TypeORM `QueryRunner` or Prisma `$transaction`. Partial writes on failure leave the database in an inconsistent state. |

---

## 17. Report Improvements

Found a missing pattern, incorrect advice, or a better way? File a GitHub issue:

**[Report a NestJS patterns improvement](https://github.com/trinsiklabs/cruxdev/issues/new?labels=patterns:nestjs&title=[NestJS]%20)**

Use the `patterns:nestjs` label. CruxDev's issue monitoring system picks these up, evaluates them, and updates this document. All improvements flow through the BIP (Build-in-Public) pipeline — accepted changes generate a blog post and X announcement.
