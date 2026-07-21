# Crate `cli`

## Purpose

The binary entry point of the workspace. It provides the agent UX: CLI arguments, interactive I/O (REPL/chat), and user-facing output.

Package: `cli` · run with: `cargo run -p cli`

## Responsibilities

**Does:**

- provides a TUI for interaction between the user and the agent.

**Does not:**

- interact with a database.

## Current State

Scaffold only: `src/main.rs` prints `Hello, world!`. There are no dependencies.

## Dependencies

None yet. When `core` or other crates are added, depend on them through a path (`{ path = "../…" }`) or workspace dependencies.

## Commands

```bash
cargo run -p cli
cargo check -p cli
cargo test -p cli
```
