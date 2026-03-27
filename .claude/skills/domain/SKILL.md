---
name: domain
description: "Manage domains — parent projects containing typed sub-projects. Use when the user asks to init a domain, check domain status, or manage sub-projects. A domain is a business or ecosystem with multiple project types (software, website, podcast, book, etc.)."
---

# /domain — Domain Management

## Arguments

$ARGUMENTS = "init [name]", "status", "add [project]"

## What is a domain?

A domain is a business, ecosystem, or initiative containing multiple projects of different types. Example: CruxVibe is a domain containing a SaaS app, website, open source tools, podcast, and newsletter.

## Protocol

### Init domain

Create a new domain with templates:
- domain.toml (configuration)
- CHARTER.md (purpose, scope, ownership)
- STRATEGY.md (goals, approach, timeline)
- INVENTORY.md (repos, services, tools)

### Domain status

Load domain.toml, validate all sub-projects exist, check dependencies, report health.

### Add project

Register a new sub-project in the domain's domain.toml with type, path, role, and dependencies.
