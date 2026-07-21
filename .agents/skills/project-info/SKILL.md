---
name: project-info
description: >-
  Canonical living memory and workflows for ai_agent, a Rust Cargo workspace
  for a modular Harness agent using local LLMs and Docker-isolated task
  execution. Use for all repository work, project explanations, architecture
  changes, and the pre-commit memory synchronization pass.
---

# ai_agent

Treat this file as the project's canonical living memory. Load it before any
repository work. Keep it concise and current: describe what is true now, not the
history of how it became true. Crate-specific details remain in each crate's
`AGENT.md` and must not be duplicated here unless they affect the workspace.

## Memory Synchronization

When asked to update project memory, especially before a commit:

1. Inspect `git status`, the complete staged and unstaged diff, root
   `Cargo.toml`, the crate list, and affected crates' `AGENT.md` files.
2. Verify every existing statement in this file against the repository.
3. Record durable project-wide facts: goals, current capabilities, workspace
   structure, architectural decisions, invariants, and established workflows.
4. Update or remove stale facts. Do not add plans, guesses, implementation
   trivia, temporary debugging state, commit summaries, or a changelog.
5. Keep crate-local facts in the relevant `AGENT.md`; update both files only
   when a crate change also affects project-wide memory.
6. Preserve user decisions that cannot be inferred from code unless the user
   explicitly changes them.
7. Report what memory changed and any unresolved contradiction. Do not commit.

## Product

An educational Rust project exploring local LLMs. Its target is a modular,
multi-service Harness agent that orchestrates LLMs and tools through an
observe → decide → act → update-state loop. Tasks run in isolated Docker
containers for safety and reproducibility.

Status: **early scaffold**; the workspace exists, but agent logic does not.

## Workspace

- Rust edition 2024, Cargo workspace, `resolver = "2"`
- Communicate with the user in **Russian**
- Current member: `crates/cli` (binary `cli`; currently prints `Hello, world!`)
- Run with `cargo run -p cli`
- Do not commit `target/`, secrets, or changes not explicitly requested

Each crate must contain `crates/<name>/AGENT.md` defining its purpose,
boundaries, dependencies, and key decisions. Read it before changing that crate
and update it when those facts change. A new crate is incomplete without one.

## Architecture

Keep responsibilities separated as the project grows:

| Crate | Responsibility |
|---|---|
| `cli` | CLI/TUI UX, arguments, REPL/IO, entry point |
| `core` (planned) | Agent loop, state, budgets, stopping policy |
| `tools` (planned) | Tool schemas, implementations, side effects |
| Other crates | Providers, memory, and other focused capabilities as needed |

Agent invariants:

- explicit stop conditions and step/token/wall-time budgets;
- tool failures are first-class outcomes, never silently retried;
- secrets and destructive actions require controls and cannot be exposed to
  arbitrary tool calls.

## Repository Rules

- The agent is a conversational advisor, not an implementation agent: it must
  never write or modify code and should answer questions as an experienced
  senior Rust developer.
- The agent may edit only Markdown (`.md`) files, and only when the user
  explicitly requests the edit.
- Create packages only under `crates/` and add them to root workspace members.
- Put shared dependencies in `[workspace.dependencies]`; reference them with
  `{ workspace = true }` in member crates.
- Prefer small, focused diffs; add documentation only when requested.
- Do not alter the workspace layout without discussing it with the user.
- Ask when product goals, the LLM stack, or tool scope are unspecified.
- Before edits, compare workspace members with actual crates and read every
  affected crate's `AGENT.md`.

For a new crate: create it under `crates/`, register it in the workspace, add
its `AGENT.md`, connect required path/workspace dependencies, then run
`cargo check -p <name>`.

Common checks: `cargo build`, `cargo test`, `cargo check -p cli`.
