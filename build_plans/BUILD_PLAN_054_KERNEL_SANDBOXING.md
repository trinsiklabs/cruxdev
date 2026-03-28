# BUILD_PLAN_054: Kernel-Level Sandboxing

**Status:** CONVERGED
**Priority:** Nice to Have
**Competitor:** Codex (Apple Seatbelt on macOS, Landlock + seccomp on Linux)

## Context

Codex uses OS-kernel enforcement for process isolation. CruxDev's safety gates are rule-based (3 failures = rollback, 15-min timeout). For autonomous execution, kernel-level sandboxing would be a stronger safety guarantee.

## Phase 1: Research & Design

- [ ] 1.1 Research Apple Seatbelt API (sandbox_init, Scheme-like profile language)
- [ ] 1.2 Research Linux Landlock LSM + seccomp BPF
- [ ] 1.3 Design sandbox profile: deny network by default, read-only .git, read-write workspace only
- [ ] 1.4 Determine if Rust has good bindings (landlock crate, seccomp crate)

## Phase 2: macOS Seatbelt Integration

- [ ] 2.1 Implement Seatbelt sandbox profile for convergence subprocess execution
- [ ] 2.2 Deny network, deny write outside project dir, deny process execution outside allowlist
- [ ] 2.3 Opt-in via config (not default — requires testing)

## Phase 3: Linux Landlock Integration

- [ ] 3.1 Implement Landlock filesystem restrictions
- [ ] 3.2 Implement seccomp syscall filtering
- [ ] 3.3 Same profile as macOS: deny network, restrict filesystem

## Phase 4: Tests

- [ ] 4.1 Test: sandboxed process cannot write outside project dir
- [ ] 4.2 Test: sandboxed process cannot access network
- [ ] 4.3 Test: convergence works normally within sandbox

## Verification

```bash
cd rust && cargo test -- --nocapture
cd rust && cargo clippy -- -D warnings
```
